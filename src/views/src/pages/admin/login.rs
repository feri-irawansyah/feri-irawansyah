use leptos::prelude::*;
use leptos_router::NavigateOptions;
use leptos_router::hooks::use_navigate;

#[server]
pub async fn check_already_logged_in() -> Result<bool, ServerFnError> {
    use actix_web::{HttpMessage, HttpRequest};
    use leptos_actix::extract;
    use modules::auth::Claims;

    let req = extract::<HttpRequest>()
        .await
        .map_err(|e| ServerFnError::new(e.to_string()))?;

    if req.extensions().get::<Claims>().is_some() {
        leptos_actix::redirect("/admin");
        return Ok(true);
    }

    Ok(false)
}

#[server]
pub async fn login_action(email: String, password: String) -> Result<(), ServerFnError> {
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
    // Secure requires HTTPS — only set it in production (behind Nginx/TLS),
    // otherwise the cookie would be silently dropped during local `cargo
    // leptos watch` dev (plain http://127.0.0.1:3000).
    let secure = if leptos_options.env == leptos::config::Env::PROD {
        "; Secure"
    } else {
        ""
    };

    let ip = req
        .connection_info()
        .realip_remote_addr()
        .unwrap_or("unknown")
        .to_string();

    let result = auth_svc
        .login(&email, &password, &ip)
        .await
        .map_err(|e| ServerFnError::new(e.to_string()))?;

    let response = use_context::<ResponseOptions>()
        .ok_or_else(|| ServerFnError::new("No response context"))?;

    response.append_header(
        SET_COOKIE,
        HeaderValue::from_str(&format!(
            "access_token={}; HttpOnly; SameSite=Strict; Path=/; Max-Age=900{secure}",
            result.access_token
        ))
        .unwrap(),
    );
    response.append_header(
        SET_COOKIE,
        HeaderValue::from_str(&format!(
            "refresh_token={}; HttpOnly; SameSite=Strict; Path=/; Max-Age=604800{secure}",
            result.refresh_token
        ))
        .unwrap(),
    );

    Ok(())
}

#[allow(non_snake_case)]
#[component]
pub fn LoginPage() -> impl IntoView {
    let already_logged_in = Resource::new_blocking(|| (), |_| check_already_logged_in());
    let login = ServerAction::<LoginAction>::new();
    let navigate = use_navigate();

    // SSR handled inside check_already_logged_in via leptos_actix::redirect.
    // This Effect handles client-side navigation (e.g. navigating to /admin/login via router).
    let navigate_clone = navigate.clone();
    Effect::new(move |_| {
        if already_logged_in.get().is_some_and(|r| r.unwrap_or(false)) {
            navigate_clone("/admin", NavigateOptions::default());
        }
    });

    Effect::new(move |_| {
        if login
            .value()
            .with(|v| v.as_ref().map(|r| r.is_ok()).unwrap_or(false))
        {
            navigate("/admin", NavigateOptions::default());
        }
    });

    let error_msg = move || {
        login.value().with(|v| {
            v.as_ref()
                .and_then(|r| r.as_ref().err())
                .map(|e| e.to_string())
        })
    };

    view! {
        <Suspense fallback=|| ()>
            {move || {
                // Saat SSR redirect sudah di-set, render kosong agar tidak flash form
                if already_logged_in.get().is_some_and(|r| r.unwrap_or(false)) {
                    return view! { <div></div> }.into_any();
                }
                view! {
                    <div class="min-h-screen flex items-center justify-center bg-base">
                        <div class="w-full max-w-sm px-8 py-10 bg-surface border border-line rounded-xl shadow-sm">
                            <h1 class="text-2xl font-extrabold mb-1 text-fg">"Admin"</h1>
                            <p class="text-muted text-sm mb-8">"Masuk ke panel admin"</p>

                            <ActionForm action=login>
                                <div class="flex flex-col gap-4">
                                    <div>
                                        <label attr:for="login-email" class="block text-sm font-medium mb-1.5 text-fg">"Email"</label>
                                        <input
                                            id="login-email"
                                            type="email"
                                            name="email"
                                            required
                                            autocomplete="email"
                                            class="w-full px-3 py-2 bg-base border border-line rounded-lg text-fg placeholder:text-muted focus:outline-none focus:border-teal-500 transition-colors text-sm"
                                            placeholder="admin@example.com"
                                        />
                                    </div>
                                    <div>
                                        <label attr:for="login-password" class="block text-sm font-medium mb-1.5 text-fg">"Password"</label>
                                        <input
                                            id="login-password"
                                            type="password"
                                            name="password"
                                            required
                                            autocomplete="current-password"
                                            class="w-full px-3 py-2 bg-base border border-line rounded-lg text-fg placeholder:text-muted focus:outline-none focus:border-teal-500 transition-colors text-sm"
                                            placeholder="••••••••"
                                        />
                                    </div>

                                    {move || error_msg().map(|e| view! {
                                        <p class="text-red-400 text-sm py-1">{e}</p>
                                    })}

                                    <button
                                        type="submit"
                                        class="w-full py-2.5 mt-1 bg-teal-600 hover:bg-teal-500 text-white font-semibold rounded-lg transition-colors text-sm cursor-pointer disabled:opacity-60"
                                        disabled=move || login.pending().get()>
                                        {move || if login.pending().get() { "Masuk..." } else { "Masuk" }}
                                    </button>
                                </div>
                            </ActionForm>
                        </div>
                    </div>
                }.into_any()
            }}
        </Suspense>
    }
}
