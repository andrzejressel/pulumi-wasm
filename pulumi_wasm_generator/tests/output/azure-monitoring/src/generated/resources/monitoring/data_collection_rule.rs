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
pub mod data_collection_rule {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct DataCollectionRuleArgs {
        /// The resource ID of the Data Collection Endpoint that this rule can be used with.
        #[builder(into, default)]
        pub data_collection_endpoint_id: pulumi_wasm_rust::Output<Option<String>>,
        /// One or more `data_flow` blocks as defined below.
        #[builder(into)]
        pub data_flows: pulumi_wasm_rust::Output<
            Vec<super::super::types::monitoring::DataCollectionRuleDataFlow>,
        >,
        /// A `data_sources` block as defined below. This property is optional and can be omitted if the rule is meant to be used via direct calls to the provisioned endpoint.
        #[builder(into, default)]
        pub data_sources: pulumi_wasm_rust::Output<
            Option<super::super::types::monitoring::DataCollectionRuleDataSources>,
        >,
        /// The description of the Data Collection Rule.
        #[builder(into, default)]
        pub description: pulumi_wasm_rust::Output<Option<String>>,
        /// A `destinations` block as defined below.
        #[builder(into)]
        pub destinations: pulumi_wasm_rust::Output<
            super::super::types::monitoring::DataCollectionRuleDestinations,
        >,
        /// An `identity` block as defined below.
        #[builder(into, default)]
        pub identity: pulumi_wasm_rust::Output<
            Option<super::super::types::monitoring::DataCollectionRuleIdentity>,
        >,
        /// The kind of the Data Collection Rule. Possible values are `Linux`, `Windows`, `AgentDirectToStore` and `WorkspaceTransforms`. A rule of kind `Linux` does not allow for `windows_event_log` data sources. And a rule of kind `Windows` does not allow for `syslog` data sources. If kind is not specified, all kinds of data sources are allowed.
        ///
        /// > **NOTE** Once `kind` has been set, changing it forces a new Data Collection Rule to be created.
        #[builder(into, default)]
        pub kind: pulumi_wasm_rust::Output<Option<String>>,
        /// The Azure Region where the Data Collection Rule should exist. Changing this forces a new Data Collection Rule to be created.
        #[builder(into, default)]
        pub location: pulumi_wasm_rust::Output<Option<String>>,
        /// The name which should be used for this Data Collection Rule. Changing this forces a new Data Collection Rule to be created.
        #[builder(into, default)]
        pub name: pulumi_wasm_rust::Output<Option<String>>,
        /// The name of the Resource Group where the Data Collection Rule should exist. Changing this forces a new Data Collection Rule to be created.
        #[builder(into)]
        pub resource_group_name: pulumi_wasm_rust::Output<String>,
        /// A `stream_declaration` block as defined below.
        #[builder(into, default)]
        pub stream_declarations: pulumi_wasm_rust::Output<
            Option<
                Vec<super::super::types::monitoring::DataCollectionRuleStreamDeclaration>,
            >,
        >,
        /// A mapping of tags which should be assigned to the Data Collection Rule.
        #[builder(into, default)]
        pub tags: pulumi_wasm_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
    }
    #[allow(dead_code)]
    pub struct DataCollectionRuleResult {
        /// The resource ID of the Data Collection Endpoint that this rule can be used with.
        pub data_collection_endpoint_id: pulumi_wasm_rust::Output<Option<String>>,
        /// One or more `data_flow` blocks as defined below.
        pub data_flows: pulumi_wasm_rust::Output<
            Vec<super::super::types::monitoring::DataCollectionRuleDataFlow>,
        >,
        /// A `data_sources` block as defined below. This property is optional and can be omitted if the rule is meant to be used via direct calls to the provisioned endpoint.
        pub data_sources: pulumi_wasm_rust::Output<
            Option<super::super::types::monitoring::DataCollectionRuleDataSources>,
        >,
        /// The description of the Data Collection Rule.
        pub description: pulumi_wasm_rust::Output<Option<String>>,
        /// A `destinations` block as defined below.
        pub destinations: pulumi_wasm_rust::Output<
            super::super::types::monitoring::DataCollectionRuleDestinations,
        >,
        /// An `identity` block as defined below.
        pub identity: pulumi_wasm_rust::Output<
            Option<super::super::types::monitoring::DataCollectionRuleIdentity>,
        >,
        /// The immutable ID of the Data Collection Rule.
        pub immutable_id: pulumi_wasm_rust::Output<String>,
        /// The kind of the Data Collection Rule. Possible values are `Linux`, `Windows`, `AgentDirectToStore` and `WorkspaceTransforms`. A rule of kind `Linux` does not allow for `windows_event_log` data sources. And a rule of kind `Windows` does not allow for `syslog` data sources. If kind is not specified, all kinds of data sources are allowed.
        ///
        /// > **NOTE** Once `kind` has been set, changing it forces a new Data Collection Rule to be created.
        pub kind: pulumi_wasm_rust::Output<Option<String>>,
        /// The Azure Region where the Data Collection Rule should exist. Changing this forces a new Data Collection Rule to be created.
        pub location: pulumi_wasm_rust::Output<String>,
        /// The name which should be used for this Data Collection Rule. Changing this forces a new Data Collection Rule to be created.
        pub name: pulumi_wasm_rust::Output<String>,
        /// The name of the Resource Group where the Data Collection Rule should exist. Changing this forces a new Data Collection Rule to be created.
        pub resource_group_name: pulumi_wasm_rust::Output<String>,
        /// A `stream_declaration` block as defined below.
        pub stream_declarations: pulumi_wasm_rust::Output<
            Option<
                Vec<super::super::types::monitoring::DataCollectionRuleStreamDeclaration>,
            >,
        >,
        /// A mapping of tags which should be assigned to the Data Collection Rule.
        pub tags: pulumi_wasm_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(name: &str, args: DataCollectionRuleArgs) -> DataCollectionRuleResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let data_collection_endpoint_id_binding = args
            .data_collection_endpoint_id
            .get_inner();
        let data_flows_binding = args.data_flows.get_inner();
        let data_sources_binding = args.data_sources.get_inner();
        let description_binding = args.description.get_inner();
        let destinations_binding = args.destinations.get_inner();
        let identity_binding = args.identity.get_inner();
        let kind_binding = args.kind.get_inner();
        let location_binding = args.location.get_inner();
        let name_binding = args.name.get_inner();
        let resource_group_name_binding = args.resource_group_name.get_inner();
        let stream_declarations_binding = args.stream_declarations.get_inner();
        let tags_binding = args.tags.get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "azure:monitoring/dataCollectionRule:DataCollectionRule".into(),
            name: name.to_string(),
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
            results: Vec::from([
                register_interface::ResultField {
                    name: "dataCollectionEndpointId".into(),
                },
                register_interface::ResultField {
                    name: "dataFlows".into(),
                },
                register_interface::ResultField {
                    name: "dataSources".into(),
                },
                register_interface::ResultField {
                    name: "description".into(),
                },
                register_interface::ResultField {
                    name: "destinations".into(),
                },
                register_interface::ResultField {
                    name: "identity".into(),
                },
                register_interface::ResultField {
                    name: "immutableId".into(),
                },
                register_interface::ResultField {
                    name: "kind".into(),
                },
                register_interface::ResultField {
                    name: "location".into(),
                },
                register_interface::ResultField {
                    name: "name".into(),
                },
                register_interface::ResultField {
                    name: "resourceGroupName".into(),
                },
                register_interface::ResultField {
                    name: "streamDeclarations".into(),
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
        DataCollectionRuleResult {
            data_collection_endpoint_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("dataCollectionEndpointId").unwrap(),
            ),
            data_flows: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("dataFlows").unwrap(),
            ),
            data_sources: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("dataSources").unwrap(),
            ),
            description: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("description").unwrap(),
            ),
            destinations: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("destinations").unwrap(),
            ),
            identity: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("identity").unwrap(),
            ),
            immutable_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("immutableId").unwrap(),
            ),
            kind: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("kind").unwrap(),
            ),
            location: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("location").unwrap(),
            ),
            name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("name").unwrap(),
            ),
            resource_group_name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("resourceGroupName").unwrap(),
            ),
            stream_declarations: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("streamDeclarations").unwrap(),
            ),
            tags: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("tags").unwrap(),
            ),
        }
    }
}