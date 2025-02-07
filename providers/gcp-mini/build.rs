use std::path::Path;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    pulumi_gestalt_build::generate_from_schema_with_filter(
        Path::new("../gcp.json"),
        &["compute", "storage"],
    )?;
    Ok(())
}
