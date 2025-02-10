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
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod random_integer {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct RandomIntegerArgs {
        /// Arbitrary map of values that, when changed, will trigger recreation of resource. See the main provider documentation for more information.
        #[builder(into, default)]
        pub keepers: pulumi_gestalt_rust::InputOrOutput<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// The maximum inclusive value of the range.
        #[builder(into)]
        pub max: pulumi_gestalt_rust::InputOrOutput<i32>,
        /// The minimum inclusive value of the range.
        #[builder(into)]
        pub min: pulumi_gestalt_rust::InputOrOutput<i32>,
        /// A custom seed to always produce the same value.
        #[builder(into, default)]
        pub seed: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
    }
    #[allow(dead_code)]
    pub struct RandomIntegerResult {
        /// Arbitrary map of values that, when changed, will trigger recreation of resource. See the main provider documentation for more information.
        pub keepers: pulumi_gestalt_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// The maximum inclusive value of the range.
        pub max: pulumi_gestalt_rust::Output<i32>,
        /// The minimum inclusive value of the range.
        pub min: pulumi_gestalt_rust::Output<i32>,
        /// The random integer result.
        pub result: pulumi_gestalt_rust::Output<i32>,
        /// A custom seed to always produce the same value.
        pub seed: pulumi_gestalt_rust::Output<Option<String>>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: RandomIntegerArgs,
    ) -> RandomIntegerResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let keepers_binding = args.keepers.get_output(context);
        let max_binding = args.max.get_output(context);
        let min_binding = args.min.get_output(context);
        let seed_binding = args.seed.get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "random:index/randomInteger:RandomInteger".into(),
            name: name.to_string(),
            version: super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "keepers".into(),
                    value: keepers_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "max".into(),
                    value: max_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "min".into(),
                    value: min_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "seed".into(),
                    value: seed_binding.get_id(),
                },
            ],
        };
        let o = context.register_resource(request);
        RandomIntegerResult {
            keepers: o.get_field("keepers"),
            max: o.get_field("max"),
            min: o.get_field("min"),
            result: o.get_field("result"),
            seed: o.get_field("seed"),
        }
    }
}
