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
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod ledger {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct LedgerArgs {
        /// A list of `azuread_based_service_principal` blocks as defined below.
        #[builder(into)]
        pub azuread_based_service_principals: pulumi_gestalt_rust::InputOrOutput<
            Vec<
                super::super::types::confidentialledger::LedgerAzureadBasedServicePrincipal,
            >,
        >,
        /// A list of `certificate_based_security_principal` blocks as defined below.
        #[builder(into, default)]
        pub certificate_based_security_principals: pulumi_gestalt_rust::InputOrOutput<
            Option<
                Vec<
                    super::super::types::confidentialledger::LedgerCertificateBasedSecurityPrincipal,
                >,
            >,
        >,
        /// Specifies the type of Confidential Ledger. Possible values are `Private` and `Public`. Changing this forces a new resource to be created.
        #[builder(into)]
        pub ledger_type: pulumi_gestalt_rust::InputOrOutput<String>,
        /// Specifies the supported Azure location where the Confidential Ledger exists. Changing this forces a new resource to be created.
        #[builder(into, default)]
        pub location: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Specifies the name of the Confidential Ledger. Changing this forces a new resource to be created.
        #[builder(into, default)]
        pub name: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The name of the Resource Group where the Confidential Ledger exists. Changing this forces a new resource to be created.
        #[builder(into)]
        pub resource_group_name: pulumi_gestalt_rust::InputOrOutput<String>,
        /// A mapping of tags to assign to the Confidential Ledger.
        #[builder(into, default)]
        pub tags: pulumi_gestalt_rust::InputOrOutput<
            Option<std::collections::HashMap<String, String>>,
        >,
    }
    #[allow(dead_code)]
    pub struct LedgerResult {
        /// A list of `azuread_based_service_principal` blocks as defined below.
        pub azuread_based_service_principals: pulumi_gestalt_rust::Output<
            Vec<
                super::super::types::confidentialledger::LedgerAzureadBasedServicePrincipal,
            >,
        >,
        /// A list of `certificate_based_security_principal` blocks as defined below.
        pub certificate_based_security_principals: pulumi_gestalt_rust::Output<
            Option<
                Vec<
                    super::super::types::confidentialledger::LedgerCertificateBasedSecurityPrincipal,
                >,
            >,
        >,
        /// The Identity Service Endpoint for this Confidential Ledger.
        pub identity_service_endpoint: pulumi_gestalt_rust::Output<String>,
        /// The Endpoint for this Confidential Ledger.
        pub ledger_endpoint: pulumi_gestalt_rust::Output<String>,
        /// Specifies the type of Confidential Ledger. Possible values are `Private` and `Public`. Changing this forces a new resource to be created.
        pub ledger_type: pulumi_gestalt_rust::Output<String>,
        /// Specifies the supported Azure location where the Confidential Ledger exists. Changing this forces a new resource to be created.
        pub location: pulumi_gestalt_rust::Output<String>,
        /// Specifies the name of the Confidential Ledger. Changing this forces a new resource to be created.
        pub name: pulumi_gestalt_rust::Output<String>,
        /// The name of the Resource Group where the Confidential Ledger exists. Changing this forces a new resource to be created.
        pub resource_group_name: pulumi_gestalt_rust::Output<String>,
        /// A mapping of tags to assign to the Confidential Ledger.
        pub tags: pulumi_gestalt_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: LedgerArgs,
    ) -> LedgerResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let azuread_based_service_principals_binding = args
            .azuread_based_service_principals
            .get_output(context);
        let certificate_based_security_principals_binding = args
            .certificate_based_security_principals
            .get_output(context);
        let ledger_type_binding = args.ledger_type.get_output(context);
        let location_binding = args.location.get_output(context);
        let name_binding = args.name.get_output(context);
        let resource_group_name_binding = args.resource_group_name.get_output(context);
        let tags_binding = args.tags.get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "azure:confidentialledger/ledger:Ledger".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "azureadBasedServicePrincipals".into(),
                    value: azuread_based_service_principals_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "certificateBasedSecurityPrincipals".into(),
                    value: certificate_based_security_principals_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "ledgerType".into(),
                    value: ledger_type_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "location".into(),
                    value: location_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "name".into(),
                    value: name_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "resourceGroupName".into(),
                    value: resource_group_name_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "tags".into(),
                    value: tags_binding.get_id(),
                },
            ],
        };
        let o = context.register_resource(request);
        LedgerResult {
            azuread_based_service_principals: o
                .get_field("azureadBasedServicePrincipals"),
            certificate_based_security_principals: o
                .get_field("certificateBasedSecurityPrincipals"),
            identity_service_endpoint: o.get_field("identityServiceEndpoint"),
            ledger_endpoint: o.get_field("ledgerEndpoint"),
            ledger_type: o.get_field("ledgerType"),
            location: o.get_field("location"),
            name: o.get_field("name"),
            resource_group_name: o.get_field("resourceGroupName"),
            tags: o.get_field("tags"),
        }
    }
}
