fn main() -> Result<(), Box<dyn std::error::Error>> {
    pulumi_wasm_build::generate("docker", "4.5.3")?;
    Ok(())
}
