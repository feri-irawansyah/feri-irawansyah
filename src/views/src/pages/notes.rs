use leptos::prelude::*;
use leptos_router::hooks::use_params_map;
use modules::notes::NoteView;

#[server]
pub async fn get_all_notes() -> Result<Vec<NoteView>, ServerFnError> {
    use actix_web::web::Data;
    use leptos_actix::extract;
    use modules::notes::NoteRepository;
    use repositories::notes::NoteRepositoryImpl;
    let pool = extract::<Data<repositories::database::PgPool>>()
        .await
        .map_err(|e| ServerFnError::new(e.to_string()))?;
    NoteRepositoryImpl::new(pool.get_ref().clone())
        .find_all()
        .await
        .map_err(|e| ServerFnError::new(e.to_string()))
}

#[server]
pub async fn get_note_by_slug(slug: String) -> Result<Option<NoteView>, ServerFnError> {
    use actix_web::web::Data;
    use leptos_actix::extract;
    use modules::notes::NoteRepository;
    use repositories::notes::NoteRepositoryImpl;
    let pool = extract::<Data<repositories::database::PgPool>>()
        .await
        .map_err(|e| ServerFnError::new(e.to_string()))?;
    NoteRepositoryImpl::new(pool.get_ref().clone())
        .find_by_slug(&slug)
        .await
        .map_err(|e| ServerFnError::new(e.to_string()))
}

#[component]
pub fn NotesPage() -> impl IntoView {
    let notes = Resource::new(|| (), |_| get_all_notes());

    view! {
        <div class="py-4">
            <div class="max-w-5xl mx-auto px-6">
                <header class="py-12 pb-8">
                    <h1 class="text-[2.25rem] font-extrabold mb-2">"Notes"</h1>
                    <p class="text-muted text-[1.05rem]">
                        "Articles, learnings, and thoughts on software development."
                    </p>
                </header>

                <Suspense fallback=|| view! {
                    <div class="text-center text-muted py-8">"Loading notes..."</div>
                }>
                    {move || notes.get().map(|r| match r {
                        Ok(items) if items.is_empty() => view! {
                            <div class="text-center text-muted py-12">
                                <p>"No notes published yet."</p>
                            </div>
                        }.into_any(),
                        Ok(items) => view! {
                            <div class="flex flex-col">
                                {items.into_iter().map(|n| {
                                    let href = format!("/notes/{}", n.slug);
                                    let img_url = format!(
                                        "https://vjwknqthtunirowwtrvj.supabase.co/storage/v1/object/public/feri-irawansyah.my.id/assets/img/notes/{}.webp",
                                        n.slug
                                    );
                                    view! {
                                        <article class="py-8 border-b border-line first:border-t first:border-line">
                                            <a href=href.clone() class="flex gap-5 items-start group no-underline">
                                                // Thumbnail
                                                <div class="w-[140px] h-[90px] rounded-lg overflow-hidden shrink-0 bg-surface border border-line">
                                                    <img
                                                        src=img_url
                                                        alt=n.title.clone()
                                                        class="w-full h-full object-cover"
                                                        loading="lazy"
                                                        on:error=move |_e: leptos::ev::ErrorEvent| {
                                                            #[cfg(target_arch = "wasm32")]
                                                            {
                                                                use wasm_bindgen::JsCast;
                                                                if let Some(img) = _e.target()
                                                                    .and_then(|t| t.dyn_into::<web_sys::HtmlImageElement>().ok())
                                                                {
                                                                    img.set_src("https://vjwknqthtunirowwtrvj.supabase.co/storage/v1/object/public/feri-irawansyah.my.id/assets/img/notes/default.webp");
                                                                }
                                                            }
                                                        }
                                                    />
                                                </div>
                                                // Content
                                                <div class="flex-1 min-w-0">
                                                    <div class="flex gap-3 items-center mb-1.5">
                                                        <span class="text-xs font-semibold text-violet-400 uppercase tracking-[0.06em]">
                                                            {n.category.clone()}
                                                        </span>
                                                        <span class="text-xs text-muted">
                                                            {n.last_update.format("%d %b %Y").to_string()}
                                                        </span>
                                                    </div>
                                                    <h2 class="text-[1.05rem] font-bold mb-1.5 text-fg group-hover:text-violet-400 transition-colors leading-snug">
                                                        {n.title.clone()}
                                                    </h2>
                                                    <p class="text-[0.875rem] text-muted mb-2.5 line-clamp-2 leading-relaxed">
                                                        {n.description.clone()}
                                                    </p>
                                                    <div class="flex flex-wrap gap-1.5">
                                                        {n.hashtag.into_iter().take(4).map(|tag| view! {
                                                            <span class="text-xs px-2 py-0.5 rounded-full bg-line text-muted">
                                                                "#" {tag}
                                                            </span>
                                                        }).collect_view()}
                                                    </div>
                                                </div>
                                            </a>
                                        </article>
                                    }
                                }).collect_view()}
                            </div>
                        }.into_any(),
                        Err(e) => view! {
                            <p class="text-red-400 py-4">"Failed to load: " {e.to_string()}</p>
                        }.into_any(),
                    })}
                </Suspense>
            </div>
        </div>
    }
}

