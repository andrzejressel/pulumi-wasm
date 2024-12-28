fn main() -> Result<(), Box<dyn std::error::Error>> {
    pulumi_wasm_build::generate("docker", "4.5.3")?;
    pulumi_wasm_build::generate("cloudflare", "5.45.0")?;
    Ok(())
}