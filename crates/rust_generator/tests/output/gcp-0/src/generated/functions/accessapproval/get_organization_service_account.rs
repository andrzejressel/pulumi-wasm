pub mod get_organization_service_account {
    #[derive(pulumi_wasm_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GetOrganizationServiceAccountArgs {
        /// The organization ID the service account was created for.
        #[builder(into)]
        pub organization_id: pulumi_wasm_rust::InputOrOutput<String>,
    }
    #[allow(dead_code)]
    pub struct GetOrganizationServiceAccountResult {
        /// The email address of the service account. This value is
        /// often used to refer to the service account in order to grant IAM permissions.
        pub account_email: pulumi_wasm_rust::Output<String>,
        /// The provider-assigned unique ID for this managed resource.
        pub id: pulumi_wasm_rust::Output<String>,
        /// The Access Approval service account resource name. Format is "organizations/{organization_id}/serviceAccount".
        pub name: pulumi_wasm_rust::Output<String>,
        pub organization_id: pulumi_wasm_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn invoke(
        context: &pulumi_wasm_rust::PulumiContext,
        args: GetOrganizationServiceAccountArgs,
    ) -> GetOrganizationServiceAccountResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let organization_id_binding = args
            .organization_id
            .get_output(context)
            .get_inner();
        let request = register_interface::ResourceInvokeRequest {
            token: "gcp:accessapproval/getOrganizationServiceAccount:getOrganizationServiceAccount"
                .into(),
            version: super::super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "organizationId".into(),
                    value: &organization_id_binding,
                },
            ]),
        };
        let o = register_interface::invoke(context.get_inner(), &request);
        GetOrganizationServiceAccountResult {
            account_email: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("accountEmail"),
            ),
            id: pulumi_wasm_rust::__private::into_domain(o.extract_field("id")),
            name: pulumi_wasm_rust::__private::into_domain(o.extract_field("name")),
            organization_id: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("organizationId"),
            ),
        }
    }
}
