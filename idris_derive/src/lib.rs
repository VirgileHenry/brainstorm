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
    let mut generics = input.generics.clone();

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
                    generics
                        .make_where_clause()
                        .predicates
                        .push(syn::parse_quote! { #inner_ty: idris::Idris });
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
    let (impl_generics, ty_generics, where_clause) = generics.split_for_impl();

    quote::quote! {
        impl #impl_generics idris::Idris for #enum_name #ty_generics #where_clause {
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

#[proc_macro_derive(ConstVariants)]
pub fn const_variants_derive(stream: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let input = syn::parse_macro_input!(stream as syn::DeriveInput);

    let enum_name = &input.ident;
    let enum_data = match &input.data {
        syn::Data::Enum(data) => data,
        _ => {
            return syn::Error::new_spanned(input, "ConstVariants is built for enums only!")
                .to_compile_error()
                .into();
        }
    };
    let mut generics = input.generics.clone();

    // COUNT is recomputed here (unit -> +1, recursive -> +inner COUNT) so the
    // derive stands alone; it mirrors idris' walk, same variant order.
    let mut count_terms: Vec<proc_macro2::TokenStream> = Vec::new();
    let mut fill_stmts: Vec<proc_macro2::TokenStream> = Vec::new();
    let mut placeholder: Option<proc_macro2::TokenStream> = None;

    for variant in enum_data.variants.iter() {
        let ident = &variant.ident;
        match &variant.fields {
            syn::Fields::Unit => {
                count_terms.push(quote::quote! { 1 });
                if placeholder.is_none() {
                    placeholder = Some(quote::quote! { Self::#ident });
                }
                fill_stmts.push(quote::quote! {
                    out[w] = Self::#ident;
                    w += 1;
                });
            }
            syn::Fields::Unnamed(unnamed) if unnamed.unnamed.len() == 1 => {
                let inner_ty = &unnamed.unnamed.first().unwrap().ty;
                generics
                    .make_where_clause()
                    .predicates
                    .push(syn::parse_quote! { #inner_ty: idris::ConstVariants });
                count_terms.push(quote::quote! {
                    <#inner_ty as idris::ConstVariants>::VARIANTS.len()
                });
                fill_stmts.push(quote::quote! {
                    {
                        let inner = <#inner_ty as idris::ConstVariants>::VARIANTS;
                        let mut j = 0;
                        while j < inner.len() {
                            out[w] = Self::#ident(inner[j]);
                            w += 1;
                            j += 1;
                        }
                    }
                });
            }
            syn::Fields::Named(named) => {
                // A named variant is a single leaf (one id), matching the Idris derive.
                let field_inits: Vec<_> = named
                    .named
                    .iter()
                    .map(|f| {
                        let name = f.ident.as_ref().unwrap();
                        let ty = &f.ty;
                        quote::quote! { #name: <#ty>::zero() }
                    })
                    .collect();
                let ctor = quote::quote! { Self::#ident { #( #field_inits, )* } };

                count_terms.push(quote::quote! { 1 });
                if placeholder.is_none() {
                    placeholder = Some(ctor.clone());
                }
                fill_stmts.push(quote::quote! {
                    out[w] = #ctor;
                    w += 1;
                });
            }
            _ => {
                return syn::Error::new_spanned(
                    variant,
                    "ConstVariants only supports unit variants and single-field unnamed (recursive) variants",
                )
                .to_compile_error()
                .into();
            }
        }
    }

    let (impl_generics, ty_generics, where_clause) = generics.split_for_impl();

    let body = match placeholder {
        Some(placeholder) => quote::quote! {
            const VARIANTS: &'static [Self] = &{
                const LEN: usize = 0 #( + #count_terms )*;
                let mut out = [#placeholder; LEN];
                let mut w = 0usize;
                #( #fill_stmts )*
                out
            };
        },
        None => quote::quote! {
            const VARIANTS: &'static [Self] = &[];
        },
    };

    quote::quote! {
        impl #impl_generics idris::ConstVariants for #enum_name #ty_generics #where_clause {
            #body
        }
    }
    .into()
}
