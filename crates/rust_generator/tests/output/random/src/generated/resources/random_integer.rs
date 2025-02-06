/// The resource `random.RandomInteger` generates random values from a given range, described by the `min` and `max` attributes of a given resource.
///
/// This resource can be used in conjunction with resources that have the `create_before_destroy` lifecycle flag set, to avoid conflicts with unique names during the brief period where both the old and new resources exist concurrently.
///
/// ## Example Usage
///
/// ```yaml
/// resources:
///   # The following example shows how to generate a random priority
///   # between 1 and 50000 for a aws_alb_listener_rule resource:
///   priority:
///     type: random:RandomInteger
///     properties:
///       min: 1
///       max: 50000
///       keepers:
///         listener_arn: ${var.listener_arn}
///   main:
///     type: aws:alb:ListenerRule
///     properties:
///       listenerArn: ${priority.keepers.listenerArn}
///       priority: ${priority.result}
///       actions:
///         - type: forward
///           targetGroupArn: ${var.target_group_arn}
/// ```
///
/// ## Import
///
/// Random integers can be imported using the result, min, and max, with an optional seed. This can be used to replace a config value with a value interpolated from the random provider without experiencing diffs. Example (values are separated by a ,)
///
/// ```sh
///  $ pulumi import random:index/randomInteger:RandomInteger priority 15390,1,50000
/// ```
///
///
pub mod random_integer {
    #[derive(pulumi_wasm_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct RandomIntegerArgs {
        /// Arbitrary map of values that, when changed, will trigger recreation of resource. See the main provider documentation for more information.
        #[builder(into, default)]
        pub keepers: pulumi_wasm_rust::InputOrOutput<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// The maximum inclusive value of the range.
        #[builder(into)]
        pub max: pulumi_wasm_rust::InputOrOutput<i32>,
        /// The minimum inclusive value of the range.
        #[builder(into)]
        pub min: pulumi_wasm_rust::InputOrOutput<i32>,
        /// A custom seed to always produce the same value.
        #[builder(into, default)]
        pub seed: pulumi_wasm_rust::InputOrOutput<Option<String>>,
    }
    #[allow(dead_code)]
    pub struct RandomIntegerResult {
        /// Arbitrary map of values that, when changed, will trigger recreation of resource. See the main provider documentation for more information.
        pub keepers: pulumi_wasm_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// The maximum inclusive value of the range.
        pub max: pulumi_wasm_rust::Output<i32>,
        /// The minimum inclusive value of the range.
        pub min: pulumi_wasm_rust::Output<i32>,
        /// The random integer result.
        pub result: pulumi_wasm_rust::Output<i32>,
        /// A custom seed to always produce the same value.
        pub seed: pulumi_wasm_rust::Output<Option<String>>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_wasm_rust::PulumiContext,
        name: &str,
        args: RandomIntegerArgs,
    ) -> RandomIntegerResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let keepers_binding = args.keepers.get_output(context).get_inner();
        let max_binding = args.max.get_output(context).get_inner();
        let min_binding = args.min.get_output(context).get_inner();
        let seed_binding = args.seed.get_output(context).get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "random:index/randomInteger:RandomInteger".into(),
            name: name.to_string(),
            version: super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "keepers".into(),
                    value: &keepers_binding,
                },
                register_interface::ObjectField {
                    name: "max".into(),
                    value: &max_binding,
                },
                register_interface::ObjectField {
                    name: "min".into(),
                    value: &min_binding,
                },
                register_interface::ObjectField {
                    name: "seed".into(),
                    value: &seed_binding,
                },
            ]),
        };
        let o = register_interface::register(context.get_inner(), &request);
        RandomIntegerResult {
            keepers: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("keepers"),
            ),
            max: pulumi_wasm_rust::__private::into_domain(o.extract_field("max")),
            min: pulumi_wasm_rust::__private::into_domain(o.extract_field("min")),
            result: pulumi_wasm_rust::__private::into_domain(o.extract_field("result")),
            seed: pulumi_wasm_rust::__private::into_domain(o.extract_field("seed")),
        }
    }
}
