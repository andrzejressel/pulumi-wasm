#[test]
fn test() -> anyhow::Result<()> {
    pulumi_wasm_generator::generate_files(
        std::path::Path::new("tests/schemas/pulumi-resource-random.json"),
        std::path::Path::new("tests/result.json")
    )?;
    Ok(())
}