fn main() -> Result<(), Box<dyn std::error::Error>> {
    tonic_build::configure()
        .build_transport(false)
        .build_client(false)
        .build_server(false)
        .compile(
            &[
                "../proto/pulumi/plugin.proto",
                "../proto/pulumi/engine.proto",
                "../proto/pulumi/resource.proto",
            ],
            &["../proto"],
        )?;
    Ok(())
}
