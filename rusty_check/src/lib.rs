#[cfg(feature = "mocking")]
pub mod mocks {
    pub use mockall;
    pub use rusty_check_macros::rustymock;
    struct MethodDeclaration {
        declaration_literal: String,
        name: String,
        args: String,
    }
    pub trait ComposableMock {
        fn get_methods(&self) -> Vec<String>;
    }
}

pub use rusty_check_macros::rusty_check;
