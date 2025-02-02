mod client;
mod enums;
mod header;
mod internal_enums;
mod models;
mod prelude;

use datamodel::builtin_connectors::{MONGODB, POSTGRES};
use prisma_client_rust_sdk::{GenerateArgs, PrismaGenerator};
use quote::quote;
use serde::Deserialize;

fn default_module_path() -> String {
    "prisma".to_string()
}

#[derive(Deserialize)]
pub struct PrismaClientRustGenerator {
    #[serde(default = "default_module_path")]
    module_path: String,
}

impl PrismaGenerator for PrismaClientRustGenerator {
    const NAME: &'static str = "Prisma Client Rust";
    const DEFAULT_OUTPUT: &'static str = "./prisma.rs";

    fn generate(self, args: GenerateArgs) -> String {
        let mut header = header::generate(&args);

        header.extend(models::generate(
            &args,
            self.module_path.parse().expect("Invalid module path"),
        ));

        let internal_enums = internal_enums::generate(&args);
        let client = client::generate(&args);

        let extra_pub_uses = match &args.connector {
            c if c.is_provider(POSTGRES.name()) | c.is_provider(MONGODB.name()) => quote!(
                pub use _prisma::QueryMode;
            ),
            _ => quote!(),
        };

        header.extend(quote! {
            pub mod _prisma {
                #client
                #internal_enums
            }

            pub use _prisma::PrismaClient;
            #extra_pub_uses
        });

        header.extend(enums::generate(&args));

        header.to_string()
    }
}
