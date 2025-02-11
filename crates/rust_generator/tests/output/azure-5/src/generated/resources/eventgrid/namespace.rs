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
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod namespace {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct NamespaceArgs {
        /// Specifies the Capacity / Throughput Units for an Eventgrid Namespace. Valid values can be between `1` and `40`.
        #[builder(into, default)]
        pub capacity: pulumi_gestalt_rust::InputOrOutput<Option<i32>>,
        /// An `identity` block as defined below.
        #[builder(into, default)]
        pub identity: pulumi_gestalt_rust::InputOrOutput<
            Option<super::super::types::eventgrid::NamespaceIdentity>,
        >,
        /// One or more `inbound_ip_rule` blocks as defined below.
        #[builder(into, default)]
        pub inbound_ip_rules: pulumi_gestalt_rust::InputOrOutput<
            Option<Vec<super::super::types::eventgrid::NamespaceInboundIpRule>>,
        >,
        /// Specifies the supported Azure location where the resource should exist. Changing this forces a new resource to be created.
        #[builder(into, default)]
        pub location: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Specifies the name of the Event Grid Namespace resource. Changing this forces a new resource to be created.
        #[builder(into, default)]
        pub name: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Whether or not public network access is allowed for this server. Defaults to `Enabled`.
        #[builder(into, default)]
        pub public_network_access: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The name of the resource group in which the Event Grid Namespace should exist. Changing this forces a new resource to be created.
        #[builder(into)]
        pub resource_group_name: pulumi_gestalt_rust::InputOrOutput<String>,
        /// Defines which tier to use. The only possible value is `Standard`. Defaults to `Standard`.
        #[builder(into, default)]
        pub sku: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// A mapping of tags to assign to the resource.
        #[builder(into, default)]
        pub tags: pulumi_gestalt_rust::InputOrOutput<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// A `topic_spaces_configuration` block as defined below.
        #[builder(into, default)]
        pub topic_spaces_configurations: pulumi_gestalt_rust::InputOrOutput<
            Option<
                Vec<super::super::types::eventgrid::NamespaceTopicSpacesConfiguration>,
            >,
        >,
    }
    #[allow(dead_code)]
    pub struct NamespaceResult {
        /// Specifies the Capacity / Throughput Units for an Eventgrid Namespace. Valid values can be between `1` and `40`.
        pub capacity: pulumi_gestalt_rust::Output<Option<i32>>,
        /// An `identity` block as defined below.
        pub identity: pulumi_gestalt_rust::Output<
            Option<super::super::types::eventgrid::NamespaceIdentity>,
        >,
        /// One or more `inbound_ip_rule` blocks as defined below.
        pub inbound_ip_rules: pulumi_gestalt_rust::Output<
            Option<Vec<super::super::types::eventgrid::NamespaceInboundIpRule>>,
        >,
        /// Specifies the supported Azure location where the resource should exist. Changing this forces a new resource to be created.
        pub location: pulumi_gestalt_rust::Output<String>,
        /// Specifies the name of the Event Grid Namespace resource. Changing this forces a new resource to be created.
        pub name: pulumi_gestalt_rust::Output<String>,
        /// Whether or not public network access is allowed for this server. Defaults to `Enabled`.
        pub public_network_access: pulumi_gestalt_rust::Output<Option<String>>,
        /// The name of the resource group in which the Event Grid Namespace should exist. Changing this forces a new resource to be created.
        pub resource_group_name: pulumi_gestalt_rust::Output<String>,
        /// Defines which tier to use. The only possible value is `Standard`. Defaults to `Standard`.
        pub sku: pulumi_gestalt_rust::Output<Option<String>>,
        /// A mapping of tags to assign to the resource.
        pub tags: pulumi_gestalt_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// A `topic_spaces_configuration` block as defined below.
        pub topic_spaces_configurations: pulumi_gestalt_rust::Output<
            Option<
                Vec<super::super::types::eventgrid::NamespaceTopicSpacesConfiguration>,
            >,
        >,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: NamespaceArgs,
    ) -> NamespaceResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let capacity_binding = args.capacity.get_output(context);
        let identity_binding = args.identity.get_output(context);
        let inbound_ip_rules_binding = args.inbound_ip_rules.get_output(context);
        let location_binding = args.location.get_output(context);
        let name_binding = args.name.get_output(context);
        let public_network_access_binding = args
            .public_network_access
            .get_output(context);
        let resource_group_name_binding = args.resource_group_name.get_output(context);
        let sku_binding = args.sku.get_output(context);
        let tags_binding = args.tags.get_output(context);
        let topic_spaces_configurations_binding = args
            .topic_spaces_configurations
            .get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "azure:eventgrid/namespace:Namespace".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "capacity".into(),
                    value: &capacity_binding.drop_type(),
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
                    name: "location".into(),
                    value: &location_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "name".into(),
                    value: &name_binding.drop_type(),
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
                    name: "sku".into(),
                    value: &sku_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "tags".into(),
                    value: &tags_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "topicSpacesConfigurations".into(),
                    value: &topic_spaces_configurations_binding.drop_type(),
                },
            ],
        };
        let o = context.register_resource(request);
        NamespaceResult {
            capacity: o.get_field("capacity"),
            identity: o.get_field("identity"),
            inbound_ip_rules: o.get_field("inboundIpRules"),
            location: o.get_field("location"),
            name: o.get_field("name"),
            public_network_access: o.get_field("publicNetworkAccess"),
            resource_group_name: o.get_field("resourceGroupName"),
            sku: o.get_field("sku"),
            tags: o.get_field("tags"),
            topic_spaces_configurations: o.get_field("topicSpacesConfigurations"),
        }
    }
}
