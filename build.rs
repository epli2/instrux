use std::{fs, path::Path};
use typify::{TypeSpace, TypeSpaceSettings};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let content = fs::read_to_string("./schema/instrux.schema.json")?;
    let schema = serde_json::from_str::<schemars::schema::RootSchema>(&content)?;

    let mut type_space = TypeSpace::new(TypeSpaceSettings::default().with_struct_builder(true));
    type_space.add_root_schema(schema)?;

    let contents = prettyplease::unparse(&syn::parse2::<syn::File>(type_space.to_stream())?);

    let out_file = Path::new("./generated/models.rs");
    fs::write(out_file, contents)?;

    Ok(())
}
