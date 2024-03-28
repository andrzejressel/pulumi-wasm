use crate::bindings::component::pulumi_wasm::{output_interface, pulumi_provider_random_interface};
use pulumi_wasm_rust::output::Output;

pub struct RandomStringArgs<'a> {
    pub name: &'a str,
    pub length: Output<i32>,
}

pub struct RandomString {
    pub result: Output<String>,
}

pub fn create_random_string(args: RandomStringArgs) -> RandomString {
    let length = clone(args.length);
    let args = pulumi_provider_random_interface::RandomStringArgs {
        name: args.name.into(),
        length: &length,
    };
    let result = pulumi_provider_random_interface::create_random_string(&args);

    RandomString {
        result: random_to_domain_mapper(result.result),
    }
}

fn random_to_domain_mapper<F: serde::Serialize>(
    random: pulumi_provider_random_interface::Output,
) -> Output<F> {
    unsafe {
        let inner = random.take_handle();
        Output::<F>::new_from_handle(inner)
    }
}

fn clone<T>(output: Output<T>) -> output_interface::Output {
    unsafe {
        let inner = output.get_inner();
        let cloned = inner.duplicate();
        pulumi_provider_random_interface::Output::from_handle(cloned.take_handle())
    }
}
