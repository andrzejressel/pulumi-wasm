pub mod get_delegated_administrators {
    #[derive(pulumi_wasm_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GetDelegatedAdministratorsArgs {
        /// Specifies a service principal name. If specified, then the operation lists the delegated administrators only for the specified service. If you don't specify a service principal, the operation lists all delegated administrators for all services in your organization.
        #[builder(into, default)]
        pub service_principal: pulumi_wasm_rust::InputOrOutput<Option<String>>,
    }
    #[allow(dead_code)]
    pub struct GetDelegatedAdministratorsResult {
        /// The list of delegated administrators in your organization, which have the following attributes:
        pub delegated_administrators: pulumi_wasm_rust::Output<
            Vec<
                super::super::super::types::organizations::GetDelegatedAdministratorsDelegatedAdministrator,
            >,
        >,
        /// The provider-assigned unique ID for this managed resource.
        pub id: pulumi_wasm_rust::Output<String>,
        pub service_principal: pulumi_wasm_rust::Output<Option<String>>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn invoke(
        context: &pulumi_wasm_rust::PulumiContext,
        args: GetDelegatedAdministratorsArgs,
    ) -> GetDelegatedAdministratorsResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let service_principal_binding = args
            .service_principal
            .get_output(context)
            .get_inner();
        let request = register_interface::ResourceInvokeRequest {
            token: "aws:organizations/getDelegatedAdministrators:getDelegatedAdministrators"
                .into(),
            version: super::super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "servicePrincipal".into(),
                    value: &service_principal_binding,
                },
            ]),
        };
        let o = register_interface::invoke(context.get_inner(), &request);
        GetDelegatedAdministratorsResult {
            delegated_administrators: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("delegatedAdministrators"),
            ),
            id: pulumi_wasm_rust::__private::into_domain(o.extract_field("id")),
            service_principal: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("servicePrincipal"),
            ),
        }
    }
}
