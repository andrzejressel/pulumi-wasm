/// Manages an AI Services account.
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
///   exampleAIServices:
///     type: azure:cognitive:AIServices
///     name: example
///     properties:
///       name: example-account
///       location: ${example.location}
///       resourceGroupName: ${example.name}
///       skuName: S0
///       tags:
///         Acceptance: Test
/// ```
///
/// ## Import
///
/// AI Services Account can be imported using the `resource id`, e.g.
///
/// ```sh
/// $ pulumi import azure:cognitive/aIServices:AIServices account1 /subscriptions/00000000-0000-0000-0000-000000000000/resourceGroups/group1/providers/Microsoft.CognitiveServices/accounts/account1
/// ```
///
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod ai_services {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct AIServicesArgs {
        /// The subdomain name used for token-based authentication. This property is required when `network_acls` is specified. Changing this forces a new resource to be created.
        #[builder(into, default)]
        pub custom_subdomain_name: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// A `customer_managed_key` block as documented below.
        #[builder(into, default)]
        pub customer_managed_key: pulumi_gestalt_rust::InputOrOutput<
            Option<super::super::types::cognitive::AiServicesCustomerManagedKey>,
        >,
        /// List of FQDNs allowed for the AI Services Account.
        #[builder(into, default)]
        pub fqdns: pulumi_gestalt_rust::InputOrOutput<Option<Vec<String>>>,
        /// An `identity` block as defined below.
        #[builder(into, default)]
        pub identity: pulumi_gestalt_rust::InputOrOutput<
            Option<super::super::types::cognitive::AiServicesIdentity>,
        >,
        /// Whether local authentication is enabled for the AI Services Account. Defaults to `true`.
        #[builder(into, default)]
        pub local_authentication_enabled: pulumi_gestalt_rust::InputOrOutput<
            Option<bool>,
        >,
        /// Specifies the supported Azure location where the resource exists. Changing this forces a new resource to be created.
        #[builder(into, default)]
        pub location: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Specifies the name of the AI Services Account. Changing this forces a new resource to be created.
        #[builder(into, default)]
        pub name: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// A `network_acls` block as defined below. When this property is specified, `custom_subdomain_name` is also required to be set.
        #[builder(into, default)]
        pub network_acls: pulumi_gestalt_rust::InputOrOutput<
            Option<super::super::types::cognitive::AiServicesNetworkAcls>,
        >,
        /// Whether outbound network access is restricted for the AI Services Account. Defaults to `false`.
        #[builder(into, default)]
        pub outbound_network_access_restricted: pulumi_gestalt_rust::InputOrOutput<
            Option<bool>,
        >,
        /// Whether public network access is allowed for the AI Services Account. Possible values are `Enabled` and `Disabled`. Defaults to `Enabled`.
        #[builder(into, default)]
        pub public_network_access: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The name of the resource group in which the AI Services Account is created. Changing this forces a new resource to be created.
        #[builder(into)]
        pub resource_group_name: pulumi_gestalt_rust::InputOrOutput<String>,
        /// Specifies the SKU Name for this AI Services Account. Possible values are `F0`, `F1`, `S0`, `S`, `S1`, `S2`, `S3`, `S4`, `S5`, `S6`, `P0`, `P1`, `P2`, `E0` and `DC0`.
        ///
        /// > **NOTE:** SKU `DC0` is the commitment tier for AI Services Account containers running in disconnected environments. You must obtain approval from Microsoft by submitting the [request form](https://aka.ms/csdisconnectedcontainers) first, before you can use this SKU. More information on [Purchase a commitment plan to use containers in disconnected environments](https://learn.microsoft.com/en-us/azure/cognitive-services/containers/disconnected-containers?tabs=stt#purchase-a-commitment-plan-to-use-containers-in-disconnected-environments).
        #[builder(into)]
        pub sku_name: pulumi_gestalt_rust::InputOrOutput<String>,
        /// A `storage` block as defined below.
        #[builder(into, default)]
        pub storages: pulumi_gestalt_rust::InputOrOutput<
            Option<Vec<super::super::types::cognitive::AiServicesStorage>>,
        >,
        /// A mapping of tags to assign to the resource.
        #[builder(into, default)]
        pub tags: pulumi_gestalt_rust::InputOrOutput<
            Option<std::collections::HashMap<String, String>>,
        >,
    }
    #[allow(dead_code)]
    pub struct AIServicesResult {
        /// The subdomain name used for token-based authentication. This property is required when `network_acls` is specified. Changing this forces a new resource to be created.
        pub custom_subdomain_name: pulumi_gestalt_rust::Output<Option<String>>,
        /// A `customer_managed_key` block as documented below.
        pub customer_managed_key: pulumi_gestalt_rust::Output<
            Option<super::super::types::cognitive::AiServicesCustomerManagedKey>,
        >,
        /// The endpoint used to connect to the AI Services Account.
        pub endpoint: pulumi_gestalt_rust::Output<String>,
        /// List of FQDNs allowed for the AI Services Account.
        pub fqdns: pulumi_gestalt_rust::Output<Option<Vec<String>>>,
        /// An `identity` block as defined below.
        pub identity: pulumi_gestalt_rust::Output<
            Option<super::super::types::cognitive::AiServicesIdentity>,
        >,
        /// Whether local authentication is enabled for the AI Services Account. Defaults to `true`.
        pub local_authentication_enabled: pulumi_gestalt_rust::Output<Option<bool>>,
        /// Specifies the supported Azure location where the resource exists. Changing this forces a new resource to be created.
        pub location: pulumi_gestalt_rust::Output<String>,
        /// Specifies the name of the AI Services Account. Changing this forces a new resource to be created.
        pub name: pulumi_gestalt_rust::Output<String>,
        /// A `network_acls` block as defined below. When this property is specified, `custom_subdomain_name` is also required to be set.
        pub network_acls: pulumi_gestalt_rust::Output<
            Option<super::super::types::cognitive::AiServicesNetworkAcls>,
        >,
        /// Whether outbound network access is restricted for the AI Services Account. Defaults to `false`.
        pub outbound_network_access_restricted: pulumi_gestalt_rust::Output<
            Option<bool>,
        >,
        /// A primary access key which can be used to connect to the AI Services Account.
        pub primary_access_key: pulumi_gestalt_rust::Output<String>,
        /// Whether public network access is allowed for the AI Services Account. Possible values are `Enabled` and `Disabled`. Defaults to `Enabled`.
        pub public_network_access: pulumi_gestalt_rust::Output<Option<String>>,
        /// The name of the resource group in which the AI Services Account is created. Changing this forces a new resource to be created.
        pub resource_group_name: pulumi_gestalt_rust::Output<String>,
        /// The secondary access key which can be used to connect to the AI Services Account.
        pub secondary_access_key: pulumi_gestalt_rust::Output<String>,
        /// Specifies the SKU Name for this AI Services Account. Possible values are `F0`, `F1`, `S0`, `S`, `S1`, `S2`, `S3`, `S4`, `S5`, `S6`, `P0`, `P1`, `P2`, `E0` and `DC0`.
        ///
        /// > **NOTE:** SKU `DC0` is the commitment tier for AI Services Account containers running in disconnected environments. You must obtain approval from Microsoft by submitting the [request form](https://aka.ms/csdisconnectedcontainers) first, before you can use this SKU. More information on [Purchase a commitment plan to use containers in disconnected environments](https://learn.microsoft.com/en-us/azure/cognitive-services/containers/disconnected-containers?tabs=stt#purchase-a-commitment-plan-to-use-containers-in-disconnected-environments).
        pub sku_name: pulumi_gestalt_rust::Output<String>,
        /// A `storage` block as defined below.
        pub storages: pulumi_gestalt_rust::Output<
            Option<Vec<super::super::types::cognitive::AiServicesStorage>>,
        >,
        /// A mapping of tags to assign to the resource.
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
        args: AIServicesArgs,
    ) -> AIServicesResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let custom_subdomain_name_binding = args
            .custom_subdomain_name
            .get_output(context);
        let customer_managed_key_binding = args.customer_managed_key.get_output(context);
        let fqdns_binding = args.fqdns.get_output(context);
        let identity_binding = args.identity.get_output(context);
        let local_authentication_enabled_binding = args
            .local_authentication_enabled
            .get_output(context);
        let location_binding = args.location.get_output(context);
        let name_binding = args.name.get_output(context);
        let network_acls_binding = args.network_acls.get_output(context);
        let outbound_network_access_restricted_binding = args
            .outbound_network_access_restricted
            .get_output(context);
        let public_network_access_binding = args
            .public_network_access
            .get_output(context);
        let resource_group_name_binding = args.resource_group_name.get_output(context);
        let sku_name_binding = args.sku_name.get_output(context);
        let storages_binding = args.storages.get_output(context);
        let tags_binding = args.tags.get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "azure:cognitive/aIServices:AIServices".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "customSubdomainName".into(),
                    value: &custom_subdomain_name_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "customerManagedKey".into(),
                    value: &customer_managed_key_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "fqdns".into(),
                    value: &fqdns_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "identity".into(),
                    value: &identity_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "localAuthenticationEnabled".into(),
                    value: &local_authentication_enabled_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "location".into(),
                    value: &location_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "name".into(),
                    value: &name_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "networkAcls".into(),
                    value: &network_acls_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "outboundNetworkAccessRestricted".into(),
                    value: &outbound_network_access_restricted_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "publicNetworkAccess".into(),
                    value: &public_network_access_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "resourceGroupName".into(),
                    value: &resource_group_name_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "skuName".into(),
                    value: &sku_name_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "storages".into(),
                    value: &storages_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "tags".into(),
                    value: &tags_binding.drop_type(),
                },
            ],
        };
        let o = context.register_resource(request);
        AIServicesResult {
            custom_subdomain_name: o.get_field("customSubdomainName"),
            customer_managed_key: o.get_field("customerManagedKey"),
            endpoint: o.get_field("endpoint"),
            fqdns: o.get_field("fqdns"),
            identity: o.get_field("identity"),
            local_authentication_enabled: o.get_field("localAuthenticationEnabled"),
            location: o.get_field("location"),
            name: o.get_field("name"),
            network_acls: o.get_field("networkAcls"),
            outbound_network_access_restricted: o
                .get_field("outboundNetworkAccessRestricted"),
            primary_access_key: o.get_field("primaryAccessKey"),
            public_network_access: o.get_field("publicNetworkAccess"),
            resource_group_name: o.get_field("resourceGroupName"),
            secondary_access_key: o.get_field("secondaryAccessKey"),
            sku_name: o.get_field("skuName"),
            storages: o.get_field("storages"),
            tags: o.get_field("tags"),
        }
    }
}
