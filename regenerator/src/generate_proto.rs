use anyhow::Result;
use std::fs;

pub(crate) fn regenerate_proto() -> Result<()> {
    let proto_dir = "external/pulumi/proto";
    let out_location = "crates/proto/src";
    let model_location = format!("{}/mini", out_location);
    let full_location = format!("{}/full", out_location);

    fs::create_dir_all(&model_location)?;
    fs::create_dir_all(&full_location)?;

    tonic_build::configure()
        .build_transport(false)
        .build_client(false)
        .build_server(false)
        .out_dir(model_location)
        .compile_protos(
            &[
                format!("{}/pulumi/plugin.proto", proto_dir),
                format!("{}/pulumi/engine.proto", proto_dir),
                format!("{}/pulumi/resource.proto", proto_dir),
            ],
            &[proto_dir],
        )?;

    tonic_build::configure()
        .build_transport(true)
        .build_client(true)
        .build_server(true)
        .out_dir(full_location)
        .compile_protos(
            &[
                format!("{}/pulumi/plugin.proto", proto_dir),
                format!("{}/pulumi/engine.proto", proto_dir),
                format!("{}/pulumi/resource.proto", proto_dir),
            ],
            &[proto_dir],
        )?;

    Ok(())
}
