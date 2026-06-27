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
                <link rel="stylesheet" href="/public/bi/bootstrap-icons.min.css"/>
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

    view! {
        <Stylesheet id="leptos" href="/pkg/feri-irawansyah.css"/>
        <Title text="Feri Irawansyah — Principal Engineer"/>
        <Meta name="description" content="Portfolio of Feri Irawansyah, Principal Engineer"/>

        <Router>
            // ── Overlay Sidebar ──────────────────────────────────────────────
            <aside class="fixed left-4 top-1/2 -translate-y-1/2 h-[80vh] flex flex-col items-start gap-2 z-50">

                // Logo — always violet pill
                <A href="/" attr:class="group/item inline-flex items-center h-12 rounded-full bg-violet-600 hover:bg-violet-500 transition-colors duration-200 px-3.5 hover:pr-5 no-underline mb-4">
                    <i class="bi bi-code-slash text-white text-[1.25rem] shrink-0"></i>
                    <span class="text-sm font-bold text-white whitespace-nowrap max-w-0 group-hover/item:max-w-[130px] overflow-hidden transition-all duration-200 group-hover/item:ml-2">
                        "Feri"
                    </span>
                </A>

                // Nav items
                <A href="/" attr:class="group/item inline-flex items-center h-12 rounded-full bg-line hover:bg-violet-600 [&.active]:bg-violet-600 transition-colors duration-200 px-3.5 hover:pr-5 [&.active]:pr-5 no-underline">
                    <i class="bi bi-house-fill text-muted group-hover/item:text-white text-[1.25rem] transition-colors duration-200 shrink-0"></i>
                    <span class="text-sm font-medium text-white whitespace-nowrap max-w-0 group-hover/item:max-w-[130px] overflow-hidden transition-all duration-200 group-hover/item:ml-2">
                        "Home"
                    </span>
                </A>

                <A href="/portfolio" attr:class="group/item inline-flex items-center h-12 rounded-full bg-line hover:bg-violet-600 [&.active]:bg-violet-600 transition-colors duration-200 px-3.5 hover:pr-5 [&.active]:pr-5 no-underline">
                    <i class="bi bi-grid-fill text-muted group-hover/item:text-white text-[1.25rem] transition-colors duration-200 shrink-0"></i>
                    <span class="text-sm font-medium text-white whitespace-nowrap max-w-0 group-hover/item:max-w-[130px] overflow-hidden transition-all duration-200 group-hover/item:ml-2">
                        "Portfolio"
                    </span>
                </A>

                <A href="/experience" attr:class="group/item inline-flex items-center h-12 rounded-full bg-line hover:bg-violet-600 [&.active]:bg-violet-600 transition-colors duration-200 px-3.5 hover:pr-5 [&.active]:pr-5 no-underline">
                    <i class="bi bi-person-workspace text-muted group-hover/item:text-white text-[1.25rem] transition-colors duration-200 shrink-0"></i>
                    <span class="text-sm font-medium text-white whitespace-nowrap max-w-0 group-hover/item:max-w-[130px] overflow-hidden transition-all duration-200 group-hover/item:ml-2">
                        "Experience"
                    </span>
                </A>

                <A href="/notes" attr:class="group/item inline-flex items-center h-12 rounded-full bg-line hover:bg-violet-600 [&.active]:bg-violet-600 transition-colors duration-200 px-3.5 hover:pr-5 [&.active]:pr-5 no-underline">
                    <i class="bi bi-journal-text text-muted group-hover/item:text-white text-[1.25rem] transition-colors duration-200 shrink-0"></i>
                    <span class="text-sm font-medium text-white whitespace-nowrap max-w-0 group-hover/item:max-w-[130px] overflow-hidden transition-all duration-200 group-hover/item:ml-2">
                        "Notes"
                    </span>
                </A>

                <A href="/skills" attr:class="group/item inline-flex items-center h-12 rounded-full bg-line hover:bg-violet-600 [&.active]:bg-violet-600 transition-colors duration-200 px-3.5 hover:pr-5 [&.active]:pr-5 no-underline">
                    <i class="bi bi-cpu text-muted group-hover/item:text-white text-[1.25rem] transition-colors duration-200 shrink-0"></i>
                    <span class="text-sm font-medium text-white whitespace-nowrap max-w-0 group-hover/item:max-w-[130px] overflow-hidden transition-all duration-200 group-hover/item:ml-2">
                        "Skills"
                    </span>
                </A>

                // Social links — pushed to bottom
                <div class="flex-1"></div>

                <a href="https://github.com/feri-irawansyah" target="_blank"
                    class="group/item inline-flex items-center h-12 rounded-full bg-line hover:bg-violet-600 transition-colors duration-200 px-3.5 hover:pr-5 no-underline">
                    <i class="bi bi-github text-muted group-hover/item:text-white text-[1.25rem] transition-colors duration-200 shrink-0"></i>
                    <span class="text-sm font-medium text-white whitespace-nowrap max-w-0 group-hover/item:max-w-[130px] overflow-hidden transition-all duration-200 group-hover/item:ml-2">
                        "GitHub"
                    </span>
                </a>

                <a href="https://linkedin.com/in/feri-irawansyah" target="_blank"
                    class="group/item inline-flex items-center h-12 rounded-full bg-line hover:bg-violet-600 transition-colors duration-200 px-3.5 hover:pr-5 no-underline">
                    <i class="bi bi-linkedin text-muted group-hover/item:text-white text-[1.25rem] transition-colors duration-200 shrink-0"></i>
                    <span class="text-sm font-medium text-white whitespace-nowrap max-w-0 group-hover/item:max-w-[130px] overflow-hidden transition-all duration-200 group-hover/item:ml-2">
                        "LinkedIn"
                    </span>
                </a>
            </aside>

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
