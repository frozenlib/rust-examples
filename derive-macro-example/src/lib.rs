extern crate proc_macro;
use quote::quote;
use syn::*;

#[proc_macro_derive(MyDebug)]
pub fn derive_debug(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    if let Data::Struct(s) = &input.data {
        let field_types = s.fields.iter().map(|field| &field.ty);
        let struct_name = &input.ident;
        let (impl_generics, ty_generics, where_clause) = input.generics.split_for_impl();
        let where_clause = if let Some(where_clause) = where_clause {
            quote! { #where_clause, #(#field_types : std::fmt::Debug,)*  }
        } else {
            quote! { where #(#field_types : std::fmt::Debug,)* }
        };
        return quote! {
            impl #impl_generics std::fmt::Debug for #struct_name #ty_generics
            #where_clause {
                fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                    todo!()
                }
            }
        }
        .into();
    }
    panic!("unexpected input.")
}
