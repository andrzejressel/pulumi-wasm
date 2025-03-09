use pulumi_gestalt_rust_integration::{
    Context, InvokeResourceRequest, ObjectField, RegisterResourceRequest,
};

fn generate_random_value(ctx: &Context) {
    let output = ctx.create_output("16".to_string(), false);
    let inputs = vec![ObjectField {
        name: "length".to_string(),
        value: &output,
    }];

    let register_resource_request = RegisterResourceRequest {
        type_: "random:index/randomString:RandomString".to_string(),
        name: "my_name".to_string(),
        version: "4.15.1".to_string(),
        inputs: &inputs,
    };

    let composite_output = ctx.register_resource(register_resource_request);
    let output_result = composite_output.get_field("result".to_string());
    output_result.add_export("result".to_string());
}

fn run_command(ctx: &Context) {
    let output = ctx.create_output("\"whoami\"".to_string(), false);

    let inputs = vec![ObjectField {
        name: "command".to_string(),
        value: &output,
    }];

    let register_resource_request = InvokeResourceRequest {
        token: "command:local:run".to_string(),
        version: "1.0.2".to_string(),
        inputs: &inputs,
    };

    let compose_output = ctx.invoke_resource(register_resource_request);

    let stdout_output = compose_output.get_field("stdout".to_string());

    stdout_output.add_export("whoami_stdout".to_string());
}

fn perform_operations_on_outputs(ctx: &Context) {
    let output = ctx.create_output("16".to_string(), false);

    let output_2 = output.map(Box::new(|s| {
        let i = s.parse::<i32>().unwrap();
        (i * 2).to_string()
    }));
    let output_3 = output_2.map(Box::new(|_| "\"my_string\"".to_string()));

    let output_4 = output.combine(&[&output_2, &output_3]);

    output_2.add_export("double_length".to_string());
    output_3.add_export("static_string".to_string());
    output_4.add_export("combined".to_string());
}

fn main() {
    let ctx = Context::create_context();

    generate_random_value(&ctx);
    run_command(&ctx);
    perform_operations_on_outputs(&ctx);

    ctx.finish();
}
