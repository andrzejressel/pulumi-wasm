use anyhow::Error;
use pulumi_wasm_random::resource::random_string::{random_string, RandomStringArgs};
use pulumi_wasm_rust::Output;
use pulumi_wasm_rust::{add_export, pulumi_main};

#[pulumi_main]
fn test_main() -> Result<(), Error> {
    let length: Output<i32> = Output::new(&4);
    let random_string_1 = random_string(
        "test_1",
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

    let new_length = random_string_1.result.map(|s| s.len() as i32);

    let random_string_2 = random_string(
        "test_2",
        RandomStringArgs {
            keepers: None.into(),
            length: new_length,
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

    let random_string_3 = random_string(
        "test_3",
        RandomStringArgs {
            keepers: None.into(),
            length: random_string_2.length.map(|i| i * 2),
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

    add_export("result", &random_string_1.result);
    add_export("number_1", &random_string_1.length);
    add_export("number_2", &random_string_2.length);
    add_export("number_3", &random_string_3.length);
    Ok(())
}
