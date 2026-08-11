use crate::i18n::*;
use leptos::prelude::*;
use leptos_meta::*;
use leptos_router::{
    components::{A, Route, Router, Routes},
    hooks::use_location,
    path,
};

use crate::pages::{
    about::AboutPage,
    admin::{
        AdminDashboard, AdminExperiencePage, AdminLaboratoryPage, AdminLogsPage, AdminNotesPage,
        AdminPortfolioPage, AdminSkillsPage, CacheManagementPage, LoginPage, UsersPage,
    },
    contact::ContactPage,
    cv::CvPage,
    experience::ExperiencePage,
    home::HomePage,
    journey::JourneyPage,
    laboratory::{LaboratoryCategoryPage, LaboratoryDetailPage, LaboratoryPage},
    not_found::NotFoundPage,
    notes::{NotePage, NotesPage},
    portfolio::PortfolioPage,
    skills::SkillsPage,
};

#[allow(non_snake_case)]
#[component]
fn PageTransition(children: Children) -> impl IntoView {
    let location = use_location();
    let (tick, set_tick) = signal(0u32);

    Effect::new(move |prev: Option<String>| {
        let current = location.pathname.get();
        if let Some(prev) = prev
            && prev != current
        {
            set_tick.update(|t| *t += 1);
            #[cfg(target_arch = "wasm32")]
            {
                let _ = leptos::web_sys::window().map(|w| w.scroll_to_with_x_and_y(0.0, 0.0));
            }
        }
        current
    });

    view! {
        <div
            class=("page-anim-a", move || tick.get() % 2 == 0)
            class=("page-anim-b", move || tick.get() % 2 != 0)
        >
            {children()}
        </div>
    }
}

