//! src/routes/admin/mod.rs
pub use dashboard::admin_dashboard;
pub use logout::log_out;
pub use newsletters::*;
pub use password::*;

mod dashboard;
mod logout;
mod newsletters;
mod password;
