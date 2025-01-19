/// Manages an EventGrid Namespace
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
///   exampleNamespace:
///     type: azure:eventgrid:Namespace
///     name: example
///     properties:
///       name: my-eventgrid-namespace
///       location: ${example.location}
///       resourceGroupName: ${example.name}
///       tags:
///         environment: Production
/// ```
///
/// ## Import
///
/// EventGrid Namespace's can be imported using the `resource id`, e.g.
///
/// ```sh
/// $ pulumi import azure:eventgrid/namespace:Namespace namespace1 /subscriptions/00000000-0000-0000-0000-000000000000/resourceGroups/group1/providers/Microsoft.EventGrid/namespaces/namespace1
/// ```
///
pub mod namespace {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct NamespaceArgs {
        /// Specifies the Capacity / Throughput Units for an Eventgrid Namespace. Valid values can be between `1` and `40`.
        #[builder(into, default)]
        pub capacity: pulumi_wasm_rust::Output<Option<i32>>,
        /// An `identity` block as defined below.
        #[builder(into, default)]
        pub identity: pulumi_wasm_rust::Output<
            Option<super::super::types::eventgrid::NamespaceIdentity>,
        >,
        /// One or more `inbound_ip_rule` blocks as defined below.
        #[builder(into, default)]
        pub inbound_ip_rules: pulumi_wasm_rust::Output<
            Option<Vec<super::super::types::eventgrid::NamespaceInboundIpRule>>,
        >,
        /// Specifies the supported Azure location where the resource should exist. Changing this forces a new resource to be created.
        #[builder(into, default)]
        pub location: pulumi_wasm_rust::Output<Option<String>>,
        /// Specifies the name of the Event Grid Namespace resource. Changing this forces a new resource to be created.
        #[builder(into, default)]
        pub name: pulumi_wasm_rust::Output<Option<String>>,
        /// Whether or not public network access is allowed for this server. Defaults to `Enabled`.
        #[builder(into, default)]
        pub public_network_access: pulumi_wasm_rust::Output<Option<String>>,
        /// The name of the resource group in which the Event Grid Namespace should exist. Changing this forces a new resource to be created.
        #[builder(into)]
        pub resource_group_name: pulumi_wasm_rust::Output<String>,
        /// Defines which tier to use. The only possible value is `Standard`. Defaults to `Standard`.
        #[builder(into, default)]
        pub sku: pulumi_wasm_rust::Output<Option<String>>,
        /// A mapping of tags to assign to the resource.
        #[builder(into, default)]
        pub tags: pulumi_wasm_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// A `topic_spaces_configuration` block as defined below.
        #[builder(into, default)]
        pub topic_spaces_configurations: pulumi_wasm_rust::Output<
            Option<
                Vec<super::super::types::eventgrid::NamespaceTopicSpacesConfiguration>,
            >,
        >,
    }
    #[allow(dead_code)]
    pub struct NamespaceResult {
        /// Specifies the Capacity / Throughput Units for an Eventgrid Namespace. Valid values can be between `1` and `40`.
        pub capacity: pulumi_wasm_rust::Output<Option<i32>>,
        /// An `identity` block as defined below.
        pub identity: pulumi_wasm_rust::Output<
            Option<super::super::types::eventgrid::NamespaceIdentity>,
        >,
        /// One or more `inbound_ip_rule` blocks as defined below.
        pub inbound_ip_rules: pulumi_wasm_rust::Output<
            Option<Vec<super::super::types::eventgrid::NamespaceInboundIpRule>>,
        >,
        /// Specifies the supported Azure location where the resource should exist. Changing this forces a new resource to be created.
        pub location: pulumi_wasm_rust::Output<String>,
        /// Specifies the name of the Event Grid Namespace resource. Changing this forces a new resource to be created.
        pub name: pulumi_wasm_rust::Output<String>,
        /// Whether or not public network access is allowed for this server. Defaults to `Enabled`.
        pub public_network_access: pulumi_wasm_rust::Output<Option<String>>,
        /// The name of the resource group in which the Event Grid Namespace should exist. Changing this forces a new resource to be created.
        pub resource_group_name: pulumi_wasm_rust::Output<String>,
        /// Defines which tier to use. The only possible value is `Standard`. Defaults to `Standard`.
        pub sku: pulumi_wasm_rust::Output<Option<String>>,
        /// A mapping of tags to assign to the resource.
        pub tags: pulumi_wasm_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// A `topic_spaces_configuration` block as defined below.
        pub topic_spaces_configurations: pulumi_wasm_rust::Output<
            Option<
                Vec<super::super::types::eventgrid::NamespaceTopicSpacesConfiguration>,
            >,
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
        let identity_binding = args.identity.get_inner();
        let inbound_ip_rules_binding = args.inbound_ip_rules.get_inner();
        let location_binding = args.location.get_inner();
        let name_binding = args.name.get_inner();
        let public_network_access_binding = args.public_network_access.get_inner();
        let resource_group_name_binding = args.resource_group_name.get_inner();
        let sku_binding = args.sku.get_inner();
        let tags_binding = args.tags.get_inner();
        let topic_spaces_configurations_binding = args
            .topic_spaces_configurations
            .get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "azure:eventgrid/namespace:Namespace".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "capacity".into(),
                    value: &capacity_binding,
                },
                register_interface::ObjectField {
                    name: "identity".into(),
                    value: &identity_binding,
                },
                register_interface::ObjectField {
                    name: "inboundIpRules".into(),
                    value: &inbound_ip_rules_binding,
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
                    name: "publicNetworkAccess".into(),
                    value: &public_network_access_binding,
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
                register_interface::ObjectField {
                    name: "topicSpacesConfigurations".into(),
                    value: &topic_spaces_configurations_binding,
                },
            ]),
            results: Vec::from([
                register_interface::ResultField {
                    name: "capacity".into(),
                },
                register_interface::ResultField {
                    name: "identity".into(),
                },
                register_interface::ResultField {
                    name: "inboundIpRules".into(),
                },
                register_interface::ResultField {
                    name: "location".into(),
                },
                register_interface::ResultField {
                    name: "name".into(),
                },
                register_interface::ResultField {
                    name: "publicNetworkAccess".into(),
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
                register_interface::ResultField {
                    name: "topicSpacesConfigurations".into(),
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
            identity: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("identity").unwrap(),
            ),
            inbound_ip_rules: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("inboundIpRules").unwrap(),
            ),
            location: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("location").unwrap(),
            ),
            name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("name").unwrap(),
            ),
            public_network_access: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("publicNetworkAccess").unwrap(),
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
            topic_spaces_configurations: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("topicSpacesConfigurations").unwrap(),
            ),
        }
    }
}
