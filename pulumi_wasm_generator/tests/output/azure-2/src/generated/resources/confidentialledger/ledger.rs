/// Manages a Confidential Ledger.
///
/// ## Example Usage
///
/// ```yaml
/// resources:
///   example:
///     type: azure:core:ResourceGroup
///     properties:
///       name: example-resources
///       location: West Europe
///   ledger:
///     type: azure:confidentialledger:Ledger
///     properties:
///       name: example-ledger
///       resourceGroupName: ${example.name}
///       location: ${example.location}
///       ledgerType: Private
///       azureadBasedServicePrincipals:
///         - principalId: ${current.objectId}
///           tenantId: ${current.tenantId}
///           ledgerRoleName: Administrator
/// variables:
///   current:
///     fn::invoke:
///       function: azure:core:getClientConfig
///       arguments: {}
/// ```
///
/// ## Import
///
/// Confidential Ledgers can be imported using the `resource id`, e.g.
///
/// ```sh
/// $ pulumi import azure:confidentialledger/ledger:Ledger example /subscriptions/12345678-1234-9876-4563-123456789012/resourceGroups/example-group/providers/Microsoft.ConfidentialLedger/ledgers/example-ledger
/// ```
///
pub mod ledger {
    #[derive(pulumi_wasm_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct LedgerArgs {
        /// A list of `azuread_based_service_principal` blocks as defined below.
        #[builder(into)]
        pub azuread_based_service_principals: pulumi_wasm_rust::InputOrOutput<
            Vec<
                super::super::types::confidentialledger::LedgerAzureadBasedServicePrincipal,
            >,
        >,
        /// A list of `certificate_based_security_principal` blocks as defined below.
        #[builder(into, default)]
        pub certificate_based_security_principals: pulumi_wasm_rust::InputOrOutput<
            Option<
                Vec<
                    super::super::types::confidentialledger::LedgerCertificateBasedSecurityPrincipal,
                >,
            >,
        >,
        /// Specifies the type of Confidential Ledger. Possible values are `Private` and `Public`. Changing this forces a new resource to be created.
        #[builder(into)]
        pub ledger_type: pulumi_wasm_rust::InputOrOutput<String>,
        /// Specifies the supported Azure location where the Confidential Ledger exists. Changing this forces a new resource to be created.
        #[builder(into, default)]
        pub location: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        /// Specifies the name of the Confidential Ledger. Changing this forces a new resource to be created.
        #[builder(into, default)]
        pub name: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        /// The name of the Resource Group where the Confidential Ledger exists. Changing this forces a new resource to be created.
        #[builder(into)]
        pub resource_group_name: pulumi_wasm_rust::InputOrOutput<String>,
        /// A mapping of tags to assign to the Confidential Ledger.
        #[builder(into, default)]
        pub tags: pulumi_wasm_rust::InputOrOutput<
            Option<std::collections::HashMap<String, String>>,
        >,
    }
    #[allow(dead_code)]
    pub struct LedgerResult {
        /// A list of `azuread_based_service_principal` blocks as defined below.
        pub azuread_based_service_principals: pulumi_wasm_rust::Output<
            Vec<
                super::super::types::confidentialledger::LedgerAzureadBasedServicePrincipal,
            >,
        >,
        /// A list of `certificate_based_security_principal` blocks as defined below.
        pub certificate_based_security_principals: pulumi_wasm_rust::Output<
            Option<
                Vec<
                    super::super::types::confidentialledger::LedgerCertificateBasedSecurityPrincipal,
                >,
            >,
        >,
        /// The Identity Service Endpoint for this Confidential Ledger.
        pub identity_service_endpoint: pulumi_wasm_rust::Output<String>,
        /// The Endpoint for this Confidential Ledger.
        pub ledger_endpoint: pulumi_wasm_rust::Output<String>,
        /// Specifies the type of Confidential Ledger. Possible values are `Private` and `Public`. Changing this forces a new resource to be created.
        pub ledger_type: pulumi_wasm_rust::Output<String>,
        /// Specifies the supported Azure location where the Confidential Ledger exists. Changing this forces a new resource to be created.
        pub location: pulumi_wasm_rust::Output<String>,
        /// Specifies the name of the Confidential Ledger. Changing this forces a new resource to be created.
        pub name: pulumi_wasm_rust::Output<String>,
        /// The name of the Resource Group where the Confidential Ledger exists. Changing this forces a new resource to be created.
        pub resource_group_name: pulumi_wasm_rust::Output<String>,
        /// A mapping of tags to assign to the Confidential Ledger.
        pub tags: pulumi_wasm_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_wasm_rust::PulumiContext,
        name: &str,
        args: LedgerArgs,
    ) -> LedgerResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let azuread_based_service_principals_binding = args
            .azuread_based_service_principals
            .get_output(context)
            .get_inner();
        let certificate_based_security_principals_binding = args
            .certificate_based_security_principals
            .get_output(context)
            .get_inner();
        let ledger_type_binding = args.ledger_type.get_output(context).get_inner();
        let location_binding = args.location.get_output(context).get_inner();
        let name_binding = args.name.get_output(context).get_inner();
        let resource_group_name_binding = args
            .resource_group_name
            .get_output(context)
            .get_inner();
        let tags_binding = args.tags.get_output(context).get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "azure:confidentialledger/ledger:Ledger".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "azureadBasedServicePrincipals".into(),
                    value: &azuread_based_service_principals_binding,
                },
                register_interface::ObjectField {
                    name: "certificateBasedSecurityPrincipals".into(),
                    value: &certificate_based_security_principals_binding,
                },
                register_interface::ObjectField {
                    name: "ledgerType".into(),
                    value: &ledger_type_binding,
                },
                register_interface::ObjectField {
                    name: "location".into(),
                    value: &location_binding,
                },
                register_interface::ObjectField {
                    name: "name".into(),
                    value: &name_binding,
                },
                register_interface::ObjectField {
                    name: "resourceGroupName".into(),
                    value: &resource_group_name_binding,
                },
                register_interface::ObjectField {
                    name: "tags".into(),
                    value: &tags_binding,
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
        let o = register_interface::register(context.get_inner(), &request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        LedgerResult {
            azuread_based_service_principals: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("azureadBasedServicePrincipals").unwrap(),
            ),
            certificate_based_security_principals: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("certificateBasedSecurityPrincipals").unwrap(),
            ),
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
