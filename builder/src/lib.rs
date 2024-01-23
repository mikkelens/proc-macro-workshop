#![allow(unused)]
use proc_macro2::{Ident, TokenStream};
use syn::{
	Data, DeriveInput, Fields,
	__private::{quote::quote, ToTokens},
	spanned::Spanned
};

#[proc_macro_derive(Builder)]
pub fn derive(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
	let input = syn::parse_macro_input!(input as DeriveInput);

	let name = input.ident.clone();
	let builder_name = Ident::new(&(name.to_string() + "Builder"), input.span());

	let convert_impl = quote!(impl #name {
		pub fn builder() {}
	});

	let field_inits = match input.data {
		Data::Struct(s) => s.fields.into_iter().map(|field| {
			quote!(
				#field: None
			)
		}),
		_ => unimplemented!("Derive macro used on non-struct type!")
	};
	let builder_def: TokenStream = quote! {
		pub struct #builder_name {
			#(#field_inits),*
		}
	};

	TokenStream::from_iter([convert_impl, builder_def].into_iter()).into()
}