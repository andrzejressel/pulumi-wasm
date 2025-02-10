/// The resource `random.RandomShuffle` generates a random permutation of a list of strings given as an argument.
///
/// ## Example Usage
///
/// ```yaml
/// resources:
///   az:
///     type: random:RandomShuffle
///     properties:
///       inputs:
///         - us-west-1a
///         - us-west-1c
///         - us-west-1d
///         - us-west-1e
///       resultCount: 2
///   example:
///     type: aws:elb:LoadBalancer
///     properties:
///       # Place the ELB in any two of the given availability zones, selected
///       #   # at random.
///       availabilityZones: ${az.results}
/// ```
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod random_shuffle {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct RandomShuffleArgs {
        /// The list of strings to shuffle.
        #[builder(into)]
        pub inputs: pulumi_gestalt_rust::InputOrOutput<Vec<String>>,
        /// Arbitrary map of values that, when changed, will trigger recreation of resource. See the main provider documentation for more information.
        #[builder(into, default)]
        pub keepers: pulumi_gestalt_rust::InputOrOutput<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// The number of results to return. Defaults to the number of items in the `input` list. If fewer items are requested, some elements will be excluded from the result. If more items are requested, items will be repeated in the result but not more frequently than the number of items in the input list.
        #[builder(into, default)]
        pub result_count: pulumi_gestalt_rust::InputOrOutput<Option<i32>>,
        /// Arbitrary string with which to seed the random number generator, in order to produce less-volatile permutations of the list.
        #[builder(into, default)]
        pub seed: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
    }
    #[allow(dead_code)]
    pub struct RandomShuffleResult {
        /// The list of strings to shuffle.
        pub inputs: pulumi_gestalt_rust::Output<Vec<String>>,
        /// Arbitrary map of values that, when changed, will trigger recreation of resource. See the main provider documentation for more information.
        pub keepers: pulumi_gestalt_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// The number of results to return. Defaults to the number of items in the `input` list. If fewer items are requested, some elements will be excluded from the result. If more items are requested, items will be repeated in the result but not more frequently than the number of items in the input list.
        pub result_count: pulumi_gestalt_rust::Output<Option<i32>>,
        /// Random permutation of the list of strings given in `input`.
        pub results: pulumi_gestalt_rust::Output<Vec<String>>,
        /// Arbitrary string with which to seed the random number generator, in order to produce less-volatile permutations of the list.
        pub seed: pulumi_gestalt_rust::Output<Option<String>>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: RandomShuffleArgs,
    ) -> RandomShuffleResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let inputs_binding = args.inputs.get_output(context);
        let keepers_binding = args.keepers.get_output(context);
        let result_count_binding = args.result_count.get_output(context);
        let seed_binding = args.seed.get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "random:index/randomShuffle:RandomShuffle".into(),
            name: name.to_string(),
            version: super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "inputs".into(),
                    value: inputs_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "keepers".into(),
                    value: keepers_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "resultCount".into(),
                    value: result_count_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "seed".into(),
                    value: seed_binding.get_id(),
                },
            ],
        };
        let o = context.register_resource(request);
        RandomShuffleResult {
            inputs: o.get_field("inputs"),
            keepers: o.get_field("keepers"),
            result_count: o.get_field("resultCount"),
            results: o.get_field("results"),
            seed: o.get_field("seed"),
        }
    }
}
