use proc_macro::TokenStream;
use proc_macro2::{Ident, Span};
use quote::quote;
use syn::{parse_macro_input, Data, DeriveInput, Field};

#[proc_macro_derive(DefaultBuilder, attributes())]
pub fn derive_builder(input: TokenStream) -> TokenStream {
  let DeriveInput { ident, data, .. } = parse_macro_input!(input as DeriveInput);

  let s = match data {
    Data::Struct(s) => s,
    _ => panic!("must derive on struct"),
  };

  let builder_ident = Ident::new(&format!("{ident}Builder"), Span::call_site());

  let builder_struct_fields = s.fields.iter().filter_map(|Field { ident, ty, .. }| {
    let Some(ident) = ident else {
      return None;
    };
    let builder_struct_field = quote! {
      #ident: derive_default_builder::make_option!(#ty)
    };
    Some(builder_struct_field)
  });

  let fn_build_fields = s.fields.iter().filter_map(|Field { ident, .. }| {
    let Some(ident) = ident else {
      return None;
    };
    let fn_build_line = quote! {
      #ident: self.#ident.unwrap_or_default(),
    };
    Some(fn_build_line)
  });

  let field_set_fns = s.fields.iter().filter_map(|Field { ident, ty, .. }| {
    let Some(ident) = ident else {
      return None;
    };
    let fn_name = Ident::new(&format!("set_{ident}"), Span::call_site());
    let field_set_fn = quote! {
      pub fn #fn_name(&mut self, #ident: impl Into<#ty>) -> #builder_ident {
        self.#ident = Some(#ident.into());
        self
      }
    };
    Some(field_set_fn)
  });

  quote! {
    #[derive(Default)]
    pub struct #builder_ident {
      #(#builder_struct_fields),*
    }

    impl #builder_ident {
      pub fn build(self) -> #ident {
        #ident {
          #(#fn_build_fields),*
        }
      }

      #(#field_set_fns)*
    }
  }
  .into()
}
