#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod get_permission_sets {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GetPermissionSetsArgs {
        /// ARN of the SSO Instance associated with the permission set.
        #[builder(into)]
        pub instance_arn: pulumi_gestalt_rust::InputOrOutput<String>,
    }
    #[allow(dead_code)]
    pub struct GetPermissionSetsResult {
        /// Set of string contain the ARN of all Permission Sets.
        pub arns: pulumi_gestalt_rust::Output<Vec<String>>,
        pub id: pulumi_gestalt_rust::Output<String>,
        pub instance_arn: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn invoke(
        context: &pulumi_gestalt_rust::Context,
        args: GetPermissionSetsArgs,
    ) -> GetPermissionSetsResult {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::register_interface;
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let instance_arn_binding = args.instance_arn.get_output(context);
        let request = pulumi_gestalt_rust::InvokeResourceRequest {
            token: "aws:ssoadmin/getPermissionSets:getPermissionSets".into(),
            version: super::super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "instanceArn".into(),
                    value: instance_arn_binding.get_id(),
                },
            ],
        };
        let o = context.invoke_resource(request);
        GetPermissionSetsResult {
            arns: o.get_field("arns"),
            id: o.get_field("id"),
            instance_arn: o.get_field("instanceArn"),
        }
    }
}
