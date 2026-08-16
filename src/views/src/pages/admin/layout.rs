use leptos::prelude::*;
use leptos_router::{NavigateOptions, components::A, hooks::use_location, hooks::use_navigate};

#[server]
pub async fn logout_action() -> Result<(), ServerFnError> {
    use actix_web::{
        HttpRequest,
        http::header::{HeaderValue, SET_COOKIE},
        web::Data,
    };
    use leptos_actix::{ResponseOptions, extract};
    use std::sync::Arc;

    let req = extract::<HttpRequest>()
        .await
        .map_err(|e| ServerFnError::new(e.to_string()))?;
    let auth_svc = extract::<Data<Arc<dyn modules::auth::AuthService>>>()
        .await
        .map_err(|e| ServerFnError::new(e.to_string()))?;
    let leptos_options = extract::<Data<leptos::config::LeptosOptions>>()
        .await
        .map_err(|e| ServerFnError::new(e.to_string()))?;
    let secure = if leptos_options.env == leptos::config::Env::PROD {
        "; Secure"
    } else {
        ""
    };

    if let Some(cookie) = req.cookie("refresh_token") {
        let _ = auth_svc.logout(cookie.value()).await;
    }

    let response = use_context::<ResponseOptions>()
        .ok_or_else(|| ServerFnError::new("No response context"))?;

    response.append_header(
        SET_COOKIE,
        HeaderValue::from_str(&format!(
            "access_token=; HttpOnly; SameSite=Strict; Path=/; Max-Age=0{secure}"
        ))
        .unwrap(),
    );
    response.append_header(
        SET_COOKIE,
        HeaderValue::from_str(&format!(
            "refresh_token=; HttpOnly; SameSite=Strict; Path=/; Max-Age=0{secure}"
        ))
        .unwrap(),
    );

    Ok(())
}

