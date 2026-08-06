//! Wires up every service trait object once at startup and groups them so
//! `main.rs`'s HTTP bootstrap doesn't need one `Arc::new(XxxServiceImpl::new(..))`
//! block and one `.app_data(..)` call per domain inline.

use std::sync::Arc;

use actix_web::web;
use connectors::cache::{CacheConn, CacheStore};
use connectors::supabase::StorageStore;
use modules::auth::AuthService;
use modules::cache::CacheService;
use modules::certifications::CertService;
use modules::experience::ExperienceService;
use modules::laboratory::LaboratoryService;
use modules::notes::NoteService;
use modules::portfolio::PortfolioService;
use modules::positions::PositionService;
use modules::skills::SkillService;
use modules::users::UserService;
use repositories::auth::AuthRepositoryImpl;
use repositories::certifications::CertRepositoryImpl;
use repositories::database::PgPool;
use repositories::experience::ExperienceRepositoryImpl;
use repositories::laboratory::LaboratoryRepositoryImpl;
use repositories::notes::NoteRepositoryImpl;
use repositories::portfolio::PortfolioRepositoryImpl;
use repositories::positions::PositionRepositoryImpl;
use repositories::skills::SkillRepositoryImpl;
use repositories::users::UserRepositoryImpl;
use services::auth::{AuthServiceDeps, AuthServiceImpl};
use services::cache::{CacheServiceDeps, CacheServiceImpl};
use services::certifications::{CertServiceDeps, CertServiceImpl};
use services::experience::{ExperienceServiceDeps, ExperienceServiceImpl};
use services::laboratory::{LaboratoryServiceDeps, LaboratoryServiceImpl};
use services::notes::{NoteServiceDeps, NoteServiceImpl};
use services::portfolio::{PortfolioServiceDeps, PortfolioServiceImpl};
use services::positions::{PositionServiceDeps, PositionServiceImpl};
use services::skills::{SkillServiceDeps, SkillServiceImpl};
use services::users::{UserServiceDeps, UserServiceImpl};

/// Every service trait object the app needs, wired once and cheap to clone
/// (each field is an `Arc`) into each Actix worker.
#[derive(Clone)]
pub struct AppServices {
    pub auth: Arc<dyn AuthService>,
    pub user: Arc<dyn UserService>,
    pub note: Arc<dyn NoteService>,
    pub laboratory: Arc<dyn LaboratoryService>,
    pub skill: Arc<dyn SkillService>,
    pub portfolio: Arc<dyn PortfolioService>,
    pub experience: Arc<dyn ExperienceService>,
    pub position: Arc<dyn PositionService>,
    pub cert: Arc<dyn CertService>,
    pub cache: Arc<dyn CacheService>,
    pub storage: Arc<dyn StorageStore>,
}

impl AppServices {
    pub fn build(
        pool: PgPool,
        cache_conn: CacheConn,
        storage: Arc<dyn StorageStore>,
        jwt_secret: String,
    ) -> Self {
        let cache_store: Arc<dyn CacheStore> = Arc::new(cache_conn.clone());

        let auth = Arc::new(AuthServiceImpl::new(AuthServiceDeps {
            auth_repo: Arc::new(AuthRepositoryImpl::new(pool.clone())),
            jwt_secret,
        })) as Arc<dyn AuthService>;

        let user = Arc::new(UserServiceImpl::new(UserServiceDeps {
            user_repo: Arc::new(UserRepositoryImpl::new(pool.clone())),
        })) as Arc<dyn UserService>;

        let note = Arc::new(NoteServiceImpl::new(NoteServiceDeps {
            note_repo: Arc::new(NoteRepositoryImpl::new(pool.clone())),
            cache: cache_store,
        })) as Arc<dyn NoteService>;

        let laboratory = Arc::new(LaboratoryServiceImpl::new(LaboratoryServiceDeps {
            laboratory_repo: Arc::new(LaboratoryRepositoryImpl::new(pool.clone())),
        })) as Arc<dyn LaboratoryService>;

        let skill = Arc::new(SkillServiceImpl::new(SkillServiceDeps {
            skill_repo: Arc::new(SkillRepositoryImpl::new(pool.clone())),
            cache: cache_conn.clone(),
        })) as Arc<dyn SkillService>;

        let portfolio = Arc::new(PortfolioServiceImpl::new(PortfolioServiceDeps {
            portfolio_repo: Arc::new(PortfolioRepositoryImpl::new(pool.clone())),
        })) as Arc<dyn PortfolioService>;

        let experience_repo = Arc::new(ExperienceRepositoryImpl::new(pool.clone()));
        let experience = Arc::new(ExperienceServiceImpl::new(ExperienceServiceDeps {
            experience_repo: experience_repo.clone(),
        })) as Arc<dyn ExperienceService>;

        let position = Arc::new(PositionServiceImpl::new(PositionServiceDeps {
            position_repo: Arc::new(PositionRepositoryImpl::new(pool.clone())),
            experience_repo,
        })) as Arc<dyn PositionService>;

        let cert = Arc::new(CertServiceImpl::new(CertServiceDeps {
            cert_repo: Arc::new(CertRepositoryImpl::new(pool.clone())),
        })) as Arc<dyn CertService>;

        let cache = Arc::new(CacheServiceImpl::new(CacheServiceDeps { conn: cache_conn }))
            as Arc<dyn CacheService>;

        Self {
            auth,
            user,
            note,
            laboratory,
            skill,
            portfolio,
            experience,
            position,
            cert,
            cache,
            storage,
        }
    }

    /// Registers every service as `app_data` in one call, so the Actix app
    /// factory closure doesn't need a `.app_data(..)` line per domain.
    pub fn configure(&self, cfg: &mut web::ServiceConfig) {
        cfg.app_data(web::Data::new(self.auth.clone()))
            .app_data(web::Data::new(self.user.clone()))
            .app_data(web::Data::new(self.note.clone()))
            .app_data(web::Data::new(self.laboratory.clone()))
            .app_data(web::Data::new(self.skill.clone()))
            .app_data(web::Data::new(self.portfolio.clone()))
            .app_data(web::Data::new(self.experience.clone()))
            .app_data(web::Data::new(self.position.clone()))
            .app_data(web::Data::new(self.cert.clone()))
            .app_data(web::Data::new(self.cache.clone()))
            .app_data(web::Data::new(self.storage.clone()));
    }
}
