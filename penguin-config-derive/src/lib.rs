use proc_macro::TokenStream;
use syn::{parse_macro_input, DeriveInput};
use quote::quote;

use darling::FromDeriveInput;

#[derive(FromDeriveInput)]
#[darling(attributes(penguin_config))]
struct Opts {
    path: String,
}

const ERROR_MSG: &str = r#"penguin_config: invalid macro attributes.
        Usage:
            #[penguin_config(path = "path/to/json_file.json")]
        "#;


#[proc_macro_derive(PenguinConfigFile, attributes(penguin_config))]
pub fn derive(input: TokenStream) -> TokenStream {
    let input: DeriveInput = parse_macro_input!(input as DeriveInput);
    let opts = Opts::from_derive_input(&input).expect(ERROR_MSG);

    let file_path = opts.path;
    let ident = input.ident;

    let output = quote! {
        impl PenguinConfig for #ident {
            fn read_config() -> Self {
                Deserializer::file_path(#file_path).deserialize()
            }
        }
    };

    output.into()
}




// Generates a json file from the structure at the given path. Requires a default implementation of
// the structure.
#[proc_macro_derive(PenguinConfigGenerate, attributes(penguin_config))]
pub fn generate(input: TokenStream) -> TokenStream {
    let input: DeriveInput = parse_macro_input!(input as DeriveInput);
    let opts = Opts::from_derive_input(&input).expect(ERROR_MSG);

    let file_path = opts.path;
    let ident = input.ident;

    let output = quote! {
        impl PenguinConfigGenerate for #ident {
            fn generate_penguin_config_file() {
                Serializer::file_path(#file_path).serialize(&Self::default())
            }
        }
    };

    output.into()
}

