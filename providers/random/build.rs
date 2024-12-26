fn main() -> Result<(), Box<dyn std::error::Error>> {
    pulumi_wasm_build::generate("random", "4.15.0")?;
    Ok(())
}