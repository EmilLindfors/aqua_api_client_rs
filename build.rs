use schemars::schema::{Schema, SchemaObject};
use std::{fs, path::Path};
use typify::{TypeSpace, TypeSpaceImpl, TypeSpaceSettings, UnknownPolicy};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    for entry in fs::read_dir("./schema")? {
        let entry = entry?;
        let path = entry.path();
        if path.is_file() {
            let file_name = path
                .file_name()
                .map(|s| s.to_string_lossy().to_string())
                .unwrap();
            let content = fs::read_to_string(path)?;

            let schema = serde_json::from_str::<schemars::schema::RootSchema>(&content)?;
            let mut settings = TypeSpaceSettings::default();
            // Add any external crate dependencies
            settings.with_unknown_crates(UnknownPolicy::Allow);

            // Add derives for all types
            settings
                .with_derive("Debug".to_string())
                .with_derive("Clone".to_string());

            // Enable struct builders
            settings.with_struct_builder(true);

            let mut type_space = TypeSpace::new(&settings);
            type_space.add_root_schema(schema)?;

            let contents =
                prettyplease::unparse(&syn::parse2::<syn::File>(type_space.to_stream())?);

            // Write the generated code
            let mut mod_gen: std::path::PathBuf = Path::new("src").to_path_buf();
            mod_gen.push("domain/generated");
            mod_gen.push(format!("{}.rs", file_name.replace(".json", "")));
            fs::write(mod_gen, contents)?;
        }
    }

    Ok(())
}
