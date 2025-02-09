#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod get_user_group {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GetUserGroupArgs {
        /// Name of the user group.
        #[builder(into)]
        pub name: pulumi_gestalt_rust::InputOrOutput<String>,
        /// User pool the client belongs to.
        #[builder(into)]
        pub user_pool_id: pulumi_gestalt_rust::InputOrOutput<String>,
    }
    #[allow(dead_code)]
    pub struct GetUserGroupResult {
        /// Description of the user group.
        pub description: pulumi_gestalt_rust::Output<String>,
        /// A comma-delimited string concatenating `name` and `user_pool_id`.
        pub id: pulumi_gestalt_rust::Output<String>,
        pub name: pulumi_gestalt_rust::Output<String>,
        /// Precedence of the user group.
        pub precedence: pulumi_gestalt_rust::Output<i32>,
        /// ARN of the IAM role to be associated with the user group.
        pub role_arn: pulumi_gestalt_rust::Output<String>,
        pub user_pool_id: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn invoke(
        context: &pulumi_gestalt_rust::Context,
        args: GetUserGroupArgs,
    ) -> GetUserGroupResult {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::register_interface;
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let name_binding = args.name.get_output(context);
        let user_pool_id_binding = args.user_pool_id.get_output(context);
        let request = pulumi_gestalt_rust::InvokeResourceRequest {
            token: "aws:cognito/getUserGroup:getUserGroup".into(),
            version: super::super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "name".into(),
                    value: name_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "userPoolId".into(),
                    value: user_pool_id_binding.get_id(),
                },
            ],
        };
        let o = context.invoke_resource(request);
        GetUserGroupResult {
            description: o.get_field("description"),
            id: o.get_field("id"),
            name: o.get_field("name"),
            precedence: o.get_field("precedence"),
            role_arn: o.get_field("roleArn"),
            user_pool_id: o.get_field("userPoolId"),
        }
    }
}
