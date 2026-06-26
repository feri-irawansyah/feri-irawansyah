use leptos::prelude::*;
use leptos_router::hooks::use_params_map;
use modules::notes::NoteView;

#[server]
pub async fn get_all_notes() -> Result<Vec<NoteView>, ServerFnError> {
    use actix_web::web::Data;
    use leptos_actix::extract;
    use modules::notes::NoteRepository;
    use repositories::notes::NoteRepositoryImpl;
    let pool = extract::<Data<repositories::database::PgPool>>().await
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
    let pool = extract::<Data<repositories::database::PgPool>>().await
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
        <div class="page-notes">
            <div class="container">
                <header class="page-header">
                    <h1>"Notes"</h1>
                    <p>"Articles, learnings, and thoughts on software development."</p>
                </header>

                <Suspense fallback=|| view! { <div class="loading">"Loading notes..."</div> }>
                    {move || notes.get().map(|r| match r {
                        Ok(items) if items.is_empty() => view! {
                            <div class="empty-state">
                                <p>"No notes published yet."</p>
                            </div>
                        }.into_any(),
                        Ok(items) => view! {
                            <div class="note-list">
                                {items.into_iter().map(|n| {
                                    let href = format!("/notes/{}", n.slug);
                                    view! {
                                        <article class="note-item">
                                            <div class="note-meta">
                                                <span class="note-category">{n.category.clone()}</span>
                                                <span class="note-date">
                                                    {n.last_update.format("%B %d, %Y").to_string()}
                                                </span>
                                            </div>
                                            <h2 class="note-title">
                                                <a href=href>{n.title}</a>
                                            </h2>
                                            <p class="note-desc">{n.description}</p>
                                            <div class="note-tags">
                                                {n.hashtag.into_iter().map(|tag| view! {
                                                    <span class="tag">"#" {tag}</span>
                                                }).collect_view()}
                                            </div>
                                        </article>
                                    }
                                }).collect_view()}
                            </div>
                        }.into_any(),
                        Err(e) => view! {
                            <p class="error-state">"Failed to load: " {e.to_string()}</p>
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
    let slug   = move || params.with(|p| p.get("slug").unwrap_or_default());
    let note   = Resource::new(slug, |s| get_note_by_slug(s));

    view! {
        <div class="page-note">
            <div class="container container-sm">
                <Suspense fallback=|| view! { <div class="loading">"Loading..."</div> }>
                    {move || note.get().map(|r| match r {
                        Ok(Some(n)) => view! {
                            <article class="note-article">
                                <header class="article-header">
                                    <div class="note-meta">
                                        <span class="note-category">{n.category.clone()}</span>
                                        <span class="note-date">
                                            {n.last_update.format("%B %d, %Y").to_string()}
                                        </span>
                                    </div>
                                    <h1 class="article-title">{n.title}</h1>
                                    <p class="article-desc">{n.description}</p>
                                    <div class="note-tags">
                                        {n.hashtag.into_iter().map(|tag| view! {
                                            <span class="tag">"#" {tag}</span>
                                        }).collect_view()}
                                    </div>
                                </header>
                                <div class="article-content">
                                    {n.content}
                                </div>
                            </article>
                        }.into_any(),
                        Ok(None) => view! {
                            <div class="not-found">
                                <h1>"404"</h1>
                                <p>"Note not found."</p>
                                <a href="/notes" class="btn btn-outline">"← Back to Notes"</a>
                            </div>
                        }.into_any(),
                        Err(e) => view! {
                            <p class="error-state">{e.to_string()}</p>
                        }.into_any(),
                    })}
                </Suspense>
            </div>
        </div>
    }
}
