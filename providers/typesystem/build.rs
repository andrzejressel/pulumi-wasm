fn main() -> Result<(), Box<dyn std::error::Error>> {
    pulumi_gestalt_build::generate_from_schema("typesystem.json".as_ref())?;
    Ok(())
}
