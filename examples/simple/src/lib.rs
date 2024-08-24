use anyhow::Error;
use pulumi_wasm_random::random_string;
use pulumi_wasm_random::random_string::RandomStringArgs;
use pulumi_wasm_rust::Output;
use pulumi_wasm_rust::{add_export, pulumi_main};

#[pulumi_main]
fn test_main() -> Result<(), Error> {
    let length: Output<i32> = Output::new(&12).map(|i: i32| i * 3);
    let random_string = random_string::create(
        "test",
        RandomStringArgs {
            keepers: Output::empty(),
            length,
            lower: Output::empty(),
            min_lower: Output::empty(),
            min_numeric: Output::empty(),
            min_special: Output::empty(),
            min_upper: Output::empty(),
            number: Output::empty(),
            numeric: Output::empty(),
            override_special: Output::empty(),
            special: Output::empty(),
            upper: Output::empty(),
        },
    );

    // Tests preview behaviour for unknown fields
    let t = random_string.result.map(|s| format!("Result: {s}"));

    // Tests number mapping
    let number = random_string.min_upper.map(|i| i * 2);

    let val1 = Output::new(&1);
    let val2 = Output::new(&"abc".to_string());

    // Outputs can be reused
    let combined = Output::combine2(val1, val2);
    let combined_2 = Output::combine2(val1, val2);

    let combined_string = combined.map(|values| format!("Values: {values:?}"));
    let combined_2_string = combined_2.map(|values| format!("Values: {values:?}"));

    add_export("result", &random_string.result);
    add_export("transformed_result", &t);
    add_export("number", &number);
    add_export("combined_string", &combined_string);
    add_export("combined_2_string", &combined_2_string);
    Ok(())
}
