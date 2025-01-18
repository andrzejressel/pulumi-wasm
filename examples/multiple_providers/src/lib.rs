use anyhow::Result;
use pulumi_wasm_providers_docker::container;
use pulumi_wasm_providers_docker::container::ContainerArgs;
use pulumi_wasm_providers_random::random_string;
use pulumi_wasm_providers_random::random_string::RandomStringArgs;
use pulumi_wasm_rust::Output;
use pulumi_wasm_rust::{add_export, pulumi_main};

#[pulumi_main]
fn test_main() -> Result<()> {
    let length: Output<i32> = Output::new(&12).map(|i: i32| i * 3);
    let random_string = random_string::create(
        "test",
        RandomStringArgs::builder().length(length).build_struct(),
    );

    // Tests preview behaviour for unknown fields
    let t = random_string.result.map(|s| format!("Result: {s}"));

    // Tests number mapping
    let number = random_string.min_upper.map(|i| i * 2);

    let cont = container::create(
        "container",
        ContainerArgs::builder()
            .attach(true)
            .command(["echo", "Hello World!"])
            .image("public.ecr.aws/ubuntu/ubuntu:latest")
            .logs(true)
            .must_run(false)
            .build_struct(),
    );

    add_export("logs", &cont.container_logs);
    add_export("result", &random_string.result);
    add_export("transformed_result", &t);
    add_export("number", &number);
    Ok(())
}
