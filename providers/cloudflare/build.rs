fn main() -> Result<(), Box<dyn std::error::Error>> {
    pulumi_gestalt_rust_build::generate("cloudflare", "5.43.1")?;
    Ok(())
}
