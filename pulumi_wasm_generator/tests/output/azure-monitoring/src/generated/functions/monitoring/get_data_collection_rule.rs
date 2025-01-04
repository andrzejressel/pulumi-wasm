pub mod get_data_collection_rule {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GetDataCollectionRuleArgs {
        /// Specifies the name of the Data Collection Rule.
        #[builder(into)]
        pub name: pulumi_wasm_rust::Output<String>,
        /// Specifies the name of the resource group the Data Collection Rule is located in.
        #[builder(into)]
        pub resource_group_name: pulumi_wasm_rust::Output<String>,
    }
    #[allow(dead_code)]
    pub struct GetDataCollectionRuleResult {
        /// The resource ID of the Data Collection Endpoint that this rule can be used with.
        pub data_collection_endpoint_id: pulumi_wasm_rust::Output<String>,
        /// One or more `data_flow` blocks as defined below.
        pub data_flows: pulumi_wasm_rust::Output<
            Vec<super::super::super::types::monitoring::GetDataCollectionRuleDataFlow>,
        >,
        /// A `data_sources` block as defined below. This property is optional and can be omitted if the rule is meant to be used via direct calls to the provisioned endpoint.
        pub data_sources: pulumi_wasm_rust::Output<
            Vec<super::super::super::types::monitoring::GetDataCollectionRuleDataSource>,
        >,
        /// The description of the Data Collection Rule.
        pub description: pulumi_wasm_rust::Output<String>,
        /// Specifies a list of destination names. A `azure_monitor_metrics` data source only allows for stream of kind `Microsoft-InsightsMetrics`.
        pub destinations: pulumi_wasm_rust::Output<
            Vec<super::super::super::types::monitoring::GetDataCollectionRuleDestination>,
        >,
        /// The provider-assigned unique ID for this managed resource.
        pub id: pulumi_wasm_rust::Output<String>,
        /// An `identity` block as defined below.
        pub identities: pulumi_wasm_rust::Output<
            Vec<super::super::super::types::monitoring::GetDataCollectionRuleIdentity>,
        >,
        pub immutable_id: pulumi_wasm_rust::Output<String>,
        /// The kind of the Data Collection Rule. Possible values are `Linux`, `Windows`,and `AgentDirectToStore`. A rule of kind `Linux` does not allow for `windows_event_log` data sources. And a rule of kind `Windows` does not allow for `syslog` data sources. If kind is not specified, all kinds of data sources are allowed.
        pub kind: pulumi_wasm_rust::Output<String>,
        /// The Azure Region where the Data Collection Rule should exist. Changing this forces a new Data Collection Rule to be created.
        pub location: pulumi_wasm_rust::Output<String>,
        /// The name which should be used for this data source. This name should be unique across all data sources regardless of type within the Data Collection Rule.
        pub name: pulumi_wasm_rust::Output<String>,
        pub resource_group_name: pulumi_wasm_rust::Output<String>,
        /// A `stream_declaration` block as defined below.
        pub stream_declarations: pulumi_wasm_rust::Output<
            Vec<
                super::super::super::types::monitoring::GetDataCollectionRuleStreamDeclaration,
            >,
        >,
        /// A mapping of tags which should be assigned to the Data Collection Rule.
        pub tags: pulumi_wasm_rust::Output<std::collections::HashMap<String, String>>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn invoke(args: GetDataCollectionRuleArgs) -> GetDataCollectionRuleResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let name_binding = args.name.get_inner();
        let resource_group_name_binding = args.resource_group_name.get_inner();
        let request = register_interface::ResourceInvokeRequest {
            token: "azure:monitoring/getDataCollectionRule:getDataCollectionRule".into(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "name".into(),
                    value: &name_binding,
                },
                register_interface::ObjectField {
                    name: "resourceGroupName".into(),
                    value: &resource_group_name_binding,
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
                    name: "id".into(),
                },
                register_interface::ResultField {
                    name: "identities".into(),
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
        let o = register_interface::invoke(&request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        GetDataCollectionRuleResult {
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
            id: pulumi_wasm_rust::__private::into_domain(hashmap.remove("id").unwrap()),
            identities: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("identities").unwrap(),
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
