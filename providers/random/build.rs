fn main() -> Result<(), Box<dyn std::error::Error>> {
    pulumi_gestalt_build::generate("random", "4.15.1")?;
    Ok(())
}
