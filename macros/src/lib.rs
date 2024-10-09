extern crate proc_macro;

use proc_macro::TokenStream;
use quote::quote;
use syn::{Data, DeriveInput, parse_macro_input, Ident, DataStruct, Type, LitInt, Fields};

// TODO: add dumping generation

// first macro its a bit shit
// it takes the structure definition and generates a parse(), matching the field types to the reader functions

#[proc_macro_derive(AutoParse, attributes(size))]
pub fn impl_parse(item: TokenStream) -> TokenStream {
	let input: DeriveInput = parse_macro_input!(item as DeriveInput);
	let struct_ident: &Ident = &input.ident;
	match &input.data {
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
	}
}

#[proc_macro_derive(Dumpable)]
pub fn dumpable_derive(input: TokenStream) -> TokenStream {
	// Parse the input tokens into a syntax tree
	let input = parse_macro_input!(input as DeriveInput);

	// Get the name of the struct we are deriving the macro for
	let name = input.ident;

	// Match on the data (struct, enum, etc.) of the input
	let display_impl = match input.data {
		Data::Struct(data) => {
			let fields = match data.fields {
				Fields::Named(fields_named) => {
					fields_named.named.into_iter().map(|f| {
						let field_name = f.ident.unwrap();
						let field_name_str = field_name.to_string();
						// Generate code for displaying each field with tabs and name
						quote! {
                            write!(f, "\t\t{:<20} {}\n", #field_name_str, self.#field_name)?;
                        }
					})
				}
				_ => unimplemented!("Only named fields are supported"),
			};

			// Generate the final Display trait implementation for the struct
			quote! {
                impl std::fmt::Display for #name {
                    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(f, "{}:\n", stringify!(#name))?;
                        #(#fields)*
                        Ok(())
                    }
                }
            }
		}
		_ => unimplemented!("PrettyDisplay is only implemented for structs"),
	};

	// Convert into TokenStream
	TokenStream::from(display_impl)
}