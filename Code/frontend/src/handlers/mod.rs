// handlers/mod.rs - Module declarations for all handler modules

pub mod normal;
pub mod edit;
pub mod create;
pub mod delete;
pub mod select;

// Re-export commonly used handler functionality
pub use normal::*;
pub use edit::*;
pub use create::*;
pub use delete::*;
pub use select::*;