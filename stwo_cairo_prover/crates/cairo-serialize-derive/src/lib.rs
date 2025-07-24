use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, Data, DeriveInput, Fields};

#[proc_macro_derive(CairoSerialize)]
pub fn derive_cairo_serialize(input: TokenStream) -> TokenStream {
    // Parse the input tokens into a syntax tree.
    let input = parse_macro_input!(input as DeriveInput);

    let struct_name = input.ident;
    let (impl_generics, ty_generics, where_clause) = input.generics.split_for_impl();

    // Extract the fields of the struct.
    let fields = match input.data {
        Data::Struct(ref data_struct) => match &data_struct.fields {
            Fields::Named(ref fields_named) => &fields_named.named,
            Fields::Unnamed(_) | Fields::Unit => {
                return syn::Error::new_spanned(
                    struct_name,
                    "CairoSerialize can only be derived for structs with named fields.",
                )
                .to_compile_error()
                .into();
            }
        },
        _ => {
            return syn::Error::new_spanned(
                struct_name,
                "CairoSerialize can only be derived for structs.",
            )
            .to_compile_error()
            .into();
        }
    };

    // Generate code to serialize each field in the order they appear.
    let serialize_body = fields.iter().map(|f| {
        let field_name = &f.ident;
        quote! {
            CairoSerialize::serialize(&self.#field_name, output);
        }
    });

    // Implement `CairoSerialize` for the type.
    let expanded = quote! {
        impl #impl_generics ::stwo_cairo_serialize::CairoSerialize for #struct_name #ty_generics #where_clause {
            fn serialize(&self, output: &mut Vec<::starknet_ff::FieldElement>) {
                #(#serialize_body)*
            }
        }
    };

    TokenStream::from(expanded)
}

#[proc_macro_derive(CairoDeserialize)]
pub fn derive_cairo_deserialize(input: TokenStream) -> TokenStream {
    // Parse the input tokens into a syntax tree.
    let input = parse_macro_input!(input as DeriveInput);

    let struct_name = input.ident;
    let (impl_generics, ty_generics, where_clause) = input.generics.split_for_impl();

    // Extract the fields of the struct.
    let fields = match input.data {
        Data::Struct(ref data_struct) => match &data_struct.fields {
            Fields::Named(ref fields_named) => &fields_named.named,
            Fields::Unnamed(_) | Fields::Unit => {
                return syn::Error::new_spanned(
                    struct_name,
                    "CairoDeserialize can only be derived for structs with named fields.",
                )
                .to_compile_error()
                .into();
            }
        },
        _ => {
            return syn::Error::new_spanned(
                struct_name,
                "CairoDeserialize can only be derived for structs.",
            )
            .to_compile_error()
            .into();
        }
    };

    // Generate code to serialize each field in the order they appear.
    let deserialize_body = fields.iter().map(|f| {
        let field_name = &f.ident;
        quote! {
            #field_name: ::stwo_cairo_serialize::CairoDeserialize::deserialize(data),
        }
    });

    // Implement `CairoSerialize` for the type.
    let expanded = quote! {
        impl #impl_generics ::stwo_cairo_serialize::CairoDeserialize for #struct_name #ty_generics #where_clause {
            fn deserialize<'a>(
                data: &mut impl Iterator<Item = &'a ::starknet_ff::FieldElement>,
            ) -> Self {
                Self{ #(#deserialize_body)* }
            }
        }
    };

    TokenStream::from(expanded)
}

#[proc_macro_derive(CompactBinary)]
pub fn derive_compact_binary(input: TokenStream) -> TokenStream {
    // Parse the input tokens into a syntax tree.
    let input = parse_macro_input!(input as DeriveInput);

    let struct_name = input.ident;
    let (impl_generics, ty_generics, where_clause) = input.generics.split_for_impl();

    // Extract the fields of the struct.
    let fields = match input.data {
        Data::Struct(ref data_struct) => match &data_struct.fields {
            Fields::Named(ref fields_named) => &fields_named.named,
            Fields::Unnamed(_) | Fields::Unit => {
                return syn::Error::new_spanned(
                    struct_name,
                    "CompactBinary can only be derived for structs with named fields.",
                )
                .to_compile_error()
                .into();
            }
        },
        _ => {
            return syn::Error::new_spanned(
                struct_name,
                "CompactBinary can only be derived for structs.",
            )
            .to_compile_error()
            .into();
        }
    };

    // Generate code to serialize each field in the order they appear.
    let compact_serialize_body = fields.iter().map(|f| {
        let field_name = &f.ident;
        quote! {
            CompactBinary::compact_serialize(&self.#field_name, output);
        }
    });

    // Generate code to deserialize each field in the order they appear.
    let compact_deserialize_let_bindings = fields.iter().map(|f| {
        let field_name = &f.ident;
        quote! {
            let (input, #field_name) = ::stwo_cairo_serialize::CompactBinary::compact_deserialize(input);
        }
    });
    let compact_deserialize_struct_fields = fields.iter().map(|f| {
        let field_name = &f.ident;
        quote! { #field_name }
    });

    // Implement `CompactBinary` for the type.
    let expanded = quote! {
        impl #impl_generics ::stwo_cairo_serialize::CompactBinary for #struct_name #ty_generics #where_clause {
            fn compact_serialize(&self, output: &mut Vec<u8>) {
                #(#compact_serialize_body)*
            }

            fn compact_deserialize<'a>(mut input: &'a [u8]) -> (&'a [u8], Self) {
                #(#compact_deserialize_let_bindings)*
                (input, Self { #(#compact_deserialize_struct_fields),* })
            }
        }
    };

    TokenStream::from(expanded)
}
