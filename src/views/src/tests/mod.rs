//! All `views` tests live here instead of next to their source files, so
//! `src/views/src/**` stays free of `_*_test.rs` siblings. Every test module
//! below only needs items visible crate-wide (`pub` or `pub(crate)`), since
//! this tree is a sibling of the modules under test, not a descendant.

mod admin_layout_test;
mod assets_test;
mod components_markdown_test;
mod components_skeleton_test;
mod markdown_cache_test;
mod markdown_circuit_test;
mod markdown_test;
mod notes_test;
