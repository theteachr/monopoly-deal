use proc_macro::{TokenStream, TokenTree};

#[proc_macro_derive(Card)]
pub fn derive_card(input: TokenStream) -> TokenStream {
	let struct_name = match input.into_iter().nth(2).unwrap() {
        TokenTree::Ident(ident) => ident.to_string(),
        _ => panic!("`Card` not derived on a struct"),
    };

    format!("\
    impl Card for {} {{
        fn id(&self) -> usize {{
            self.id
        }}

        fn value(&self) -> u8 {{
            self.value
        }}
    }}", struct_name).parse().unwrap()
}
