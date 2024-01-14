use proc_macro::TokenStream;
use proc_macro2::{Ident, Span};
use quote::quote;
use syn::{parse_macro_input, Data, DeriveInput, Field, Generics, Type};

#[proc_macro_derive(DefaultBuilder)]
pub fn derive_builder(input: TokenStream) -> TokenStream {
  let DeriveInput {
    ident,
    data,
    generics,
    vis,
    ..
  } = parse_macro_input!(input as DeriveInput);

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

  let builder_default_fields = s.fields.iter().filter_map(|Field { ident, .. }| {
    let Some(ident) = ident else {
      return None;
    };
    Some(builder_default_field(ident))
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
    Some(field_set_fn(ident, ty, &builder_ident, &generics))
  });

  let (impl_generics, ty_generics, where_clause) = generics.split_for_impl();

  quote! {
    #[automatically_derived]
    #vis struct #builder_ident #generics {
      #(#builder_struct_fields),*
    }

    #[automatically_derived]
    impl #impl_generics Default for #builder_ident #ty_generics {
      fn default() -> #builder_ident #ty_generics {
        #builder_ident {
          #(#builder_default_fields),*
        }
      }
    }

    #[automatically_derived]
    impl #impl_generics #builder_ident #ty_generics #where_clause {
      pub fn build(self) -> #ident #ty_generics {
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

fn builder_default_field(ident: &Ident) -> proc_macro2::TokenStream {
  quote! {
    #ident: Default::default()
  }
}

fn fn_build_field(ident: &Ident, ty: &Type) -> proc_macro2::TokenStream {
  quote! {
    #ident: derive_default_builder::value_maybe_as_option!(#ty, self.#ident.unwrap_or_default(), self.#ident)
  }
}

fn field_set_fn(
  ident: &Ident,
  ty: &Type,
  builder_ident: &Ident,
  generics: &Generics,
) -> proc_macro2::TokenStream {
  let fn_name = Ident::new(&format!("set_{ident}"), Span::call_site());
  let (_, ty_generics, _) = generics.split_for_impl();
  quote! {
    pub fn #fn_name(mut self, #ident: impl Into<#ty>) -> #builder_ident #ty_generics {
      self.#ident = derive_default_builder::value_as_option!(#ty, #ident.into());
      self
    }
  }
}
