/// Manages an EventGrid Domain
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
///   exampleDomain:
///     type: azure:eventgrid:Domain
///     name: example
///     properties:
///       name: my-eventgrid-domain
///       location: ${example.location}
///       resourceGroupName: ${example.name}
///       tags:
///         environment: Production
/// ```
///
/// ## Import
///
/// EventGrid Domains can be imported using the `resource id`, e.g.
///
/// ```sh
/// $ pulumi import azure:eventgrid/domain:Domain domain1 /subscriptions/00000000-0000-0000-0000-000000000000/resourceGroups/group1/providers/Microsoft.EventGrid/domains/domain1
/// ```
///
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod domain {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct DomainArgs {
        /// Whether to create the domain topic when the first event subscription at the scope of the domain topic is created. Defaults to `true`.
        #[builder(into, default)]
        pub auto_create_topic_with_first_subscription: pulumi_gestalt_rust::InputOrOutput<
            Option<bool>,
        >,
        /// Whether to delete the domain topic when the last event subscription at the scope of the domain topic is deleted. Defaults to `true`.
        #[builder(into, default)]
        pub auto_delete_topic_with_last_subscription: pulumi_gestalt_rust::InputOrOutput<
            Option<bool>,
        >,
        /// An `identity` block as defined below.
        #[builder(into, default)]
        pub identity: pulumi_gestalt_rust::InputOrOutput<
            Option<super::super::types::eventgrid::DomainIdentity>,
        >,
        /// One or more `inbound_ip_rule` blocks as defined below.
        #[builder(into, default)]
        pub inbound_ip_rules: pulumi_gestalt_rust::InputOrOutput<
            Option<Vec<super::super::types::eventgrid::DomainInboundIpRule>>,
        >,
        /// A `input_mapping_default_values` block as defined below. Changing this forces a new resource to be created.
        #[builder(into, default)]
        pub input_mapping_default_values: pulumi_gestalt_rust::InputOrOutput<
            Option<super::super::types::eventgrid::DomainInputMappingDefaultValues>,
        >,
        /// A `input_mapping_fields` block as defined below. Changing this forces a new resource to be created.
        #[builder(into, default)]
        pub input_mapping_fields: pulumi_gestalt_rust::InputOrOutput<
            Option<super::super::types::eventgrid::DomainInputMappingFields>,
        >,
        /// Specifies the schema in which incoming events will be published to this domain. Allowed values are `CloudEventSchemaV1_0`, `CustomEventSchema`, or `EventGridSchema`. Defaults to `EventGridSchema`. Changing this forces a new resource to be created.
        #[builder(into, default)]
        pub input_schema: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Whether local authentication methods is enabled for the EventGrid Domain. Defaults to `true`.
        #[builder(into, default)]
        pub local_auth_enabled: pulumi_gestalt_rust::InputOrOutput<Option<bool>>,
        /// Specifies the supported Azure location where the resource exists. Changing this forces a new resource to be created.
        #[builder(into, default)]
        pub location: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Specifies the name of the EventGrid Domain resource. Changing this forces a new resource to be created.
        #[builder(into, default)]
        pub name: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Whether or not public network access is allowed for this server. Defaults to `true`.
        #[builder(into, default)]
        pub public_network_access_enabled: pulumi_gestalt_rust::InputOrOutput<
            Option<bool>,
        >,
        /// The name of the resource group in which the EventGrid Domain exists. Changing this forces a new resource to be created.
        #[builder(into)]
        pub resource_group_name: pulumi_gestalt_rust::InputOrOutput<String>,
        /// A mapping of tags to assign to the resource.
        #[builder(into, default)]
        pub tags: pulumi_gestalt_rust::InputOrOutput<
            Option<std::collections::HashMap<String, String>>,
        >,
    }
    #[allow(dead_code)]
    pub struct DomainResult {
        /// Whether to create the domain topic when the first event subscription at the scope of the domain topic is created. Defaults to `true`.
        pub auto_create_topic_with_first_subscription: pulumi_gestalt_rust::Output<
            Option<bool>,
        >,
        /// Whether to delete the domain topic when the last event subscription at the scope of the domain topic is deleted. Defaults to `true`.
        pub auto_delete_topic_with_last_subscription: pulumi_gestalt_rust::Output<
            Option<bool>,
        >,
        /// The Endpoint associated with the EventGrid Domain.
        pub endpoint: pulumi_gestalt_rust::Output<String>,
        /// An `identity` block as defined below.
        pub identity: pulumi_gestalt_rust::Output<
            Option<super::super::types::eventgrid::DomainIdentity>,
        >,
        /// One or more `inbound_ip_rule` blocks as defined below.
        pub inbound_ip_rules: pulumi_gestalt_rust::Output<
            Option<Vec<super::super::types::eventgrid::DomainInboundIpRule>>,
        >,
        /// A `input_mapping_default_values` block as defined below. Changing this forces a new resource to be created.
        pub input_mapping_default_values: pulumi_gestalt_rust::Output<
            Option<super::super::types::eventgrid::DomainInputMappingDefaultValues>,
        >,
        /// A `input_mapping_fields` block as defined below. Changing this forces a new resource to be created.
        pub input_mapping_fields: pulumi_gestalt_rust::Output<
            Option<super::super::types::eventgrid::DomainInputMappingFields>,
        >,
        /// Specifies the schema in which incoming events will be published to this domain. Allowed values are `CloudEventSchemaV1_0`, `CustomEventSchema`, or `EventGridSchema`. Defaults to `EventGridSchema`. Changing this forces a new resource to be created.
        pub input_schema: pulumi_gestalt_rust::Output<Option<String>>,
        /// Whether local authentication methods is enabled for the EventGrid Domain. Defaults to `true`.
        pub local_auth_enabled: pulumi_gestalt_rust::Output<Option<bool>>,
        /// Specifies the supported Azure location where the resource exists. Changing this forces a new resource to be created.
        pub location: pulumi_gestalt_rust::Output<String>,
        /// Specifies the name of the EventGrid Domain resource. Changing this forces a new resource to be created.
        pub name: pulumi_gestalt_rust::Output<String>,
        /// The Primary Shared Access Key associated with the EventGrid Domain.
        pub primary_access_key: pulumi_gestalt_rust::Output<String>,
        /// Whether or not public network access is allowed for this server. Defaults to `true`.
        pub public_network_access_enabled: pulumi_gestalt_rust::Output<Option<bool>>,
        /// The name of the resource group in which the EventGrid Domain exists. Changing this forces a new resource to be created.
        pub resource_group_name: pulumi_gestalt_rust::Output<String>,
        /// The Secondary Shared Access Key associated with the EventGrid Domain.
        pub secondary_access_key: pulumi_gestalt_rust::Output<String>,
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
        args: DomainArgs,
    ) -> DomainResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let auto_create_topic_with_first_subscription_binding = args
            .auto_create_topic_with_first_subscription
            .get_output(context);
        let auto_delete_topic_with_last_subscription_binding = args
            .auto_delete_topic_with_last_subscription
            .get_output(context);
        let identity_binding = args.identity.get_output(context);
        let inbound_ip_rules_binding = args.inbound_ip_rules.get_output(context);
        let input_mapping_default_values_binding = args
            .input_mapping_default_values
            .get_output(context);
        let input_mapping_fields_binding = args.input_mapping_fields.get_output(context);
        let input_schema_binding = args.input_schema.get_output(context);
        let local_auth_enabled_binding = args.local_auth_enabled.get_output(context);
        let location_binding = args.location.get_output(context);
        let name_binding = args.name.get_output(context);
        let public_network_access_enabled_binding = args
            .public_network_access_enabled
            .get_output(context);
        let resource_group_name_binding = args.resource_group_name.get_output(context);
        let tags_binding = args.tags.get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "azure:eventgrid/domain:Domain".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "autoCreateTopicWithFirstSubscription".into(),
                    value: &auto_create_topic_with_first_subscription_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "autoDeleteTopicWithLastSubscription".into(),
                    value: &auto_delete_topic_with_last_subscription_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "identity".into(),
                    value: &identity_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "inboundIpRules".into(),
                    value: &inbound_ip_rules_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "inputMappingDefaultValues".into(),
                    value: &input_mapping_default_values_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "inputMappingFields".into(),
                    value: &input_mapping_fields_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "inputSchema".into(),
                    value: &input_schema_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "localAuthEnabled".into(),
                    value: &local_auth_enabled_binding.drop_type(),
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
                    name: "publicNetworkAccessEnabled".into(),
                    value: &public_network_access_enabled_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "resourceGroupName".into(),
                    value: &resource_group_name_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "tags".into(),
                    value: &tags_binding.drop_type(),
                },
            ],
        };
        let o = context.register_resource(request);
        DomainResult {
            auto_create_topic_with_first_subscription: o
                .get_field("autoCreateTopicWithFirstSubscription"),
            auto_delete_topic_with_last_subscription: o
                .get_field("autoDeleteTopicWithLastSubscription"),
            endpoint: o.get_field("endpoint"),
            identity: o.get_field("identity"),
            inbound_ip_rules: o.get_field("inboundIpRules"),
            input_mapping_default_values: o.get_field("inputMappingDefaultValues"),
            input_mapping_fields: o.get_field("inputMappingFields"),
            input_schema: o.get_field("inputSchema"),
            local_auth_enabled: o.get_field("localAuthEnabled"),
            location: o.get_field("location"),
            name: o.get_field("name"),
            primary_access_key: o.get_field("primaryAccessKey"),
            public_network_access_enabled: o.get_field("publicNetworkAccessEnabled"),
            resource_group_name: o.get_field("resourceGroupName"),
            secondary_access_key: o.get_field("secondaryAccessKey"),
            tags: o.get_field("tags"),
        }
    }
}
