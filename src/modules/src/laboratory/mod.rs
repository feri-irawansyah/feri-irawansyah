#[path = "models/laboratory_view.rs"]
pub mod laboratory_view;
#[path = "models/laboratory_command.rs"]
pub mod laboratory_command;
#[path = "contracts/repository.rs"]
pub mod repository;
#[path = "contracts/services.rs"]
pub mod laboratory_service;

pub use laboratory_command::*;
pub use laboratory_service::*;
pub use laboratory_view::*;
pub use repository::*;