// Shell lives inside <Router> so it can call use_location()
#[allow(non_snake_case)]
#[component]
fn Shell(is_dark: ReadSignal<bool>, set_is_dark: WriteSignal<bool>) -> impl IntoView {
    let pathname = use_location().pathname;
    let is_admin = move || pathname.get().starts_with("/admin");
    let (nav_open, set_nav_open) = signal(false);
    let i18n = use_i18n();

    // Fan-out offsets (dx, dy in px from the FAB center) for a crescent arc above the button.
    let fan_style = move |dx: f64, dy: f64| {
        if nav_open.get() {
            format!(
                "left:50%;bottom:0;transform:translate(calc(-50% + {dx}px), -{dy}px) scale(1);opacity:1;transition:transform 0.3s cubic-bezier(0.34,1.56,0.64,1), opacity 0.2s;"
            )
        } else {
            "left:50%;bottom:0;transform:translate(-50%, 0) scale(0.2);opacity:0;transition:transform 0.2s ease-in, opacity 0.15s;".to_string()
        }
    };

    view! {
        // Skip-to-content for keyboard/screen-reader users
        <a
            href="#main-content"
            class="sr-only focus:not-sr-only focus:fixed focus:top-4 focus:left-4 focus:z-100 focus:px-4 focus:py-2 focus:bg-teal-600 focus:text-white focus:rounded-lg focus:text-sm focus:font-medium"
        >
            "Skip to content"
        </a>

        // ── Left Sidebar — public pages only, desktop/tablet only ────────
        <Show when=move || !is_admin()>
            <aside
                aria-label="Main navigation"
                class="hidden md:flex fixed left-4 top-1/2 -translate-y-1/2 h-[80vh] flex-col items-start justify-center gap-3 z-50"
            >
                <A href="/" attr:class="group/item inline-flex items-center h-12 rounded-full bg-line hover:bg-teal-600 [&.active]:bg-teal-500 transition-colors duration-200 px-3.5 hover:pr-5 [&.active]:pr-5 no-underline">
                    <i class="bi bi-house-fill text-muted group-hover/item:text-white text-[1.25rem] transition-colors duration-200 shrink-0"></i>
                    <span class="text-sm font-medium text-white whitespace-nowrap max-w-0 group-hover/item:max-w-32.5 overflow-hidden transition-all duration-200 group-hover/item:ml-2">
                        "Home"
                    </span>
                </A>
                <A href="/about" attr:class="group/item inline-flex items-center h-12 rounded-full bg-line hover:bg-teal-600 [&.active]:bg-teal-500 transition-colors duration-200 px-3.5 hover:pr-5 [&.active]:pr-5 no-underline">
                    <i class="bi bi-person-fill text-muted group-hover/item:text-white text-[1.25rem] transition-colors duration-200 shrink-0"></i>
                    <span class="text-sm font-medium text-white whitespace-nowrap max-w-0 group-hover/item:max-w-32.5 overflow-hidden transition-all duration-200 group-hover/item:ml-2">
                        "About"
                    </span>
                </A>
                <A href="/portfolio" attr:class="group/item inline-flex items-center h-12 rounded-full bg-line hover:bg-teal-600 [&.active]:bg-teal-500 transition-colors duration-200 px-3.5 hover:pr-5 [&.active]:pr-5 no-underline">
                    <i class="bi bi-grid-fill text-muted group-hover/item:text-white text-[1.25rem] transition-colors duration-200 shrink-0"></i>
                    <span class="text-sm font-medium text-white whitespace-nowrap max-w-0 group-hover/item:max-w-32.5 overflow-hidden transition-all duration-200 group-hover/item:ml-2">
                        "Portfolio"
                    </span>
                </A>
                <A href="/experience" attr:class="group/item inline-flex items-center h-12 rounded-full bg-line hover:bg-teal-600 [&.active]:bg-teal-500 transition-colors duration-200 px-3.5 hover:pr-5 [&.active]:pr-5 no-underline">
                    <i class="bi bi-person-workspace text-muted group-hover/item:text-white text-[1.25rem] transition-colors duration-200 shrink-0"></i>
                    <span class="text-sm font-medium text-white whitespace-nowrap max-w-0 group-hover/item:max-w-32.5 overflow-hidden transition-all duration-200 group-hover/item:ml-2">
                        "Experience"
                    </span>
                </A>
                <A href="/notes" attr:class="group/item inline-flex items-center h-12 rounded-full bg-line hover:bg-teal-600 [&.active]:bg-teal-500 transition-colors duration-200 px-3.5 hover:pr-5 [&.active]:pr-5 no-underline">
                    <i class="bi bi-journal-text text-muted group-hover/item:text-white text-[1.25rem] transition-colors duration-200 shrink-0"></i>
                    <span class="text-sm font-medium text-white whitespace-nowrap max-w-0 group-hover/item:max-w-32.5 overflow-hidden transition-all duration-200 group-hover/item:ml-2">
                        "Notes"
                    </span>
                </A>
                <A href="/skills" attr:class="group/item inline-flex items-center h-12 rounded-full bg-line hover:bg-teal-600 [&.active]:bg-teal-500 transition-colors duration-200 px-3.5 hover:pr-5 [&.active]:pr-5 no-underline">
                    <i class="bi bi-cpu text-muted group-hover/item:text-white text-[1.25rem] transition-colors duration-200 shrink-0"></i>
                    <span class="text-sm font-medium text-white whitespace-nowrap max-w-0 group-hover/item:max-w-32.5 overflow-hidden transition-all duration-200 group-hover/item:ml-2">
                        "Skills"
                    </span>
                </A>
                <A href="/laboratory" attr:class="group/item inline-flex items-center h-12 rounded-full bg-line hover:bg-teal-600 [&.active]:bg-teal-500 transition-colors duration-200 px-3.5 hover:pr-5 [&.active]:pr-5 no-underline">
                    <i class="bi bi-flask text-muted group-hover/item:text-white text-[1.25rem] transition-colors duration-200 shrink-0"></i>
                    <span class="text-sm font-medium text-white whitespace-nowrap max-w-0 group-hover/item:max-w-32.5 overflow-hidden transition-all duration-200 group-hover/item:ml-2">
                        "Lab"
                    </span>
                </A>
            </aside>
        </Show>

        // ── Bottom Nav — public pages only, mobile only ──────────────────
        <Show when=move || !is_admin()>
            <div
                class="md:hidden fixed inset-0 z-40 bg-black/40 transition-opacity duration-200"
                class:opacity-0=move || !nav_open.get()
                class:pointer-events-none=move || !nav_open.get()
                on:click=move |_| set_nav_open.set(false)
            ></div>

            <nav
                aria-label="Mobile navigation"
                class="md:hidden fixed bottom-6 left-1/2 -translate-x-1/2 z-50"
            >
                <A href="/" attr:aria-label="Home"
                    attr:class="group/item absolute w-11 h-11 rounded-full bg-surface border border-line shadow-lg flex items-center justify-center text-muted hover:text-teal-500 hover:border-teal-500 [&.active]:text-teal-500 [&.active]:border-teal-500 no-underline"
                    attr:style=move || fan_style(-93.6, 16.5)
                    on:click=move |_| set_nav_open.set(false)>
                    <i class="bi bi-house-fill text-[1.05rem]"></i>
                </A>
                <A href="/about" attr:aria-label="About"
                    attr:class="group/item absolute w-11 h-11 rounded-full bg-surface border border-line shadow-lg flex items-center justify-center text-muted hover:text-teal-500 hover:border-teal-500 [&.active]:text-teal-500 [&.active]:border-teal-500 no-underline"
                    attr:style=move || fan_style(-76.2, 56.7)
                    on:click=move |_| set_nav_open.set(false)>
                    <i class="bi bi-person-fill text-[1.05rem]"></i>
                </A>
                <A href="/portfolio" attr:aria-label="Portfolio"
                    attr:class="group/item absolute w-11 h-11 rounded-full bg-surface border border-line shadow-lg flex items-center justify-center text-muted hover:text-teal-500 hover:border-teal-500 [&.active]:text-teal-500 [&.active]:border-teal-500 no-underline"
                    attr:style=move || fan_style(-42.6, 84.9)
                    on:click=move |_| set_nav_open.set(false)>
                    <i class="bi bi-grid-fill text-[1.05rem]"></i>
                </A>
                <A href="/experience" attr:aria-label="Experience"
                    attr:class="group/item absolute w-11 h-11 rounded-full bg-surface border border-line shadow-lg flex items-center justify-center text-muted hover:text-teal-500 hover:border-teal-500 [&.active]:text-teal-500 [&.active]:border-teal-500 no-underline"
                    attr:style=move || fan_style(0.0, 95.0)
                    on:click=move |_| set_nav_open.set(false)>
                    <i class="bi bi-person-workspace text-[1.05rem]"></i>
                </A>
                <A href="/notes" attr:aria-label="Notes"
                    attr:class="group/item absolute w-11 h-11 rounded-full bg-surface border border-line shadow-lg flex items-center justify-center text-muted hover:text-teal-500 hover:border-teal-500 [&.active]:text-teal-500 [&.active]:border-teal-500 no-underline"
                    attr:style=move || fan_style(42.6, 84.9)
                    on:click=move |_| set_nav_open.set(false)>
                    <i class="bi bi-journal-text text-[1.05rem]"></i>
                </A>
                <A href="/skills" attr:aria-label="Skills"
                    attr:class="group/item absolute w-11 h-11 rounded-full bg-surface border border-line shadow-lg flex items-center justify-center text-muted hover:text-teal-500 hover:border-teal-500 [&.active]:text-teal-500 [&.active]:border-teal-500 no-underline"
                    attr:style=move || fan_style(76.2, 56.7)
                    on:click=move |_| set_nav_open.set(false)>
                    <i class="bi bi-cpu text-[1.05rem]"></i>
                </A>
                <A href="/laboratory" attr:aria-label="Laboratory"
                    attr:class="group/item absolute w-11 h-11 rounded-full bg-surface border border-line shadow-lg flex items-center justify-center text-muted hover:text-teal-500 hover:border-teal-500 [&.active]:text-teal-500 [&.active]:border-teal-500 no-underline"
                    attr:style=move || fan_style(93.6, 16.5)
                    on:click=move |_| set_nav_open.set(false)>
                    <i class="bi bi-flask text-[1.05rem]"></i>
                </A>

                // Toggle FAB
                <button
                    on:click=move |_| set_nav_open.update(|o| *o = !*o)
                    aria-label=move || if nav_open.get() { "Close navigation" } else { "Open navigation" }
                    aria-expanded=move || nav_open.get().to_string()
                    class="relative w-14 h-14 rounded-full bg-teal-600 hover:bg-teal-500 text-white flex items-center justify-center shadow-lg shadow-teal-900/30 transition-transform duration-200 cursor-pointer"
                    class:rotate-45=move || nav_open.get()>
                    <i class="bi bi-grid-3x3-gap-fill text-[1.2rem]"></i>
                </button>
            </nav>
        </Show>

        // ── Top Right: Language + Dark Mode — public pages only ──────────
        <Show when=move || !is_admin()>
            <div class="fixed top-4 right-4 flex items-center gap-2 z-50">
                <button
                    on:click=move |_| {
                        let next = match i18n.get_locale() {
                            Locale::id => Locale::en,
                            Locale::en => Locale::id,
                        };
                        i18n.set_locale(next);
                    }
                    aria-label=move || match i18n.get_locale() {
                        Locale::id => "Switch to English",
                        Locale::en => "Ganti ke Bahasa Indonesia",
                    }
                    class="w-9 h-9 rounded-full bg-line flex items-center justify-center text-muted hover:bg-teal-600 hover:text-white transition-colors duration-200 cursor-pointer text-xs font-bold">
                    {move || match i18n.get_locale() {
                        Locale::id => "EN",
                        Locale::en => "ID",
                    }}
                </button>
                <button
                    on:click=move |_| set_is_dark.update(|d| *d = !*d)
                    aria-label=move || if is_dark.get() { "Switch to light mode" } else { "Switch to dark mode" }
                    class="w-9 h-9 rounded-full bg-line flex items-center justify-center text-muted hover:bg-teal-600 hover:text-white transition-colors duration-200 cursor-pointer">
                    {move || if is_dark.get() {
                        view! { <i class="bi bi-sun-fill text-[1.05rem]"></i> }.into_any()
                    } else {
                        view! { <i class="bi bi-moon-fill text-[1.05rem]"></i> }.into_any()
                    }}
                </button>
            </div>
        </Show>

        // ── Main content ─────────────────────────────────────────────────
        <div class=move || {
            if is_admin() { "min-h-screen" } else { "flex flex-col min-h-screen pb-20 md:pb-0" }
        }>
            <main id="main-content" class="flex-1">
                <PageTransition>
                    <Routes fallback=|| view! { <NotFoundPage/> }>
                        <Route path=path!("/")              view=HomePage/>
                        <Route path=path!("/portfolio")     view=PortfolioPage/>
                        <Route path=path!("/experience")    view=ExperiencePage/>
                        <Route path=path!("/notes")         view=NotesPage/>
                        <Route path=path!("/notes/:slug")   view=NotePage/>
                        <Route path=path!("/skills")        view=SkillsPage/>
                        <Route path=path!("/laboratory")    view=LaboratoryPage/>
                        <Route path=path!("/laboratory/:category") view=LaboratoryCategoryPage/>
                        <Route path=path!("/laboratory/:category/:slug") view=LaboratoryDetailPage/>
                        <Route path=path!("/contact")       view=ContactPage/>
                        <Route path=path!("/cv")            view=CvPage/>
                        <Route path=path!("/about")         view=AboutPage/>
                        <Route path=path!("/about/journey/:slug") view=JourneyPage/>
                        <Route path=path!("/admin/login")   view=LoginPage/>
                        <Route path=path!("/admin")         view=AdminDashboard/>
                        <Route path=path!("/admin/users")   view=UsersPage/>
                        <Route path=path!("/admin/experience") view=AdminExperiencePage/>
                        <Route path=path!("/admin/portfolio") view=AdminPortfolioPage/>
                        <Route path=path!("/admin/notes")   view=AdminNotesPage/>
                        <Route path=path!("/admin/skills")  view=AdminSkillsPage/>
                        <Route path=path!("/admin/laboratory") view=AdminLaboratoryPage/>
                        <Route path=path!("/admin/cache")      view=CacheManagementPage/>
                        <Route path=path!("/admin/logs")       view=AdminLogsPage/>
                    </Routes>
                </PageTransition>
            </main>

            // ── Footer — public pages only ────────────────────────────────
            <Show when=move || !is_admin()>
                <footer class="border-t border-line py-8 text-center text-muted text-sm">
                    <p>"© 2026 Feri Irawansyah. Built with Rust + Leptos."</p>
                </footer>
            </Show>
        </div>
    }
}

