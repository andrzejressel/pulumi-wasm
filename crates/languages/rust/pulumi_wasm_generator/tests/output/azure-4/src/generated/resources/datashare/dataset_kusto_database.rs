/// Manages a Data Share Kusto Database Dataset.
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
///   exampleDatabase:
///     type: azure:kusto:Database
///     name: example
///     properties:
///       name: examplekd
///       resourceGroupName: ${example.name}
///       location: ${example.location}
///       clusterName: ${exampleCluster.name}
///   exampleAssignment:
///     type: azure:authorization:Assignment
///     name: example
///     properties:
///       scope: ${exampleCluster.id}
///       roleDefinitionName: Contributor
///       principalId: ${exampleAccount.identity.principalId}
///   exampleDatasetKustoDatabase:
///     type: azure:datashare:DatasetKustoDatabase
///     name: example
///     properties:
///       name: example-dskd
///       shareId: ${exampleShare.id}
///       kustoDatabaseId: ${exampleDatabase.id}
///     options:
///       dependsOn:
///         - ${exampleAssignment}
/// ```
///
/// ## Import
///
/// Data Share Kusto Database Datasets can be imported using the `resource id`, e.g.
///
/// ```sh
/// $ pulumi import azure:datashare/datasetKustoDatabase:DatasetKustoDatabase example /subscriptions/00000000-0000-0000-0000-000000000000/resourceGroups/group1/providers/Microsoft.DataShare/accounts/account1/shares/share1/dataSets/dataSet1
/// ```
///
pub mod dataset_kusto_database {
    #[derive(pulumi_wasm_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct DatasetKustoDatabaseArgs {
        /// The resource ID of the Kusto Cluster Database to be shared with the receiver. Changing this forces a new Data Share Kusto Database Dataset to be created.
        #[builder(into)]
        pub kusto_database_id: pulumi_wasm_rust::InputOrOutput<String>,
        /// The name which should be used for this Data Share Kusto Database Dataset. Changing this forces a new Data Share Kusto Database Dataset to be created.
        #[builder(into, default)]
        pub name: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        /// The resource ID of the Data Share where this Data Share Kusto Database Dataset should be created. Changing this forces a new Data Share Kusto Database Dataset to be created.
        #[builder(into)]
        pub share_id: pulumi_wasm_rust::InputOrOutput<String>,
    }
    #[allow(dead_code)]
    pub struct DatasetKustoDatabaseResult {
        /// The name of the Data Share Dataset.
        pub display_name: pulumi_wasm_rust::Output<String>,
        /// The location of the Kusto Cluster.
        pub kusto_cluster_location: pulumi_wasm_rust::Output<String>,
        /// The resource ID of the Kusto Cluster Database to be shared with the receiver. Changing this forces a new Data Share Kusto Database Dataset to be created.
        pub kusto_database_id: pulumi_wasm_rust::Output<String>,
        /// The name which should be used for this Data Share Kusto Database Dataset. Changing this forces a new Data Share Kusto Database Dataset to be created.
        pub name: pulumi_wasm_rust::Output<String>,
        /// The resource ID of the Data Share where this Data Share Kusto Database Dataset should be created. Changing this forces a new Data Share Kusto Database Dataset to be created.
        pub share_id: pulumi_wasm_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_wasm_rust::PulumiContext,
        name: &str,
        args: DatasetKustoDatabaseArgs,
    ) -> DatasetKustoDatabaseResult {
        use pulumi_wasm_rust::__private::pulumi_gestalt_adapter_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let kusto_database_id_binding = args
            .kusto_database_id
            .get_output(context)
            .get_inner();
        let name_binding = args.name.get_output(context).get_inner();
        let share_id_binding = args.share_id.get_output(context).get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "azure:datashare/datasetKustoDatabase:DatasetKustoDatabase".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "kustoDatabaseId".into(),
                    value: &kusto_database_id_binding,
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
        };
        let o = register_interface::register(context.get_inner(), &request);
        DatasetKustoDatabaseResult {
            display_name: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("displayName"),
            ),
            kusto_cluster_location: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("kustoClusterLocation"),
            ),
            kusto_database_id: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("kustoDatabaseId"),
            ),
            name: pulumi_wasm_rust::__private::into_domain(o.extract_field("name")),
            share_id: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("shareId"),
            ),
        }
    }
}
