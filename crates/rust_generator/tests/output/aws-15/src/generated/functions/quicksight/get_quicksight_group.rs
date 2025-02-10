#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod get_quicksight_group {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GetQuicksightGroupArgs {
        /// AWS account ID.
        #[builder(into, default)]
        pub aws_account_id: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The name of the group that you want to match.
        ///
        /// The following arguments are optional:
        #[builder(into)]
        pub group_name: pulumi_gestalt_rust::InputOrOutput<String>,
        /// QuickSight namespace. Defaults to `default`.
        #[builder(into, default)]
        pub namespace: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
    }
    #[allow(dead_code)]
    pub struct GetQuicksightGroupResult {
        /// The Amazon Resource Name (ARN) for the group.
        pub arn: pulumi_gestalt_rust::Output<String>,
        pub aws_account_id: pulumi_gestalt_rust::Output<String>,
        /// The group description.
        pub description: pulumi_gestalt_rust::Output<String>,
        pub group_name: pulumi_gestalt_rust::Output<String>,
        /// The provider-assigned unique ID for this managed resource.
        pub id: pulumi_gestalt_rust::Output<String>,
        pub namespace: pulumi_gestalt_rust::Output<Option<String>>,
        /// The principal ID of the group.
        pub principal_id: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn invoke(
        context: &pulumi_gestalt_rust::Context,
        args: GetQuicksightGroupArgs,
    ) -> GetQuicksightGroupResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let aws_account_id_binding = args.aws_account_id.get_output(context);
        let group_name_binding = args.group_name.get_output(context);
        let namespace_binding = args.namespace.get_output(context);
        let request = pulumi_gestalt_rust::InvokeResourceRequest {
            token: "aws:quicksight/getQuicksightGroup:getQuicksightGroup".into(),
            version: super::super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "awsAccountId".into(),
                    value: aws_account_id_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "groupName".into(),
                    value: group_name_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "namespace".into(),
                    value: namespace_binding.get_id(),
                },
            ],
        };
        let o = context.invoke_resource(request);
        GetQuicksightGroupResult {
            arn: o.get_field("arn"),
            aws_account_id: o.get_field("awsAccountId"),
            description: o.get_field("description"),
            group_name: o.get_field("groupName"),
            id: o.get_field("id"),
            namespace: o.get_field("namespace"),
            principal_id: o.get_field("principalId"),
        }
    }
}
