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
};

pub fn shell(options: LeptosOptions) -> impl IntoView {
    view! {
        <!DOCTYPE html>
        <html lang="en">
            <head>
                <meta charset="utf-8"/>
                <meta name="viewport" content="width=device-width, initial-scale=1"/>
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
            <header class="navbar">
                <A href="/" attr:class="nav-brand">"Feri Irawansyah"</A>
                <nav class="nav-links">
                    <A href="/portfolio">"Portfolio"</A>
                    <A href="/experience">"Experience"</A>
                    <A href="/notes">"Notes"</A>
                </nav>
            </header>

            <main class="main-content">
                <Routes fallback=|| view! { <NotFoundPage/> }>
                    <Route path=path!("/")              view=HomePage/>
                    <Route path=path!("/portfolio")     view=PortfolioPage/>
                    <Route path=path!("/experience")    view=ExperiencePage/>
                    <Route path=path!("/notes")         view=NotesPage/>
                    <Route path=path!("/notes/:slug")   view=NotePage/>
                </Routes>
            </main>

            <footer class="footer">
                <div class="container">
                    <p>"© 2026 Feri Irawansyah. Built with Rust + Leptos."</p>
                </div>
            </footer>
        </Router>
    }
}
