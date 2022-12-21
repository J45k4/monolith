use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, DeriveInput, Attribute};

trait AsyncFn<Args> {
    type Output;

    fn call(&self, args: Args) -> Self::Output;
}

#[proc_macro_derive(AsyncFn, attributes(get))]
pub fn async_fn_derive(input: TokenStream) -> TokenStream {
    // Parse the input token stream into a derive input
    let input = parse_macro_input!(input as DeriveInput);

    // Extract the name of the function from the derive input
    let name = input.ident;

    // Extract the path from the attributes
    let path = input
        .attrs
        .iter()
        .find(|attr| attr.path.is_ident("get"))
        .map(|attr| {
            let lit = attr.parse_meta().unwrap();
            match lit {
                Meta::NameValue(nv) => nv.lit.to_string(),
                _ => panic!("expected a value"),
            }
        });

    // Generate the output token stream
    let expanded = quote! {
        struct #name<Args>(fn(Args) -> ());

        impl<Args> #name<Args> {
            fn new(f: fn(Args) -> ()) -> Self {
                Self(f)
            }
        }

        impl<Args> AsyncFn<Args> for #name<Args> {
            type Output = ();

            fn call(&self, args: Args) -> Self::Output {
                (self.0)(args)
            }
        }

        impl<Args> #name<Args> {
            fn path(&self) -> &'static str {
                #path
            }
        }

        static #name: #name<()> = #name::new(#name);
    };

    expanded.into()
}
