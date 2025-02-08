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
        context: &pulumi_gestalt_rust::PulumiContext,
        args: GetUserGroupArgs,
    ) -> GetUserGroupResult {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::register_interface;
        use std::collections::HashMap;
        let name_binding = args.name.get_output(context).get_inner();
        let user_pool_id_binding = args.user_pool_id.get_output(context).get_inner();
        let request = register_interface::ResourceInvokeRequest {
            token: "aws:cognito/getUserGroup:getUserGroup".into(),
            version: super::super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "name".into(),
                    value: &name_binding,
                },
                register_interface::ObjectField {
                    name: "userPoolId".into(),
                    value: &user_pool_id_binding,
                },
            ]),
        };
        let o = register_interface::invoke(context.get_inner(), &request);
        GetUserGroupResult {
            description: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("description"),
            ),
            id: pulumi_gestalt_rust::__private::into_domain(o.extract_field("id")),
            name: pulumi_gestalt_rust::__private::into_domain(o.extract_field("name")),
            precedence: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("precedence"),
            ),
            role_arn: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("roleArn"),
            ),
            user_pool_id: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("userPoolId"),
            ),
        }
    }
}
