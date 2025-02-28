pub trait Code {
    fn get_code(&self) -> proc_macro2::TokenStream;
}
