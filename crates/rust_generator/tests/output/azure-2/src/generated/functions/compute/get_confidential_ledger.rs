#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod get_confidential_ledger {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GetConfidentialLedgerArgs {
        /// Specifies the name of this Confidential Ledger.
        #[builder(into)]
        pub name: pulumi_gestalt_rust::InputOrOutput<String>,
        /// Specifies the name of the Resource Group where this Confidential Ledger exists.
        #[builder(into)]
        pub resource_group_name: pulumi_gestalt_rust::InputOrOutput<String>,
    }
    #[allow(dead_code)]
    pub struct GetConfidentialLedgerResult {
        pub azuread_based_service_principals: pulumi_gestalt_rust::Output<
            Vec<
                super::super::super::types::compute::GetConfidentialLedgerAzureadBasedServicePrincipal,
            >,
        >,
        pub certificate_based_security_principals: pulumi_gestalt_rust::Output<
            Vec<
                super::super::super::types::compute::GetConfidentialLedgerCertificateBasedSecurityPrincipal,
            >,
        >,
        /// The provider-assigned unique ID for this managed resource.
        pub id: pulumi_gestalt_rust::Output<String>,
        /// The Identity Service Endpoint for this Confidential Ledger.
        pub identity_service_endpoint: pulumi_gestalt_rust::Output<String>,
        /// The Endpoint for this Confidential Ledger.
        pub ledger_endpoint: pulumi_gestalt_rust::Output<String>,
        /// The type of Confidential Ledger.
        pub ledger_type: pulumi_gestalt_rust::Output<String>,
        /// The supported Azure location where the Confidential Ledger exists.
        pub location: pulumi_gestalt_rust::Output<String>,
        pub name: pulumi_gestalt_rust::Output<String>,
        pub resource_group_name: pulumi_gestalt_rust::Output<String>,
        /// A mapping of tags to assign to the Confidential Ledger.
        pub tags: pulumi_gestalt_rust::Output<std::collections::HashMap<String, String>>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn invoke(
        context: &pulumi_gestalt_rust::PulumiContext,
        args: GetConfidentialLedgerArgs,
    ) -> GetConfidentialLedgerResult {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::register_interface;
        use std::collections::HashMap;
        let name_binding_1 = args.name.get_output(context);
        let name_binding = name_binding_1.get_inner();
        let resource_group_name_binding_1 = args.resource_group_name.get_output(context);
        let resource_group_name_binding = resource_group_name_binding_1.get_inner();
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
        };
        let o = register_interface::invoke(context.get_inner(), &request);
        GetConfidentialLedgerResult {
            azuread_based_service_principals: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("azureadBasedServicePrincipals"),
            ),
            certificate_based_security_principals: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("certificateBasedSecurityPrincipals"),
            ),
            id: pulumi_gestalt_rust::__private::into_domain(o.extract_field("id")),
            identity_service_endpoint: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("identityServiceEndpoint"),
            ),
            ledger_endpoint: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("ledgerEndpoint"),
            ),
            ledger_type: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("ledgerType"),
            ),
            location: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("location"),
            ),
            name: pulumi_gestalt_rust::__private::into_domain(o.extract_field("name")),
            resource_group_name: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("resourceGroupName"),
            ),
            tags: pulumi_gestalt_rust::__private::into_domain(o.extract_field("tags")),
        }
    }
}
