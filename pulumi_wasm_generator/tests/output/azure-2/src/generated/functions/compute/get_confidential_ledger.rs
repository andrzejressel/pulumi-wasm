pub mod get_confidential_ledger {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GetConfidentialLedgerArgs {
        /// Specifies the name of this Confidential Ledger.
        #[builder(into)]
        pub name: pulumi_wasm_rust::Output<String>,
        /// Specifies the name of the Resource Group where this Confidential Ledger exists.
        #[builder(into)]
        pub resource_group_name: pulumi_wasm_rust::Output<String>,
    }
    #[allow(dead_code)]
    pub struct GetConfidentialLedgerResult {
        pub azuread_based_service_principals: pulumi_wasm_rust::Output<
            Vec<
                super::super::super::types::compute::GetConfidentialLedgerAzureadBasedServicePrincipal,
            >,
        >,
        pub certificate_based_security_principals: pulumi_wasm_rust::Output<
            Vec<
                super::super::super::types::compute::GetConfidentialLedgerCertificateBasedSecurityPrincipal,
            >,
        >,
        /// The provider-assigned unique ID for this managed resource.
        pub id: pulumi_wasm_rust::Output<String>,
        /// The Identity Service Endpoint for this Confidential Ledger.
        pub identity_service_endpoint: pulumi_wasm_rust::Output<String>,
        /// The Endpoint for this Confidential Ledger.
        pub ledger_endpoint: pulumi_wasm_rust::Output<String>,
        /// The type of Confidential Ledger.
        pub ledger_type: pulumi_wasm_rust::Output<String>,
        /// The supported Azure location where the Confidential Ledger exists.
        pub location: pulumi_wasm_rust::Output<String>,
        pub name: pulumi_wasm_rust::Output<String>,
        pub resource_group_name: pulumi_wasm_rust::Output<String>,
        /// A mapping of tags to assign to the Confidential Ledger.
        pub tags: pulumi_wasm_rust::Output<std::collections::HashMap<String, String>>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn invoke(args: GetConfidentialLedgerArgs) -> GetConfidentialLedgerResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let name_binding = args.name.get_inner();
        let resource_group_name_binding = args.resource_group_name.get_inner();
        let request = register_interface::ResourceInvokeRequest {
            token: "azure:compute/getConfidentialLedger:getConfidentialLedger".into(),
            version: super::super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "name".into(),
                    value: &name_binding,
                },
                register_interface::ObjectField {
                    name: "resourceGroupName".into(),
                    value: &resource_group_name_binding,
                },
            ]),
            results: Vec::from([
                register_interface::ResultField {
                    name: "azureadBasedServicePrincipals".into(),
                },
                register_interface::ResultField {
                    name: "certificateBasedSecurityPrincipals".into(),
                },
                register_interface::ResultField {
                    name: "id".into(),
                },
                register_interface::ResultField {
                    name: "identityServiceEndpoint".into(),
                },
                register_interface::ResultField {
                    name: "ledgerEndpoint".into(),
                },
                register_interface::ResultField {
                    name: "ledgerType".into(),
                },
                register_interface::ResultField {
                    name: "location".into(),
                },
                register_interface::ResultField {
                    name: "name".into(),
                },
                register_interface::ResultField {
                    name: "resourceGroupName".into(),
                },
                register_interface::ResultField {
                    name: "tags".into(),
                },
            ]),
        };
        let o = register_interface::invoke(&request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        GetConfidentialLedgerResult {
            azuread_based_service_principals: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("azureadBasedServicePrincipals").unwrap(),
            ),
            certificate_based_security_principals: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("certificateBasedSecurityPrincipals").unwrap(),
            ),
            id: pulumi_wasm_rust::__private::into_domain(hashmap.remove("id").unwrap()),
            identity_service_endpoint: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("identityServiceEndpoint").unwrap(),
            ),
            ledger_endpoint: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("ledgerEndpoint").unwrap(),
            ),
            ledger_type: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("ledgerType").unwrap(),
            ),
            location: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("location").unwrap(),
            ),
            name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("name").unwrap(),
            ),
            resource_group_name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("resourceGroupName").unwrap(),
            ),
            tags: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("tags").unwrap(),
            ),
        }
    }
}
