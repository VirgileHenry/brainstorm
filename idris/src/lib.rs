#[proc_macro_derive(Idris)]
pub fn idris_derive(stream: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let input = syn::parse_macro_input!(stream as syn::DeriveInput);

    let enum_name = &input.ident;
    let enum_data = match &input.data {
        syn::Data::Enum(data) => data,
        _ => return quote::quote! { compile_error!("Idris only works for enums") }.into(),
    };

    let variant_count = enum_data.variants.len();
    let variant_names = enum_data.variants.iter().map(|variant| {
        let ident = &variant.ident;
        let variant = match &variant.fields {
            syn::Fields::Unit => quote::quote! { #ident },
            syn::Fields::Unnamed(unnamed) => {
                let fields = (0..unnamed.unnamed.len()).map(|_| quote::quote! { _, });
                quote::quote! { #ident ( #( #fields )* ) }
            }
            syn::Fields::Named(_) => quote::quote! { #ident { .. } },
        };
        variant
    });
    let variant_ids = (0..variant_count).into_iter();

    quote::quote! {
        impl #enum_name {
            pub const COUNT: usize = #variant_count;
            pub fn id(&self) -> usize {
                match self {
                    #( Self::#variant_names => #variant_ids, )*
                }
            }
        }
    }
    .into()
}
