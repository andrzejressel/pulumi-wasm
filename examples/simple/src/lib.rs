use anyhow::Error;
use log::info;
use pulumi_wasm_random::random_string;
use pulumi_wasm_random::random_string::RandomStringArgs;
use pulumi_wasm_rust::Output;
use pulumi_wasm_rust::{add_export, pulumi_main};

#[pulumi_main]
fn test_main() -> Result<(), Error> {
    let length: Output<i32> = Output::new(&12).map(|i: i32| i * 3);

    let random_string = random_string::create(
        "test",
        RandomStringArgs::builder().length(length).build_struct(),
    );

    // Tests preview behaviour for unknown fields
    let t = random_string.result.map(|s| format!("Result: {s}"));

    // Tests number mapping
    let number = random_string.min_upper.map(|i| i * 2);

    // Optional values are deserialized as None
    let keepers = random_string.keepers.map(|map| format!("Keepers: {map:?}"));

    let val1 = Output::new(&1);
    let val2 = Output::new(&"abc".to_string());

    // Outputs can be reused
    let combined = Output::combine2(val1, val2);
    let combined_2 = Output::combine2(val1, val2);

    let combined_string = combined.map(|values| format!("Values: {values:?}"));
    let combined_2_string = combined_2.map(|values| format!("Values: {values:?}"));

    let random_string_2 = random_string::create(
        "test_2",
        RandomStringArgs::builder()
            .length(keepers.map(|s| s.len() as i32))
            .build_struct(),
    );

    add_export("result", &random_string.result);
    add_export("transformed_result", &t);
    add_export("number", &number);
    add_export("combined_string", &combined_string);
    add_export("combined_2_string", &combined_2_string);
    add_export("keepers", &keepers);
    add_export("result_2", &random_string_2.result);
    Ok(())
}
