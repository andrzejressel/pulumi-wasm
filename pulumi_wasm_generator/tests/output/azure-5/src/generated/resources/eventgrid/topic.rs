/// Manages an EventGrid Topic
///
/// > **Note:** at this time EventGrid Topic's are only available in a limited number of regions.
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
///   exampleTopic:
///     type: azure:eventgrid:Topic
///     name: example
///     properties:
///       name: my-eventgrid-topic
///       location: ${example.location}
///       resourceGroupName: ${example.name}
///       tags:
///         environment: Production
/// ```
///
/// ## Import
///
/// EventGrid Topic's can be imported using the `resource id`, e.g.
///
/// ```sh
/// $ pulumi import azure:eventgrid/topic:Topic topic1 /subscriptions/00000000-0000-0000-0000-000000000000/resourceGroups/group1/providers/Microsoft.EventGrid/topics/topic1
/// ```
///
pub mod topic {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct TopicArgs {
        /// An `identity` block as defined below.
        #[builder(into, default)]
        pub identity: pulumi_wasm_rust::Output<
            Option<super::super::types::eventgrid::TopicIdentity>,
        >,
        /// One or more `inbound_ip_rule` blocks as defined below.
        #[builder(into, default)]
        pub inbound_ip_rules: pulumi_wasm_rust::Output<
            Option<Vec<super::super::types::eventgrid::TopicInboundIpRule>>,
        >,
        /// A `input_mapping_default_values` block as defined below. Changing this forces a new resource to be created.
        #[builder(into, default)]
        pub input_mapping_default_values: pulumi_wasm_rust::Output<
            Option<super::super::types::eventgrid::TopicInputMappingDefaultValues>,
        >,
        /// A `input_mapping_fields` block as defined below. Changing this forces a new resource to be created.
        #[builder(into, default)]
        pub input_mapping_fields: pulumi_wasm_rust::Output<
            Option<super::super::types::eventgrid::TopicInputMappingFields>,
        >,
        /// Specifies the schema in which incoming events will be published to this domain. Allowed values are `CloudEventSchemaV1_0`, `CustomEventSchema`, or `EventGridSchema`. Defaults to `EventGridSchema`. Changing this forces a new resource to be created.
        #[builder(into, default)]
        pub input_schema: pulumi_wasm_rust::Output<Option<String>>,
        /// Whether local authentication methods is enabled for the EventGrid Topic. Defaults to `true`.
        #[builder(into, default)]
        pub local_auth_enabled: pulumi_wasm_rust::Output<Option<bool>>,
        /// Specifies the supported Azure location where the resource exists. Changing this forces a new resource to be created.
        #[builder(into, default)]
        pub location: pulumi_wasm_rust::Output<Option<String>>,
        /// Specifies the name of the EventGrid Topic resource. Changing this forces a new resource to be created.
        #[builder(into, default)]
        pub name: pulumi_wasm_rust::Output<Option<String>>,
        /// Whether or not public network access is allowed for this server. Defaults to `true`.
        #[builder(into, default)]
        pub public_network_access_enabled: pulumi_wasm_rust::Output<Option<bool>>,
        /// The name of the resource group in which the EventGrid Topic exists. Changing this forces a new resource to be created.
        #[builder(into)]
        pub resource_group_name: pulumi_wasm_rust::Output<String>,
        /// A mapping of tags to assign to the resource.
        #[builder(into, default)]
        pub tags: pulumi_wasm_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
    }
    #[allow(dead_code)]
    pub struct TopicResult {
        /// The Endpoint associated with the EventGrid Topic.
        pub endpoint: pulumi_wasm_rust::Output<String>,
        /// An `identity` block as defined below.
        pub identity: pulumi_wasm_rust::Output<
            Option<super::super::types::eventgrid::TopicIdentity>,
        >,
        /// One or more `inbound_ip_rule` blocks as defined below.
        pub inbound_ip_rules: pulumi_wasm_rust::Output<
            Option<Vec<super::super::types::eventgrid::TopicInboundIpRule>>,
        >,
        /// A `input_mapping_default_values` block as defined below. Changing this forces a new resource to be created.
        pub input_mapping_default_values: pulumi_wasm_rust::Output<
            Option<super::super::types::eventgrid::TopicInputMappingDefaultValues>,
        >,
        /// A `input_mapping_fields` block as defined below. Changing this forces a new resource to be created.
        pub input_mapping_fields: pulumi_wasm_rust::Output<
            Option<super::super::types::eventgrid::TopicInputMappingFields>,
        >,
        /// Specifies the schema in which incoming events will be published to this domain. Allowed values are `CloudEventSchemaV1_0`, `CustomEventSchema`, or `EventGridSchema`. Defaults to `EventGridSchema`. Changing this forces a new resource to be created.
        pub input_schema: pulumi_wasm_rust::Output<Option<String>>,
        /// Whether local authentication methods is enabled for the EventGrid Topic. Defaults to `true`.
        pub local_auth_enabled: pulumi_wasm_rust::Output<Option<bool>>,
        /// Specifies the supported Azure location where the resource exists. Changing this forces a new resource to be created.
        pub location: pulumi_wasm_rust::Output<String>,
        /// Specifies the name of the EventGrid Topic resource. Changing this forces a new resource to be created.
        pub name: pulumi_wasm_rust::Output<String>,
        /// The Primary Shared Access Key associated with the EventGrid Topic.
        pub primary_access_key: pulumi_wasm_rust::Output<String>,
        /// Whether or not public network access is allowed for this server. Defaults to `true`.
        pub public_network_access_enabled: pulumi_wasm_rust::Output<Option<bool>>,
        /// The name of the resource group in which the EventGrid Topic exists. Changing this forces a new resource to be created.
        pub resource_group_name: pulumi_wasm_rust::Output<String>,
        /// The Secondary Shared Access Key associated with the EventGrid Topic.
        pub secondary_access_key: pulumi_wasm_rust::Output<String>,
        /// A mapping of tags to assign to the resource.
        pub tags: pulumi_wasm_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(name: &str, args: TopicArgs) -> TopicResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let identity_binding = args.identity.get_inner();
        let inbound_ip_rules_binding = args.inbound_ip_rules.get_inner();
        let input_mapping_default_values_binding = args
            .input_mapping_default_values
            .get_inner();
        let input_mapping_fields_binding = args.input_mapping_fields.get_inner();
        let input_schema_binding = args.input_schema.get_inner();
        let local_auth_enabled_binding = args.local_auth_enabled.get_inner();
        let location_binding = args.location.get_inner();
        let name_binding = args.name.get_inner();
        let public_network_access_enabled_binding = args
            .public_network_access_enabled
            .get_inner();
        let resource_group_name_binding = args.resource_group_name.get_inner();
        let tags_binding = args.tags.get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "azure:eventgrid/topic:Topic".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "identity".into(),
                    value: &identity_binding,
                },
                register_interface::ObjectField {
                    name: "inboundIpRules".into(),
                    value: &inbound_ip_rules_binding,
                },
                register_interface::ObjectField {
                    name: "inputMappingDefaultValues".into(),
                    value: &input_mapping_default_values_binding,
                },
                register_interface::ObjectField {
                    name: "inputMappingFields".into(),
                    value: &input_mapping_fields_binding,
                },
                register_interface::ObjectField {
                    name: "inputSchema".into(),
                    value: &input_schema_binding,
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
                    name: "name".into(),
                    value: &name_binding,
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
                    name: "tags".into(),
                    value: &tags_binding,
                },
            ]),
            results: Vec::from([
                register_interface::ResultField {
                    name: "endpoint".into(),
                },
                register_interface::ResultField {
                    name: "identity".into(),
                },
                register_interface::ResultField {
                    name: "inboundIpRules".into(),
                },
                register_interface::ResultField {
                    name: "inputMappingDefaultValues".into(),
                },
                register_interface::ResultField {
                    name: "inputMappingFields".into(),
                },
                register_interface::ResultField {
                    name: "inputSchema".into(),
                },
                register_interface::ResultField {
                    name: "localAuthEnabled".into(),
                },
                register_interface::ResultField {
                    name: "location".into(),
                },
                register_interface::ResultField {
                    name: "name".into(),
                },
                register_interface::ResultField {
                    name: "primaryAccessKey".into(),
                },
                register_interface::ResultField {
                    name: "publicNetworkAccessEnabled".into(),
                },
                register_interface::ResultField {
                    name: "resourceGroupName".into(),
                },
                register_interface::ResultField {
                    name: "secondaryAccessKey".into(),
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
        TopicResult {
            endpoint: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("endpoint").unwrap(),
            ),
            identity: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("identity").unwrap(),
            ),
            inbound_ip_rules: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("inboundIpRules").unwrap(),
            ),
            input_mapping_default_values: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("inputMappingDefaultValues").unwrap(),
            ),
            input_mapping_fields: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("inputMappingFields").unwrap(),
            ),
            input_schema: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("inputSchema").unwrap(),
            ),
            local_auth_enabled: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("localAuthEnabled").unwrap(),
            ),
            location: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("location").unwrap(),
            ),
            name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("name").unwrap(),
            ),
            primary_access_key: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("primaryAccessKey").unwrap(),
            ),
            public_network_access_enabled: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("publicNetworkAccessEnabled").unwrap(),
            ),
            resource_group_name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("resourceGroupName").unwrap(),
            ),
            secondary_access_key: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("secondaryAccessKey").unwrap(),
            ),
            tags: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("tags").unwrap(),
            ),
        }
    }
}
