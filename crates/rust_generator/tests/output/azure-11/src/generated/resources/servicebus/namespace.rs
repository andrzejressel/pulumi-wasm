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
/// $ pulumi import azure:servicebus/namespace:Namespace example /subscriptions/00000000-0000-0000-0000-000000000000/resourceGroups/mygroup1/providers/Microsoft.ServiceBus/namespaces/sbns1
/// ```
///
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod namespace {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct NamespaceArgs {
        /// Specifies the capacity. When `sku` is `Premium`, capacity can be `1`, `2`, `4`, `8` or `16`. When `sku` is `Basic` or `Standard`, capacity can be `0` only.
        #[builder(into, default)]
        pub capacity: pulumi_gestalt_rust::InputOrOutput<Option<i32>>,
        /// An `customer_managed_key` block as defined below.
        #[builder(into, default)]
        pub customer_managed_key: pulumi_gestalt_rust::InputOrOutput<
            Option<super::super::types::servicebus::NamespaceCustomerManagedKey>,
        >,
        /// An `identity` block as defined below.
        #[builder(into, default)]
        pub identity: pulumi_gestalt_rust::InputOrOutput<
            Option<super::super::types::servicebus::NamespaceIdentity>,
        >,
        /// Whether or not SAS authentication is enabled for the Service Bus namespace. Defaults to `true`.
        #[builder(into, default)]
        pub local_auth_enabled: pulumi_gestalt_rust::InputOrOutput<Option<bool>>,
        /// Specifies the supported Azure location where the resource exists. Changing this forces a new resource to be created.
        #[builder(into, default)]
        pub location: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The minimum supported TLS version for this Service Bus Namespace. Valid values are: `1.0`, `1.1` and `1.2`. Defaults to `1.2`.
        #[builder(into, default)]
        pub minimum_tls_version: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Specifies the name of the ServiceBus Namespace resource . Changing this forces a new resource to be created.
        #[builder(into, default)]
        pub name: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// An `network_rule_set` block as defined below.
        #[builder(into, default)]
        pub network_rule_set: pulumi_gestalt_rust::InputOrOutput<
            Option<super::super::types::servicebus::NamespaceNetworkRuleSet>,
        >,
        /// Specifies the number messaging partitions. Only valid when `sku` is `Premium` and the minimum number is `1`. Possible values include `0`, `1`, `2`, and `4`. Defaults to `0` for Standard, Basic namespace. Changing this forces a new resource to be created.
        ///
        /// > **Note:** It's not possible to change the partitioning option on any existing namespace. The number of partitions can only be set during namespace creation. Please check the doc https://learn.microsoft.com/en-us/azure/service-bus-messaging/enable-partitions-premium for more feature restrictions.
        #[builder(into, default)]
        pub premium_messaging_partitions: pulumi_gestalt_rust::InputOrOutput<
            Option<i32>,
        >,
        /// Is public network access enabled for the Service Bus Namespace? Defaults to `true`.
        #[builder(into, default)]
        pub public_network_access_enabled: pulumi_gestalt_rust::InputOrOutput<
            Option<bool>,
        >,
        /// The name of the resource group in which to Changing this forces a new resource to be created.
        /// create the namespace.
        #[builder(into)]
        pub resource_group_name: pulumi_gestalt_rust::InputOrOutput<String>,
        /// Defines which tier to use. Options are `Basic`, `Standard` or `Premium`. Please note that setting this field to `Premium` will force the creation of a new resource.
        #[builder(into)]
        pub sku: pulumi_gestalt_rust::InputOrOutput<String>,
        /// A mapping of tags to assign to the resource.
        #[builder(into, default)]
        pub tags: pulumi_gestalt_rust::InputOrOutput<
            Option<std::collections::HashMap<String, String>>,
        >,
    }
    #[allow(dead_code)]
    pub struct NamespaceResult {
        /// Specifies the capacity. When `sku` is `Premium`, capacity can be `1`, `2`, `4`, `8` or `16`. When `sku` is `Basic` or `Standard`, capacity can be `0` only.
        pub capacity: pulumi_gestalt_rust::Output<Option<i32>>,
        /// An `customer_managed_key` block as defined below.
        pub customer_managed_key: pulumi_gestalt_rust::Output<
            Option<super::super::types::servicebus::NamespaceCustomerManagedKey>,
        >,
        /// The primary connection string for the authorization rule `RootManageSharedAccessKey`.
        pub default_primary_connection_string: pulumi_gestalt_rust::Output<String>,
        /// The primary access key for the authorization rule `RootManageSharedAccessKey`.
        pub default_primary_key: pulumi_gestalt_rust::Output<String>,
        /// The secondary connection string for the authorization rule `RootManageSharedAccessKey`.
        pub default_secondary_connection_string: pulumi_gestalt_rust::Output<String>,
        /// The secondary access key for the authorization rule `RootManageSharedAccessKey`.
        pub default_secondary_key: pulumi_gestalt_rust::Output<String>,
        /// The URL to access the ServiceBus Namespace.
        pub endpoint: pulumi_gestalt_rust::Output<String>,
        /// An `identity` block as defined below.
        pub identity: pulumi_gestalt_rust::Output<
            Option<super::super::types::servicebus::NamespaceIdentity>,
        >,
        /// Whether or not SAS authentication is enabled for the Service Bus namespace. Defaults to `true`.
        pub local_auth_enabled: pulumi_gestalt_rust::Output<Option<bool>>,
        /// Specifies the supported Azure location where the resource exists. Changing this forces a new resource to be created.
        pub location: pulumi_gestalt_rust::Output<String>,
        /// The minimum supported TLS version for this Service Bus Namespace. Valid values are: `1.0`, `1.1` and `1.2`. Defaults to `1.2`.
        pub minimum_tls_version: pulumi_gestalt_rust::Output<Option<String>>,
        /// Specifies the name of the ServiceBus Namespace resource . Changing this forces a new resource to be created.
        pub name: pulumi_gestalt_rust::Output<String>,
        /// An `network_rule_set` block as defined below.
        pub network_rule_set: pulumi_gestalt_rust::Output<
            super::super::types::servicebus::NamespaceNetworkRuleSet,
        >,
        /// Specifies the number messaging partitions. Only valid when `sku` is `Premium` and the minimum number is `1`. Possible values include `0`, `1`, `2`, and `4`. Defaults to `0` for Standard, Basic namespace. Changing this forces a new resource to be created.
        ///
        /// > **Note:** It's not possible to change the partitioning option on any existing namespace. The number of partitions can only be set during namespace creation. Please check the doc https://learn.microsoft.com/en-us/azure/service-bus-messaging/enable-partitions-premium for more feature restrictions.
        pub premium_messaging_partitions: pulumi_gestalt_rust::Output<Option<i32>>,
        /// Is public network access enabled for the Service Bus Namespace? Defaults to `true`.
        pub public_network_access_enabled: pulumi_gestalt_rust::Output<Option<bool>>,
        /// The name of the resource group in which to Changing this forces a new resource to be created.
        /// create the namespace.
        pub resource_group_name: pulumi_gestalt_rust::Output<String>,
        /// Defines which tier to use. Options are `Basic`, `Standard` or `Premium`. Please note that setting this field to `Premium` will force the creation of a new resource.
        pub sku: pulumi_gestalt_rust::Output<String>,
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
        context: &pulumi_gestalt_rust::PulumiContext,
        name: &str,
        args: NamespaceArgs,
    ) -> NamespaceResult {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::register_interface;
        use std::collections::HashMap;
        let capacity_binding_1 = args.capacity.get_output(context);
        let capacity_binding = capacity_binding_1.get_inner();
        let customer_managed_key_binding_1 = args
            .customer_managed_key
            .get_output(context);
        let customer_managed_key_binding = customer_managed_key_binding_1.get_inner();
        let identity_binding_1 = args.identity.get_output(context);
        let identity_binding = identity_binding_1.get_inner();
        let local_auth_enabled_binding_1 = args.local_auth_enabled.get_output(context);
        let local_auth_enabled_binding = local_auth_enabled_binding_1.get_inner();
        let location_binding_1 = args.location.get_output(context);
        let location_binding = location_binding_1.get_inner();
        let minimum_tls_version_binding_1 = args.minimum_tls_version.get_output(context);
        let minimum_tls_version_binding = minimum_tls_version_binding_1.get_inner();
        let name_binding_1 = args.name.get_output(context);
        let name_binding = name_binding_1.get_inner();
        let network_rule_set_binding_1 = args.network_rule_set.get_output(context);
        let network_rule_set_binding = network_rule_set_binding_1.get_inner();
        let premium_messaging_partitions_binding_1 = args
            .premium_messaging_partitions
            .get_output(context);
        let premium_messaging_partitions_binding = premium_messaging_partitions_binding_1
            .get_inner();
        let public_network_access_enabled_binding_1 = args
            .public_network_access_enabled
            .get_output(context);
        let public_network_access_enabled_binding = public_network_access_enabled_binding_1
            .get_inner();
        let resource_group_name_binding_1 = args.resource_group_name.get_output(context);
        let resource_group_name_binding = resource_group_name_binding_1.get_inner();
        let sku_binding_1 = args.sku.get_output(context);
        let sku_binding = sku_binding_1.get_inner();
        let tags_binding_1 = args.tags.get_output(context);
        let tags_binding = tags_binding_1.get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "azure:servicebus/namespace:Namespace".into(),
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
        };
        let o = register_interface::register(context.get_inner(), &request);
        NamespaceResult {
            capacity: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("capacity"),
            ),
            customer_managed_key: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("customerManagedKey"),
            ),
            default_primary_connection_string: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("defaultPrimaryConnectionString"),
            ),
            default_primary_key: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("defaultPrimaryKey"),
            ),
            default_secondary_connection_string: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("defaultSecondaryConnectionString"),
            ),
            default_secondary_key: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("defaultSecondaryKey"),
            ),
            endpoint: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("endpoint"),
            ),
            identity: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("identity"),
            ),
            local_auth_enabled: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("localAuthEnabled"),
            ),
            location: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("location"),
            ),
            minimum_tls_version: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("minimumTlsVersion"),
            ),
            name: pulumi_gestalt_rust::__private::into_domain(o.extract_field("name")),
            network_rule_set: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("networkRuleSet"),
            ),
            premium_messaging_partitions: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("premiumMessagingPartitions"),
            ),
            public_network_access_enabled: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("publicNetworkAccessEnabled"),
            ),
            resource_group_name: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("resourceGroupName"),
            ),
            sku: pulumi_gestalt_rust::__private::into_domain(o.extract_field("sku")),
            tags: pulumi_gestalt_rust::__private::into_domain(o.extract_field("tags")),
        }
    }
}
