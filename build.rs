use std::{fs, path::Path};

use schemars::schema::{InstanceType, Schema, SingleOrVec};
use typify::{TypeSpace, TypeSpaceSettings, UnknownPolicy};



fn main() -> Result<(), Box<dyn std::error::Error>> {
    let content = std::fs::read_to_string("./schema/fish_health.json")?;
    let schema = serde_json::from_str::<schemars::schema::RootSchema>(&content)?;

    let mut settings = TypeSpaceSettings::default();
    // Add any external crate dependencies
    settings.with_unknown_crates(UnknownPolicy::Allow);

    // Add derives for all types
    settings
        .with_derive("Debug".to_string())
        .with_derive("Clone".to_string())
        .with_derive("PartialEq".to_string());

        // Enable struct builders
    settings.with_struct_builder(true);

    let mut type_space = TypeSpace::new(&settings);
    type_space.add_root_schema(schema)?;

    let contents = prettyplease::unparse(&syn::parse2::<syn::File>(type_space.to_stream())?);

    // Write the generated code
    let mut mod_gen = Path::new("src").to_path_buf();
    mod_gen.push("fish_health.rs");
    fs::write(mod_gen, contents)?;

    Ok(())
}
