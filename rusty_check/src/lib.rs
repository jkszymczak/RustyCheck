#[cfg(feature = "mocking")]
pub mod mocks {
    pub use mockall;
    pub use rusty_check_macros::rustymock;
    pub use rusty_check_macros::traits::ComposableMock;
}

pub use rusty_check_macros::rusty_check;
