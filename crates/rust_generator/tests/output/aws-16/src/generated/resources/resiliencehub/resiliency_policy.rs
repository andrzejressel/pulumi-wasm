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
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod resiliency_policy {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct ResiliencyPolicyArgs {
        /// Data Location Constraint of the Policy.
        /// Valid values are `AnyLocation`, `SameContinent`, and `SameCountry`.
        #[builder(into, default)]
        pub data_location_constraint: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Description of Resiliency Policy.
        #[builder(into, default)]
        pub description: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Name of Resiliency Policy.
        /// Must be between 2 and 60 characters long.
        /// Must start with an alphanumeric character and contain alphanumeric characters, underscores, or hyphens.
        #[builder(into, default)]
        pub name: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The type of resiliency policy to be created, including the recovery time objective (RTO) and recovery point objective (RPO) in seconds. See `policy`.
        ///
        /// The following arguments are optional:
        #[builder(into, default)]
        pub policy: pulumi_gestalt_rust::InputOrOutput<
            Option<super::super::types::resiliencehub::ResiliencyPolicyPolicy>,
        >,
        /// A map of tags to assign to the resource. If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
        #[builder(into, default)]
        pub tags: pulumi_gestalt_rust::InputOrOutput<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// Resiliency Policy Tier.
        /// Valid values are `MissionCritical`, `Critical`, `Important`, `CoreServices`, `NonCritical`, and `NotApplicable`.
        #[builder(into)]
        pub tier: pulumi_gestalt_rust::InputOrOutput<String>,
        #[builder(into, default)]
        pub timeouts: pulumi_gestalt_rust::InputOrOutput<
            Option<super::super::types::resiliencehub::ResiliencyPolicyTimeouts>,
        >,
    }
    #[allow(dead_code)]
    pub struct ResiliencyPolicyResult {
        /// ARN of the Resiliency Policy.
        pub arn: pulumi_gestalt_rust::Output<String>,
        /// Data Location Constraint of the Policy.
        /// Valid values are `AnyLocation`, `SameContinent`, and `SameCountry`.
        pub data_location_constraint: pulumi_gestalt_rust::Output<String>,
        /// Description of Resiliency Policy.
        pub description: pulumi_gestalt_rust::Output<Option<String>>,
        /// Estimated Cost Tier of the Resiliency Policy.
        pub estimated_cost_tier: pulumi_gestalt_rust::Output<String>,
        /// Name of Resiliency Policy.
        /// Must be between 2 and 60 characters long.
        /// Must start with an alphanumeric character and contain alphanumeric characters, underscores, or hyphens.
        pub name: pulumi_gestalt_rust::Output<String>,
        /// The type of resiliency policy to be created, including the recovery time objective (RTO) and recovery point objective (RPO) in seconds. See `policy`.
        ///
        /// The following arguments are optional:
        pub policy: pulumi_gestalt_rust::Output<
            Option<super::super::types::resiliencehub::ResiliencyPolicyPolicy>,
        >,
        /// A map of tags to assign to the resource. If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
        pub tags: pulumi_gestalt_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// A map of tags assigned to the resource, including those inherited from the provider `default_tags` configuration block.
        pub tags_all: pulumi_gestalt_rust::Output<
            std::collections::HashMap<String, String>,
        >,
        /// Resiliency Policy Tier.
        /// Valid values are `MissionCritical`, `Critical`, `Important`, `CoreServices`, `NonCritical`, and `NotApplicable`.
        pub tier: pulumi_gestalt_rust::Output<String>,
        pub timeouts: pulumi_gestalt_rust::Output<
            Option<super::super::types::resiliencehub::ResiliencyPolicyTimeouts>,
        >,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: ResiliencyPolicyArgs,
    ) -> ResiliencyPolicyResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let data_location_constraint_binding = args
            .data_location_constraint
            .get_output(context);
        let description_binding = args.description.get_output(context);
        let name_binding = args.name.get_output(context);
        let policy_binding = args.policy.get_output(context);
        let tags_binding = args.tags.get_output(context);
        let tier_binding = args.tier.get_output(context);
        let timeouts_binding = args.timeouts.get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "aws:resiliencehub/resiliencyPolicy:ResiliencyPolicy".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "dataLocationConstraint".into(),
                    value: data_location_constraint_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "description".into(),
                    value: description_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "name".into(),
                    value: name_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "policy".into(),
                    value: policy_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "tags".into(),
                    value: tags_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "tier".into(),
                    value: tier_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "timeouts".into(),
                    value: timeouts_binding.get_id(),
                },
            ],
        };
        let o = context.register_resource(request);
        ResiliencyPolicyResult {
            arn: o.get_field("arn"),
            data_location_constraint: o.get_field("dataLocationConstraint"),
            description: o.get_field("description"),
            estimated_cost_tier: o.get_field("estimatedCostTier"),
            name: o.get_field("name"),
            policy: o.get_field("policy"),
            tags: o.get_field("tags"),
            tags_all: o.get_field("tagsAll"),
            tier: o.get_field("tier"),
            timeouts: o.get_field("timeouts"),
        }
    }
}
