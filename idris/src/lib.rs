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

    let mut offset = quote::quote! { 0 };
    let mut match_arms = Vec::new();

    for variant in enum_data.variants.iter() {
        let ident = &variant.ident;
        let rec = match is_variant_recursive(variant) {
            Ok(rec) => rec,
            Err(error) => return error.to_compile_error().into(),
        };

        let current_offset = offset.clone();

        if rec {
            /* If the variant is recursive, use idris generated functions to get count and offset */
            /* Ensure the variant contains exactly one unnamed field */
            let inner_ty = match &variant.fields {
                syn::Fields::Unnamed(u) if u.unnamed.len() == 1 => &u.unnamed[0].ty,
                _ => {
                    return syn::Error::new_spanned(variant, "#[idris(rec)] requires exactly one unnamed field")
                        .to_compile_error()
                        .into();
                }
            };
            match_arms.push(quote::quote! {
                Self::#ident(inner) => (#current_offset + inner.id()) as #repr_ty
            });
            offset = quote::quote! {
                #offset + < #inner_ty >::COUNT
            };
        } else {
            let pat = match &variant.fields {
                syn::Fields::Unit => quote::quote! { Self::#ident },
                syn::Fields::Unnamed(_) => quote::quote! { Self::#ident(..) },
                syn::Fields::Named(_) => quote::quote! { Self::#ident { .. } },
            };
            match_arms.push(quote::quote! {
                #pat => (#current_offset) as #repr_ty
            });
            offset = quote::quote! { #offset + 1 };
        }
    }

    quote::quote! {
        impl #enum_name {
            pub const COUNT: #repr_ty = #offset as #repr_ty;
            pub const fn id(&self) -> #repr_ty {
                match self {
                    #( #match_arms, )*
                }
            }
        }
    }
    .into()
}

fn is_variant_recursive(variant: &syn::Variant) -> syn::Result<bool> {
    let mut rec = false;

    for attr in &variant.attrs {
        if attr.path().is_ident("idris") {
            attr.parse_nested_meta(|meta| {
                if meta.path.is_ident("rec") {
                    rec = true;
                    Ok(())
                } else {
                    Err(syn::Error::new_spanned(
                        attr,
                        format!("Unknown idris attribute: {:?}", meta.path.get_ident()),
                    ))
                }
            })?;
        }
    }

    Ok(rec)
}
