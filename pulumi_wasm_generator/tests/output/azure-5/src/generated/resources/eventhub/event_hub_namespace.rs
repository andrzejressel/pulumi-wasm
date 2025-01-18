/// Manages an EventHub Namespace.
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
///   exampleEventHubNamespace:
///     type: azure:eventhub:EventHubNamespace
///     name: example
///     properties:
///       name: example-namespace
///       location: ${example.location}
///       resourceGroupName: ${example.name}
///       sku: Standard
///       capacity: 2
///       tags:
///         environment: Production
/// ```
///
/// ## Import
///
/// EventHub Namespaces can be imported using the `resource id`, e.g.
///
/// ```sh
/// $ pulumi import azure:eventhub/eventHubNamespace:EventHubNamespace namespace1 /subscriptions/00000000-0000-0000-0000-000000000000/resourceGroups/group1/providers/Microsoft.EventHub/namespaces/namespace1
/// ```
///
pub mod event_hub_namespace {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct EventHubNamespaceArgs {
        /// Is Auto Inflate enabled for the EventHub Namespace?
        #[builder(into, default)]
        pub auto_inflate_enabled: pulumi_wasm_rust::Output<Option<bool>>,
        /// Specifies the Capacity / Throughput Units for a `Standard` SKU namespace. Default capacity has a maximum of `2`, but can be increased in blocks of 2 on a committed purchase basis. Defaults to `1`.
        #[builder(into, default)]
        pub capacity: pulumi_wasm_rust::Output<Option<i32>>,
        /// Specifies the ID of the EventHub Dedicated Cluster where this Namespace should created. Changing this forces a new resource to be created.
        #[builder(into, default)]
        pub dedicated_cluster_id: pulumi_wasm_rust::Output<Option<String>>,
        /// An `identity` block as defined below.
        #[builder(into, default)]
        pub identity: pulumi_wasm_rust::Output<
            Option<super::super::types::eventhub::EventHubNamespaceIdentity>,
        >,
        /// Is SAS authentication enabled for the EventHub Namespace? Defaults to `true`.
        #[builder(into, default)]
        pub local_authentication_enabled: pulumi_wasm_rust::Output<Option<bool>>,
        /// Specifies the supported Azure location where the resource exists. Changing this forces a new resource to be created.
        #[builder(into, default)]
        pub location: pulumi_wasm_rust::Output<Option<String>>,
        /// Specifies the maximum number of throughput units when Auto Inflate is Enabled. Valid values range from `1` - `20`.
        #[builder(into, default)]
        pub maximum_throughput_units: pulumi_wasm_rust::Output<Option<i32>>,
        /// The minimum supported TLS version for this EventHub Namespace. Valid values are: `1.0`, `1.1` and `1.2`. Defaults to `1.2`.
        ///
        /// > **Note** Azure Services will require TLS 1.2+ by August 2025, please see this [announcement](https://azure.microsoft.com/en-us/updates/v2/update-retirement-tls1-0-tls1-1-versions-azure-services/) for more.
        #[builder(into, default)]
        pub minimum_tls_version: pulumi_wasm_rust::Output<Option<String>>,
        /// Specifies the name of the EventHub Namespace resource. Changing this forces a new resource to be created.
        #[builder(into, default)]
        pub name: pulumi_wasm_rust::Output<Option<String>>,
        /// A `network_rulesets` block as defined below.
        #[builder(into, default)]
        pub network_rulesets: pulumi_wasm_rust::Output<
            Option<super::super::types::eventhub::EventHubNamespaceNetworkRulesets>,
        >,
        /// Is public network access enabled for the EventHub Namespace? Defaults to `true`.
        #[builder(into, default)]
        pub public_network_access_enabled: pulumi_wasm_rust::Output<Option<bool>>,
        /// The name of the resource group in which to create the namespace. Changing this forces a new resource to be created.
        #[builder(into)]
        pub resource_group_name: pulumi_wasm_rust::Output<String>,
        /// Defines which tier to use. Valid options are `Basic`, `Standard`, and `Premium`. Please note that setting this field to `Premium` will force the creation of a new resource.
        #[builder(into)]
        pub sku: pulumi_wasm_rust::Output<String>,
        /// A mapping of tags to assign to the resource.
        #[builder(into, default)]
        pub tags: pulumi_wasm_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
    }
    #[allow(dead_code)]
    pub struct EventHubNamespaceResult {
        /// Is Auto Inflate enabled for the EventHub Namespace?
        pub auto_inflate_enabled: pulumi_wasm_rust::Output<Option<bool>>,
        /// Specifies the Capacity / Throughput Units for a `Standard` SKU namespace. Default capacity has a maximum of `2`, but can be increased in blocks of 2 on a committed purchase basis. Defaults to `1`.
        pub capacity: pulumi_wasm_rust::Output<Option<i32>>,
        /// Specifies the ID of the EventHub Dedicated Cluster where this Namespace should created. Changing this forces a new resource to be created.
        pub dedicated_cluster_id: pulumi_wasm_rust::Output<Option<String>>,
        /// The primary connection string for the authorization rule `RootManageSharedAccessKey`.
        pub default_primary_connection_string: pulumi_wasm_rust::Output<String>,
        /// The alias of the primary connection string for the authorization rule `RootManageSharedAccessKey`, which is generated when disaster recovery is enabled.
        pub default_primary_connection_string_alias: pulumi_wasm_rust::Output<String>,
        /// The primary access key for the authorization rule `RootManageSharedAccessKey`.
        pub default_primary_key: pulumi_wasm_rust::Output<String>,
        /// The secondary connection string for the authorization rule `RootManageSharedAccessKey`.
        pub default_secondary_connection_string: pulumi_wasm_rust::Output<String>,
        /// The alias of the secondary connection string for the authorization rule `RootManageSharedAccessKey`, which is generated when disaster recovery is enabled.
        pub default_secondary_connection_string_alias: pulumi_wasm_rust::Output<String>,
        /// The secondary access key for the authorization rule `RootManageSharedAccessKey`.
        pub default_secondary_key: pulumi_wasm_rust::Output<String>,
        /// An `identity` block as defined below.
        pub identity: pulumi_wasm_rust::Output<
            Option<super::super::types::eventhub::EventHubNamespaceIdentity>,
        >,
        /// Is SAS authentication enabled for the EventHub Namespace? Defaults to `true`.
        pub local_authentication_enabled: pulumi_wasm_rust::Output<Option<bool>>,
        /// Specifies the supported Azure location where the resource exists. Changing this forces a new resource to be created.
        pub location: pulumi_wasm_rust::Output<String>,
        /// Specifies the maximum number of throughput units when Auto Inflate is Enabled. Valid values range from `1` - `20`.
        pub maximum_throughput_units: pulumi_wasm_rust::Output<Option<i32>>,
        /// The minimum supported TLS version for this EventHub Namespace. Valid values are: `1.0`, `1.1` and `1.2`. Defaults to `1.2`.
        ///
        /// > **Note** Azure Services will require TLS 1.2+ by August 2025, please see this [announcement](https://azure.microsoft.com/en-us/updates/v2/update-retirement-tls1-0-tls1-1-versions-azure-services/) for more.
        pub minimum_tls_version: pulumi_wasm_rust::Output<Option<String>>,
        /// Specifies the name of the EventHub Namespace resource. Changing this forces a new resource to be created.
        pub name: pulumi_wasm_rust::Output<String>,
        /// A `network_rulesets` block as defined below.
        pub network_rulesets: pulumi_wasm_rust::Output<
            super::super::types::eventhub::EventHubNamespaceNetworkRulesets,
        >,
        /// Is public network access enabled for the EventHub Namespace? Defaults to `true`.
        pub public_network_access_enabled: pulumi_wasm_rust::Output<Option<bool>>,
        /// The name of the resource group in which to create the namespace. Changing this forces a new resource to be created.
        pub resource_group_name: pulumi_wasm_rust::Output<String>,
        /// Defines which tier to use. Valid options are `Basic`, `Standard`, and `Premium`. Please note that setting this field to `Premium` will force the creation of a new resource.
        pub sku: pulumi_wasm_rust::Output<String>,
        /// A mapping of tags to assign to the resource.
        pub tags: pulumi_wasm_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(name: &str, args: EventHubNamespaceArgs) -> EventHubNamespaceResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let auto_inflate_enabled_binding = args.auto_inflate_enabled.get_inner();
        let capacity_binding = args.capacity.get_inner();
        let dedicated_cluster_id_binding = args.dedicated_cluster_id.get_inner();
        let identity_binding = args.identity.get_inner();
        let local_authentication_enabled_binding = args
            .local_authentication_enabled
            .get_inner();
        let location_binding = args.location.get_inner();
        let maximum_throughput_units_binding = args.maximum_throughput_units.get_inner();
        let minimum_tls_version_binding = args.minimum_tls_version.get_inner();
        let name_binding = args.name.get_inner();
        let network_rulesets_binding = args.network_rulesets.get_inner();
        let public_network_access_enabled_binding = args
            .public_network_access_enabled
            .get_inner();
        let resource_group_name_binding = args.resource_group_name.get_inner();
        let sku_binding = args.sku.get_inner();
        let tags_binding = args.tags.get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "azure:eventhub/eventHubNamespace:EventHubNamespace".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "autoInflateEnabled".into(),
                    value: &auto_inflate_enabled_binding,
                },
                register_interface::ObjectField {
                    name: "capacity".into(),
                    value: &capacity_binding,
                },
                register_interface::ObjectField {
                    name: "dedicatedClusterId".into(),
                    value: &dedicated_cluster_id_binding,
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
                    name: "maximumThroughputUnits".into(),
                    value: &maximum_throughput_units_binding,
                },
                register_interface::ObjectField {
                    name: "minimumTlsVersion".into(),
                    value: &minimum_tls_version_binding,
                },
                register_interface::ObjectField {
                    name: "name".into(),
                    value: &name_binding,
                },
                register_interface::ObjectField {
                    name: "networkRulesets".into(),
                    value: &network_rulesets_binding,
                },
                register_interface::ObjectField {
                    name: "publicNetworkAccessEnabled".into(),
                    value: &public_network_access_enabled_binding,
                },
                register_interface::ObjectField {
                    name: "resourceGroupName".into(),
                    value: &resource_group_name_binding,
                },
                register_interface::ObjectField {
                    name: "sku".into(),
                    value: &sku_binding,
                },
                register_interface::ObjectField {
                    name: "tags".into(),
                    value: &tags_binding,
                },
            ]),
            results: Vec::from([
                register_interface::ResultField {
                    name: "autoInflateEnabled".into(),
                },
                register_interface::ResultField {
                    name: "capacity".into(),
                },
                register_interface::ResultField {
                    name: "dedicatedClusterId".into(),
                },
                register_interface::ResultField {
                    name: "defaultPrimaryConnectionString".into(),
                },
                register_interface::ResultField {
                    name: "defaultPrimaryConnectionStringAlias".into(),
                },
                register_interface::ResultField {
                    name: "defaultPrimaryKey".into(),
                },
                register_interface::ResultField {
                    name: "defaultSecondaryConnectionString".into(),
                },
                register_interface::ResultField {
                    name: "defaultSecondaryConnectionStringAlias".into(),
                },
                register_interface::ResultField {
                    name: "defaultSecondaryKey".into(),
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
                    name: "maximumThroughputUnits".into(),
                },
                register_interface::ResultField {
                    name: "minimumTlsVersion".into(),
                },
                register_interface::ResultField {
                    name: "name".into(),
                },
                register_interface::ResultField {
                    name: "networkRulesets".into(),
                },
                register_interface::ResultField {
                    name: "publicNetworkAccessEnabled".into(),
                },
                register_interface::ResultField {
                    name: "resourceGroupName".into(),
                },
                register_interface::ResultField {
                    name: "sku".into(),
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
        EventHubNamespaceResult {
            auto_inflate_enabled: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("autoInflateEnabled").unwrap(),
            ),
            capacity: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("capacity").unwrap(),
            ),
            dedicated_cluster_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("dedicatedClusterId").unwrap(),
            ),
            default_primary_connection_string: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("defaultPrimaryConnectionString").unwrap(),
            ),
            default_primary_connection_string_alias: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("defaultPrimaryConnectionStringAlias").unwrap(),
            ),
            default_primary_key: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("defaultPrimaryKey").unwrap(),
            ),
            default_secondary_connection_string: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("defaultSecondaryConnectionString").unwrap(),
            ),
            default_secondary_connection_string_alias: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("defaultSecondaryConnectionStringAlias").unwrap(),
            ),
            default_secondary_key: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("defaultSecondaryKey").unwrap(),
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
            maximum_throughput_units: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("maximumThroughputUnits").unwrap(),
            ),
            minimum_tls_version: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("minimumTlsVersion").unwrap(),
            ),
            name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("name").unwrap(),
            ),
            network_rulesets: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("networkRulesets").unwrap(),
            ),
            public_network_access_enabled: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("publicNetworkAccessEnabled").unwrap(),
            ),
            resource_group_name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("resourceGroupName").unwrap(),
            ),
            sku: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("sku").unwrap(),
            ),
            tags: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("tags").unwrap(),
            ),
        }
    }
}
