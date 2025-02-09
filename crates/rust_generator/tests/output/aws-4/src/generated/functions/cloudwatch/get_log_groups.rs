#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod get_log_groups {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GetLogGroupsArgs {
        /// Group prefix of the Cloudwatch log groups to list
        #[builder(into, default)]
        pub log_group_name_prefix: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
    }
    #[allow(dead_code)]
    pub struct GetLogGroupsResult {
        /// Set of ARNs of the Cloudwatch log groups
        pub arns: pulumi_gestalt_rust::Output<Vec<String>>,
        /// The provider-assigned unique ID for this managed resource.
        pub id: pulumi_gestalt_rust::Output<String>,
        pub log_group_name_prefix: pulumi_gestalt_rust::Output<Option<String>>,
        /// Set of names of the Cloudwatch log groups
        pub log_group_names: pulumi_gestalt_rust::Output<Vec<String>>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn invoke(
        context: &pulumi_gestalt_rust::Context,
        args: GetLogGroupsArgs,
    ) -> GetLogGroupsResult {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::register_interface;
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let log_group_name_prefix_binding = args
            .log_group_name_prefix
            .get_output(context);
        let request = pulumi_gestalt_rust::InvokeResourceRequest {
            token: "aws:cloudwatch/getLogGroups:getLogGroups".into(),
            version: super::super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "logGroupNamePrefix".into(),
                    value: log_group_name_prefix_binding.get_id(),
                },
            ],
        };
        let o = context.invoke_resource(request);
        GetLogGroupsResult {
            arns: o.get_field("arns"),
            id: o.get_field("id"),
            log_group_name_prefix: o.get_field("logGroupNamePrefix"),
            log_group_names: o.get_field("logGroupNames"),
        }
    }
}
