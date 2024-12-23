use itertools::Itertools;
use proc_macro2::TokenStream;
use quote::quote;
use syn::{parse_macro_input, Data, DeriveInput, Fields, Path, Type};

#[proc_macro_derive(SubComponentInputs)]
pub fn derive_sub_component_inputs(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    assert_is_struct(&input);
    let name = &input.ident;
    let vec_array_fields = extract_vec_array_fields(&input);

    // TODO(Ohad): deprecate with_capacity.
    let with_capacity_method = generate_with_capacity_method(&vec_array_fields);
    let uninitialized_method = generate_uninitialized_method(&vec_array_fields);
    let bit_reverse_method = generate_bit_reverse_method(&vec_array_fields);

    let expanded = quote! {
        impl #name {
            #with_capacity_method
            #uninitialized_method
            #bit_reverse_method
        }
    };

    proc_macro::TokenStream::from(expanded)
}

#[proc_macro_derive(BitReverse)]
pub fn derive_bit_reverse(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    assert_is_struct(&input);
    let name = &input.ident;
    let vec_array_fields = extract_vec_array_fields(&input);

    // TODO(Ohad): deprecate with_capacity.
    let bit_reverse_method = generate_bit_reverse_method(&vec_array_fields);

    let expanded = quote! {
        impl #name {
            #bit_reverse_method
        }
    };

    proc_macro::TokenStream::from(expanded)
}

#[derive(Clone)]
struct VecArrayField {
    name: syn::Ident,
    array_length: usize,
}

fn assert_is_struct(input: &DeriveInput) {
    if !matches!(input.data, Data::Struct(_)) {
        panic!("Derive(SubComponentInputs) can only be applied to structs.");
    }
}

fn extract_vec_array_fields(input: &DeriveInput) -> Vec<VecArrayField> {
    let mut vec_array_fields = Vec::new();

    if let Data::Struct(data_struct) = &input.data {
        if let Fields::Named(fields) = &data_struct.fields {
            for field in &fields.named {
                // Field is an array of Vecs.
                if let Type::Array(type_array) = &field.ty {
                    if let Type::Path(element_type) = &*type_array.elem {
                        // Element is a Vec.
                        if is_vec_type(&element_type.path) {
                            // Get the array length
                            if let syn::Expr::Lit(syn::ExprLit {
                                lit: syn::Lit::Int(length_lit),
                                ..
                            }) = type_array.len.clone()
                            {
                                vec_array_fields.push(VecArrayField {
                                    name: field.ident.clone().unwrap(),
                                    array_length: length_lit.base10_parse().unwrap(),
                                });
                            }
                        }
                    }
                }
            }
        }
    }

    vec_array_fields
}

fn is_vec_type(path: &Path) -> bool {
    path.segments.len() == 1 && path.segments.first().unwrap().ident == "Vec"
}

fn generate_with_capacity_method(vec_array_fields: &[VecArrayField]) -> TokenStream {
    let field_initializations = vec_array_fields
        .iter()
        .map(|field| {
            let field_name = &field.name;
            let array_length = field.array_length;

            quote! {
                #field_name: [(); #array_length].map(|_| Vec::with_capacity(capacity))
            }
        })
        .collect_vec();

    quote! {
        fn with_capacity(capacity: usize) -> Self {
            Self {
                #(#field_initializations),*
            }
        }
    }
}

fn generate_uninitialized_method(vec_array_fields: &[VecArrayField]) -> TokenStream {
    let field_initializations = vec_array_fields
        .iter()
        .map(|field| {
            let field_name = &field.name;
            let array_length = field.array_length;

            quote! {
                #field_name: [(); #array_length].map(|_|{
                    let mut v = Vec::with_capacity(capacity);
                    v.set_len(capacity);
                    v
                })
            }
        })
        .collect_vec();

    quote! {
        unsafe fn uninitialized(capacity: usize) -> Self {
            let mut result = Self {
                #(#field_initializations),*
            };

            result
        }
    }
}

fn generate_bit_reverse_method(vec_array_fields: &[VecArrayField]) -> TokenStream {
    let field_updates = vec_array_fields
        .iter()
        .map(|field| {
            let field_name = &field.name;

            quote! {
                self.#field_name
                    .iter_mut()
                    .for_each(|vec| bit_reverse_coset_to_circle_domain_order(vec));
            }
        })
        .collect_vec();

    quote! {
        fn bit_reverse_coset_to_circle_domain_order(&mut self) {
            #(#field_updates)*
        }
    }
}
