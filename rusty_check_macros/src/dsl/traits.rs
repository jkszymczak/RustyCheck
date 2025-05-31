pub trait Code {
    fn get_code(&self) -> proc_macro2::TokenStream;
}
pub struct Argument {
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
    fn compose(&self, other: Self) -> Self;
}
