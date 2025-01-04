pub mod get_analytics_workspace {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GetAnalyticsWorkspaceArgs {
        /// Specifies the name of the Log Analytics Workspace.
        #[builder(into)]
        pub name: pulumi_wasm_rust::Output<String>,
        /// The name of the resource group in which the Log Analytics workspace is located in.
        #[builder(into)]
        pub resource_group_name: pulumi_wasm_rust::Output<String>,
    }
    #[allow(dead_code)]
    pub struct GetAnalyticsWorkspaceResult {
        /// The workspace daily quota for ingestion in GB.
        pub daily_quota_gb: pulumi_wasm_rust::Output<f64>,
        /// The provider-assigned unique ID for this managed resource.
        pub id: pulumi_wasm_rust::Output<String>,
        pub location: pulumi_wasm_rust::Output<String>,
        pub name: pulumi_wasm_rust::Output<String>,
        /// The Primary shared key for the Log Analytics Workspace.
        pub primary_shared_key: pulumi_wasm_rust::Output<String>,
        pub resource_group_name: pulumi_wasm_rust::Output<String>,
        /// The workspace data retention in days.
        pub retention_in_days: pulumi_wasm_rust::Output<i32>,
        /// The Secondary shared key for the Log Analytics Workspace.
        pub secondary_shared_key: pulumi_wasm_rust::Output<String>,
        /// The SKU of the Log Analytics Workspace.
        pub sku: pulumi_wasm_rust::Output<String>,
        /// A mapping of tags assigned to the resource.
        pub tags: pulumi_wasm_rust::Output<std::collections::HashMap<String, String>>,
        /// The Workspace (or Customer) ID for the Log Analytics Workspace.
        pub workspace_id: pulumi_wasm_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn invoke(args: GetAnalyticsWorkspaceArgs) -> GetAnalyticsWorkspaceResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let name_binding = args.name.get_inner();
        let resource_group_name_binding = args.resource_group_name.get_inner();
        let request = register_interface::ResourceInvokeRequest {
            token: "azure:operationalinsights/getAnalyticsWorkspace:getAnalyticsWorkspace"
                .into(),
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
                    name: "dailyQuotaGb".into(),
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
                    name: "primarySharedKey".into(),
                },
                register_interface::ResultField {
                    name: "resourceGroupName".into(),
                },
                register_interface::ResultField {
                    name: "retentionInDays".into(),
                },
                register_interface::ResultField {
                    name: "secondarySharedKey".into(),
                },
                register_interface::ResultField {
                    name: "sku".into(),
                },
                register_interface::ResultField {
                    name: "tags".into(),
                },
                register_interface::ResultField {
                    name: "workspaceId".into(),
                },
            ]),
        };
        let o = register_interface::invoke(&request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        GetAnalyticsWorkspaceResult {
            daily_quota_gb: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("dailyQuotaGb").unwrap(),
            ),
            id: pulumi_wasm_rust::__private::into_domain(hashmap.remove("id").unwrap()),
            location: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("location").unwrap(),
            ),
            name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("name").unwrap(),
            ),
            primary_shared_key: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("primarySharedKey").unwrap(),
            ),
            resource_group_name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("resourceGroupName").unwrap(),
            ),
            retention_in_days: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("retentionInDays").unwrap(),
            ),
            secondary_shared_key: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("secondarySharedKey").unwrap(),
            ),
            sku: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("sku").unwrap(),
            ),
            tags: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("tags").unwrap(),
            ),
            workspace_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("workspaceId").unwrap(),
            ),
        }
    }
}
