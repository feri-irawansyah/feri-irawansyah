use leptos::prelude::*;
use leptos_meta::*;
use leptos_router::{
    components::{Route, Router, Routes},
    path,
};

use crate::pages::{
    home::HomePage,
    projects::ProjectsPage,
    not_found::NotFoundPage,
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
        <Stylesheet id="leptos" href="/pkg/views.css"/>
        <Title text="Feri Irawansyah — Full Stack Developer"/>
        <Meta name="description" content="Portfolio of Feri Irawansyah, Full Stack Developer"/>

        <Router>
            <main>
                <Routes fallback=|| view! { <NotFoundPage/> }>
                    <Route path=path!("/") view=HomePage/>
                    <Route path=path!("/projects") view=ProjectsPage/>
                </Routes>
            </main>
        </Router>
    }
}
