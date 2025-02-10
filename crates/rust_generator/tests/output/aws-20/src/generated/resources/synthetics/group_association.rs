/// Provides a Synthetics Group Association resource.
///
/// ## Example Usage
///
/// ### Basic Usage
///
/// ```ignore
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let example = group_association::create(
///         "example",
///         GroupAssociationArgs::builder()
///             .canary_arn("${exampleAwsSyntheticsCanary.arn}")
///             .group_name("${exampleAwsSyntheticsGroup.name}")
///             .build_struct(),
///     );
/// }
/// ```
///
/// ## Import
///
/// Using `pulumi import`, import CloudWatch Synthetics Group Association using the `canary_arn,group_name`. For example:
///
/// ```sh
/// $ pulumi import aws:synthetics/groupAssociation:GroupAssociation example arn:aws:synthetics:us-west-2:123456789012:canary:tf-acc-test-abcd1234,examplename
/// ```
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod group_association {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GroupAssociationArgs {
        /// ARN of the canary.
        #[builder(into)]
        pub canary_arn: pulumi_gestalt_rust::InputOrOutput<String>,
        /// Name of the group that the canary will be associated with.
        #[builder(into)]
        pub group_name: pulumi_gestalt_rust::InputOrOutput<String>,
    }
    #[allow(dead_code)]
    pub struct GroupAssociationResult {
        /// ARN of the canary.
        pub canary_arn: pulumi_gestalt_rust::Output<String>,
        pub group_arn: pulumi_gestalt_rust::Output<String>,
        /// ID of the Group.
        pub group_id: pulumi_gestalt_rust::Output<String>,
        /// Name of the group that the canary will be associated with.
        pub group_name: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: GroupAssociationArgs,
    ) -> GroupAssociationResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let canary_arn_binding = args.canary_arn.get_output(context);
        let group_name_binding = args.group_name.get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "aws:synthetics/groupAssociation:GroupAssociation".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "canaryArn".into(),
                    value: canary_arn_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "groupName".into(),
                    value: group_name_binding.get_id(),
                },
            ],
        };
        let o = context.register_resource(request);
        GroupAssociationResult {
            canary_arn: o.get_field("canaryArn"),
            group_arn: o.get_field("groupArn"),
            group_id: o.get_field("groupId"),
            group_name: o.get_field("groupName"),
        }
    }
}
