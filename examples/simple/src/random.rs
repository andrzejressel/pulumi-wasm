use pulumi_rust_wasm::output::Output;
use crate::bindings::component::pulumi_wasm::{output_interface, pulumi_provider_random_interface};

pub struct RandomStringArgs<'a> {
    pub(crate) name: &'a str,
    pub(crate) length: Output<i32>,
}

pub fn create_random_string(
    args: RandomStringArgs,
) -> Output<String> {
    let length = clone(args.length);
    let args = pulumi_provider_random_interface::RandomStringArgs {
        name: args.name.into(),
        length: &length,
    };
    let result = pulumi_provider_random_interface::create_random_string(&args);

    random_to_domain_mapper::<String>(result.result)
}

fn random_to_domain_mapper<F: serde::Serialize>(random: pulumi_provider_random_interface::Output) -> Output<F> {
    unsafe {
        let inner = random.take_handle();
        Output::<F>::new_from_handle(inner)
        // output_interface::Output::from_handle(inner)
    }
}

fn clone<T>(output: Output<T>) -> output_interface::Output {
    unsafe {
        let inner = output.get_inner();
        let cloned = inner.duplicate();
        pulumi_provider_random_interface::Output::from_handle(cloned.take_handle())
    }
}