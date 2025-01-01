pub mod get_organization_service_account {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GetOrganizationServiceAccountArgs {
        /// The organization ID the service account was created for.
        #[builder(into)]
        pub organization_id: pulumi_wasm_rust::Output<String>,
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
        args: GetOrganizationServiceAccountArgs,
    ) -> GetOrganizationServiceAccountResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let organization_id_binding = args.organization_id.get_inner();
        let request = register_interface::ResourceInvokeRequest {
            token: "gcp:accessapproval/getOrganizationServiceAccount:getOrganizationServiceAccount"
                .into(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "organizationId".into(),
                    value: &organization_id_binding,
                },
            ]),
            results: Vec::from([
                register_interface::ResultField {
                    name: "accountEmail".into(),
                },
                register_interface::ResultField {
                    name: "id".into(),
                },
                register_interface::ResultField {
                    name: "name".into(),
                },
                register_interface::ResultField {
                    name: "organizationId".into(),
                },
            ]),
        };
        let o = register_interface::invoke(&request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        GetOrganizationServiceAccountResult {
            account_email: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("accountEmail").unwrap(),
            ),
            id: pulumi_wasm_rust::__private::into_domain(hashmap.remove("id").unwrap()),
            name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("name").unwrap(),
            ),
            organization_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("organizationId").unwrap(),
            ),
        }
    }
}
