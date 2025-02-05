pub mod get_user {
    #[derive(pulumi_wasm_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GetUserArgs {
        /// The Name of the API Management Service in which this User exists.
        #[builder(into)]
        pub api_management_name: pulumi_wasm_rust::InputOrOutput<String>,
        /// The Name of the Resource Group in which the API Management Service exists.
        #[builder(into)]
        pub resource_group_name: pulumi_wasm_rust::InputOrOutput<String>,
        /// The Identifier for the User.
        #[builder(into)]
        pub user_id: pulumi_wasm_rust::InputOrOutput<String>,
    }
    #[allow(dead_code)]
    pub struct GetUserResult {
        pub api_management_name: pulumi_wasm_rust::Output<String>,
        /// The Email Address used for this User.
        pub email: pulumi_wasm_rust::Output<String>,
        /// The First Name for the User.
        pub first_name: pulumi_wasm_rust::Output<String>,
        /// The provider-assigned unique ID for this managed resource.
        pub id: pulumi_wasm_rust::Output<String>,
        /// The Last Name for the User.
        pub last_name: pulumi_wasm_rust::Output<String>,
        /// Any notes about this User.
        pub note: pulumi_wasm_rust::Output<String>,
        pub resource_group_name: pulumi_wasm_rust::Output<String>,
        /// The current state of this User, for example `active`, `blocked` or `pending`.
        pub state: pulumi_wasm_rust::Output<String>,
        pub user_id: pulumi_wasm_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn invoke(
        context: &pulumi_wasm_rust::PulumiContext,
        args: GetUserArgs,
    ) -> GetUserResult {
        use pulumi_wasm_rust::__private::pulumi_gestalt_adapter_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let api_management_name_binding = args
            .api_management_name
            .get_output(context)
            .get_inner();
        let resource_group_name_binding = args
            .resource_group_name
            .get_output(context)
            .get_inner();
        let user_id_binding = args.user_id.get_output(context).get_inner();
        let request = register_interface::ResourceInvokeRequest {
            token: "azure:apimanagement/getUser:getUser".into(),
            version: super::super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "apiManagementName".into(),
                    value: &api_management_name_binding,
                },
                register_interface::ObjectField {
                    name: "resourceGroupName".into(),
                    value: &resource_group_name_binding,
                },
                register_interface::ObjectField {
                    name: "userId".into(),
                    value: &user_id_binding,
                },
            ]),
        };
        let o = register_interface::invoke(context.get_inner(), &request);
        GetUserResult {
            api_management_name: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("apiManagementName"),
            ),
            email: pulumi_wasm_rust::__private::into_domain(o.extract_field("email")),
            first_name: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("firstName"),
            ),
            id: pulumi_wasm_rust::__private::into_domain(o.extract_field("id")),
            last_name: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("lastName"),
            ),
            note: pulumi_wasm_rust::__private::into_domain(o.extract_field("note")),
            resource_group_name: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("resourceGroupName"),
            ),
            state: pulumi_wasm_rust::__private::into_domain(o.extract_field("state")),
            user_id: pulumi_wasm_rust::__private::into_domain(o.extract_field("userId")),
        }
    }
}
