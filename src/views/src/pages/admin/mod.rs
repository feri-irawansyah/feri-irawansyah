pub mod cache;
pub mod dashboard;
pub mod experience;
pub mod laboratory;
pub mod layout;
pub mod login;
pub mod logs;
pub mod notes;
pub mod portfolio;
pub mod skills;
pub mod users;

pub use cache::CacheManagementPage;
pub use dashboard::AdminDashboard;
pub use experience::AdminExperiencePage;
pub use laboratory::AdminLaboratoryPage;
pub use layout::AdminLayout;
pub use login::LoginPage;
pub use logs::AdminLogsPage;
pub use notes::AdminNotesPage;
pub use portfolio::AdminPortfolioPage;
pub use skills::AdminSkillsPage;
pub use users::UsersPage;

// Middleware attaches Claims to request extensions before any handler runs.
// Guards below just read from extensions — no re-validation needed.

/// Admin-only guard. Redirects to /admin/login if unauthenticated or not admin.
#[cfg(feature = "ssr")]
pub async fn require_admin() -> Result<modules::auth::Claims, leptos::prelude::ServerFnError> {
    use actix_web::{HttpMessage, HttpRequest};
    use leptos_actix::extract;
    use modules::auth::Claims;

    let req = extract::<HttpRequest>()
        .await
        .map_err(|e| leptos::prelude::ServerFnError::new(e.to_string()))?;

    match req.extensions().get::<Claims>().cloned() {
        Some(claims) if claims.client_category == 1 => Ok(claims),
        _ => {
            leptos_actix::redirect("/admin/login");
            Err(leptos::prelude::ServerFnError::new("Unauthorized"))
        }
    }
}

/// Visitor guard. Returns Claims if authenticated, error if not.
/// Does NOT redirect — callers decide how to surface the error in UI.
#[cfg(feature = "ssr")]
pub async fn require_visitor() -> Result<modules::auth::Claims, leptos::prelude::ServerFnError> {
    use actix_web::{HttpMessage, HttpRequest};
    use leptos_actix::extract;
    use modules::auth::Claims;

    let req = extract::<HttpRequest>()
        .await
        .map_err(|e| leptos::prelude::ServerFnError::new(e.to_string()))?;

    req.extensions()
        .get::<Claims>()
        .cloned()
        .ok_or_else(|| leptos::prelude::ServerFnError::new("Login required"))
}
