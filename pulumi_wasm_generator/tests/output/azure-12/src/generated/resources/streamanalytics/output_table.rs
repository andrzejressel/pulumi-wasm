/// Manages a Stream Analytics Output Table.
///
/// ## Example Usage
///
/// ```yaml
/// resources:
///   exampleResourceGroup:
///     type: azure:core:ResourceGroup
///     name: example
///     properties:
///       name: rg-example
///       location: West Europe
///   exampleAccount:
///     type: azure:storage:Account
///     name: example
///     properties:
///       name: examplesa
///       resourceGroupName: ${exampleResourceGroup.name}
///       location: ${exampleResourceGroup.location}
///       accountTier: Standard
///       accountReplicationType: LRS
///   exampleTable:
///     type: azure:storage:Table
///     name: example
///     properties:
///       name: exampletable
///       storageAccountName: ${exampleAccount.name}
///   exampleOutputTable:
///     type: azure:streamanalytics:OutputTable
///     name: example
///     properties:
///       name: output-to-storage-table
///       streamAnalyticsJobName: ${example.name}
///       resourceGroupName: ${example.resourceGroupName}
///       storageAccountName: ${exampleAccount.name}
///       storageAccountKey: ${exampleAccount.primaryAccessKey}
///       table: ${exampleTable.name}
///       partitionKey: foo
///       rowKey: bar
///       batchSize: 100
/// variables:
///   example:
///     fn::invoke:
///       function: azure:streamanalytics:getJob
///       arguments:
///         name: example-job
///         resourceGroupName: ${exampleResourceGroup.name}
/// ```
///
/// ## Import
///
/// Stream Analytics Output to Table can be imported using the `resource id`, e.g.
///
/// ```sh
/// $ pulumi import azure:streamanalytics/outputTable:OutputTable example /subscriptions/00000000-0000-0000-0000-000000000000/resourceGroups/group1/providers/Microsoft.StreamAnalytics/streamingJobs/job1/outputs/output1
/// ```
///
pub mod output_table {
    #[derive(pulumi_wasm_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct OutputTableArgs {
        /// The number of records for a batch operation. Must be between `1` and `100`.
        #[builder(into)]
        pub batch_size: pulumi_wasm_rust::InputOrOutput<i32>,
        /// A list of the column names to be removed from output event entities.
        #[builder(into, default)]
        pub columns_to_removes: pulumi_wasm_rust::InputOrOutput<Option<Vec<String>>>,
        /// The name of the Stream Output. Changing this forces a new resource to be created.
        #[builder(into, default)]
        pub name: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        /// The name of the output column that contains the partition key.
        #[builder(into)]
        pub partition_key: pulumi_wasm_rust::InputOrOutput<String>,
        /// The name of the Resource Group where the Stream Analytics Job exists. Changing this forces a new resource to be created.
        #[builder(into)]
        pub resource_group_name: pulumi_wasm_rust::InputOrOutput<String>,
        /// The name of the output column that contains the row key.
        #[builder(into)]
        pub row_key: pulumi_wasm_rust::InputOrOutput<String>,
        /// The Access Key which should be used to connect to this Storage Account.
        #[builder(into)]
        pub storage_account_key: pulumi_wasm_rust::InputOrOutput<String>,
        /// The name of the Storage Account.
        #[builder(into)]
        pub storage_account_name: pulumi_wasm_rust::InputOrOutput<String>,
        /// The name of the Stream Analytics Job. Changing this forces a new resource to be created.
        #[builder(into)]
        pub stream_analytics_job_name: pulumi_wasm_rust::InputOrOutput<String>,
        /// The name of the table where the stream should be output to.
        #[builder(into)]
        pub table: pulumi_wasm_rust::InputOrOutput<String>,
    }
    #[allow(dead_code)]
    pub struct OutputTableResult {
        /// The number of records for a batch operation. Must be between `1` and `100`.
        pub batch_size: pulumi_wasm_rust::Output<i32>,
        /// A list of the column names to be removed from output event entities.
        pub columns_to_removes: pulumi_wasm_rust::Output<Option<Vec<String>>>,
        /// The name of the Stream Output. Changing this forces a new resource to be created.
        pub name: pulumi_wasm_rust::Output<String>,
        /// The name of the output column that contains the partition key.
        pub partition_key: pulumi_wasm_rust::Output<String>,
        /// The name of the Resource Group where the Stream Analytics Job exists. Changing this forces a new resource to be created.
        pub resource_group_name: pulumi_wasm_rust::Output<String>,
        /// The name of the output column that contains the row key.
        pub row_key: pulumi_wasm_rust::Output<String>,
        /// The Access Key which should be used to connect to this Storage Account.
        pub storage_account_key: pulumi_wasm_rust::Output<String>,
        /// The name of the Storage Account.
        pub storage_account_name: pulumi_wasm_rust::Output<String>,
        /// The name of the Stream Analytics Job. Changing this forces a new resource to be created.
        pub stream_analytics_job_name: pulumi_wasm_rust::Output<String>,
        /// The name of the table where the stream should be output to.
        pub table: pulumi_wasm_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_wasm_rust::PulumiContext,
        name: &str,
        args: OutputTableArgs,
    ) -> OutputTableResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let batch_size_binding = args.batch_size.get_output(context).get_inner();
        let columns_to_removes_binding = args
            .columns_to_removes
            .get_output(context)
            .get_inner();
        let name_binding = args.name.get_output(context).get_inner();
        let partition_key_binding = args.partition_key.get_output(context).get_inner();
        let resource_group_name_binding = args
            .resource_group_name
            .get_output(context)
            .get_inner();
        let row_key_binding = args.row_key.get_output(context).get_inner();
        let storage_account_key_binding = args
            .storage_account_key
            .get_output(context)
            .get_inner();
        let storage_account_name_binding = args
            .storage_account_name
            .get_output(context)
            .get_inner();
        let stream_analytics_job_name_binding = args
            .stream_analytics_job_name
            .get_output(context)
            .get_inner();
        let table_binding = args.table.get_output(context).get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "azure:streamanalytics/outputTable:OutputTable".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "batchSize".into(),
                    value: &batch_size_binding,
                },
                register_interface::ObjectField {
                    name: "columnsToRemoves".into(),
                    value: &columns_to_removes_binding,
                },
                register_interface::ObjectField {
                    name: "name".into(),
                    value: &name_binding,
                },
                register_interface::ObjectField {
                    name: "partitionKey".into(),
                    value: &partition_key_binding,
                },
                register_interface::ObjectField {
                    name: "resourceGroupName".into(),
                    value: &resource_group_name_binding,
                },
                register_interface::ObjectField {
                    name: "rowKey".into(),
                    value: &row_key_binding,
                },
                register_interface::ObjectField {
                    name: "storageAccountKey".into(),
                    value: &storage_account_key_binding,
                },
                register_interface::ObjectField {
                    name: "storageAccountName".into(),
                    value: &storage_account_name_binding,
                },
                register_interface::ObjectField {
                    name: "streamAnalyticsJobName".into(),
                    value: &stream_analytics_job_name_binding,
                },
                register_interface::ObjectField {
                    name: "table".into(),
                    value: &table_binding,
                },
            ]),
            results: Vec::from([
                register_interface::ResultField {
                    name: "batchSize".into(),
                },
                register_interface::ResultField {
                    name: "columnsToRemoves".into(),
                },
                register_interface::ResultField {
                    name: "name".into(),
                },
                register_interface::ResultField {
                    name: "partitionKey".into(),
                },
                register_interface::ResultField {
                    name: "resourceGroupName".into(),
                },
                register_interface::ResultField {
                    name: "rowKey".into(),
                },
                register_interface::ResultField {
                    name: "storageAccountKey".into(),
                },
                register_interface::ResultField {
                    name: "storageAccountName".into(),
                },
                register_interface::ResultField {
                    name: "streamAnalyticsJobName".into(),
                },
                register_interface::ResultField {
                    name: "table".into(),
                },
            ]),
        };
        let o = register_interface::register(context.get_inner(), &request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        OutputTableResult {
            batch_size: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("batchSize").unwrap(),
            ),
            columns_to_removes: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("columnsToRemoves").unwrap(),
            ),
            name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("name").unwrap(),
            ),
            partition_key: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("partitionKey").unwrap(),
            ),
            resource_group_name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("resourceGroupName").unwrap(),
            ),
            row_key: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("rowKey").unwrap(),
            ),
            storage_account_key: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("storageAccountKey").unwrap(),
            ),
            storage_account_name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("storageAccountName").unwrap(),
            ),
            stream_analytics_job_name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("streamAnalyticsJobName").unwrap(),
            ),
            table: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("table").unwrap(),
            ),
        }
    }
}
