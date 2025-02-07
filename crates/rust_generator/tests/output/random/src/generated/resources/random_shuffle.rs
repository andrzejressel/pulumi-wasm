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
        context: &pulumi_gestalt_rust::PulumiContext,
        name: &str,
        args: RandomShuffleArgs,
    ) -> RandomShuffleResult {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::register_interface;
        use std::collections::HashMap;
        let inputs_binding = args.inputs.get_output(context).get_inner();
        let keepers_binding = args.keepers.get_output(context).get_inner();
        let result_count_binding = args.result_count.get_output(context).get_inner();
        let seed_binding = args.seed.get_output(context).get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "random:index/randomShuffle:RandomShuffle".into(),
            name: name.to_string(),
            version: super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "inputs".into(),
                    value: &inputs_binding,
                },
                register_interface::ObjectField {
                    name: "keepers".into(),
                    value: &keepers_binding,
                },
                register_interface::ObjectField {
                    name: "resultCount".into(),
                    value: &result_count_binding,
                },
                register_interface::ObjectField {
                    name: "seed".into(),
                    value: &seed_binding,
                },
            ]),
        };
        let o = register_interface::register(context.get_inner(), &request);
        RandomShuffleResult {
            inputs: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("inputs"),
            ),
            keepers: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("keepers"),
            ),
            result_count: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("resultCount"),
            ),
            results: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("results"),
            ),
            seed: pulumi_gestalt_rust::__private::into_domain(o.extract_field("seed")),
        }
    }
}
