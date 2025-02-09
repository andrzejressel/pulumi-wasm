#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod get_user {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GetUserArgs {
        /// The Name of the API Management Service in which this User exists.
        #[builder(into)]
        pub api_management_name: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The Name of the Resource Group in which the API Management Service exists.
        #[builder(into)]
        pub resource_group_name: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The Identifier for the User.
        #[builder(into)]
        pub user_id: pulumi_gestalt_rust::InputOrOutput<String>,
    }
    #[allow(dead_code)]
    pub struct GetUserResult {
        pub api_management_name: pulumi_gestalt_rust::Output<String>,
        /// The Email Address used for this User.
        pub email: pulumi_gestalt_rust::Output<String>,
        /// The First Name for the User.
        pub first_name: pulumi_gestalt_rust::Output<String>,
        /// The provider-assigned unique ID for this managed resource.
        pub id: pulumi_gestalt_rust::Output<String>,
        /// The Last Name for the User.
        pub last_name: pulumi_gestalt_rust::Output<String>,
        /// Any notes about this User.
        pub note: pulumi_gestalt_rust::Output<String>,
        pub resource_group_name: pulumi_gestalt_rust::Output<String>,
        /// The current state of this User, for example `active`, `blocked` or `pending`.
        pub state: pulumi_gestalt_rust::Output<String>,
        pub user_id: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn invoke(
        context: &pulumi_gestalt_rust::Context,
        args: GetUserArgs,
    ) -> GetUserResult {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::register_interface;
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let api_management_name_binding = args.api_management_name.get_output(context);
        let resource_group_name_binding = args.resource_group_name.get_output(context);
        let user_id_binding = args.user_id.get_output(context);
        let request = pulumi_gestalt_rust::InvokeResourceRequest {
            token: "azure:apimanagement/getUser:getUser".into(),
            version: super::super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "apiManagementName".into(),
                    value: api_management_name_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "resourceGroupName".into(),
                    value: resource_group_name_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "userId".into(),
                    value: user_id_binding.get_id(),
                },
            ],
        };
        let o = context.invoke_resource(request);
        GetUserResult {
            api_management_name: o.get_field("apiManagementName"),
            email: o.get_field("email"),
            first_name: o.get_field("firstName"),
            id: o.get_field("id"),
            last_name: o.get_field("lastName"),
            note: o.get_field("note"),
            resource_group_name: o.get_field("resourceGroupName"),
            state: o.get_field("state"),
            user_id: o.get_field("userId"),
        }
    }
}
