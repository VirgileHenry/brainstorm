/// Arguments for the idris macro.
struct IdrisArgs {
    /// Representation of the id and count integers.
    ///
    /// Defaults to `usize`.
    repr: syn::Type,
}

impl IdrisArgs {
    fn from_input(input: &syn::DeriveInput) -> syn::Result<Self> {
        let mut repr: syn::Type = syn::parse_quote!(usize);

        for attr in &input.attrs {
            if attr.path().is_ident("idris") {
                attr.parse_nested_meta(|meta| {
                    if meta.path.is_ident("repr") {
                        let value: syn::Type = meta.value()?.parse()?;
                        repr = value;
                        Ok(())
                    } else {
                        Err(syn::Error::new_spanned(
                            input,
                            format!("Unknown idris arg: {:?}", meta.path.get_ident()),
                        ))
                    }
                })?;
            }
        }

        Ok(Self { repr })
    }
}

#[proc_macro_derive(Idris, attributes(idris))]
pub fn idris_derive(stream: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let input = syn::parse_macro_input!(stream as syn::DeriveInput);
    let args = match IdrisArgs::from_input(&input) {
        Ok(args) => args,
        Err(error) => return error.into_compile_error().into(),
    };
    let repr_ty = &args.repr;

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

    for variant in enum_data.variants.iter() {
        let ident = &variant.ident;

        let current_offset = offset.current();

        match &variant.fields {
            syn::Fields::Unit => {
                let pat = quote::quote! { Self::#ident };
                match_arms.push(quote::quote! {
                    #pat => (#current_offset) as #repr_ty
                });
                offset.increment();
            }
            syn::Fields::Named(_) => {
                let pat = quote::quote! { Self::#ident { .. } };
                match_arms.push(quote::quote! {
                    #pat => (#current_offset) as #repr_ty
                });
                offset.increment();
            }
            syn::Fields::Unnamed(unnamed) => match unnamed.unnamed.first() {
                Some(field) => {
                    let inner_ty = field.ty.clone();
                    match_arms.push(quote::quote! {
                        Self::#ident ( inner ) => (#current_offset + inner.id())
                    });
                    offset.add_rec(quote::quote! {
                        < #inner_ty as idris::Idris<#repr_ty>>::COUNT
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
        impl idris::Idris<#repr_ty> for #enum_name {
            const COUNT: #repr_ty = #count as #repr_ty;
            fn id(&self) -> #repr_ty {
                match self {
                    #( #match_arms, )*
                }
            }
        }
    }
    .into()
}
