#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod get_data_collection_rule {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GetDataCollectionRuleArgs {
        /// Specifies the name of the Data Collection Rule.
        #[builder(into)]
        pub name: pulumi_gestalt_rust::InputOrOutput<String>,
        /// Specifies the name of the resource group the Data Collection Rule is located in.
        #[builder(into)]
        pub resource_group_name: pulumi_gestalt_rust::InputOrOutput<String>,
    }
    #[allow(dead_code)]
    pub struct GetDataCollectionRuleResult {
        /// The resource ID of the Data Collection Endpoint that this rule can be used with.
        pub data_collection_endpoint_id: pulumi_gestalt_rust::Output<String>,
        /// One or more `data_flow` blocks as defined below.
        pub data_flows: pulumi_gestalt_rust::Output<
            Vec<super::super::super::types::monitoring::GetDataCollectionRuleDataFlow>,
        >,
        /// A `data_sources` block as defined below. This property is optional and can be omitted if the rule is meant to be used via direct calls to the provisioned endpoint.
        pub data_sources: pulumi_gestalt_rust::Output<
            Vec<super::super::super::types::monitoring::GetDataCollectionRuleDataSource>,
        >,
        /// The description of the Data Collection Rule.
        pub description: pulumi_gestalt_rust::Output<String>,
        /// Specifies a list of destination names. A `azure_monitor_metrics` data source only allows for stream of kind `Microsoft-InsightsMetrics`.
        pub destinations: pulumi_gestalt_rust::Output<
            Vec<super::super::super::types::monitoring::GetDataCollectionRuleDestination>,
        >,
        /// The provider-assigned unique ID for this managed resource.
        pub id: pulumi_gestalt_rust::Output<String>,
        /// An `identity` block as defined below.
        pub identities: pulumi_gestalt_rust::Output<
            Vec<super::super::super::types::monitoring::GetDataCollectionRuleIdentity>,
        >,
        pub immutable_id: pulumi_gestalt_rust::Output<String>,
        /// The kind of the Data Collection Rule. Possible values are `Linux`, `Windows`,and `AgentDirectToStore`. A rule of kind `Linux` does not allow for `windows_event_log` data sources. And a rule of kind `Windows` does not allow for `syslog` data sources. If kind is not specified, all kinds of data sources are allowed.
        pub kind: pulumi_gestalt_rust::Output<String>,
        /// The Azure Region where the Data Collection Rule should exist. Changing this forces a new Data Collection Rule to be created.
        pub location: pulumi_gestalt_rust::Output<String>,
        /// The name which should be used for this data source. This name should be unique across all data sources regardless of type within the Data Collection Rule.
        pub name: pulumi_gestalt_rust::Output<String>,
        pub resource_group_name: pulumi_gestalt_rust::Output<String>,
        /// A `stream_declaration` block as defined below.
        pub stream_declarations: pulumi_gestalt_rust::Output<
            Vec<
                super::super::super::types::monitoring::GetDataCollectionRuleStreamDeclaration,
            >,
        >,
        /// A mapping of tags which should be assigned to the Data Collection Rule.
        pub tags: pulumi_gestalt_rust::Output<std::collections::HashMap<String, String>>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn invoke(
        context: &pulumi_gestalt_rust::PulumiContext,
        args: GetDataCollectionRuleArgs,
    ) -> GetDataCollectionRuleResult {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::register_interface;
        use std::collections::HashMap;
        let name_binding_1 = args.name.get_output(context);
        let name_binding = name_binding_1.get_inner();
        let resource_group_name_binding_1 = args.resource_group_name.get_output(context);
        let resource_group_name_binding = resource_group_name_binding_1.get_inner();
        let request = register_interface::ResourceInvokeRequest {
            token: "azure:monitoring/getDataCollectionRule:getDataCollectionRule".into(),
            version: super::super::super::get_version(),
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
        };
        let o = register_interface::invoke(context.get_inner(), &request);
        GetDataCollectionRuleResult {
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
            id: pulumi_gestalt_rust::__private::into_domain(o.extract_field("id")),
            identities: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("identities"),
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
