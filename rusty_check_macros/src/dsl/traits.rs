pub trait Code {
    fn get_code(&self) -> proc_macro2::TokenStream;
}

pub struct MethodDeclaration {
    pub declaration_literal: String,
    pub name: String,
    pub args: String,
}
pub trait ComposableMock {
    fn get_methods() -> Vec<MethodDeclaration>;
}
