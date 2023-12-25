use proc_macro::{Ident, Span, TokenStream};
use syn::{parse::Parse, parse_macro_input, DeriveInput, __private::quote::quote};

#[proc_macro_derive(Builder)]
pub fn derive(input: TokenStream) -> TokenStream {
	let input = parse_macro_input!(input as DeriveInput);
	let name = Ident::new(&format!("{}Builder", input.ident), Span::call_site());

	let expanded = quote! {
		pub struct #name {
			#
		}
		impl #name {
			pub fn builder() -> #name {
				#name::new()
			}
		}
	};

	// unimplemented!()
	TokenStream::new()
}