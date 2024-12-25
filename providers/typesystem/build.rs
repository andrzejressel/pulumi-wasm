use pulumi_wasm_generator;
fn main() -> Result<(), Box<dyn std::error::Error>> {
    pulumi_wasm_generator::generate_from_schema("typesystem.json".as_ref())?;
    Ok(())
}