#[allow(non_snake_case)]
#[component]
pub fn AdminLayout(children: Children) -> impl IntoView {
    let logout = ServerAction::<LogoutAction>::new();
    let navigate = use_navigate();
    let pathname = use_location().pathname;

    let (is_dark, set_is_dark) =
        use_context::<(ReadSignal<bool>, WriteSignal<bool>)>().expect("dark mode context missing");

    Effect::new(move |_| {
        if logout
            .value()
            .with(|v| v.as_ref().map(|r| r.is_ok()).unwrap_or(false))
        {
            navigate("/admin/login", NavigateOptions::default());
        }
    });

    let page_title = move || {
        let p = pathname.get();
        if p.starts_with("/admin/users") {
            ("Manajemen User", "bi-people-fill")
        } else if p.starts_with("/admin/experience") {
            ("Experience", "bi-person-workspace")
        } else if p.starts_with("/admin/portfolio") {
            ("Portfolio", "bi-grid-fill")
        } else if p.starts_with("/admin/notes") {
            ("Notes", "bi-journal-text")
        } else if p.starts_with("/admin/skills") {
            ("Skills", "bi-cpu-fill")
        } else if p.starts_with("/admin/laboratory") {
            ("Laboratorium", "bi-flask")
        } else if p.starts_with("/admin/cache") {
            ("Cache Management", "bi-server")
        } else if p.starts_with("/admin/logs") {
            ("Log Management", "bi-file-text")
        } else {
            ("Dashboard", "bi-speedometer2")
        }
    };

    view! {
        // Fixed sidebar
        <aside class="fixed left-0 top-0 h-screen w-60 z-40 bg-surface border-r border-line border-t-2 border-t-teal-500 flex flex-col">

            // Brand
            <div class="h-18 flex items-center px-5 border-b border-line shrink-0">
                <div class="flex items-center gap-3">
                    <div class="w-8 h-8 rounded-lg bg-teal-500/15 flex items-center justify-center shrink-0">
                        <i class="bi bi-terminal-fill text-teal-500 text-sm"></i>
                    </div>
                    <div>
                        <p class="text-sm font-bold text-fg leading-none">"Feri Irawansyah"</p>
                        <p class="text-[10px] text-teal-500 font-semibold uppercase tracking-widest mt-0.5">"Admin Panel"</p>
                    </div>
                </div>
            </div>

            // Nav links
            <nav class="flex-1 p-3 flex flex-col gap-0.5 overflow-y-auto">
                <p class="text-[10px] font-semibold text-muted uppercase tracking-widest px-3 pt-3 pb-2">"Navigasi"</p>

                <A
                    href="/admin"
                    attr:class="flex items-center gap-3 px-3 py-2.5 rounded-xl text-sm font-medium text-muted hover:text-fg hover:bg-teal-500/10 [&.active]:bg-teal-500/15 [&.active]:text-teal-400 transition-colors no-underline group"
                >
                    <span class="w-7 h-7 rounded-lg bg-line group-[.active]:bg-teal-500/20 group-hover:bg-teal-500/10 flex items-center justify-center shrink-0 transition-colors">
                        <i class="bi bi-speedometer2 text-sm"></i>
                    </span>
                    "Dashboard"
                </A>

                <A
                    href="/admin/users"
                    attr:class="flex items-center gap-3 px-3 py-2.5 rounded-xl text-sm font-medium text-muted hover:text-fg hover:bg-teal-500/10 [&.active]:bg-teal-500/15 [&.active]:text-teal-400 transition-colors no-underline group"
                >
                    <span class="w-7 h-7 rounded-lg bg-line group-[.active]:bg-teal-500/20 group-hover:bg-teal-500/10 flex items-center justify-center shrink-0 transition-colors">
                        <i class="bi bi-people text-sm"></i>
                    </span>
                    "Users"
                </A>

                <A
                    href="/admin/experience"
                    attr:class="flex items-center gap-3 px-3 py-2.5 rounded-xl text-sm font-medium text-muted hover:text-fg hover:bg-teal-500/10 [&.active]:bg-teal-500/15 [&.active]:text-teal-400 transition-colors no-underline group"
                >
                    <span class="w-7 h-7 rounded-lg bg-line group-[.active]:bg-teal-500/20 group-hover:bg-teal-500/10 flex items-center justify-center shrink-0 transition-colors">
                        <i class="bi bi-person-workspace text-sm"></i>
                    </span>
                    "Experience"
                </A>

                <A
                    href="/admin/portfolio"
                    attr:class="flex items-center gap-3 px-3 py-2.5 rounded-xl text-sm font-medium text-muted hover:text-fg hover:bg-teal-500/10 [&.active]:bg-teal-500/15 [&.active]:text-teal-400 transition-colors no-underline group"
                >
                    <span class="w-7 h-7 rounded-lg bg-line group-[.active]:bg-teal-500/20 group-hover:bg-teal-500/10 flex items-center justify-center shrink-0 transition-colors">
                        <i class="bi bi-grid-fill text-sm"></i>
                    </span>
                    "Portfolio"
                </A>

                <A
                    href="/admin/notes"
                    attr:class="flex items-center gap-3 px-3 py-2.5 rounded-xl text-sm font-medium text-muted hover:text-fg hover:bg-teal-500/10 [&.active]:bg-teal-500/15 [&.active]:text-teal-400 transition-colors no-underline group"
                >
                    <span class="w-7 h-7 rounded-lg bg-line group-[.active]:bg-teal-500/20 group-hover:bg-teal-500/10 flex items-center justify-center shrink-0 transition-colors">
                        <i class="bi bi-journal-text text-sm"></i>
                    </span>
                    "Notes"
                </A>

                <A
                    href="/admin/skills"
                    attr:class="flex items-center gap-3 px-3 py-2.5 rounded-xl text-sm font-medium text-muted hover:text-fg hover:bg-teal-500/10 [&.active]:bg-teal-500/15 [&.active]:text-teal-400 transition-colors no-underline group"
                >
                    <span class="w-7 h-7 rounded-lg bg-line group-[.active]:bg-teal-500/20 group-hover:bg-teal-500/10 flex items-center justify-center shrink-0 transition-colors">
                        <i class="bi bi-cpu-fill text-sm"></i>
                    </span>
                    "Skills"
                </A>

                <A
                    href="/admin/laboratory"
                    attr:class="flex items-center gap-3 px-3 py-2.5 rounded-xl text-sm font-medium text-muted hover:text-fg hover:bg-teal-500/10 [&.active]:bg-teal-500/15 [&.active]:text-teal-400 transition-colors no-underline group"
                >
                    <span class="w-7 h-7 rounded-lg bg-line group-[.active]:bg-teal-500/20 group-hover:bg-teal-500/10 flex items-center justify-center shrink-0 transition-colors">
                        <i class="bi bi-flask text-sm"></i>
                    </span>
                    "Laboratorium"
                </A>

                <p class="text-[10px] font-semibold text-muted uppercase tracking-widest px-3 pt-4 pb-2">"System"</p>

                <A
                    href="/admin/cache"
                    attr:class="flex items-center gap-3 px-3 py-2.5 rounded-xl text-sm font-medium text-muted hover:text-fg hover:bg-teal-500/10 [&.active]:bg-teal-500/15 [&.active]:text-teal-400 transition-colors no-underline group"
                >
                    <span class="w-7 h-7 rounded-lg bg-line group-[.active]:bg-teal-500/20 group-hover:bg-teal-500/10 flex items-center justify-center shrink-0 transition-colors">
                        <i class="bi bi-server text-sm"></i>
                    </span>
                    "Cache"
                </A>

                <A
                    href="/admin/logs"
                    attr:class="flex items-center gap-3 px-3 py-2.5 rounded-xl text-sm font-medium text-muted hover:text-fg hover:bg-teal-500/10 [&.active]:bg-teal-500/15 [&.active]:text-teal-400 transition-colors no-underline group"
                >
                    <span class="w-7 h-7 rounded-lg bg-line group-[.active]:bg-teal-500/20 group-hover:bg-teal-500/10 flex items-center justify-center shrink-0 transition-colors">
                        <i class="bi bi-file-text text-sm"></i>
                    </span>
                    "Logs"
                </A>
            </nav>

            // Bottom actions
            <div class="p-3 border-t border-line shrink-0">
                <a
                    href="/"
                    class="flex items-center gap-3 px-3 py-2.5 rounded-xl text-sm font-medium text-muted hover:text-fg hover:bg-teal-500/10 transition-colors no-underline group mb-0.5"
                >
                    <span class="w-7 h-7 rounded-lg bg-line group-hover:bg-teal-500/10 flex items-center justify-center shrink-0 transition-colors">
                        <i class="bi bi-globe text-sm"></i>
                    </span>
                    "Ke Website"
                </a>
                <ActionForm action=logout>
                    <button
                        type="submit"
                        disabled=move || logout.pending().get()
                        class="w-full flex items-center gap-3 px-3 py-2.5 rounded-xl text-sm font-medium text-muted hover:text-red-400 hover:bg-red-500/10 transition-colors cursor-pointer disabled:opacity-50 group"
                    >
                        <span class="w-7 h-7 rounded-lg bg-line group-hover:bg-red-500/10 flex items-center justify-center shrink-0 transition-colors">
                            <i class="bi bi-box-arrow-right text-sm"></i>
                        </span>
                        {move || if logout.pending().get() { "Keluar..." } else { "Logout" }}
                    </button>
                </ActionForm>
            </div>
        </aside>

        // Content area
        <div class="ml-60 flex flex-col min-h-screen">
            // Sticky header
            <header class="sticky top-0 z-30 bg-surface/80 backdrop-blur-sm border-b border-line border-t-2 border-t-teal-500 px-6 h-18 flex items-center justify-between shrink-0">
                <div class="flex items-center gap-2.5">
                    <span class="w-6 h-6 rounded-md bg-teal-500/15 flex items-center justify-center">
                        <i class={move || format!("bi {} text-teal-500 text-xs", page_title().1)}></i>
                    </span>
                    <h1 class="text-sm font-semibold text-fg">{move || page_title().0}</h1>
                </div>

                <div class="flex items-center gap-2">
                    <button
                        on:click=move |_| set_is_dark.update(|d| *d = !*d)
                        aria-label=move || if is_dark.get() { "Switch to light mode" } else { "Switch to dark mode" }
                        class="w-8 h-8 rounded-lg bg-teal-500/10 flex items-center justify-center text-teal-500 hover:bg-teal-500/20 transition-colors duration-200 cursor-pointer"
                    >
                        {move || if is_dark.get() {
                            view! { <i class="bi bi-sun-fill text-sm"></i> }.into_any()
                        } else {
                            view! { <i class="bi bi-moon-fill text-sm"></i> }.into_any()
                        }}
                    </button>
                </div>
            </header>

            // Page content
            <main class="flex-1">
                {children()}
            </main>
        </div>
    }
}

