/// Manages a Logic App Integration Account Agreement.
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
///   test:
///     type: azure:logicapps:IntegrationAccount
///     properties:
///       name: example-ia
///       location: ${example.location}
///       resourceGroupName: ${example.name}
///       skuName: Standard
///   host:
///     type: azure:logicapps:IntegrationAccountPartner
///     properties:
///       name: example-hostpartner
///       resourceGroupName: ${example.name}
///       integrationAccountName: ${test.name}
///       businessIdentities:
///         - qualifier: AS2Identity
///           value: FabrikamNY
///   guest:
///     type: azure:logicapps:IntegrationAccountPartner
///     properties:
///       name: example-guestpartner
///       resourceGroupName: ${example.name}
///       integrationAccountName: ${test.name}
///       businessIdentities:
///         - qualifier: AS2Identity
///           value: FabrikamDC
///   testIntegrationAccountAgreement:
///     type: azure:logicapps:IntegrationAccountAgreement
///     name: test
///     properties:
///       name: example-agreement
///       resourceGroupName: ${example.name}
///       integrationAccountName: ${test.name}
///       agreementType: AS2
///       hostPartnerName: ${host.name}
///       guestPartnerName: ${guest.name}
///       content:
///         fn::invoke:
///           function: std:file
///           arguments:
///             input: testdata/integration_account_agreement_content_as2.json
///           return: result
///       hostIdentity:
///         qualifier: AS2Identity
///         value: FabrikamNY
///       guestIdentity:
///         qualifier: AS2Identity
///         value: FabrikamDC
/// ```
///
/// ## Import
///
/// Logic App Integration Account Agreements can be imported using the `resource id`, e.g.
///
/// ```sh
/// $ pulumi import azure:logicapps/integrationAccountAgreement:IntegrationAccountAgreement example /subscriptions/00000000-0000-0000-0000-000000000000/resourceGroups/group1/providers/Microsoft.Logic/integrationAccounts/account1/agreements/agreement1
/// ```
///
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod integration_account_agreement {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct IntegrationAccountAgreementArgs {
        /// The type of the Logic App Integration Account Agreement. Possible values are `AS2`, `X12` and `Edifact`.
        #[builder(into)]
        pub agreement_type: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The content of the Logic App Integration Account Agreement.
        #[builder(into)]
        pub content: pulumi_gestalt_rust::InputOrOutput<String>,
        /// A `guest_identity` block as documented below.
        #[builder(into)]
        pub guest_identity: pulumi_gestalt_rust::InputOrOutput<
            super::super::types::logicapps::IntegrationAccountAgreementGuestIdentity,
        >,
        /// The name of the guest Logic App Integration Account Partner.
        #[builder(into)]
        pub guest_partner_name: pulumi_gestalt_rust::InputOrOutput<String>,
        /// A `host_identity` block as documented below.
        #[builder(into)]
        pub host_identity: pulumi_gestalt_rust::InputOrOutput<
            super::super::types::logicapps::IntegrationAccountAgreementHostIdentity,
        >,
        /// The name of the host Logic App Integration Account Partner.
        #[builder(into)]
        pub host_partner_name: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The name of the Logic App Integration Account. Changing this forces a new resource to be created.
        #[builder(into)]
        pub integration_account_name: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The metadata of the Logic App Integration Account Agreement.
        #[builder(into, default)]
        pub metadata: pulumi_gestalt_rust::InputOrOutput<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// The name which should be used for this Logic App Integration Account Agreement. Changing this forces a new resource to be created.
        #[builder(into, default)]
        pub name: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The name of the Resource Group where the Logic App Integration Account Agreement should exist. Changing this forces a new resource to be created.
        #[builder(into)]
        pub resource_group_name: pulumi_gestalt_rust::InputOrOutput<String>,
    }
    #[allow(dead_code)]
    pub struct IntegrationAccountAgreementResult {
        /// The type of the Logic App Integration Account Agreement. Possible values are `AS2`, `X12` and `Edifact`.
        pub agreement_type: pulumi_gestalt_rust::Output<String>,
        /// The content of the Logic App Integration Account Agreement.
        pub content: pulumi_gestalt_rust::Output<String>,
        /// A `guest_identity` block as documented below.
        pub guest_identity: pulumi_gestalt_rust::Output<
            super::super::types::logicapps::IntegrationAccountAgreementGuestIdentity,
        >,
        /// The name of the guest Logic App Integration Account Partner.
        pub guest_partner_name: pulumi_gestalt_rust::Output<String>,
        /// A `host_identity` block as documented below.
        pub host_identity: pulumi_gestalt_rust::Output<
            super::super::types::logicapps::IntegrationAccountAgreementHostIdentity,
        >,
        /// The name of the host Logic App Integration Account Partner.
        pub host_partner_name: pulumi_gestalt_rust::Output<String>,
        /// The name of the Logic App Integration Account. Changing this forces a new resource to be created.
        pub integration_account_name: pulumi_gestalt_rust::Output<String>,
        /// The metadata of the Logic App Integration Account Agreement.
        pub metadata: pulumi_gestalt_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// The name which should be used for this Logic App Integration Account Agreement. Changing this forces a new resource to be created.
        pub name: pulumi_gestalt_rust::Output<String>,
        /// The name of the Resource Group where the Logic App Integration Account Agreement should exist. Changing this forces a new resource to be created.
        pub resource_group_name: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: IntegrationAccountAgreementArgs,
    ) -> IntegrationAccountAgreementResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let agreement_type_binding = args.agreement_type.get_output(context);
        let content_binding = args.content.get_output(context);
        let guest_identity_binding = args.guest_identity.get_output(context);
        let guest_partner_name_binding = args.guest_partner_name.get_output(context);
        let host_identity_binding = args.host_identity.get_output(context);
        let host_partner_name_binding = args.host_partner_name.get_output(context);
        let integration_account_name_binding = args
            .integration_account_name
            .get_output(context);
        let metadata_binding = args.metadata.get_output(context);
        let name_binding = args.name.get_output(context);
        let resource_group_name_binding = args.resource_group_name.get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "azure:logicapps/integrationAccountAgreement:IntegrationAccountAgreement"
                .into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "agreementType".into(),
                    value: &agreement_type_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "content".into(),
                    value: &content_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "guestIdentity".into(),
                    value: &guest_identity_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "guestPartnerName".into(),
                    value: &guest_partner_name_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "hostIdentity".into(),
                    value: &host_identity_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "hostPartnerName".into(),
                    value: &host_partner_name_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "integrationAccountName".into(),
                    value: &integration_account_name_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "metadata".into(),
                    value: &metadata_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "name".into(),
                    value: &name_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "resourceGroupName".into(),
                    value: &resource_group_name_binding.drop_type(),
                },
            ],
        };
        let o = context.register_resource(request);
        IntegrationAccountAgreementResult {
            agreement_type: o.get_field("agreementType"),
            content: o.get_field("content"),
            guest_identity: o.get_field("guestIdentity"),
            guest_partner_name: o.get_field("guestPartnerName"),
            host_identity: o.get_field("hostIdentity"),
            host_partner_name: o.get_field("hostPartnerName"),
            integration_account_name: o.get_field("integrationAccountName"),
            metadata: o.get_field("metadata"),
            name: o.get_field("name"),
            resource_group_name: o.get_field("resourceGroupName"),
        }
    }
}
