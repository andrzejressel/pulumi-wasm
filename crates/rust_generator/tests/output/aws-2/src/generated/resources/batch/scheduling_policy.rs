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
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod scheduling_policy {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct SchedulingPolicyArgs {
        #[builder(into, default)]
        pub fair_share_policy: pulumi_gestalt_rust::InputOrOutput<
            Option<super::super::types::batch::SchedulingPolicyFairSharePolicy>,
        >,
        /// Specifies the name of the scheduling policy.
        #[builder(into, default)]
        pub name: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Key-value map of resource tags. If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
        #[builder(into, default)]
        pub tags: pulumi_gestalt_rust::InputOrOutput<
            Option<std::collections::HashMap<String, String>>,
        >,
    }
    #[allow(dead_code)]
    pub struct SchedulingPolicyResult {
        /// The Amazon Resource Name of the scheduling policy.
        pub arn: pulumi_gestalt_rust::Output<String>,
        pub fair_share_policy: pulumi_gestalt_rust::Output<
            Option<super::super::types::batch::SchedulingPolicyFairSharePolicy>,
        >,
        /// Specifies the name of the scheduling policy.
        pub name: pulumi_gestalt_rust::Output<String>,
        /// Key-value map of resource tags. If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
        pub tags: pulumi_gestalt_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// A map of tags assigned to the resource, including those inherited from the provider `default_tags` configuration block.
        pub tags_all: pulumi_gestalt_rust::Output<
            std::collections::HashMap<String, String>,
        >,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: SchedulingPolicyArgs,
    ) -> SchedulingPolicyResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let fair_share_policy_binding = args.fair_share_policy.get_output(context);
        let name_binding = args.name.get_output(context);
        let tags_binding = args.tags.get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "aws:batch/schedulingPolicy:SchedulingPolicy".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "fairSharePolicy".into(),
                    value: &fair_share_policy_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "name".into(),
                    value: &name_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "tags".into(),
                    value: &tags_binding.drop_type(),
                },
            ],
        };
        let o = context.register_resource(request);
        SchedulingPolicyResult {
            arn: o.get_field("arn"),
            fair_share_policy: o.get_field("fairSharePolicy"),
            name: o.get_field("name"),
            tags: o.get_field("tags"),
            tags_all: o.get_field("tagsAll"),
        }
    }
}
