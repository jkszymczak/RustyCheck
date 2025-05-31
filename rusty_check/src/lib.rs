#[cfg(feature = "mocking")]
pub mod mocks {
    pub use mockall;
    pub use rusty_check_macros::compose_mocks;
    #[cfg(feature = "unstable")]
    pub use rusty_check_macros::{append_to_registry, rustymock};
}

pub use rusty_check_macros::rusty_check;
