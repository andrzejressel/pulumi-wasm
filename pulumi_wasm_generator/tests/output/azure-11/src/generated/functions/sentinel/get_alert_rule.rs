pub mod get_alert_rule {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GetAlertRuleArgs {
        /// The ID of the Log Analytics Workspace this Sentinel Alert Rule belongs to.
        #[builder(into)]
        pub log_analytics_workspace_id: pulumi_wasm_rust::Output<String>,
        /// The name which should be used for this Sentinel Alert Rule.
        #[builder(into)]
        pub name: pulumi_wasm_rust::Output<String>,
    }
    #[allow(dead_code)]
    pub struct GetAlertRuleResult {
        /// The provider-assigned unique ID for this managed resource.
        pub id: pulumi_wasm_rust::Output<String>,
        pub log_analytics_workspace_id: pulumi_wasm_rust::Output<String>,
        pub name: pulumi_wasm_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn invoke(args: GetAlertRuleArgs) -> GetAlertRuleResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let log_analytics_workspace_id_binding = args
            .log_analytics_workspace_id
            .get_inner();
        let name_binding = args.name.get_inner();
        let request = register_interface::ResourceInvokeRequest {
            token: "azure:sentinel/getAlertRule:getAlertRule".into(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "logAnalyticsWorkspaceId".into(),
                    value: &log_analytics_workspace_id_binding,
                },
                register_interface::ObjectField {
                    name: "name".into(),
                    value: &name_binding,
                },
            ]),
            results: Vec::from([
                register_interface::ResultField {
                    name: "id".into(),
                },
                register_interface::ResultField {
                    name: "logAnalyticsWorkspaceId".into(),
                },
                register_interface::ResultField {
                    name: "name".into(),
                },
            ]),
        };
        let o = register_interface::invoke(&request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        GetAlertRuleResult {
            id: pulumi_wasm_rust::__private::into_domain(hashmap.remove("id").unwrap()),
            log_analytics_workspace_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("logAnalyticsWorkspaceId").unwrap(),
            ),
            name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("name").unwrap(),
            ),
        }
    }
}
