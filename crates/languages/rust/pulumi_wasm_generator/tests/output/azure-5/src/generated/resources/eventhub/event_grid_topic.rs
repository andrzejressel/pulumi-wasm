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
/// $ pulumi import azure:eventhub/eventGridTopic:EventGridTopic topic1 /subscriptions/00000000-0000-0000-0000-000000000000/resourceGroups/group1/providers/Microsoft.EventGrid/topics/topic1
/// ```
///
pub mod event_grid_topic {
    #[derive(pulumi_wasm_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct EventGridTopicArgs {
        /// An `identity` block as defined below.
        #[builder(into, default)]
        pub identity: pulumi_wasm_rust::InputOrOutput<
            Option<super::super::types::eventhub::EventGridTopicIdentity>,
        >,
        /// One or more `inbound_ip_rule` blocks as defined below.
        #[builder(into, default)]
        pub inbound_ip_rules: pulumi_wasm_rust::InputOrOutput<
            Option<Vec<super::super::types::eventhub::EventGridTopicInboundIpRule>>,
        >,
        /// A `input_mapping_default_values` block as defined below. Changing this forces a new resource to be created.
        #[builder(into, default)]
        pub input_mapping_default_values: pulumi_wasm_rust::InputOrOutput<
            Option<
                super::super::types::eventhub::EventGridTopicInputMappingDefaultValues,
            >,
        >,
        /// A `input_mapping_fields` block as defined below. Changing this forces a new resource to be created.
        #[builder(into, default)]
        pub input_mapping_fields: pulumi_wasm_rust::InputOrOutput<
            Option<super::super::types::eventhub::EventGridTopicInputMappingFields>,
        >,
        /// Specifies the schema in which incoming events will be published to this domain. Allowed values are `CloudEventSchemaV1_0`, `CustomEventSchema`, or `EventGridSchema`. Defaults to `EventGridSchema`. Changing this forces a new resource to be created.
        #[builder(into, default)]
        pub input_schema: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        /// Whether local authentication methods is enabled for the EventGrid Topic. Defaults to `true`.
        #[builder(into, default)]
        pub local_auth_enabled: pulumi_wasm_rust::InputOrOutput<Option<bool>>,
        /// Specifies the supported Azure location where the resource exists. Changing this forces a new resource to be created.
        #[builder(into, default)]
        pub location: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        /// Specifies the name of the EventGrid Topic resource. Changing this forces a new resource to be created.
        #[builder(into, default)]
        pub name: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        /// Whether or not public network access is allowed for this server. Defaults to `true`.
        #[builder(into, default)]
        pub public_network_access_enabled: pulumi_wasm_rust::InputOrOutput<Option<bool>>,
        /// The name of the resource group in which the EventGrid Topic exists. Changing this forces a new resource to be created.
        #[builder(into)]
        pub resource_group_name: pulumi_wasm_rust::InputOrOutput<String>,
        /// A mapping of tags to assign to the resource.
        #[builder(into, default)]
        pub tags: pulumi_wasm_rust::InputOrOutput<
            Option<std::collections::HashMap<String, String>>,
        >,
    }
    #[allow(dead_code)]
    pub struct EventGridTopicResult {
        /// The Endpoint associated with the EventGrid Topic.
        pub endpoint: pulumi_wasm_rust::Output<String>,
        /// An `identity` block as defined below.
        pub identity: pulumi_wasm_rust::Output<
            Option<super::super::types::eventhub::EventGridTopicIdentity>,
        >,
        /// One or more `inbound_ip_rule` blocks as defined below.
        pub inbound_ip_rules: pulumi_wasm_rust::Output<
            Option<Vec<super::super::types::eventhub::EventGridTopicInboundIpRule>>,
        >,
        /// A `input_mapping_default_values` block as defined below. Changing this forces a new resource to be created.
        pub input_mapping_default_values: pulumi_wasm_rust::Output<
            Option<
                super::super::types::eventhub::EventGridTopicInputMappingDefaultValues,
            >,
        >,
        /// A `input_mapping_fields` block as defined below. Changing this forces a new resource to be created.
        pub input_mapping_fields: pulumi_wasm_rust::Output<
            Option<super::super::types::eventhub::EventGridTopicInputMappingFields>,
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
    pub fn create(
        context: &pulumi_wasm_rust::PulumiContext,
        name: &str,
        args: EventGridTopicArgs,
    ) -> EventGridTopicResult {
        use pulumi_wasm_rust::__private::pulumi_gestalt_adapter_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let identity_binding = args.identity.get_output(context).get_inner();
        let inbound_ip_rules_binding = args
            .inbound_ip_rules
            .get_output(context)
            .get_inner();
        let input_mapping_default_values_binding = args
            .input_mapping_default_values
            .get_output(context)
            .get_inner();
        let input_mapping_fields_binding = args
            .input_mapping_fields
            .get_output(context)
            .get_inner();
        let input_schema_binding = args.input_schema.get_output(context).get_inner();
        let local_auth_enabled_binding = args
            .local_auth_enabled
            .get_output(context)
            .get_inner();
        let location_binding = args.location.get_output(context).get_inner();
        let name_binding = args.name.get_output(context).get_inner();
        let public_network_access_enabled_binding = args
            .public_network_access_enabled
            .get_output(context)
            .get_inner();
        let resource_group_name_binding = args
            .resource_group_name
            .get_output(context)
            .get_inner();
        let tags_binding = args.tags.get_output(context).get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "azure:eventhub/eventGridTopic:EventGridTopic".into(),
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
        };
        let o = register_interface::register(context.get_inner(), &request);
        EventGridTopicResult {
            endpoint: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("endpoint"),
            ),
            identity: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("identity"),
            ),
            inbound_ip_rules: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("inboundIpRules"),
            ),
            input_mapping_default_values: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("inputMappingDefaultValues"),
            ),
            input_mapping_fields: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("inputMappingFields"),
            ),
            input_schema: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("inputSchema"),
            ),
            local_auth_enabled: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("localAuthEnabled"),
            ),
            location: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("location"),
            ),
            name: pulumi_wasm_rust::__private::into_domain(o.extract_field("name")),
            primary_access_key: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("primaryAccessKey"),
            ),
            public_network_access_enabled: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("publicNetworkAccessEnabled"),
            ),
            resource_group_name: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("resourceGroupName"),
            ),
            secondary_access_key: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("secondaryAccessKey"),
            ),
            tags: pulumi_wasm_rust::__private::into_domain(o.extract_field("tags")),
        }
    }
}
