/// Manages a ServiceBus Namespace.
///
/// ## Example Usage
///
/// ```yaml
/// resources:
///   example:
///     type: azure:core:ResourceGroup
///     properties:
///       name: my-servicebus
///       location: West Europe
///   exampleNamespace:
///     type: azure:servicebus:Namespace
///     name: example
///     properties:
///       name: tfex-servicebus-namespace
///       location: ${example.location}
///       resourceGroupName: ${example.name}
///       sku: Standard
///       tags:
///         source: example
/// ```
///
/// ## Import
///
/// Service Bus Namespace can be imported using the `resource id`, e.g.
///
/// ```sh
/// $ pulumi import azure:eventhub/namespace:Namespace example /subscriptions/00000000-0000-0000-0000-000000000000/resourceGroups/mygroup1/providers/Microsoft.ServiceBus/namespaces/sbns1
/// ```
///
pub mod namespace {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct NamespaceArgs {
        /// Specifies the capacity. When `sku` is `Premium`, capacity can be `1`, `2`, `4`, `8` or `16`. When `sku` is `Basic` or `Standard`, capacity can be `0` only.
        #[builder(into, default)]
        pub capacity: pulumi_wasm_rust::Output<Option<i32>>,
        /// An `customer_managed_key` block as defined below.
        #[builder(into, default)]
        pub customer_managed_key: pulumi_wasm_rust::Output<
            Option<super::super::types::eventhub::NamespaceCustomerManagedKey>,
        >,
        /// An `identity` block as defined below.
        #[builder(into, default)]
        pub identity: pulumi_wasm_rust::Output<
            Option<super::super::types::eventhub::NamespaceIdentity>,
        >,
        /// Whether or not SAS authentication is enabled for the Service Bus namespace. Defaults to `true`.
        #[builder(into, default)]
        pub local_auth_enabled: pulumi_wasm_rust::Output<Option<bool>>,
        /// Specifies the supported Azure location where the resource exists. Changing this forces a new resource to be created.
        #[builder(into, default)]
        pub location: pulumi_wasm_rust::Output<Option<String>>,
        /// The minimum supported TLS version for this Service Bus Namespace. Valid values are: `1.0`, `1.1` and `1.2`. Defaults to `1.2`.
        #[builder(into, default)]
        pub minimum_tls_version: pulumi_wasm_rust::Output<Option<String>>,
        /// Specifies the name of the ServiceBus Namespace resource . Changing this forces a new resource to be created.
        #[builder(into, default)]
        pub name: pulumi_wasm_rust::Output<Option<String>>,
        /// An `network_rule_set` block as defined below.
        #[builder(into, default)]
        pub network_rule_set: pulumi_wasm_rust::Output<
            Option<super::super::types::eventhub::NamespaceNetworkRuleSet>,
        >,
        /// Specifies the number messaging partitions. Only valid when `sku` is `Premium` and the minimum number is `1`. Possible values include `0`, `1`, `2`, and `4`. Defaults to `0` for Standard, Basic namespace. Changing this forces a new resource to be created.
        ///
        /// > **Note:** It's not possible to change the partitioning option on any existing namespace. The number of partitions can only be set during namespace creation. Please check the doc https://learn.microsoft.com/en-us/azure/service-bus-messaging/enable-partitions-premium for more feature restrictions.
        #[builder(into, default)]
        pub premium_messaging_partitions: pulumi_wasm_rust::Output<Option<i32>>,
        /// Is public network access enabled for the Service Bus Namespace? Defaults to `true`.
        #[builder(into, default)]
        pub public_network_access_enabled: pulumi_wasm_rust::Output<Option<bool>>,
        /// The name of the resource group in which to Changing this forces a new resource to be created.
        /// create the namespace.
        #[builder(into)]
        pub resource_group_name: pulumi_wasm_rust::Output<String>,
        /// Defines which tier to use. Options are `Basic`, `Standard` or `Premium`. Please note that setting this field to `Premium` will force the creation of a new resource.
        #[builder(into)]
        pub sku: pulumi_wasm_rust::Output<String>,
        /// A mapping of tags to assign to the resource.
        #[builder(into, default)]
        pub tags: pulumi_wasm_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
    }
    #[allow(dead_code)]
    pub struct NamespaceResult {
        /// Specifies the capacity. When `sku` is `Premium`, capacity can be `1`, `2`, `4`, `8` or `16`. When `sku` is `Basic` or `Standard`, capacity can be `0` only.
        pub capacity: pulumi_wasm_rust::Output<Option<i32>>,
        /// An `customer_managed_key` block as defined below.
        pub customer_managed_key: pulumi_wasm_rust::Output<
            Option<super::super::types::eventhub::NamespaceCustomerManagedKey>,
        >,
        /// The primary connection string for the authorization rule `RootManageSharedAccessKey`.
        pub default_primary_connection_string: pulumi_wasm_rust::Output<String>,
        /// The primary access key for the authorization rule `RootManageSharedAccessKey`.
        pub default_primary_key: pulumi_wasm_rust::Output<String>,
        /// The secondary connection string for the authorization rule `RootManageSharedAccessKey`.
        pub default_secondary_connection_string: pulumi_wasm_rust::Output<String>,
        /// The secondary access key for the authorization rule `RootManageSharedAccessKey`.
        pub default_secondary_key: pulumi_wasm_rust::Output<String>,
        /// The URL to access the ServiceBus Namespace.
        pub endpoint: pulumi_wasm_rust::Output<String>,
        /// An `identity` block as defined below.
        pub identity: pulumi_wasm_rust::Output<
            Option<super::super::types::eventhub::NamespaceIdentity>,
        >,
        /// Whether or not SAS authentication is enabled for the Service Bus namespace. Defaults to `true`.
        pub local_auth_enabled: pulumi_wasm_rust::Output<Option<bool>>,
        /// Specifies the supported Azure location where the resource exists. Changing this forces a new resource to be created.
        pub location: pulumi_wasm_rust::Output<String>,
        /// The minimum supported TLS version for this Service Bus Namespace. Valid values are: `1.0`, `1.1` and `1.2`. Defaults to `1.2`.
        pub minimum_tls_version: pulumi_wasm_rust::Output<Option<String>>,
        /// Specifies the name of the ServiceBus Namespace resource . Changing this forces a new resource to be created.
        pub name: pulumi_wasm_rust::Output<String>,
        /// An `network_rule_set` block as defined below.
        pub network_rule_set: pulumi_wasm_rust::Output<
            super::super::types::eventhub::NamespaceNetworkRuleSet,
        >,
        /// Specifies the number messaging partitions. Only valid when `sku` is `Premium` and the minimum number is `1`. Possible values include `0`, `1`, `2`, and `4`. Defaults to `0` for Standard, Basic namespace. Changing this forces a new resource to be created.
        ///
        /// > **Note:** It's not possible to change the partitioning option on any existing namespace. The number of partitions can only be set during namespace creation. Please check the doc https://learn.microsoft.com/en-us/azure/service-bus-messaging/enable-partitions-premium for more feature restrictions.
        pub premium_messaging_partitions: pulumi_wasm_rust::Output<Option<i32>>,
        /// Is public network access enabled for the Service Bus Namespace? Defaults to `true`.
        pub public_network_access_enabled: pulumi_wasm_rust::Output<Option<bool>>,
        /// The name of the resource group in which to Changing this forces a new resource to be created.
        /// create the namespace.
        pub resource_group_name: pulumi_wasm_rust::Output<String>,
        /// Defines which tier to use. Options are `Basic`, `Standard` or `Premium`. Please note that setting this field to `Premium` will force the creation of a new resource.
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
    pub fn create(name: &str, args: NamespaceArgs) -> NamespaceResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let capacity_binding = args.capacity.get_inner();
        let customer_managed_key_binding = args.customer_managed_key.get_inner();
        let identity_binding = args.identity.get_inner();
        let local_auth_enabled_binding = args.local_auth_enabled.get_inner();
        let location_binding = args.location.get_inner();
        let minimum_tls_version_binding = args.minimum_tls_version.get_inner();
        let name_binding = args.name.get_inner();
        let network_rule_set_binding = args.network_rule_set.get_inner();
        let premium_messaging_partitions_binding = args
            .premium_messaging_partitions
            .get_inner();
        let public_network_access_enabled_binding = args
            .public_network_access_enabled
            .get_inner();
        let resource_group_name_binding = args.resource_group_name.get_inner();
        let sku_binding = args.sku.get_inner();
        let tags_binding = args.tags.get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "azure:eventhub/namespace:Namespace".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "capacity".into(),
                    value: &capacity_binding,
                },
                register_interface::ObjectField {
                    name: "customerManagedKey".into(),
                    value: &customer_managed_key_binding,
                },
                register_interface::ObjectField {
                    name: "identity".into(),
                    value: &identity_binding,
                },
                register_interface::ObjectField {
                    name: "localAuthEnabled".into(),
                    value: &local_auth_enabled_binding,
                },
                register_interface::ObjectField {
                    name: "location".into(),
                    value: &location_binding,
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
                    name: "networkRuleSet".into(),
                    value: &network_rule_set_binding,
                },
                register_interface::ObjectField {
                    name: "premiumMessagingPartitions".into(),
                    value: &premium_messaging_partitions_binding,
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
                    name: "capacity".into(),
                },
                register_interface::ResultField {
                    name: "customerManagedKey".into(),
                },
                register_interface::ResultField {
                    name: "defaultPrimaryConnectionString".into(),
                },
                register_interface::ResultField {
                    name: "defaultPrimaryKey".into(),
                },
                register_interface::ResultField {
                    name: "defaultSecondaryConnectionString".into(),
                },
                register_interface::ResultField {
                    name: "defaultSecondaryKey".into(),
                },
                register_interface::ResultField {
                    name: "endpoint".into(),
                },
                register_interface::ResultField {
                    name: "identity".into(),
                },
                register_interface::ResultField {
                    name: "localAuthEnabled".into(),
                },
                register_interface::ResultField {
                    name: "location".into(),
                },
                register_interface::ResultField {
                    name: "minimumTlsVersion".into(),
                },
                register_interface::ResultField {
                    name: "name".into(),
                },
                register_interface::ResultField {
                    name: "networkRuleSet".into(),
                },
                register_interface::ResultField {
                    name: "premiumMessagingPartitions".into(),
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
        NamespaceResult {
            capacity: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("capacity").unwrap(),
            ),
            customer_managed_key: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("customerManagedKey").unwrap(),
            ),
            default_primary_connection_string: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("defaultPrimaryConnectionString").unwrap(),
            ),
            default_primary_key: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("defaultPrimaryKey").unwrap(),
            ),
            default_secondary_connection_string: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("defaultSecondaryConnectionString").unwrap(),
            ),
            default_secondary_key: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("defaultSecondaryKey").unwrap(),
            ),
            endpoint: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("endpoint").unwrap(),
            ),
            identity: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("identity").unwrap(),
            ),
            local_auth_enabled: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("localAuthEnabled").unwrap(),
            ),
            location: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("location").unwrap(),
            ),
            minimum_tls_version: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("minimumTlsVersion").unwrap(),
            ),
            name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("name").unwrap(),
            ),
            network_rule_set: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("networkRuleSet").unwrap(),
            ),
            premium_messaging_partitions: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("premiumMessagingPartitions").unwrap(),
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
