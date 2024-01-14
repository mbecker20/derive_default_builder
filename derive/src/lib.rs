use proc_macro::TokenStream;
use proc_macro2::{Ident, Span};
use quote::quote;
use syn::{parse_macro_input, Data, DeriveInput, Field, Type};

#[proc_macro_derive(DefaultBuilder)]
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
    Some(builder_struct_field(ident, ty))
  });

  let fn_build_fields = s.fields.iter().filter_map(|Field { ident, ty, .. }| {
    let Some(ident) = ident else {
      return None;
    };
    Some(fn_build_field(ident, ty))
  });

  let field_set_fns = s.fields.iter().filter_map(|Field { ident, ty, .. }| {
    let Some(ident) = ident else {
      return None;
    };
    Some(field_set_fn(ident, ty, &builder_ident))
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

fn builder_struct_field(ident: &Ident, ty: &Type) -> proc_macro2::TokenStream {
  quote! {
    #ident: derive_default_builder::make_option!(#ty)
  }
}

fn fn_build_field(ident: &Ident, ty: &Type) -> proc_macro2::TokenStream {
  quote! {
    #ident: derive_default_builder::value_maybe_as_option!(#ty, self.#ident.unwrap_or_default(), self.#ident)
  }
}

fn field_set_fn(ident: &Ident, ty: &Type, builder_ident: &Ident) -> proc_macro2::TokenStream {
  let fn_name = Ident::new(&format!("set_{ident}"), Span::call_site());
  quote! {
    pub fn #fn_name(mut self, #ident: impl Into<#ty>) -> #builder_ident {
      self.#ident = derive_default_builder::value_as_option!(#ty, #ident.into());
      self
    }
  }
}
