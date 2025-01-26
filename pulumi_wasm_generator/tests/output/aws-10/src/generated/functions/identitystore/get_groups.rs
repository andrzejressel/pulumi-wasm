pub mod get_groups {
    #[derive(pulumi_wasm_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GetGroupsArgs {
        /// Identity Store ID associated with the Single Sign-On (SSO) Instance.
        #[builder(into)]
        pub identity_store_id: pulumi_wasm_rust::InputOrOutput<String>,
    }
    #[allow(dead_code)]
    pub struct GetGroupsResult {
        /// List of Identity Store Groups
        pub groups: pulumi_wasm_rust::Output<
            Vec<super::super::super::types::identitystore::GetGroupsGroup>,
        >,
        /// The provider-assigned unique ID for this managed resource.
        pub id: pulumi_wasm_rust::Output<String>,
        pub identity_store_id: pulumi_wasm_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn invoke(
        context: &pulumi_wasm_rust::PulumiContext,
        args: GetGroupsArgs,
    ) -> GetGroupsResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let identity_store_id_binding = args
            .identity_store_id
            .get_output(context)
            .get_inner();
        let request = register_interface::ResourceInvokeRequest {
            token: "aws:identitystore/getGroups:getGroups".into(),
            version: super::super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "identityStoreId".into(),
                    value: &identity_store_id_binding,
                },
            ]),
        };
        let o = register_interface::invoke(context.get_inner(), &request);
        GetGroupsResult {
            groups: pulumi_wasm_rust::__private::into_domain(o.extract_field("groups")),
            id: pulumi_wasm_rust::__private::into_domain(o.extract_field("id")),
            identity_store_id: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("identityStoreId"),
            ),
        }
    }
}
