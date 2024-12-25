use pulumi_wasm_generator;
fn main() -> Result<(), Box<dyn std::error::Error>> {
    pulumi_wasm_generator::generate("docker", "4.5.3")?;
    Ok(())
}