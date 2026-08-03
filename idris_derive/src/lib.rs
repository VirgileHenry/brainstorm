#[proc_macro_derive(Idris, attributes(idris))]
pub fn idris_derive(stream: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let input = syn::parse_macro_input!(stream as syn::DeriveInput);

    let enum_name = &input.ident;
    let enum_data = match &input.data {
        syn::Data::Enum(data) => data,
        _ => {
            return syn::Error::new_spanned(input, "Idris is built for enums only!")
                .to_compile_error()
                .into();
        }
    };

    #[derive(Debug, Clone)]
    struct Offset {
        numeric: usize,
        recursive: Vec<proc_macro2::TokenStream>,
    }

    impl Offset {
        fn increment(&mut self) {
            self.numeric += 1;
        }

        fn add_rec(&mut self, rec: proc_macro2::TokenStream) {
            self.recursive.push(rec);
        }

        fn current(&self) -> proc_macro2::TokenStream {
            let numeric = self.numeric;
            let recursive = self.recursive.iter();
            quote::quote! { #numeric #( + #recursive )* }
        }
    }

    let mut offset = Offset {
        numeric: 0,
        recursive: Vec::new(),
    };

    let mut match_arms = Vec::new();
    let mut name_match_arms = Vec::new();

    for variant in enum_data.variants.iter() {
        let ident = &variant.ident;

        let current_offset = offset.current();

        match &variant.fields {
            syn::Fields::Unit => {
                let pat = quote::quote! { Self::#ident };
                match_arms.push(quote::quote! {
                    #pat => (#current_offset)
                });
                offset.increment();
                let name = ident.to_string();
                name_match_arms.push(quote::quote! {
                    if id == (#current_offset) {
                        return #name;
                    }
                });
            }
            syn::Fields::Named(_) => {
                let pat = quote::quote! { Self::#ident { .. } };
                match_arms.push(quote::quote! {
                    #pat => (#current_offset)
                });
                offset.increment();
                let name = ident.to_string();
                name_match_arms.push(quote::quote! {
                    if id == (#current_offset) {
                        return #name;
                    }
                });
            }
            syn::Fields::Unnamed(unnamed) => match unnamed.unnamed.first() {
                Some(field) => {
                    let inner_ty = field.ty.clone();
                    match_arms.push(quote::quote! {
                        Self::#ident ( inner ) => (#current_offset + inner.id())
                    });
                    name_match_arms.push(quote::quote! {
                        if (#current_offset) <= id && id < (#current_offset) + < #inner_ty as idris::Idris >::COUNT {
                            return < #inner_ty as idris::Idris >::name_from_id(id - ( #current_offset ));
                        }
                    });
                    offset.add_rec(quote::quote! {
                        < #inner_ty as idris::Idris >::COUNT
                    });
                }
                None => {
                    return syn::Error::new_spanned(
                        variant,
                        "Unnamed fields are considered recursive, and require exactly one field",
                    )
                    .to_compile_error()
                    .into();
                }
            },
        };
    }

    let count = offset.current();

    quote::quote! {
        impl idris::Idris for #enum_name {
            const COUNT: usize = #count;
            fn id(&self) -> usize {
                match self {
                    #( #match_arms, )*
                }
            }
            fn name_from_id(id: usize) -> &'static str {
                #( #name_match_arms )*
                panic!("Invalid id ({}) for {}", id, stringify!(#enum_name))
            }
        }
    }
    .into()
}
