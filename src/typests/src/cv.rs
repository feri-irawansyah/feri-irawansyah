use crate::world::{ReportWorld, format_diagnostics};
use std::sync::LazyLock;
use typst::foundations::Bytes;

type CvOutput = (Vec<u8>, Vec<String>);

const TEMPLATE: &str = include_str!("../templates/feri-irawansyah.typ");

const ICON_ENVELOPE: &[u8] = include_bytes!("../templates/svg/envelope.svg");
const ICON_PHONE: &[u8] = include_bytes!("../templates/svg/telephone-inbound.svg");
const ICON_GITHUB: &[u8] = include_bytes!("../templates/svg/github.svg");
const ICON_GEO: &[u8] = include_bytes!("../templates/svg/geo-fill.svg");
const ICON_GLOBE: &[u8] = include_bytes!("../templates/svg/globe-check.svg");
const PHOTO: &[u8] = include_bytes!("../templates/img/feri-irawansyah.jpg");

/// Compiled once on first access, cached for the lifetime of the process.
static RENDERED: LazyLock<Result<CvOutput, String>> = LazyLock::new(compile);

fn compile() -> Result<CvOutput, String> {
    let extra_files = vec![
        (
            "svg/envelope.svg".to_string(),
            Bytes::new(ICON_ENVELOPE.to_vec()),
        ),
        (
            "svg/telephone-inbound.svg".to_string(),
            Bytes::new(ICON_PHONE.to_vec()),
        ),
        (
            "svg/github.svg".to_string(),
            Bytes::new(ICON_GITHUB.to_vec()),
        ),
        (
            "svg/geo-fill.svg".to_string(),
            Bytes::new(ICON_GEO.to_vec()),
        ),
        (
            "svg/globe-check.svg".to_string(),
            Bytes::new(ICON_GLOBE.to_vec()),
        ),
        (
            "feri-irawansyah.jpg".to_string(),
            Bytes::new(PHOTO.to_vec()),
        ),
    ];

    let world = ReportWorld::new("/feri-irawansyah.typ", TEMPLATE.to_string(), extra_files);
    let compiled = typst::compile(&world);
    let doc = compiled.output.map_err(|d| format_diagnostics(&d))?;

    let pdf = typst_pdf::pdf(&doc, &typst_pdf::PdfOptions::default())
        .map_err(|d| format_diagnostics(&d))?;

    let svgs: Vec<String> = doc.pages.iter().map(typst_svg::svg).collect();

    Ok((pdf, svgs))
}

/// Returns the compiled CV as PDF bytes.
pub fn cv_pdf() -> Result<&'static [u8], String> {
    RENDERED
        .as_ref()
        .map(|(pdf, _)| pdf.as_slice())
        .map_err(Clone::clone)
}

/// Returns the SVG string for the given page (0-indexed).
pub fn cv_svg(page: usize) -> Result<&'static str, String> {
    RENDERED
        .as_ref()
        .map_err(Clone::clone)
        .and_then(|(_, svgs)| {
            svgs.get(page)
                .map(|s| s.as_str())
                .ok_or_else(|| format!("Page {page} not found"))
        })
}

pub fn cv_page_count() -> Result<usize, String> {
    RENDERED
        .as_ref()
        .map(|(_, svgs)| svgs.len())
        .map_err(Clone::clone)
}
