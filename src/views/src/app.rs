use leptos::prelude::*;
use leptos_meta::*;
use leptos_router::{
    components::{A, Route, Router, Routes},
    path,
};

use crate::pages::{
    experience::ExperiencePage,
    home::HomePage,
    not_found::NotFoundPage,
    notes::{NotePage, NotesPage},
    portfolio::PortfolioPage,
    skills::SkillsPage,
};

pub fn shell(options: LeptosOptions) -> impl IntoView {
    view! {
        <!DOCTYPE html>
        <html lang="en">
            <head>
                <meta charset="utf-8"/>
                <meta name="viewport" content="width=device-width, initial-scale=1"/>
                <link rel="icon" href="/public/favicon.ico" type="image/x-icon"/>
                <link rel="stylesheet" href="/public/bi/bootstrap-icons.min.css"/>
                <script inner_html="(function(){document.documentElement.classList.toggle('dark',localStorage.theme==='dark'||(!('theme' in localStorage)&&window.matchMedia('(prefers-color-scheme: dark)').matches))})()"></script>
                <AutoReload options=options.clone()/>
                <HydrationScripts options/>
                <MetaTags/>
            </head>
            <body>
                <App/>
            </body>
        </html>
    }
}

#[component]
pub fn App() -> impl IntoView {
    provide_meta_context();

    let (is_dark, set_is_dark) = signal(false);

    // Initialize signal from localStorage, fallback to system preference
    Effect::new(move |_| {
        #[cfg(target_arch = "wasm32")]
        {
            use leptos::web_sys;
            let stored = web_sys::window()
                .and_then(|w| w.local_storage().ok())
                .flatten()
                .and_then(|s| s.get_item("theme").ok())
                .flatten();
            let dark = match stored.as_deref() {
                Some("dark") => true,
                Some("light") => false,
                _ => web_sys::window()
                    .and_then(|w| w.match_media("(prefers-color-scheme: dark)").ok())
                    .flatten()
                    .map(|mql| mql.matches())
                    .unwrap_or(false),
            };
            set_is_dark.set(dark);
        }
    });

    // Apply .dark class to <html> and persist to localStorage
    Effect::new(move |_| {
        let _dark = is_dark.get();
        #[cfg(target_arch = "wasm32")]
        let dark = _dark;
        #[cfg(target_arch = "wasm32")]
        {
            use leptos::web_sys;
            if let Some(doc) = web_sys::window().and_then(|w| w.document()) {
                if let Some(html) = doc.document_element() {
                    if dark {
                        let _ = html.class_list().add_1("dark");
                    } else {
                        let _ = html.class_list().remove_1("dark");
                    }
                }
            }
            if let Some(storage) = web_sys::window()
                .and_then(|w| w.local_storage().ok())
                .flatten()
            {
                let _ = storage.set_item("theme", if dark { "dark" } else { "light" });
            }
        }
    });

    view! {
        <Stylesheet id="leptos" href="/pkg/feri-irawansyah.css"/>
        <Title text="Feri Irawansyah — Principal Engineer"/>
        <Meta name="description" content="Portfolio of Feri Irawansyah, Principal Engineer"/>

        <Router>
            // ── Left Sidebar ─────────────────────────────────────────────────
            <aside class="fixed left-4 top-1/2 -translate-y-1/2 h-[80vh] flex flex-col items-start justify-center gap-3 z-50">
                <A href="/" attr:class="group/item inline-flex items-center h-12 rounded-full bg-line hover:bg-teal-600 [&.active]:bg-teal-500 transition-colors duration-200 px-3.5 hover:pr-5 [&.active]:pr-5 no-underline">
                    <i class="bi bi-house-fill text-muted group-hover/item:text-white text-[1.25rem] transition-colors duration-200 shrink-0"></i>
                    <span class="text-sm font-medium text-white whitespace-nowrap max-w-0 group-hover/item:max-w-[130px] overflow-hidden transition-all duration-200 group-hover/item:ml-2">
                        "Home"
                    </span>
                </A>
                <A href="/portfolio" attr:class="group/item inline-flex items-center h-12 rounded-full bg-line hover:bg-teal-600 [&.active]:bg-teal-500 transition-colors duration-200 px-3.5 hover:pr-5 [&.active]:pr-5 no-underline">
                    <i class="bi bi-grid-fill text-muted group-hover/item:text-white text-[1.25rem] transition-colors duration-200 shrink-0"></i>
                    <span class="text-sm font-medium text-white whitespace-nowrap max-w-0 group-hover/item:max-w-[130px] overflow-hidden transition-all duration-200 group-hover/item:ml-2">
                        "Portfolio"
                    </span>
                </A>
                <A href="/experience" attr:class="group/item inline-flex items-center h-12 rounded-full bg-line hover:bg-teal-600 [&.active]:bg-teal-500 transition-colors duration-200 px-3.5 hover:pr-5 [&.active]:pr-5 no-underline">
                    <i class="bi bi-person-workspace text-muted group-hover/item:text-white text-[1.25rem] transition-colors duration-200 shrink-0"></i>
                    <span class="text-sm font-medium text-white whitespace-nowrap max-w-0 group-hover/item:max-w-[130px] overflow-hidden transition-all duration-200 group-hover/item:ml-2">
                        "Experience"
                    </span>
                </A>
                <A href="/notes" attr:class="group/item inline-flex items-center h-12 rounded-full bg-line hover:bg-teal-600 [&.active]:bg-teal-500 transition-colors duration-200 px-3.5 hover:pr-5 [&.active]:pr-5 no-underline">
                    <i class="bi bi-journal-text text-muted group-hover/item:text-white text-[1.25rem] transition-colors duration-200 shrink-0"></i>
                    <span class="text-sm font-medium text-white whitespace-nowrap max-w-0 group-hover/item:max-w-[130px] overflow-hidden transition-all duration-200 group-hover/item:ml-2">
                        "Notes"
                    </span>
                </A>
                <A href="/skills" attr:class="group/item inline-flex items-center h-12 rounded-full bg-line hover:bg-teal-600 [&.active]:bg-teal-500 transition-colors duration-200 px-3.5 hover:pr-5 [&.active]:pr-5 no-underline">
                    <i class="bi bi-cpu text-muted group-hover/item:text-white text-[1.25rem] transition-colors duration-200 shrink-0"></i>
                    <span class="text-sm font-medium text-white whitespace-nowrap max-w-0 group-hover/item:max-w-[130px] overflow-hidden transition-all duration-200 group-hover/item:ml-2">
                        "Skills"
                    </span>
                </A>
            </aside>

            // ── Top Right: Social + Dark Mode Toggle ─────────────────────────
            <div class="fixed top-4 right-4 flex items-center gap-2 z-50">
                <a href="https://github.com/feri-irawansyah" target="_blank"
                    class="w-9 h-9 rounded-full bg-line flex items-center justify-center text-muted hover:bg-teal-600 hover:text-white transition-colors duration-200 no-underline">
                    <i class="bi bi-github text-[1.05rem]"></i>
                </a>
                <a href="https://linkedin.com/in/feri-irawansyah" target="_blank"
                    class="w-9 h-9 rounded-full bg-line flex items-center justify-center text-muted hover:bg-teal-600 hover:text-white transition-colors duration-200 no-underline">
                    <i class="bi bi-linkedin text-[1.05rem]"></i>
                </a>
                <button
                    on:click=move |_| set_is_dark.update(|d| *d = !*d)
                    class="w-9 h-9 rounded-full bg-line flex items-center justify-center text-muted hover:bg-teal-600 hover:text-white transition-colors duration-200 cursor-pointer">
                    {move || if is_dark.get() {
                        view! { <i class="bi bi-sun-fill text-[1.05rem]"></i> }.into_any()
                    } else {
                        view! { <i class="bi bi-moon-fill text-[1.05rem]"></i> }.into_any()
                    }}
                </button>
            </div>

            // ── Main content (full width, sidebar is overlay) ────────────────
            <div class="flex flex-col min-h-screen">
                <main class="flex-1">
                    <Routes fallback=|| view! { <NotFoundPage/> }>
                        <Route path=path!("/")              view=HomePage/>
                        <Route path=path!("/portfolio")     view=PortfolioPage/>
                        <Route path=path!("/experience")    view=ExperiencePage/>
                        <Route path=path!("/notes")         view=NotesPage/>
                        <Route path=path!("/notes/:slug")   view=NotePage/>
                        <Route path=path!("/skills")        view=SkillsPage/>
                    </Routes>
                </main>

                <footer class="border-t border-line py-8 text-center text-muted text-sm">
                    <p>"© 2026 Feri Irawansyah. Built with Rust + Leptos."</p>
                </footer>
            </div>
        </Router>
    }
}
