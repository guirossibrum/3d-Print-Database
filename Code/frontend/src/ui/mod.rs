// ui/mod.rs - Module declarations for all UI modules

pub mod draw;
pub mod popups;
pub mod layout;

// Re-export commonly used UI functionality
pub use draw::*;
pub use popups::*;
pub use layout::*;