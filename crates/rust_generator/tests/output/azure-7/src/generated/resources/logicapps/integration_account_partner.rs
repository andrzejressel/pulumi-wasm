/// Manages a Logic App Integration Account Partner.
///
/// ## Example Usage
///
/// ```ignore
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let example = resource_group::create(
///         "example",
///         ResourceGroupArgs::builder()
///             .location("West Europe")
///             .name("example-resources")
///             .build_struct(),
///     );
///     let exampleIntegrationAccount = integration_account::create(
///         "exampleIntegrationAccount",
///         IntegrationAccountArgs::builder()
///             .location("${example.location}")
///             .name("example-ia")
///             .resource_group_name("${example.name}")
///             .sku_name("Standard")
///             .build_struct(),
///     );
///     let exampleIntegrationAccountPartner = integration_account_partner::create(
///         "exampleIntegrationAccountPartner",
///         IntegrationAccountPartnerArgs::builder()
///             .business_identities(
///                 vec![
///                     IntegrationAccountPartnerBusinessIdentity::builder().qualifier("ZZ")
///                     .value("AA").build_struct(),
///                 ],
///             )
///             .integration_account_name("${exampleIntegrationAccount.name}")
///             .name("example-iap")
///             .resource_group_name("${example.name}")
///             .build_struct(),
///     );
/// }
/// ```
///
/// ## Import
///
/// Logic App Integration Account Partners can be imported using the `resource id`, e.g.
///
/// ```sh
/// $ pulumi import azure:logicapps/integrationAccountPartner:IntegrationAccountPartner example /subscriptions/00000000-0000-0000-0000-000000000000/resourceGroups/group1/providers/Microsoft.Logic/integrationAccounts/account1/partners/partner1
/// ```
///
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod integration_account_partner {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct IntegrationAccountPartnerArgs {
        /// A `business_identity` block as documented below.
        #[builder(into)]
        pub business_identities: pulumi_gestalt_rust::InputOrOutput<
            Vec<
                super::super::types::logicapps::IntegrationAccountPartnerBusinessIdentity,
            >,
        >,
        /// The name of the Logic App Integration Account. Changing this forces a new Logic App Integration Account Partner to be created.
        #[builder(into)]
        pub integration_account_name: pulumi_gestalt_rust::InputOrOutput<String>,
        /// A JSON mapping of any Metadata for this Logic App Integration Account Partner.
        #[builder(into, default)]
        pub metadata: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The name which should be used for this Logic App Integration Account Partner. Changing this forces a new Logic App Integration Account Partner to be created.
        #[builder(into, default)]
        pub name: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The name of the Resource Group where the Logic App Integration Account Partner should exist. Changing this forces a new Logic App Integration Account Partner to be created.
        #[builder(into)]
        pub resource_group_name: pulumi_gestalt_rust::InputOrOutput<String>,
    }
    #[allow(dead_code)]
    pub struct IntegrationAccountPartnerResult {
        /// A `business_identity` block as documented below.
        pub business_identities: pulumi_gestalt_rust::Output<
            Vec<
                super::super::types::logicapps::IntegrationAccountPartnerBusinessIdentity,
            >,
        >,
        /// The name of the Logic App Integration Account. Changing this forces a new Logic App Integration Account Partner to be created.
        pub integration_account_name: pulumi_gestalt_rust::Output<String>,
        /// A JSON mapping of any Metadata for this Logic App Integration Account Partner.
        pub metadata: pulumi_gestalt_rust::Output<Option<String>>,
        /// The name which should be used for this Logic App Integration Account Partner. Changing this forces a new Logic App Integration Account Partner to be created.
        pub name: pulumi_gestalt_rust::Output<String>,
        /// The name of the Resource Group where the Logic App Integration Account Partner should exist. Changing this forces a new Logic App Integration Account Partner to be created.
        pub resource_group_name: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::PulumiContext,
        name: &str,
        args: IntegrationAccountPartnerArgs,
    ) -> IntegrationAccountPartnerResult {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::register_interface;
        use std::collections::HashMap;
        let business_identities_binding_1 = args.business_identities.get_output(context);
        let business_identities_binding = business_identities_binding_1.get_inner();
        let integration_account_name_binding_1 = args
            .integration_account_name
            .get_output(context);
        let integration_account_name_binding = integration_account_name_binding_1
            .get_inner();
        let metadata_binding_1 = args.metadata.get_output(context);
        let metadata_binding = metadata_binding_1.get_inner();
        let name_binding_1 = args.name.get_output(context);
        let name_binding = name_binding_1.get_inner();
        let resource_group_name_binding_1 = args.resource_group_name.get_output(context);
        let resource_group_name_binding = resource_group_name_binding_1.get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "azure:logicapps/integrationAccountPartner:IntegrationAccountPartner"
                .into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "businessIdentities".into(),
                    value: &business_identities_binding,
                },
                register_interface::ObjectField {
                    name: "integrationAccountName".into(),
                    value: &integration_account_name_binding,
                },
                register_interface::ObjectField {
                    name: "metadata".into(),
                    value: &metadata_binding,
                },
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
        let o = register_interface::register(context.get_inner(), &request);
        IntegrationAccountPartnerResult {
            business_identities: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("businessIdentities"),
            ),
            integration_account_name: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("integrationAccountName"),
            ),
            metadata: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("metadata"),
            ),
            name: pulumi_gestalt_rust::__private::into_domain(o.extract_field("name")),
            resource_group_name: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("resourceGroupName"),
            ),
        }
    }
}
