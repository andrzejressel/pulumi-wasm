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
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod event_hub_namespace {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct EventHubNamespaceArgs {
        /// Is Auto Inflate enabled for the EventHub Namespace?
        #[builder(into, default)]
        pub auto_inflate_enabled: pulumi_gestalt_rust::InputOrOutput<Option<bool>>,
        /// Specifies the Capacity / Throughput Units for a `Standard` SKU namespace. Default capacity has a maximum of `2`, but can be increased in blocks of 2 on a committed purchase basis. Defaults to `1`.
        #[builder(into, default)]
        pub capacity: pulumi_gestalt_rust::InputOrOutput<Option<i32>>,
        /// Specifies the ID of the EventHub Dedicated Cluster where this Namespace should created. Changing this forces a new resource to be created.
        #[builder(into, default)]
        pub dedicated_cluster_id: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// An `identity` block as defined below.
        #[builder(into, default)]
        pub identity: pulumi_gestalt_rust::InputOrOutput<
            Option<super::super::types::eventhub::EventHubNamespaceIdentity>,
        >,
        /// Is SAS authentication enabled for the EventHub Namespace? Defaults to `true`.
        #[builder(into, default)]
        pub local_authentication_enabled: pulumi_gestalt_rust::InputOrOutput<
            Option<bool>,
        >,
        /// Specifies the supported Azure location where the resource exists. Changing this forces a new resource to be created.
        #[builder(into, default)]
        pub location: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Specifies the maximum number of throughput units when Auto Inflate is Enabled. Valid values range from `1` - `20`.
        #[builder(into, default)]
        pub maximum_throughput_units: pulumi_gestalt_rust::InputOrOutput<Option<i32>>,
        /// The minimum supported TLS version for this EventHub Namespace. Valid values are: `1.0`, `1.1` and `1.2`. Defaults to `1.2`.
        ///
        /// > **Note** Azure Services will require TLS 1.2+ by August 2025, please see this [announcement](https://azure.microsoft.com/en-us/updates/v2/update-retirement-tls1-0-tls1-1-versions-azure-services/) for more.
        #[builder(into, default)]
        pub minimum_tls_version: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Specifies the name of the EventHub Namespace resource. Changing this forces a new resource to be created.
        #[builder(into, default)]
        pub name: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// A `network_rulesets` block as defined below.
        #[builder(into, default)]
        pub network_rulesets: pulumi_gestalt_rust::InputOrOutput<
            Option<super::super::types::eventhub::EventHubNamespaceNetworkRulesets>,
        >,
        /// Is public network access enabled for the EventHub Namespace? Defaults to `true`.
        #[builder(into, default)]
        pub public_network_access_enabled: pulumi_gestalt_rust::InputOrOutput<
            Option<bool>,
        >,
        /// The name of the resource group in which to create the namespace. Changing this forces a new resource to be created.
        #[builder(into)]
        pub resource_group_name: pulumi_gestalt_rust::InputOrOutput<String>,
        /// Defines which tier to use. Valid options are `Basic`, `Standard`, and `Premium`. Please note that setting this field to `Premium` will force the creation of a new resource.
        #[builder(into)]
        pub sku: pulumi_gestalt_rust::InputOrOutput<String>,
        /// A mapping of tags to assign to the resource.
        #[builder(into, default)]
        pub tags: pulumi_gestalt_rust::InputOrOutput<
            Option<std::collections::HashMap<String, String>>,
        >,
    }
    #[allow(dead_code)]
    pub struct EventHubNamespaceResult {
        /// Is Auto Inflate enabled for the EventHub Namespace?
        pub auto_inflate_enabled: pulumi_gestalt_rust::Output<Option<bool>>,
        /// Specifies the Capacity / Throughput Units for a `Standard` SKU namespace. Default capacity has a maximum of `2`, but can be increased in blocks of 2 on a committed purchase basis. Defaults to `1`.
        pub capacity: pulumi_gestalt_rust::Output<Option<i32>>,
        /// Specifies the ID of the EventHub Dedicated Cluster where this Namespace should created. Changing this forces a new resource to be created.
        pub dedicated_cluster_id: pulumi_gestalt_rust::Output<Option<String>>,
        /// The primary connection string for the authorization rule `RootManageSharedAccessKey`.
        pub default_primary_connection_string: pulumi_gestalt_rust::Output<String>,
        /// The alias of the primary connection string for the authorization rule `RootManageSharedAccessKey`, which is generated when disaster recovery is enabled.
        pub default_primary_connection_string_alias: pulumi_gestalt_rust::Output<String>,
        /// The primary access key for the authorization rule `RootManageSharedAccessKey`.
        pub default_primary_key: pulumi_gestalt_rust::Output<String>,
        /// The secondary connection string for the authorization rule `RootManageSharedAccessKey`.
        pub default_secondary_connection_string: pulumi_gestalt_rust::Output<String>,
        /// The alias of the secondary connection string for the authorization rule `RootManageSharedAccessKey`, which is generated when disaster recovery is enabled.
        pub default_secondary_connection_string_alias: pulumi_gestalt_rust::Output<
            String,
        >,
        /// The secondary access key for the authorization rule `RootManageSharedAccessKey`.
        pub default_secondary_key: pulumi_gestalt_rust::Output<String>,
        /// An `identity` block as defined below.
        pub identity: pulumi_gestalt_rust::Output<
            Option<super::super::types::eventhub::EventHubNamespaceIdentity>,
        >,
        /// Is SAS authentication enabled for the EventHub Namespace? Defaults to `true`.
        pub local_authentication_enabled: pulumi_gestalt_rust::Output<Option<bool>>,
        /// Specifies the supported Azure location where the resource exists. Changing this forces a new resource to be created.
        pub location: pulumi_gestalt_rust::Output<String>,
        /// Specifies the maximum number of throughput units when Auto Inflate is Enabled. Valid values range from `1` - `20`.
        pub maximum_throughput_units: pulumi_gestalt_rust::Output<Option<i32>>,
        /// The minimum supported TLS version for this EventHub Namespace. Valid values are: `1.0`, `1.1` and `1.2`. Defaults to `1.2`.
        ///
        /// > **Note** Azure Services will require TLS 1.2+ by August 2025, please see this [announcement](https://azure.microsoft.com/en-us/updates/v2/update-retirement-tls1-0-tls1-1-versions-azure-services/) for more.
        pub minimum_tls_version: pulumi_gestalt_rust::Output<Option<String>>,
        /// Specifies the name of the EventHub Namespace resource. Changing this forces a new resource to be created.
        pub name: pulumi_gestalt_rust::Output<String>,
        /// A `network_rulesets` block as defined below.
        pub network_rulesets: pulumi_gestalt_rust::Output<
            super::super::types::eventhub::EventHubNamespaceNetworkRulesets,
        >,
        /// Is public network access enabled for the EventHub Namespace? Defaults to `true`.
        pub public_network_access_enabled: pulumi_gestalt_rust::Output<Option<bool>>,
        /// The name of the resource group in which to create the namespace. Changing this forces a new resource to be created.
        pub resource_group_name: pulumi_gestalt_rust::Output<String>,
        /// Defines which tier to use. Valid options are `Basic`, `Standard`, and `Premium`. Please note that setting this field to `Premium` will force the creation of a new resource.
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
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: EventHubNamespaceArgs,
    ) -> EventHubNamespaceResult {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::register_interface;
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let auto_inflate_enabled_binding = args.auto_inflate_enabled.get_output(context);
        let capacity_binding = args.capacity.get_output(context);
        let dedicated_cluster_id_binding = args.dedicated_cluster_id.get_output(context);
        let identity_binding = args.identity.get_output(context);
        let local_authentication_enabled_binding = args
            .local_authentication_enabled
            .get_output(context);
        let location_binding = args.location.get_output(context);
        let maximum_throughput_units_binding = args
            .maximum_throughput_units
            .get_output(context);
        let minimum_tls_version_binding = args.minimum_tls_version.get_output(context);
        let name_binding = args.name.get_output(context);
        let network_rulesets_binding = args.network_rulesets.get_output(context);
        let public_network_access_enabled_binding = args
            .public_network_access_enabled
            .get_output(context);
        let resource_group_name_binding = args.resource_group_name.get_output(context);
        let sku_binding = args.sku.get_output(context);
        let tags_binding = args.tags.get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "azure:eventhub/eventHubNamespace:EventHubNamespace".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "autoInflateEnabled".into(),
                    value: auto_inflate_enabled_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "capacity".into(),
                    value: capacity_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "dedicatedClusterId".into(),
                    value: dedicated_cluster_id_binding.get_id(),
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
                    name: "maximumThroughputUnits".into(),
                    value: maximum_throughput_units_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "minimumTlsVersion".into(),
                    value: minimum_tls_version_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "name".into(),
                    value: name_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "networkRulesets".into(),
                    value: network_rulesets_binding.get_id(),
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
                    name: "sku".into(),
                    value: sku_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "tags".into(),
                    value: tags_binding.get_id(),
                },
            ],
        };
        let o = context.register_resource(request);
        EventHubNamespaceResult {
            auto_inflate_enabled: o.get_field("autoInflateEnabled"),
            capacity: o.get_field("capacity"),
            dedicated_cluster_id: o.get_field("dedicatedClusterId"),
            default_primary_connection_string: o
                .get_field("defaultPrimaryConnectionString"),
            default_primary_connection_string_alias: o
                .get_field("defaultPrimaryConnectionStringAlias"),
            default_primary_key: o.get_field("defaultPrimaryKey"),
            default_secondary_connection_string: o
                .get_field("defaultSecondaryConnectionString"),
            default_secondary_connection_string_alias: o
                .get_field("defaultSecondaryConnectionStringAlias"),
            default_secondary_key: o.get_field("defaultSecondaryKey"),
            identity: o.get_field("identity"),
            local_authentication_enabled: o.get_field("localAuthenticationEnabled"),
            location: o.get_field("location"),
            maximum_throughput_units: o.get_field("maximumThroughputUnits"),
            minimum_tls_version: o.get_field("minimumTlsVersion"),
            name: o.get_field("name"),
            network_rulesets: o.get_field("networkRulesets"),
            public_network_access_enabled: o.get_field("publicNetworkAccessEnabled"),
            resource_group_name: o.get_field("resourceGroupName"),
            sku: o.get_field("sku"),
            tags: o.get_field("tags"),
        }
    }
}