#[component]
pub fn NotePage() -> impl IntoView {
    let params = use_params_map();
    let slug = move || params.with(|p| p.get("slug").unwrap_or_default());
    let note = Resource::new(slug, get_note_by_slug);

    view! {
        <div class="py-4">
            <div class="max-w-[720px] mx-auto px-6">
                <Suspense fallback=|| view! {
                    <div class="text-center text-muted py-8">"Loading..."</div>
                }>
                    {move || note.get().map(|r| match r {
                        Ok(Some(n)) => {
                            let img_url = format!(
                                "https://vjwknqthtunirowwtrvj.supabase.co/storage/v1/object/public/feri-irawansyah.my.id/assets/img/notes/{}.webp",
                                n.slug
                            );
                            view! {
                            <article class="py-12">
                                // Cover image
                                <div class="w-full h-[280px] rounded-xl overflow-hidden mb-8 bg-surface border border-line">
                                    <img
                                        src=img_url
                                        alt=n.title.clone()
                                        class="w-full h-full object-cover"
                                        on:error=move |_e: leptos::ev::ErrorEvent| {
                                            #[cfg(target_arch = "wasm32")]
                                            {
                                                use wasm_bindgen::JsCast;
                                                if let Some(img) = _e.target()
                                                    .and_then(|t| t.dyn_into::<web_sys::HtmlImageElement>().ok())
                                                {
                                                    img.set_src("https://vjwknqthtunirowwtrvj.supabase.co/storage/v1/object/public/feri-irawansyah.my.id/assets/img/notes/default.webp");
                                                }
                                            }
                                        }
                                    />
                                </div>
                                <header class="mb-10 pb-8 border-b border-line">
                                    <div class="flex gap-4 items-center mb-3">
                                        <span class="text-xs font-semibold text-violet-400 uppercase tracking-[0.06em]">
                                            {n.category.clone()}
                                        </span>
                                        <span class="text-xs text-muted">
                                            {n.last_update.format("%d %B %Y").to_string()}
                                        </span>
                                    </div>
                                    <h1 class="text-3xl font-extrabold mb-2 leading-tight">{n.title}</h1>
                                    <p class="text-muted text-[1.05rem] mb-4">{n.description}</p>
                                    <div class="flex flex-wrap gap-1.5">
                                        {n.hashtag.into_iter().map(|tag| view! {
                                            <span class="text-xs px-2.5 py-0.5 rounded-full bg-line text-muted">
                                                "#" {tag}
                                            </span>
                                        }).collect_view()}
                                    </div>
                                </header>
                                <div class="article-content">
                                    {n.content}
                                </div>
                            </article>
                        }.into_any()},
                        Ok(None) => view! {
                            <div class="py-24 text-center">
                                <h1 class="text-[6rem] font-extrabold text-line leading-none">"404"</h1>
                                <p class="text-muted my-4 mb-8">"Note not found."</p>
                                <a href="/notes"
                                    class="inline-flex items-center gap-1.5 px-[1.4rem] py-[0.6rem] border border-line text-muted hover:border-indigo-500 hover:text-fg rounded text-[0.9rem] font-medium transition-colors">
                                    "← Back to Notes"
                                </a>
                            </div>
                        }.into_any(),
                        Err(e) => view! {
                            <p class="text-red-400 py-4">{e.to_string()}</p>
                        }.into_any(),
                    })}
                </Suspense>
            </div>
        </div>
    }
}