/// Error row for admin CRUD tables — spans all columns, shows message + reload button.
#[allow(non_snake_case)]
#[component]
pub fn TableErrorRow(#[prop(default = 5)] cols: usize, message: String) -> impl IntoView {
    view! {
        <tr>
            <td colspan={cols.to_string()} class="px-5 py-10 text-center">
                <div class="flex flex-col items-center gap-3">
                    <div class="w-8 h-8 rounded-lg bg-red-500/15 flex items-center justify-center">
                        <i class="bi bi-exclamation-triangle-fill text-red-400 text-sm"></i>
                    </div>
                    <p class="text-red-400 text-xs max-w-xs">{message}</p>
                    <button
                        type="button"
                        on:click=|_| {
                            #[cfg(target_arch = "wasm32")]
                            { let _ = leptos::web_sys::window().map(|w| w.location().reload()); }
                        }
                        class="text-xs text-teal-400 hover:text-teal-300 underline underline-offset-2 cursor-pointer bg-transparent border-none"
                    >
                        "Coba Lagi"
                    </button>
                </div>
            </td>
        </tr>
    }
}

/// Error card for non-table contexts — standalone block with message + reload button.
#[allow(non_snake_case)]
#[component]
pub fn ErrorCard(message: String) -> impl IntoView {
    view! {
        <div class="bg-red-500/10 border border-red-500/30 rounded-2xl p-5 flex items-start gap-3">
            <div class="w-8 h-8 rounded-lg bg-red-500/15 flex items-center justify-center shrink-0">
                <i class="bi bi-exclamation-triangle-fill text-red-400 text-sm"></i>
            </div>
            <div class="flex-1 min-w-0">
                <p class="text-red-400 text-sm leading-relaxed">{message}</p>
                <button
                    type="button"
                    on:click=|_| {
                        #[cfg(target_arch = "wasm32")]
                        { let _ = leptos::web_sys::window().map(|w| w.location().reload()); }
                    }
                    class="mt-2 text-xs text-teal-400 hover:text-teal-300 underline underline-offset-2 cursor-pointer bg-transparent border-none"
                >
                    "Coba Lagi"
                </button>
            </div>
        </div>
    }
}

