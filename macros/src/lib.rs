extern crate proc_macro;

use proc_macro::TokenStream;
use quote::quote;
use syn::{
	Data, DeriveInput, parse_macro_input,
	Ident, DataStruct, Type, LitInt
};

// TODO: add dumping generation

// first macro its a bit shit
// it takes the structure definition and generates a parse(), matching the field types to the reader functions

#[proc_macro_derive(AutoParse, attributes(size))]
pub fn impl_parse(item: TokenStream) -> TokenStream {
	let input: DeriveInput = parse_macro_input!(item as DeriveInput);
	let struct_ident: &Ident = &input.ident;
	return match &input.data {
		Data::Struct(DataStruct {fields, ..}) => {
			let parsed_fields: Vec<_> = fields
				.iter()
				.map(|field| {
					let name: &Ident = field.ident.as_ref().unwrap();
					let ty: &Type = &field.ty;

					let size: Option<usize> = field
						.attrs
						.iter()
						.find(|attr| attr.path().is_ident("size"))
						.and_then(|attr| attr.parse_args::<LitInt>().ok())
						.map(|lit| lit.base10_parse::<usize>().unwrap());

					if size.is_some() {
						quote! { #name: #ty::parse_amount(r, #size)?, }
					} else {
						quote! { #name: #ty::parse(r)?, }
					}
				})
				.collect();
			quote! {
				impl #struct_ident {
					pub fn parse(r: &mut crate::reader::BitReader) -> anyhow::Result<#struct_ident> {
						return Ok(#struct_ident {
							#(#parsed_fields)*
						})
					}
				}
			}.into()
		},
		_ => unimplemented!()
	};
}