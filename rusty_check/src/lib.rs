#[cfg(feature = "mocking")]
pub mod mocks {
    pub use mockall;
    pub use rusty_check_macros::rustymock;

    struct Argument {
        name: String,
        t: String,
    }
    pub struct MethodDeclaration {
        declaration_literal: String,
        arguments: Vec<Argument>,
        return_type: String,
    }
    pub trait ComposableMock {
        fn get_methods(&self) -> Vec<MethodDeclaration>;
        fn compose<M, C>(&self, other: M) -> C;
    }
}

pub use rusty_check_macros::rusty_check;
