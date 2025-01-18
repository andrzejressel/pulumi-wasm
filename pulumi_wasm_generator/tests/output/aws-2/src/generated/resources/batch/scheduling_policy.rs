/// Provides a Batch Scheduling Policy resource.
///
/// ## Example Usage
///
/// ```yaml
/// resources:
///   example:
///     type: aws:batch:SchedulingPolicy
///     properties:
///       name: example
///       fairSharePolicy:
///         computeReservation: 1
///         shareDecaySeconds: 3600
///         shareDistributions:
///           - shareIdentifier: A1*
///             weightFactor: 0.1
///           - shareIdentifier: A2
///             weightFactor: 0.2
///       tags:
///         Name: Example Batch Scheduling Policy
/// ```
///
/// ## Import
///
/// Using `pulumi import`, import Batch Scheduling Policy using the `arn`. For example:
///
/// ```sh
/// $ pulumi import aws:batch/schedulingPolicy:SchedulingPolicy test_policy arn:aws:batch:us-east-1:123456789012:scheduling-policy/sample
/// ```
pub mod scheduling_policy {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct SchedulingPolicyArgs {
        #[builder(into, default)]
        pub fair_share_policy: pulumi_wasm_rust::Output<
            Option<super::super::types::batch::SchedulingPolicyFairSharePolicy>,
        >,
        /// Specifies the name of the scheduling policy.
        #[builder(into, default)]
        pub name: pulumi_wasm_rust::Output<Option<String>>,
        /// Key-value map of resource tags. If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
        #[builder(into, default)]
        pub tags: pulumi_wasm_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
    }
    #[allow(dead_code)]
    pub struct SchedulingPolicyResult {
        /// The Amazon Resource Name of the scheduling policy.
        pub arn: pulumi_wasm_rust::Output<String>,
        pub fair_share_policy: pulumi_wasm_rust::Output<
            Option<super::super::types::batch::SchedulingPolicyFairSharePolicy>,
        >,
        /// Specifies the name of the scheduling policy.
        pub name: pulumi_wasm_rust::Output<String>,
        /// Key-value map of resource tags. If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
        pub tags: pulumi_wasm_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// A map of tags assigned to the resource, including those inherited from the provider `default_tags` configuration block.
        pub tags_all: pulumi_wasm_rust::Output<
            std::collections::HashMap<String, String>,
        >,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(name: &str, args: SchedulingPolicyArgs) -> SchedulingPolicyResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let fair_share_policy_binding = args.fair_share_policy.get_inner();
        let name_binding = args.name.get_inner();
        let tags_binding = args.tags.get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "aws:batch/schedulingPolicy:SchedulingPolicy".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "fairSharePolicy".into(),
                    value: &fair_share_policy_binding,
                },
                register_interface::ObjectField {
                    name: "name".into(),
                    value: &name_binding,
                },
                register_interface::ObjectField {
                    name: "tags".into(),
                    value: &tags_binding,
                },
            ]),
            results: Vec::from([
                register_interface::ResultField {
                    name: "arn".into(),
                },
                register_interface::ResultField {
                    name: "fairSharePolicy".into(),
                },
                register_interface::ResultField {
                    name: "name".into(),
                },
                register_interface::ResultField {
                    name: "tags".into(),
                },
                register_interface::ResultField {
                    name: "tagsAll".into(),
                },
            ]),
        };
        let o = register_interface::register(&request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        SchedulingPolicyResult {
            arn: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("arn").unwrap(),
            ),
            fair_share_policy: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("fairSharePolicy").unwrap(),
            ),
            name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("name").unwrap(),
            ),
            tags: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("tags").unwrap(),
            ),
            tags_all: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("tagsAll").unwrap(),
            ),
        }
    }
}
