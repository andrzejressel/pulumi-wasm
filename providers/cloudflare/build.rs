use pulumi_wasm_generator;
fn main() -> Result<(), Box<dyn std::error::Error>> {
    pulumi_wasm_generator::generate("cloudflare", "5.43.1")?;
    Ok(())
}