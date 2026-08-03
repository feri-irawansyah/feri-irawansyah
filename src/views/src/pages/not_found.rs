use crate::seo::Seo;
use leptos::prelude::*;
use leptos_meta::Meta;

#[allow(non_snake_case)]
#[component]
pub fn NotFoundPage() -> impl IntoView {
    #[cfg(feature = "ssr")]
    {
        if let Some(response) = use_context::<leptos_actix::ResponseOptions>() {
            response.set_status(actix_web::http::StatusCode::NOT_FOUND);
        }
    }

    view! {
        <Seo
            title="404 — Page Not Found · Feri Irawansyah"
            description="The page you're looking for doesn't exist."
            path="/404"
        />
        <Meta name="robots" content="noindex, nofollow"/>
        <div class="py-24 text-center">
            <h1 class="text-[6rem] font-extrabold text-line leading-none">"404"</h1>
            <p class="text-muted my-4 mb-8">"Page not found."</p>
            <a href="/"
                class="inline-flex items-center gap-1.5 px-[1.4rem] py-[0.6rem] border border-line text-muted hover:border-teal-500 hover:text-fg rounded text-[0.9rem] font-medium transition-colors">
                "Back to home"
            </a>
        </div>
    }
}
