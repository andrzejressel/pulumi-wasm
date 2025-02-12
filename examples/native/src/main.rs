use anyhow::Result;
use pulumi_gestalt_providers_random::random_string;
use pulumi_gestalt_providers_random::random_string::RandomStringArgs;
use pulumi_gestalt_rust::GestaltContext;
use pulumi_gestalt_rust::GestaltOutput;
use pulumi_gestalt_rust::ToOutput;
use pulumi_gestalt_rust::{add_export, pulumi_combine, pulumi_format, Context, Output};

pulumi_gestalt_rust::pulumi_main!();

fn pulumi_main(context: &Context) -> Result<()> {
    let length: Output<i32> = context.new_output(&12).map(|i: i32| i * 3);

    let random_string = random_string::create(
        context,
        "test",
        RandomStringArgs::builder().length(length).build_struct(),
    );

    // Tests preview behavior for unknown fields
    let t = random_string.result.map(|s| format!("Result: {s}"));

    // Tests number mapping
    let number = random_string.min_upper.map(|i| i * 2);

    // Optional values are deserialized as None
    let keepers = random_string.keepers.map(|map| format!("Keepers: {map:?}"));

    let val1 = context.new_output(&1);
    let val2 = context.new_output(&"abc".to_string());

    // Outputs can be reused
    let combined = pulumi_combine!(val1.clone(), val2.clone());
    let combined_2 = pulumi_combine!(val1, val2);

    let combined_string = pulumi_format!(&context, "Values: {:?}", combined);
    let combined_2_string = pulumi_format!(&context, "Values: {:?}", combined_2);

    let random_string_2 = random_string::create(
        context,
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
