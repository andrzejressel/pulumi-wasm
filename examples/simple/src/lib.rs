use anyhow::Error;
use pulumi_wasm_random::resource::random_string::{random_string, RandomStringArgs};
use pulumi_wasm_rust::Output;
use pulumi_wasm_rust::{add_export, pulumi_main};

#[pulumi_main]
fn test_main() -> Result<(), Error> {
    let length: Output<i32> = Output::new(&12).map(|i: i32| i * 3);
    let random_string = random_string(
        "test",
        RandomStringArgs {
            keepers: None.into(),
            length,
            lower: None.into(),
            min_lower: None.into(),
            min_numeric: None.into(),
            min_special: None.into(),
            min_upper: None.into(),
            number: None.into(),
            numeric: None.into(),
            override_special: None.into(),
            special: None.into(),
            upper: None.into(),
        },
    );

    // Tests preview behaviour for unknown fields
    let t = random_string.result.map(|s| format!("Result: {s}"));

    // Tests number mapping
    let number = random_string.min_upper.map(|i| i * 2);

    add_export("result", &random_string.result);
    add_export("transformed_result", &t);
    add_export("number", &number);
    Ok(())
}
