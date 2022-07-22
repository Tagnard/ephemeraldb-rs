use proc_macro::TokenStream;
use quote::quote;
use syn;

#[proc_macro_derive(Entry)]
pub fn entry_macro_derive(input: TokenStream) -> TokenStream {
    // Construct a representation of Rust code as a syntax tree
    // that we can manipulate
    let ast = syn::parse(input).unwrap();

    // Build the trait implementation
    impl_entry_macro(&ast)
}

fn impl_entry_macro(ast: &syn::DeriveInput) -> TokenStream {
    let name = &ast.ident;
    let gen = quote! {
        impl Entry for #name {
            fn get_id(&self) -> u32 {
                self.id
            }

            fn set_id(&mut self, id: u32) {
                self.id = id;
            }
        }

    };
    gen.into()
}
