use anyhow::Error;
use pulumi_wasm_random::random_string;
use pulumi_wasm_random::random_string::RandomStringArgs;
use pulumi_wasm_rust::Output;
use pulumi_wasm_rust::{add_export, pulumi_main};

#[pulumi_main]
fn test_main() -> Result<(), Error> {
    let length: Output<i32> = Output::new(&4);
    let random_string_1 = random_string::create(
        "test_1",
        RandomStringArgs::builder().length(length).build_struct(),
    );

    let new_length = random_string_1.result.map(|s| s.len() as i32);

    let random_string_2 = random_string::create(
        "test_2",
        RandomStringArgs::builder()
            .length(new_length)
            .build_struct(),
    );

    let random_string_3 = random_string::create(
        "test_3",
        RandomStringArgs::builder()
            .length(random_string_2.length.map(|i| i * 2))
            .build_struct(),
    );

    add_export("result", &random_string_1.result);
    add_export("number_1", &random_string_1.length);
    add_export("number_2", &random_string_2.length);
    add_export("number_3", &random_string_3.length);
    Ok(())
}
