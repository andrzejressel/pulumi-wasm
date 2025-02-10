#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod get_selection {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GetSelectionArgs {
        /// Backup plan ID associated with the selection of resources.
        #[builder(into)]
        pub plan_id: pulumi_gestalt_rust::InputOrOutput<String>,
        /// Backup selection ID.
        #[builder(into)]
        pub selection_id: pulumi_gestalt_rust::InputOrOutput<String>,
    }
    #[allow(dead_code)]
    pub struct GetSelectionResult {
        /// ARN of the IAM role that AWS Backup uses to authenticate when restoring and backing up the target resource. See the [AWS Backup Developer Guide](https://docs.aws.amazon.com/aws-backup/latest/devguide/access-control.html#managed-policies) for additional information about using AWS managed policies or creating custom policies attached to the IAM role.
        pub iam_role_arn: pulumi_gestalt_rust::Output<String>,
        /// The provider-assigned unique ID for this managed resource.
        pub id: pulumi_gestalt_rust::Output<String>,
        /// Display name of a resource selection document.
        pub name: pulumi_gestalt_rust::Output<String>,
        pub plan_id: pulumi_gestalt_rust::Output<String>,
        /// An array of strings that either contain Amazon Resource Names (ARNs) or match patterns of resources to assign to a backup plan..
        pub resources: pulumi_gestalt_rust::Output<Vec<String>>,
        pub selection_id: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn invoke(
        context: &pulumi_gestalt_rust::Context,
        args: GetSelectionArgs,
    ) -> GetSelectionResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let plan_id_binding = args.plan_id.get_output(context);
        let selection_id_binding = args.selection_id.get_output(context);
        let request = pulumi_gestalt_rust::InvokeResourceRequest {
            token: "aws:backup/getSelection:getSelection".into(),
            version: super::super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "planId".into(),
                    value: plan_id_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "selectionId".into(),
                    value: selection_id_binding.get_id(),
                },
            ],
        };
        let o = context.invoke_resource(request);
        GetSelectionResult {
            iam_role_arn: o.get_field("iamRoleArn"),
            id: o.get_field("id"),
            name: o.get_field("name"),
            plan_id: o.get_field("planId"),
            resources: o.get_field("resources"),
            selection_id: o.get_field("selectionId"),
        }
    }
}
