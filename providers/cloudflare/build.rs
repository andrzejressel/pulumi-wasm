fn main() -> Result<(), Box<dyn std::error::Error>> {
    pulumi_wasm_build::generate("cloudflare", "5.43.1")?;
    Ok(())
}