/// Animated skeleton rows for admin CRUD tables while data loads.
/// Renders `rows` shimmer rows each spanning `cols` columns.
#[allow(non_snake_case)]
#[component]
pub fn TableSkeleton(
    #[prop(default = 5)] cols: usize,
    #[prop(default = 6)] rows: usize,
) -> impl IntoView {
    let col_widths = ["w-8", "w-36", "w-24", "w-20", "w-14"];
    view! {
        {(0..rows).map(|_| {
            view! {
                <tr class="border-b border-line last:border-0">
                    {(0..cols).map(|i| {
                        let w = col_widths[i % col_widths.len()];
                        view! {
                            <td class="px-5 py-3.5">
                                <div class={format!("h-3.5 bg-line rounded-md animate-pulse {w}")}></div>
                            </td>
                        }
                    }).collect_view()}
                </tr>
            }
        }).collect_view()}
    }
}

/// Prev/Next pagination footer shared by admin CRUD tables. `resource` holds
/// one already-paginated page fetched from the server as `(rows, total_count)`
/// — the LIMIT/OFFSET happens in SQL, this only renders the controls + the
/// "showing X-Y of Z" label, kept in sync with the same 1-indexed `page` signal.
pub fn pagination_footer<T: Clone + 'static>(
    resource: LocalResource<Result<(Vec<T>, i64), ServerFnError>>,
    page: RwSignal<i64>,
    page_size: i64,
) -> impl IntoView {
    let total_pages = move || {
        resource
            .get()
            .and_then(|r| r.ok())
            .map(|(_, total)| ((total + page_size - 1) / page_size).max(1))
            .unwrap_or(1)
    };

    // If the current page is deleted down to empty (e.g. removing the last
    // row on the last page), the server keeps returning zero rows for that
    // now-out-of-range offset while `total` shrinks — snap back to the new
    // last page instead of showing a stuck-empty table.
    Effect::new(move |_| {
        if let Some(Ok((_, total))) = resource.get() {
            let tp = ((total + page_size - 1) / page_size).max(1);
            if page.get_untracked() > tp {
                page.set(tp);
            }
        }
    });

    // Comparison operators (`<`, `>=`, ...) written directly inline as a view!
    // attribute value confuse the RSX tag-parser into treating the rest of the
    // tag as text content — computing the bool in a named closure first (like
    // the `active`/`is_dark` pattern used elsewhere in this codebase) sidesteps
    // it entirely, since the raw `<`/`>` tokens never appear inside the macro.
    let is_first_page = move || page.get() <= 1;
    let is_last_page = move || page.get() >= total_pages();

    view! {
        <div class="flex items-center justify-between px-5 py-3 border-t border-line">
            <p class="text-xs text-muted">
                {move || match resource.get() {
                    Some(Ok((_, total))) if total > 0 => {
                        let cur = page.get().min(total_pages()).max(1);
                        let start = (cur - 1) * page_size + 1;
                        let end = (cur * page_size).min(total);
                        format!("Menampilkan {start}-{end} dari {total}")
                    }
                    _ => "".into(),
                }}
            </p>
            <div class="flex items-center gap-1">
                <button type="button"
                    disabled=is_first_page
                    on:click=move |_| page.update(|p| *p = (*p - 1).max(1))
                    class="w-8 h-8 flex items-center justify-center rounded-lg border border-line text-muted hover:text-fg hover:border-teal-500 disabled:opacity-40 disabled:cursor-not-allowed transition-colors cursor-pointer">
                    <i class="bi bi-chevron-left text-xs"></i>
                </button>
                <span class="text-xs text-muted px-2">
                    {move || format!("{} / {}", page.get().min(total_pages()).max(1), total_pages())}
                </span>
                <button type="button"
                    disabled=is_last_page
                    on:click=move |_| {
                        let tp = total_pages();
                        page.update(|p| if *p < tp { *p += 1; });
                    }
                    class="w-8 h-8 flex items-center justify-center rounded-lg border border-line text-muted hover:text-fg hover:border-teal-500 disabled:opacity-40 disabled:cursor-not-allowed transition-colors cursor-pointer">
                    <i class="bi bi-chevron-right text-xs"></i>
                </button>
            </div>
        </div>
    }
}
