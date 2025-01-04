/// Manages a Data Share Kusto Cluster Dataset.
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
///   exampleAccount:
///     type: azure:datashare:Account
///     name: example
///     properties:
///       name: example-dsa
///       location: ${example.location}
///       resourceGroupName: ${example.name}
///       identity:
///         type: SystemAssigned
///   exampleShare:
///     type: azure:datashare:Share
///     name: example
///     properties:
///       name: example_ds
///       accountId: ${exampleAccount.id}
///       kind: InPlace
///   exampleCluster:
///     type: azure:kusto:Cluster
///     name: example
///     properties:
///       name: examplekc
///       location: ${example.location}
///       resourceGroupName: ${example.name}
///       sku:
///         name: Dev(No SLA)_Standard_D11_v2
///         capacity: 1
///   exampleAssignment:
///     type: azure:authorization:Assignment
///     name: example
///     properties:
///       scope: ${exampleCluster.id}
///       roleDefinitionName: Contributor
///       principalId: ${exampleAccount.identity.principalId}
///   exampleDatasetKustoCluster:
///     type: azure:datashare:DatasetKustoCluster
///     name: example
///     properties:
///       name: example-dskc
///       shareId: ${exampleShare.id}
///       kustoClusterId: ${exampleCluster.id}
///     options:
///       dependsOn:
///         - ${exampleAssignment}
/// ```
///
/// ## Import
///
/// Data Share Kusto Cluster Datasets can be imported using the `resource id`, e.g.
///
/// ```sh
/// $ pulumi import azure:datashare/datasetKustoCluster:DatasetKustoCluster example /subscriptions/00000000-0000-0000-0000-000000000000/resourceGroups/group1/providers/Microsoft.DataShare/accounts/account1/shares/share1/dataSets/dataSet1
/// ```
///
pub mod dataset_kusto_cluster {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct DatasetKustoClusterArgs {
        /// The resource ID of the Kusto Cluster to be shared with the receiver. Changing this forces a new Data Share Kusto Cluster Dataset to be created.
        #[builder(into)]
        pub kusto_cluster_id: pulumi_wasm_rust::Output<String>,
        /// The name which should be used for this Data Share Kusto Cluster Dataset. Changing this forces a new Data Share Kusto Cluster Dataset to be created.
        #[builder(into, default)]
        pub name: pulumi_wasm_rust::Output<Option<String>>,
        /// The resource ID of the Data Share where this Data Share Kusto Cluster Dataset should be created. Changing this forces a new Data Share Kusto Cluster Dataset to be created.
        #[builder(into)]
        pub share_id: pulumi_wasm_rust::Output<String>,
    }
    #[allow(dead_code)]
    pub struct DatasetKustoClusterResult {
        /// The name of the Data Share Dataset.
        pub display_name: pulumi_wasm_rust::Output<String>,
        /// The resource ID of the Kusto Cluster to be shared with the receiver. Changing this forces a new Data Share Kusto Cluster Dataset to be created.
        pub kusto_cluster_id: pulumi_wasm_rust::Output<String>,
        /// The location of the Kusto Cluster.
        pub kusto_cluster_location: pulumi_wasm_rust::Output<String>,
        /// The name which should be used for this Data Share Kusto Cluster Dataset. Changing this forces a new Data Share Kusto Cluster Dataset to be created.
        pub name: pulumi_wasm_rust::Output<String>,
        /// The resource ID of the Data Share where this Data Share Kusto Cluster Dataset should be created. Changing this forces a new Data Share Kusto Cluster Dataset to be created.
        pub share_id: pulumi_wasm_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        name: &str,
        args: DatasetKustoClusterArgs,
    ) -> DatasetKustoClusterResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let kusto_cluster_id_binding = args.kusto_cluster_id.get_inner();
        let name_binding = args.name.get_inner();
        let share_id_binding = args.share_id.get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "azure:datashare/datasetKustoCluster:DatasetKustoCluster".into(),
            name: name.to_string(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "kustoClusterId".into(),
                    value: &kusto_cluster_id_binding,
                },
                register_interface::ObjectField {
                    name: "name".into(),
                    value: &name_binding,
                },
                register_interface::ObjectField {
                    name: "shareId".into(),
                    value: &share_id_binding,
                },
            ]),
            results: Vec::from([
                register_interface::ResultField {
                    name: "displayName".into(),
                },
                register_interface::ResultField {
                    name: "kustoClusterId".into(),
                },
                register_interface::ResultField {
                    name: "kustoClusterLocation".into(),
                },
                register_interface::ResultField {
                    name: "name".into(),
                },
                register_interface::ResultField {
                    name: "shareId".into(),
                },
            ]),
        };
        let o = register_interface::register(&request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        DatasetKustoClusterResult {
            display_name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("displayName").unwrap(),
            ),
            kusto_cluster_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("kustoClusterId").unwrap(),
            ),
            kusto_cluster_location: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("kustoClusterLocation").unwrap(),
            ),
            name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("name").unwrap(),
            ),
            share_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("shareId").unwrap(),
            ),
        }
    }
}
