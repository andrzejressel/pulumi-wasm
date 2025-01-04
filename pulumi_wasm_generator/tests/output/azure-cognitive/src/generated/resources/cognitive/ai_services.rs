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
pub mod ai_services {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct AIServicesArgs {
        /// The subdomain name used for token-based authentication. This property is required when `network_acls` is specified. Changing this forces a new resource to be created.
        #[builder(into, default)]
        pub custom_subdomain_name: pulumi_wasm_rust::Output<Option<String>>,
        /// A `customer_managed_key` block as documented below.
        #[builder(into, default)]
        pub customer_managed_key: pulumi_wasm_rust::Output<
            Option<super::super::types::cognitive::AiServicesCustomerManagedKey>,
        >,
        /// List of FQDNs allowed for the AI Services Account.
        #[builder(into, default)]
        pub fqdns: pulumi_wasm_rust::Output<Option<Vec<String>>>,
        /// An `identity` block as defined below.
        #[builder(into, default)]
        pub identity: pulumi_wasm_rust::Output<
            Option<super::super::types::cognitive::AiServicesIdentity>,
        >,
        /// Whether local authentication is enabled for the AI Services Account. Defaults to `true`.
        #[builder(into, default)]
        pub local_authentication_enabled: pulumi_wasm_rust::Output<Option<bool>>,
        /// Specifies the supported Azure location where the resource exists. Changing this forces a new resource to be created.
        #[builder(into, default)]
        pub location: pulumi_wasm_rust::Output<Option<String>>,
        /// Specifies the name of the AI Services Account. Changing this forces a new resource to be created.
        #[builder(into, default)]
        pub name: pulumi_wasm_rust::Output<Option<String>>,
        /// A `network_acls` block as defined below. When this property is specified, `custom_subdomain_name` is also required to be set.
        #[builder(into, default)]
        pub network_acls: pulumi_wasm_rust::Output<
            Option<super::super::types::cognitive::AiServicesNetworkAcls>,
        >,
        /// Whether outbound network access is restricted for the AI Services Account. Defaults to `false`.
        #[builder(into, default)]
        pub outbound_network_access_restricted: pulumi_wasm_rust::Output<Option<bool>>,
        /// Whether public network access is allowed for the AI Services Account. Possible values are `Enabled` and `Disabled`. Defaults to `Enabled`.
        #[builder(into, default)]
        pub public_network_access: pulumi_wasm_rust::Output<Option<String>>,
        /// The name of the resource group in which the AI Services Account is created. Changing this forces a new resource to be created.
        #[builder(into)]
        pub resource_group_name: pulumi_wasm_rust::Output<String>,
        /// Specifies the SKU Name for this AI Services Account. Possible values are `F0`, `F1`, `S0`, `S`, `S1`, `S2`, `S3`, `S4`, `S5`, `S6`, `P0`, `P1`, `P2`, `E0` and `DC0`.
        ///
        /// > **NOTE:** SKU `DC0` is the commitment tier for AI Services Account containers running in disconnected environments. You must obtain approval from Microsoft by submitting the [request form](https://aka.ms/csdisconnectedcontainers) first, before you can use this SKU. More information on [Purchase a commitment plan to use containers in disconnected environments](https://learn.microsoft.com/en-us/azure/cognitive-services/containers/disconnected-containers?tabs=stt#purchase-a-commitment-plan-to-use-containers-in-disconnected-environments).
        #[builder(into)]
        pub sku_name: pulumi_wasm_rust::Output<String>,
        /// A `storage` block as defined below.
        #[builder(into, default)]
        pub storages: pulumi_wasm_rust::Output<
            Option<Vec<super::super::types::cognitive::AiServicesStorage>>,
        >,
        /// A mapping of tags to assign to the resource.
        #[builder(into, default)]
        pub tags: pulumi_wasm_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
    }
    #[allow(dead_code)]
    pub struct AIServicesResult {
        /// The subdomain name used for token-based authentication. This property is required when `network_acls` is specified. Changing this forces a new resource to be created.
        pub custom_subdomain_name: pulumi_wasm_rust::Output<Option<String>>,
        /// A `customer_managed_key` block as documented below.
        pub customer_managed_key: pulumi_wasm_rust::Output<
            Option<super::super::types::cognitive::AiServicesCustomerManagedKey>,
        >,
        /// The endpoint used to connect to the AI Services Account.
        pub endpoint: pulumi_wasm_rust::Output<String>,
        /// List of FQDNs allowed for the AI Services Account.
        pub fqdns: pulumi_wasm_rust::Output<Option<Vec<String>>>,
        /// An `identity` block as defined below.
        pub identity: pulumi_wasm_rust::Output<
            Option<super::super::types::cognitive::AiServicesIdentity>,
        >,
        /// Whether local authentication is enabled for the AI Services Account. Defaults to `true`.
        pub local_authentication_enabled: pulumi_wasm_rust::Output<Option<bool>>,
        /// Specifies the supported Azure location where the resource exists. Changing this forces a new resource to be created.
        pub location: pulumi_wasm_rust::Output<String>,
        /// Specifies the name of the AI Services Account. Changing this forces a new resource to be created.
        pub name: pulumi_wasm_rust::Output<String>,
        /// A `network_acls` block as defined below. When this property is specified, `custom_subdomain_name` is also required to be set.
        pub network_acls: pulumi_wasm_rust::Output<
            Option<super::super::types::cognitive::AiServicesNetworkAcls>,
        >,
        /// Whether outbound network access is restricted for the AI Services Account. Defaults to `false`.
        pub outbound_network_access_restricted: pulumi_wasm_rust::Output<Option<bool>>,
        /// A primary access key which can be used to connect to the AI Services Account.
        pub primary_access_key: pulumi_wasm_rust::Output<String>,
        /// Whether public network access is allowed for the AI Services Account. Possible values are `Enabled` and `Disabled`. Defaults to `Enabled`.
        pub public_network_access: pulumi_wasm_rust::Output<Option<String>>,
        /// The name of the resource group in which the AI Services Account is created. Changing this forces a new resource to be created.
        pub resource_group_name: pulumi_wasm_rust::Output<String>,
        /// The secondary access key which can be used to connect to the AI Services Account.
        pub secondary_access_key: pulumi_wasm_rust::Output<String>,
        /// Specifies the SKU Name for this AI Services Account. Possible values are `F0`, `F1`, `S0`, `S`, `S1`, `S2`, `S3`, `S4`, `S5`, `S6`, `P0`, `P1`, `P2`, `E0` and `DC0`.
        ///
        /// > **NOTE:** SKU `DC0` is the commitment tier for AI Services Account containers running in disconnected environments. You must obtain approval from Microsoft by submitting the [request form](https://aka.ms/csdisconnectedcontainers) first, before you can use this SKU. More information on [Purchase a commitment plan to use containers in disconnected environments](https://learn.microsoft.com/en-us/azure/cognitive-services/containers/disconnected-containers?tabs=stt#purchase-a-commitment-plan-to-use-containers-in-disconnected-environments).
        pub sku_name: pulumi_wasm_rust::Output<String>,
        /// A `storage` block as defined below.
        pub storages: pulumi_wasm_rust::Output<
            Option<Vec<super::super::types::cognitive::AiServicesStorage>>,
        >,
        /// A mapping of tags to assign to the resource.
        pub tags: pulumi_wasm_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(name: &str, args: AIServicesArgs) -> AIServicesResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let custom_subdomain_name_binding = args.custom_subdomain_name.get_inner();
        let customer_managed_key_binding = args.customer_managed_key.get_inner();
        let fqdns_binding = args.fqdns.get_inner();
        let identity_binding = args.identity.get_inner();
        let local_authentication_enabled_binding = args
            .local_authentication_enabled
            .get_inner();
        let location_binding = args.location.get_inner();
        let name_binding = args.name.get_inner();
        let network_acls_binding = args.network_acls.get_inner();
        let outbound_network_access_restricted_binding = args
            .outbound_network_access_restricted
            .get_inner();
        let public_network_access_binding = args.public_network_access.get_inner();
        let resource_group_name_binding = args.resource_group_name.get_inner();
        let sku_name_binding = args.sku_name.get_inner();
        let storages_binding = args.storages.get_inner();
        let tags_binding = args.tags.get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "azure:cognitive/aIServices:AIServices".into(),
            name: name.to_string(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "customSubdomainName".into(),
                    value: &custom_subdomain_name_binding,
                },
                register_interface::ObjectField {
                    name: "customerManagedKey".into(),
                    value: &customer_managed_key_binding,
                },
                register_interface::ObjectField {
                    name: "fqdns".into(),
                    value: &fqdns_binding,
                },
                register_interface::ObjectField {
                    name: "identity".into(),
                    value: &identity_binding,
                },
                register_interface::ObjectField {
                    name: "localAuthenticationEnabled".into(),
                    value: &local_authentication_enabled_binding,
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
                    name: "networkAcls".into(),
                    value: &network_acls_binding,
                },
                register_interface::ObjectField {
                    name: "outboundNetworkAccessRestricted".into(),
                    value: &outbound_network_access_restricted_binding,
                },
                register_interface::ObjectField {
                    name: "publicNetworkAccess".into(),
                    value: &public_network_access_binding,
                },
                register_interface::ObjectField {
                    name: "resourceGroupName".into(),
                    value: &resource_group_name_binding,
                },
                register_interface::ObjectField {
                    name: "skuName".into(),
                    value: &sku_name_binding,
                },
                register_interface::ObjectField {
                    name: "storages".into(),
                    value: &storages_binding,
                },
                register_interface::ObjectField {
                    name: "tags".into(),
                    value: &tags_binding,
                },
            ]),
            results: Vec::from([
                register_interface::ResultField {
                    name: "customSubdomainName".into(),
                },
                register_interface::ResultField {
                    name: "customerManagedKey".into(),
                },
                register_interface::ResultField {
                    name: "endpoint".into(),
                },
                register_interface::ResultField {
                    name: "fqdns".into(),
                },
                register_interface::ResultField {
                    name: "identity".into(),
                },
                register_interface::ResultField {
                    name: "localAuthenticationEnabled".into(),
                },
                register_interface::ResultField {
                    name: "location".into(),
                },
                register_interface::ResultField {
                    name: "name".into(),
                },
                register_interface::ResultField {
                    name: "networkAcls".into(),
                },
                register_interface::ResultField {
                    name: "outboundNetworkAccessRestricted".into(),
                },
                register_interface::ResultField {
                    name: "primaryAccessKey".into(),
                },
                register_interface::ResultField {
                    name: "publicNetworkAccess".into(),
                },
                register_interface::ResultField {
                    name: "resourceGroupName".into(),
                },
                register_interface::ResultField {
                    name: "secondaryAccessKey".into(),
                },
                register_interface::ResultField {
                    name: "skuName".into(),
                },
                register_interface::ResultField {
                    name: "storages".into(),
                },
                register_interface::ResultField {
                    name: "tags".into(),
                },
            ]),
        };
        let o = register_interface::register(&request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        AIServicesResult {
            custom_subdomain_name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("customSubdomainName").unwrap(),
            ),
            customer_managed_key: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("customerManagedKey").unwrap(),
            ),
            endpoint: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("endpoint").unwrap(),
            ),
            fqdns: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("fqdns").unwrap(),
            ),
            identity: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("identity").unwrap(),
            ),
            local_authentication_enabled: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("localAuthenticationEnabled").unwrap(),
            ),
            location: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("location").unwrap(),
            ),
            name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("name").unwrap(),
            ),
            network_acls: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("networkAcls").unwrap(),
            ),
            outbound_network_access_restricted: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("outboundNetworkAccessRestricted").unwrap(),
            ),
            primary_access_key: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("primaryAccessKey").unwrap(),
            ),
            public_network_access: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("publicNetworkAccess").unwrap(),
            ),
            resource_group_name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("resourceGroupName").unwrap(),
            ),
            secondary_access_key: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("secondaryAccessKey").unwrap(),
            ),
            sku_name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("skuName").unwrap(),
            ),
            storages: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("storages").unwrap(),
            ),
            tags: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("tags").unwrap(),
            ),
        }
    }
}
