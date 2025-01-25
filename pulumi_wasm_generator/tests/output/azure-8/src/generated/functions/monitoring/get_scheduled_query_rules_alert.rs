pub mod get_scheduled_query_rules_alert {
    #[derive(pulumi_wasm_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GetScheduledQueryRulesAlertArgs {
        /// Specifies the name of the scheduled query rule.
        #[builder(into)]
        pub name: pulumi_wasm_rust::InputOrOutput<String>,
        /// Specifies the name of the resource group where the scheduled query rule is located.
        #[builder(into)]
        pub resource_group_name: pulumi_wasm_rust::InputOrOutput<String>,
    }
    #[allow(dead_code)]
    pub struct GetScheduledQueryRulesAlertResult {
        /// supports the following:
        pub actions: pulumi_wasm_rust::Output<
            Vec<
                super::super::super::types::monitoring::GetScheduledQueryRulesAlertAction,
            >,
        >,
        /// The list of Resource IDs referred into query.
        pub authorized_resource_ids: pulumi_wasm_rust::Output<Vec<String>>,
        /// The resource URI over which log search query is to be run.
        pub data_source_id: pulumi_wasm_rust::Output<String>,
        /// The description of the scheduled query rule.
        pub description: pulumi_wasm_rust::Output<String>,
        /// Whether this scheduled query rule is enabled.
        pub enabled: pulumi_wasm_rust::Output<bool>,
        /// Frequency at which rule condition should be evaluated.
        pub frequency: pulumi_wasm_rust::Output<i32>,
        /// The provider-assigned unique ID for this managed resource.
        pub id: pulumi_wasm_rust::Output<String>,
        /// Specifies the Azure Region where the resource should exist.
        pub location: pulumi_wasm_rust::Output<String>,
        pub name: pulumi_wasm_rust::Output<String>,
        /// Log search query.
        pub query: pulumi_wasm_rust::Output<String>,
        /// The type of query results.
        pub query_type: pulumi_wasm_rust::Output<String>,
        pub resource_group_name: pulumi_wasm_rust::Output<String>,
        /// Severity of the alert.
        pub severity: pulumi_wasm_rust::Output<i32>,
        pub tags: pulumi_wasm_rust::Output<std::collections::HashMap<String, String>>,
        /// Time for which alerts should be throttled or suppressed.
        pub throttling: pulumi_wasm_rust::Output<i32>,
        /// Time window for which data needs to be fetched for query.
        pub time_window: pulumi_wasm_rust::Output<i32>,
        /// A `trigger` block as defined below.
        pub triggers: pulumi_wasm_rust::Output<
            Vec<
                super::super::super::types::monitoring::GetScheduledQueryRulesAlertTrigger,
            >,
        >,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn invoke(
        context: &pulumi_wasm_rust::PulumiContext,
        args: GetScheduledQueryRulesAlertArgs,
    ) -> GetScheduledQueryRulesAlertResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let name_binding = args.name.get_output(context).get_inner();
        let resource_group_name_binding = args
            .resource_group_name
            .get_output(context)
            .get_inner();
        let request = register_interface::ResourceInvokeRequest {
            token: "azure:monitoring/getScheduledQueryRulesAlert:getScheduledQueryRulesAlert"
                .into(),
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
            results: Vec::from([
                register_interface::ResultField {
                    name: "actions".into(),
                },
                register_interface::ResultField {
                    name: "authorizedResourceIds".into(),
                },
                register_interface::ResultField {
                    name: "dataSourceId".into(),
                },
                register_interface::ResultField {
                    name: "description".into(),
                },
                register_interface::ResultField {
                    name: "enabled".into(),
                },
                register_interface::ResultField {
                    name: "frequency".into(),
                },
                register_interface::ResultField {
                    name: "id".into(),
                },
                register_interface::ResultField {
                    name: "location".into(),
                },
                register_interface::ResultField {
                    name: "name".into(),
                },
                register_interface::ResultField {
                    name: "query".into(),
                },
                register_interface::ResultField {
                    name: "queryType".into(),
                },
                register_interface::ResultField {
                    name: "resourceGroupName".into(),
                },
                register_interface::ResultField {
                    name: "severity".into(),
                },
                register_interface::ResultField {
                    name: "tags".into(),
                },
                register_interface::ResultField {
                    name: "throttling".into(),
                },
                register_interface::ResultField {
                    name: "timeWindow".into(),
                },
                register_interface::ResultField {
                    name: "triggers".into(),
                },
            ]),
        };
        let o = register_interface::invoke(context.get_inner(), &request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        GetScheduledQueryRulesAlertResult {
            actions: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("actions").unwrap(),
            ),
            authorized_resource_ids: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("authorizedResourceIds").unwrap(),
            ),
            data_source_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("dataSourceId").unwrap(),
            ),
            description: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("description").unwrap(),
            ),
            enabled: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("enabled").unwrap(),
            ),
            frequency: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("frequency").unwrap(),
            ),
            id: pulumi_wasm_rust::__private::into_domain(hashmap.remove("id").unwrap()),
            location: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("location").unwrap(),
            ),
            name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("name").unwrap(),
            ),
            query: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("query").unwrap(),
            ),
            query_type: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("queryType").unwrap(),
            ),
            resource_group_name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("resourceGroupName").unwrap(),
            ),
            severity: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("severity").unwrap(),
            ),
            tags: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("tags").unwrap(),
            ),
            throttling: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("throttling").unwrap(),
            ),
            time_window: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("timeWindow").unwrap(),
            ),
            triggers: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("triggers").unwrap(),
            ),
        }
    }
}
