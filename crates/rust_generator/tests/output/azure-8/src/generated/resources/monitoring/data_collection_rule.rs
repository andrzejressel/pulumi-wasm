/// Manages a Data Collection Rule.
///
/// ## Import
///
/// Data Collection Rules can be imported using the `resource id`, e.g.
///
/// ```sh
/// $ pulumi import azure:monitoring/dataCollectionRule:DataCollectionRule example /subscriptions/12345678-1234-9876-4563-123456789012/resourceGroups/group1/providers/Microsoft.Insights/dataCollectionRules/rule1
/// ```
///
#[allow(clippy::doc_lazy_continuation)]
pub mod data_collection_rule {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct DataCollectionRuleArgs {
        /// The resource ID of the Data Collection Endpoint that this rule can be used with.
        #[builder(into, default)]
        pub data_collection_endpoint_id: pulumi_gestalt_rust::InputOrOutput<
            Option<String>,
        >,
        /// One or more `data_flow` blocks as defined below.
        #[builder(into)]
        pub data_flows: pulumi_gestalt_rust::InputOrOutput<
            Vec<super::super::types::monitoring::DataCollectionRuleDataFlow>,
        >,
        /// A `data_sources` block as defined below. This property is optional and can be omitted if the rule is meant to be used via direct calls to the provisioned endpoint.
        #[builder(into, default)]
        pub data_sources: pulumi_gestalt_rust::InputOrOutput<
            Option<super::super::types::monitoring::DataCollectionRuleDataSources>,
        >,
        /// The description of the Data Collection Rule.
        #[builder(into, default)]
        pub description: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// A `destinations` block as defined below.
        #[builder(into)]
        pub destinations: pulumi_gestalt_rust::InputOrOutput<
            super::super::types::monitoring::DataCollectionRuleDestinations,
        >,
        /// An `identity` block as defined below.
        #[builder(into, default)]
        pub identity: pulumi_gestalt_rust::InputOrOutput<
            Option<super::super::types::monitoring::DataCollectionRuleIdentity>,
        >,
        /// The kind of the Data Collection Rule. Possible values are `Linux`, `Windows`, `AgentDirectToStore` and `WorkspaceTransforms`. A rule of kind `Linux` does not allow for `windows_event_log` data sources. And a rule of kind `Windows` does not allow for `syslog` data sources. If kind is not specified, all kinds of data sources are allowed.
        ///
        /// > **NOTE** Once `kind` has been set, changing it forces a new Data Collection Rule to be created.
        #[builder(into, default)]
        pub kind: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The Azure Region where the Data Collection Rule should exist. Changing this forces a new Data Collection Rule to be created.
        #[builder(into, default)]
        pub location: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The name which should be used for this Data Collection Rule. Changing this forces a new Data Collection Rule to be created.
        #[builder(into, default)]
        pub name: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The name of the Resource Group where the Data Collection Rule should exist. Changing this forces a new Data Collection Rule to be created.
        #[builder(into)]
        pub resource_group_name: pulumi_gestalt_rust::InputOrOutput<String>,
        /// A `stream_declaration` block as defined below.
        #[builder(into, default)]
        pub stream_declarations: pulumi_gestalt_rust::InputOrOutput<
            Option<
                Vec<super::super::types::monitoring::DataCollectionRuleStreamDeclaration>,
            >,
        >,
        /// A mapping of tags which should be assigned to the Data Collection Rule.
        #[builder(into, default)]
        pub tags: pulumi_gestalt_rust::InputOrOutput<
            Option<std::collections::HashMap<String, String>>,
        >,
    }
    #[allow(dead_code)]
    pub struct DataCollectionRuleResult {
        /// The resource ID of the Data Collection Endpoint that this rule can be used with.
        pub data_collection_endpoint_id: pulumi_gestalt_rust::Output<Option<String>>,
        /// One or more `data_flow` blocks as defined below.
        pub data_flows: pulumi_gestalt_rust::Output<
            Vec<super::super::types::monitoring::DataCollectionRuleDataFlow>,
        >,
        /// A `data_sources` block as defined below. This property is optional and can be omitted if the rule is meant to be used via direct calls to the provisioned endpoint.
        pub data_sources: pulumi_gestalt_rust::Output<
            Option<super::super::types::monitoring::DataCollectionRuleDataSources>,
        >,
        /// The description of the Data Collection Rule.
        pub description: pulumi_gestalt_rust::Output<Option<String>>,
        /// A `destinations` block as defined below.
        pub destinations: pulumi_gestalt_rust::Output<
            super::super::types::monitoring::DataCollectionRuleDestinations,
        >,
        /// An `identity` block as defined below.
        pub identity: pulumi_gestalt_rust::Output<
            Option<super::super::types::monitoring::DataCollectionRuleIdentity>,
        >,
        /// The immutable ID of the Data Collection Rule.
        pub immutable_id: pulumi_gestalt_rust::Output<String>,
        /// The kind of the Data Collection Rule. Possible values are `Linux`, `Windows`, `AgentDirectToStore` and `WorkspaceTransforms`. A rule of kind `Linux` does not allow for `windows_event_log` data sources. And a rule of kind `Windows` does not allow for `syslog` data sources. If kind is not specified, all kinds of data sources are allowed.
        ///
        /// > **NOTE** Once `kind` has been set, changing it forces a new Data Collection Rule to be created.
        pub kind: pulumi_gestalt_rust::Output<Option<String>>,
        /// The Azure Region where the Data Collection Rule should exist. Changing this forces a new Data Collection Rule to be created.
        pub location: pulumi_gestalt_rust::Output<String>,
        /// The name which should be used for this Data Collection Rule. Changing this forces a new Data Collection Rule to be created.
        pub name: pulumi_gestalt_rust::Output<String>,
        /// The name of the Resource Group where the Data Collection Rule should exist. Changing this forces a new Data Collection Rule to be created.
        pub resource_group_name: pulumi_gestalt_rust::Output<String>,
        /// A `stream_declaration` block as defined below.
        pub stream_declarations: pulumi_gestalt_rust::Output<
            Option<
                Vec<super::super::types::monitoring::DataCollectionRuleStreamDeclaration>,
            >,
        >,
        /// A mapping of tags which should be assigned to the Data Collection Rule.
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
        args: DataCollectionRuleArgs,
    ) -> DataCollectionRuleResult {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::register_interface;
        use std::collections::HashMap;
        let data_collection_endpoint_id_binding = args
            .data_collection_endpoint_id
            .get_output(context)
            .get_inner();
        let data_flows_binding = args.data_flows.get_output(context).get_inner();
        let data_sources_binding = args.data_sources.get_output(context).get_inner();
        let description_binding = args.description.get_output(context).get_inner();
        let destinations_binding = args.destinations.get_output(context).get_inner();
        let identity_binding = args.identity.get_output(context).get_inner();
        let kind_binding = args.kind.get_output(context).get_inner();
        let location_binding = args.location.get_output(context).get_inner();
        let name_binding = args.name.get_output(context).get_inner();
        let resource_group_name_binding = args
            .resource_group_name
            .get_output(context)
            .get_inner();
        let stream_declarations_binding = args
            .stream_declarations
            .get_output(context)
            .get_inner();
        let tags_binding = args.tags.get_output(context).get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "azure:monitoring/dataCollectionRule:DataCollectionRule".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "dataCollectionEndpointId".into(),
                    value: &data_collection_endpoint_id_binding,
                },
                register_interface::ObjectField {
                    name: "dataFlows".into(),
                    value: &data_flows_binding,
                },
                register_interface::ObjectField {
                    name: "dataSources".into(),
                    value: &data_sources_binding,
                },
                register_interface::ObjectField {
                    name: "description".into(),
                    value: &description_binding,
                },
                register_interface::ObjectField {
                    name: "destinations".into(),
                    value: &destinations_binding,
                },
                register_interface::ObjectField {
                    name: "identity".into(),
                    value: &identity_binding,
                },
                register_interface::ObjectField {
                    name: "kind".into(),
                    value: &kind_binding,
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
                    name: "resourceGroupName".into(),
                    value: &resource_group_name_binding,
                },
                register_interface::ObjectField {
                    name: "streamDeclarations".into(),
                    value: &stream_declarations_binding,
                },
                register_interface::ObjectField {
                    name: "tags".into(),
                    value: &tags_binding,
                },
            ]),
        };
        let o = register_interface::register(context.get_inner(), &request);
        DataCollectionRuleResult {
            data_collection_endpoint_id: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("dataCollectionEndpointId"),
            ),
            data_flows: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("dataFlows"),
            ),
            data_sources: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("dataSources"),
            ),
            description: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("description"),
            ),
            destinations: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("destinations"),
            ),
            identity: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("identity"),
            ),
            immutable_id: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("immutableId"),
            ),
            kind: pulumi_gestalt_rust::__private::into_domain(o.extract_field("kind")),
            location: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("location"),
            ),
            name: pulumi_gestalt_rust::__private::into_domain(o.extract_field("name")),
            resource_group_name: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("resourceGroupName"),
            ),
            stream_declarations: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("streamDeclarations"),
            ),
            tags: pulumi_gestalt_rust::__private::into_domain(o.extract_field("tags")),
        }
    }
}
