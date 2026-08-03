use std::collections::HashMap;

use chrono::{Datelike, Utc};
use typst::diag::{FileError, FileResult, SourceDiagnostic};
use typst::foundations::{Bytes, Datetime};
use typst::syntax::{FileId, Source, VirtualPath};
use typst::text::{Font, FontBook};
use typst::utils::LazyHash;
use typst::{Library, World};
use typst_kit::fonts::{FontSearcher, FontSlot};

/// Minimal [`typst::World`] backed entirely by in-memory sources/files — no filesystem access.
pub struct ReportWorld {
    library: LazyHash<Library>,
    book: LazyHash<FontBook>,
    fonts: Vec<FontSlot>,
    main: FileId,
    sources: HashMap<FileId, Source>,
    files: HashMap<FileId, Bytes>,
}

impl ReportWorld {
    /// `main_path` and `extra_files` paths are absolute virtual paths (e.g. `/cv.typ`, `/data.json`).
    pub fn new(main_path: &str, main_source: String, extra_files: Vec<(String, Bytes)>) -> Self {
        let main = FileId::new(None, VirtualPath::new(main_path));

        let mut sources = HashMap::new();
        sources.insert(main, Source::new(main, main_source));

        let mut files = HashMap::new();
        for (path, bytes) in extra_files {
            let id = FileId::new(None, VirtualPath::new(&path));
            files.insert(id, bytes);
        }

        let fonts = FontSearcher::new().search();

        Self {
            library: LazyHash::new(Library::default()),
            book: LazyHash::new(fonts.book),
            fonts: fonts.fonts,
            main,
            sources,
            files,
        }
    }
}

impl World for ReportWorld {
    fn library(&self) -> &LazyHash<Library> {
        &self.library
    }
    fn book(&self) -> &LazyHash<FontBook> {
        &self.book
    }
    fn main(&self) -> FileId {
        self.main
    }

    fn source(&self, id: FileId) -> FileResult<Source> {
        self.sources
            .get(&id)
            .cloned()
            .ok_or_else(|| FileError::NotFound(id.vpath().as_rootless_path().to_path_buf()))
    }

    fn file(&self, id: FileId) -> FileResult<Bytes> {
        self.files
            .get(&id)
            .cloned()
            .ok_or_else(|| FileError::NotFound(id.vpath().as_rootless_path().to_path_buf()))
    }

    fn font(&self, index: usize) -> Option<Font> {
        self.fonts.get(index)?.get()
    }

    fn today(&self, offset: Option<i64>) -> Option<Datetime> {
        let now = match offset {
            None => Utc::now(),
            Some(h) => Utc::now() + chrono::Duration::hours(h),
        };
        Datetime::from_ymd(now.year(), now.month() as u8, now.day() as u8)
    }
}

pub fn format_diagnostics(diags: &[SourceDiagnostic]) -> String {
    diags
        .iter()
        .map(|d| format!("[{:?}] {}", d.severity, d.message))
        .collect::<Vec<_>>()
        .join("\n")
}
