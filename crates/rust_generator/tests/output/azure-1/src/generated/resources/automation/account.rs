/// Manages a Automation Account.
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
///   exampleAccount:
///     type: azure:automation:Account
///     name: example
///     properties:
///       name: example-account
///       location: ${example.location}
///       resourceGroupName: ${example.name}
///       skuName: Basic
///       tags:
///         environment: development
/// ```
///
/// ## Import
///
/// Automation Accounts can be imported using the `resource id`, e.g.
///
/// ```sh
/// $ pulumi import azure:automation/account:Account account1 /subscriptions/00000000-0000-0000-0000-000000000000/resourceGroups/group1/providers/Microsoft.Automation/automationAccounts/account1
/// ```
///
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod account {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct AccountArgs {
        #[builder(into, default)]
        pub encryptions: pulumi_gestalt_rust::InputOrOutput<
            Option<Vec<super::super::types::automation::AccountEncryption>>,
        >,
        /// An `identity` block as defined below.
        #[builder(into, default)]
        pub identity: pulumi_gestalt_rust::InputOrOutput<
            Option<super::super::types::automation::AccountIdentity>,
        >,
        /// Whether requests using non-AAD authentication are blocked. Defaults to `true`.
        #[builder(into, default)]
        pub local_authentication_enabled: pulumi_gestalt_rust::InputOrOutput<
            Option<bool>,
        >,
        /// Specifies the supported Azure location where the resource exists. Changing this forces a new resource to be created.
        #[builder(into, default)]
        pub location: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Specifies the name of the Automation Account. Changing this forces a new resource to be created.
        #[builder(into, default)]
        pub name: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Whether public network access is allowed for the automation account. Defaults to `true`.
        #[builder(into, default)]
        pub public_network_access_enabled: pulumi_gestalt_rust::InputOrOutput<
            Option<bool>,
        >,
        /// The name of the resource group in which the Automation Account is created. Changing this forces a new resource to be created.
        #[builder(into)]
        pub resource_group_name: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The SKU of the account. Possible values are `Basic` and `Free`.
        #[builder(into)]
        pub sku_name: pulumi_gestalt_rust::InputOrOutput<String>,
        #[builder(into, default)]
        pub tags: pulumi_gestalt_rust::InputOrOutput<
            Option<std::collections::HashMap<String, String>>,
        >,
    }
    #[allow(dead_code)]
    pub struct AccountResult {
        /// The Primary Access Key for the DSC Endpoint associated with this Automation Account.
        pub dsc_primary_access_key: pulumi_gestalt_rust::Output<String>,
        /// The Secondary Access Key for the DSC Endpoint associated with this Automation Account.
        pub dsc_secondary_access_key: pulumi_gestalt_rust::Output<String>,
        /// The DSC Server Endpoint associated with this Automation Account.
        pub dsc_server_endpoint: pulumi_gestalt_rust::Output<String>,
        pub encryptions: pulumi_gestalt_rust::Output<
            Option<Vec<super::super::types::automation::AccountEncryption>>,
        >,
        /// The URL of automation hybrid service which is used for hybrid worker on-boarding With this Automation Account.
        pub hybrid_service_url: pulumi_gestalt_rust::Output<String>,
        /// An `identity` block as defined below.
        pub identity: pulumi_gestalt_rust::Output<
            Option<super::super::types::automation::AccountIdentity>,
        >,
        /// Whether requests using non-AAD authentication are blocked. Defaults to `true`.
        pub local_authentication_enabled: pulumi_gestalt_rust::Output<Option<bool>>,
        /// Specifies the supported Azure location where the resource exists. Changing this forces a new resource to be created.
        pub location: pulumi_gestalt_rust::Output<String>,
        /// Specifies the name of the Automation Account. Changing this forces a new resource to be created.
        pub name: pulumi_gestalt_rust::Output<String>,
        pub private_endpoint_connections: pulumi_gestalt_rust::Output<
            Vec<super::super::types::automation::AccountPrivateEndpointConnection>,
        >,
        /// Whether public network access is allowed for the automation account. Defaults to `true`.
        pub public_network_access_enabled: pulumi_gestalt_rust::Output<Option<bool>>,
        /// The name of the resource group in which the Automation Account is created. Changing this forces a new resource to be created.
        pub resource_group_name: pulumi_gestalt_rust::Output<String>,
        /// The SKU of the account. Possible values are `Basic` and `Free`.
        pub sku_name: pulumi_gestalt_rust::Output<String>,
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
        args: AccountArgs,
    ) -> AccountResult {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::register_interface;
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let encryptions_binding = args.encryptions.get_output(context);
        let identity_binding = args.identity.get_output(context);
        let local_authentication_enabled_binding = args
            .local_authentication_enabled
            .get_output(context);
        let location_binding = args.location.get_output(context);
        let name_binding = args.name.get_output(context);
        let public_network_access_enabled_binding = args
            .public_network_access_enabled
            .get_output(context);
        let resource_group_name_binding = args.resource_group_name.get_output(context);
        let sku_name_binding = args.sku_name.get_output(context);
        let tags_binding = args.tags.get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "azure:automation/account:Account".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "encryptions".into(),
                    value: encryptions_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "identity".into(),
                    value: identity_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "localAuthenticationEnabled".into(),
                    value: local_authentication_enabled_binding.get_id(),
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
                    name: "publicNetworkAccessEnabled".into(),
                    value: public_network_access_enabled_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "resourceGroupName".into(),
                    value: resource_group_name_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "skuName".into(),
                    value: sku_name_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "tags".into(),
                    value: tags_binding.get_id(),
                },
            ],
        };
        let o = context.register_resource(request);
        AccountResult {
            dsc_primary_access_key: o.get_field("dscPrimaryAccessKey"),
            dsc_secondary_access_key: o.get_field("dscSecondaryAccessKey"),
            dsc_server_endpoint: o.get_field("dscServerEndpoint"),
            encryptions: o.get_field("encryptions"),
            hybrid_service_url: o.get_field("hybridServiceUrl"),
            identity: o.get_field("identity"),
            local_authentication_enabled: o.get_field("localAuthenticationEnabled"),
            location: o.get_field("location"),
            name: o.get_field("name"),
            private_endpoint_connections: o.get_field("privateEndpointConnections"),
            public_network_access_enabled: o.get_field("publicNetworkAccessEnabled"),
            resource_group_name: o.get_field("resourceGroupName"),
            sku_name: o.get_field("skuName"),
            tags: o.get_field("tags"),
        }
    }
}