pub fn shell(options: LeptosOptions) -> impl IntoView {
    // Only fires in production, and only if GA_MEASUREMENT_ID is actually set —
    // keeps local dev traffic out of Analytics even if a prod .env gets copied
    let ga_id = (options.env == leptos::config::Env::PROD)
        .then(|| std::env::var("GA_MEASUREMENT_ID").ok())
        .flatten()
        .filter(|s| !s.is_empty());

    view! {
        <!DOCTYPE html>
        <html>
            <head>
                <meta charset="utf-8"/>
                <meta name="viewport" content="width=device-width, initial-scale=1"/>
                <meta name="theme-color" content="#14b8a6"/>
                <link rel="icon" href="/public/favicon.ico" type="image/x-icon"/>
                <link rel="apple-touch-icon" href="/public/favicon.webp"/>
                <link rel="alternate" type="application/rss+xml" title="Feri Irawansyah — Notes" href="/rss.xml"/>
                <link rel="stylesheet" href="/public/bi/bootstrap-icons.min.css"/>
                <script inner_html="(function(){document.documentElement.classList.toggle('dark',localStorage.theme==='dark'||(!('theme' in localStorage)&&window.matchMedia('(prefers-color-scheme: dark)').matches))})()"></script>
                {ga_id.map(|id| view! {
                    <script async src=format!("https://www.googletagmanager.com/gtag/js?id={id}")></script>
                    <script inner_html=format!(
                        "window.dataLayer=window.dataLayer||[];function gtag(){{dataLayer.push(arguments);}}gtag('js',new Date());gtag('config','{id}');"
                    )></script>
                })}
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

#[allow(non_snake_case)]
#[component]
pub fn App() -> impl IntoView {
    provide_meta_context();

    let (is_dark, set_is_dark) = signal(false);
    provide_context((is_dark, set_is_dark));

    // Initialize from localStorage / system preference
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

    // Apply .dark class to <html> and persist
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
        <Script type_="application/ld+json">
            {format!(
                r#"{{"@context":"https://schema.org","@type":"Person","name":"Feri Irawansyah","url":"https://feri-irawansyah.my.id","jobTitle":"Principal Engineer","image":"{}","email":"mailto:feryirawansyah09@gmail.com","sameAs":["https://github.com/feri-irawansyah","https://linkedin.com/in/feri-irawansyah"]}}"#,
                crate::assets::hero_image_url(),
            )}
        </Script>

        <I18nContextProvider>
            <Router>
                <Shell is_dark set_is_dark/>
            </Router>
        </I18nContextProvider>
    }
}
