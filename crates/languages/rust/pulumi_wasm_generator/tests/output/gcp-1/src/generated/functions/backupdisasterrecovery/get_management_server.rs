pub mod get_management_server {
    #[derive(pulumi_wasm_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GetManagementServerArgs {
        #[builder(into)]
        pub location: pulumi_wasm_rust::InputOrOutput<String>,
    }
    #[allow(dead_code)]
    pub struct GetManagementServerResult {
        /// The provider-assigned unique ID for this managed resource.
        pub id: pulumi_wasm_rust::Output<String>,
        pub location: pulumi_wasm_rust::Output<String>,
        pub management_uris: pulumi_wasm_rust::Output<
            Vec<
                super::super::super::types::backupdisasterrecovery::GetManagementServerManagementUri,
            >,
        >,
        pub name: pulumi_wasm_rust::Output<String>,
        pub networks: pulumi_wasm_rust::Output<
            Vec<
                super::super::super::types::backupdisasterrecovery::GetManagementServerNetwork,
            >,
        >,
        pub oauth2_client_id: pulumi_wasm_rust::Output<String>,
        pub project: pulumi_wasm_rust::Output<String>,
        pub type_: pulumi_wasm_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn invoke(
        context: &pulumi_wasm_rust::PulumiContext,
        args: GetManagementServerArgs,
    ) -> GetManagementServerResult {
        use pulumi_wasm_rust::__private::pulumi_gestalt_adapter_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let location_binding = args.location.get_output(context).get_inner();
        let request = register_interface::ResourceInvokeRequest {
            token: "gcp:backupdisasterrecovery/getManagementServer:getManagementServer"
                .into(),
            version: super::super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "location".into(),
                    value: &location_binding,
                },
            ]),
        };
        let o = register_interface::invoke(context.get_inner(), &request);
        GetManagementServerResult {
            id: pulumi_wasm_rust::__private::into_domain(o.extract_field("id")),
            location: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("location"),
            ),
            management_uris: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("managementUris"),
            ),
            name: pulumi_wasm_rust::__private::into_domain(o.extract_field("name")),
            networks: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("networks"),
            ),
            oauth2_client_id: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("oauth2ClientId"),
            ),
            project: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("project"),
            ),
            type_: pulumi_wasm_rust::__private::into_domain(o.extract_field("type")),
        }
    }
}
