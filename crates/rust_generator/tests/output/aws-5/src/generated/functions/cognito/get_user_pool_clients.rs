#[allow(clippy::doc_lazy_continuation)]
pub mod get_user_pool_clients {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GetUserPoolClientsArgs {
        /// Cognito user pool ID.
        #[builder(into)]
        pub user_pool_id: pulumi_gestalt_rust::InputOrOutput<String>,
    }
    #[allow(dead_code)]
    pub struct GetUserPoolClientsResult {
        /// List of Cognito user pool client IDs.
        pub client_ids: pulumi_gestalt_rust::Output<Vec<String>>,
        /// List of Cognito user pool client names.
        pub client_names: pulumi_gestalt_rust::Output<Vec<String>>,
        /// The provider-assigned unique ID for this managed resource.
        pub id: pulumi_gestalt_rust::Output<String>,
        pub user_pool_id: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn invoke(
        context: &pulumi_gestalt_rust::PulumiContext,
        args: GetUserPoolClientsArgs,
    ) -> GetUserPoolClientsResult {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::register_interface;
        use std::collections::HashMap;
        let user_pool_id_binding = args.user_pool_id.get_output(context).get_inner();
        let request = register_interface::ResourceInvokeRequest {
            token: "aws:cognito/getUserPoolClients:getUserPoolClients".into(),
            version: super::super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "userPoolId".into(),
                    value: &user_pool_id_binding,
                },
            ]),
        };
        let o = register_interface::invoke(context.get_inner(), &request);
        GetUserPoolClientsResult {
            client_ids: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("clientIds"),
            ),
            client_names: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("clientNames"),
            ),
            id: pulumi_gestalt_rust::__private::into_domain(o.extract_field("id")),
            user_pool_id: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("userPoolId"),
            ),
        }
    }
}
