pub mod database;

pub mod profile {
    pub mod profile_repository;
    pub use profile_repository::ProfileRepositoryImpl;
}

pub mod experiences {
    pub mod experience_repository;
    pub use experience_repository::ExperienceRepositoryImpl;
}

pub mod projects {
    pub mod project_repository;
    pub use project_repository::ProjectRepositoryImpl;
}

pub mod skills {
    pub mod skill_repository;
    pub use skill_repository::SkillRepositoryImpl;
}

pub mod contact {
    pub mod contact_repository;
    pub use contact_repository::ContactRepositoryImpl;
}
