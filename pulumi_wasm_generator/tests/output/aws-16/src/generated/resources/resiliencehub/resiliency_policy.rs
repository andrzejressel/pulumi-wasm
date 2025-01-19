/// Resource for managing an AWS Resilience Hub Resiliency Policy.
///
/// ## Example Usage
///
/// ```yaml
/// resources:
///   example:
///     type: aws:resiliencehub:ResiliencyPolicy
///     properties:
///       policyName: testexample
///       policyDescription: testexample
///       tier: NonCritical
///       dataLocationConstraint: AnyLocation
///       policy:
///         - region:
///             - rpo: 24h
///               rto: 24h
///           az:
///             - rpo: 24h
///               rto: 24h
///           hardware:
///             - rpo: 24h
///               rto: 24h
///           software:
///             - rpo: 24h
///               rto: 24h
/// ```
///
/// ## Import
///
/// Using `pulumi import`, import Resilience Hub Resiliency Policy using the `arn`. For example:
///
/// ```sh
/// $ pulumi import aws:resiliencehub/resiliencyPolicy:ResiliencyPolicy example arn:aws:resiliencehub:us-east-1:123456789012:resiliency-policy/8c1cfa29-d1dd-4421-aa68-c9f64cced4c2
/// ```
pub mod resiliency_policy {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct ResiliencyPolicyArgs {
        /// Data Location Constraint of the Policy.
        /// Valid values are `AnyLocation`, `SameContinent`, and `SameCountry`.
        #[builder(into, default)]
        pub data_location_constraint: pulumi_wasm_rust::Output<Option<String>>,
        /// Description of Resiliency Policy.
        #[builder(into, default)]
        pub description: pulumi_wasm_rust::Output<Option<String>>,
        /// Name of Resiliency Policy.
        /// Must be between 2 and 60 characters long.
        /// Must start with an alphanumeric character and contain alphanumeric characters, underscores, or hyphens.
        #[builder(into, default)]
        pub name: pulumi_wasm_rust::Output<Option<String>>,
        /// The type of resiliency policy to be created, including the recovery time objective (RTO) and recovery point objective (RPO) in seconds. See `policy`.
        ///
        /// The following arguments are optional:
        #[builder(into, default)]
        pub policy: pulumi_wasm_rust::Output<
            Option<super::super::types::resiliencehub::ResiliencyPolicyPolicy>,
        >,
        /// A map of tags to assign to the resource. If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
        #[builder(into, default)]
        pub tags: pulumi_wasm_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// Resiliency Policy Tier.
        /// Valid values are `MissionCritical`, `Critical`, `Important`, `CoreServices`, `NonCritical`, and `NotApplicable`.
        #[builder(into)]
        pub tier: pulumi_wasm_rust::Output<String>,
        #[builder(into, default)]
        pub timeouts: pulumi_wasm_rust::Output<
            Option<super::super::types::resiliencehub::ResiliencyPolicyTimeouts>,
        >,
    }
    #[allow(dead_code)]
    pub struct ResiliencyPolicyResult {
        /// ARN of the Resiliency Policy.
        pub arn: pulumi_wasm_rust::Output<String>,
        /// Data Location Constraint of the Policy.
        /// Valid values are `AnyLocation`, `SameContinent`, and `SameCountry`.
        pub data_location_constraint: pulumi_wasm_rust::Output<String>,
        /// Description of Resiliency Policy.
        pub description: pulumi_wasm_rust::Output<Option<String>>,
        /// Estimated Cost Tier of the Resiliency Policy.
        pub estimated_cost_tier: pulumi_wasm_rust::Output<String>,
        /// Name of Resiliency Policy.
        /// Must be between 2 and 60 characters long.
        /// Must start with an alphanumeric character and contain alphanumeric characters, underscores, or hyphens.
        pub name: pulumi_wasm_rust::Output<String>,
        /// The type of resiliency policy to be created, including the recovery time objective (RTO) and recovery point objective (RPO) in seconds. See `policy`.
        ///
        /// The following arguments are optional:
        pub policy: pulumi_wasm_rust::Output<
            Option<super::super::types::resiliencehub::ResiliencyPolicyPolicy>,
        >,
        /// A map of tags to assign to the resource. If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
        pub tags: pulumi_wasm_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// A map of tags assigned to the resource, including those inherited from the provider `default_tags` configuration block.
        pub tags_all: pulumi_wasm_rust::Output<
            std::collections::HashMap<String, String>,
        >,
        /// Resiliency Policy Tier.
        /// Valid values are `MissionCritical`, `Critical`, `Important`, `CoreServices`, `NonCritical`, and `NotApplicable`.
        pub tier: pulumi_wasm_rust::Output<String>,
        pub timeouts: pulumi_wasm_rust::Output<
            Option<super::super::types::resiliencehub::ResiliencyPolicyTimeouts>,
        >,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(name: &str, args: ResiliencyPolicyArgs) -> ResiliencyPolicyResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let data_location_constraint_binding = args.data_location_constraint.get_inner();
        let description_binding = args.description.get_inner();
        let name_binding = args.name.get_inner();
        let policy_binding = args.policy.get_inner();
        let tags_binding = args.tags.get_inner();
        let tier_binding = args.tier.get_inner();
        let timeouts_binding = args.timeouts.get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "aws:resiliencehub/resiliencyPolicy:ResiliencyPolicy".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "dataLocationConstraint".into(),
                    value: &data_location_constraint_binding,
                },
                register_interface::ObjectField {
                    name: "description".into(),
                    value: &description_binding,
                },
                register_interface::ObjectField {
                    name: "name".into(),
                    value: &name_binding,
                },
                register_interface::ObjectField {
                    name: "policy".into(),
                    value: &policy_binding,
                },
                register_interface::ObjectField {
                    name: "tags".into(),
                    value: &tags_binding,
                },
                register_interface::ObjectField {
                    name: "tier".into(),
                    value: &tier_binding,
                },
                register_interface::ObjectField {
                    name: "timeouts".into(),
                    value: &timeouts_binding,
                },
            ]),
            results: Vec::from([
                register_interface::ResultField {
                    name: "arn".into(),
                },
                register_interface::ResultField {
                    name: "dataLocationConstraint".into(),
                },
                register_interface::ResultField {
                    name: "description".into(),
                },
                register_interface::ResultField {
                    name: "estimatedCostTier".into(),
                },
                register_interface::ResultField {
                    name: "name".into(),
                },
                register_interface::ResultField {
                    name: "policy".into(),
                },
                register_interface::ResultField {
                    name: "tags".into(),
                },
                register_interface::ResultField {
                    name: "tagsAll".into(),
                },
                register_interface::ResultField {
                    name: "tier".into(),
                },
                register_interface::ResultField {
                    name: "timeouts".into(),
                },
            ]),
        };
        let o = register_interface::register(&request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        ResiliencyPolicyResult {
            arn: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("arn").unwrap(),
            ),
            data_location_constraint: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("dataLocationConstraint").unwrap(),
            ),
            description: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("description").unwrap(),
            ),
            estimated_cost_tier: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("estimatedCostTier").unwrap(),
            ),
            name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("name").unwrap(),
            ),
            policy: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("policy").unwrap(),
            ),
            tags: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("tags").unwrap(),
            ),
            tags_all: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("tagsAll").unwrap(),
            ),
            tier: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("tier").unwrap(),
            ),
            timeouts: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("timeouts").unwrap(),
            ),
        }
    }
}
