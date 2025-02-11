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
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod output_table {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct OutputTableArgs {
        /// The number of records for a batch operation. Must be between `1` and `100`.
        #[builder(into)]
        pub batch_size: pulumi_gestalt_rust::InputOrOutput<i32>,
        /// A list of the column names to be removed from output event entities.
        #[builder(into, default)]
        pub columns_to_removes: pulumi_gestalt_rust::InputOrOutput<Option<Vec<String>>>,
        /// The name of the Stream Output. Changing this forces a new resource to be created.
        #[builder(into, default)]
        pub name: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The name of the output column that contains the partition key.
        #[builder(into)]
        pub partition_key: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The name of the Resource Group where the Stream Analytics Job exists. Changing this forces a new resource to be created.
        #[builder(into)]
        pub resource_group_name: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The name of the output column that contains the row key.
        #[builder(into)]
        pub row_key: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The Access Key which should be used to connect to this Storage Account.
        #[builder(into)]
        pub storage_account_key: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The name of the Storage Account.
        #[builder(into)]
        pub storage_account_name: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The name of the Stream Analytics Job. Changing this forces a new resource to be created.
        #[builder(into)]
        pub stream_analytics_job_name: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The name of the table where the stream should be output to.
        #[builder(into)]
        pub table: pulumi_gestalt_rust::InputOrOutput<String>,
    }
    #[allow(dead_code)]
    pub struct OutputTableResult {
        /// The number of records for a batch operation. Must be between `1` and `100`.
        pub batch_size: pulumi_gestalt_rust::Output<i32>,
        /// A list of the column names to be removed from output event entities.
        pub columns_to_removes: pulumi_gestalt_rust::Output<Option<Vec<String>>>,
        /// The name of the Stream Output. Changing this forces a new resource to be created.
        pub name: pulumi_gestalt_rust::Output<String>,
        /// The name of the output column that contains the partition key.
        pub partition_key: pulumi_gestalt_rust::Output<String>,
        /// The name of the Resource Group where the Stream Analytics Job exists. Changing this forces a new resource to be created.
        pub resource_group_name: pulumi_gestalt_rust::Output<String>,
        /// The name of the output column that contains the row key.
        pub row_key: pulumi_gestalt_rust::Output<String>,
        /// The Access Key which should be used to connect to this Storage Account.
        pub storage_account_key: pulumi_gestalt_rust::Output<String>,
        /// The name of the Storage Account.
        pub storage_account_name: pulumi_gestalt_rust::Output<String>,
        /// The name of the Stream Analytics Job. Changing this forces a new resource to be created.
        pub stream_analytics_job_name: pulumi_gestalt_rust::Output<String>,
        /// The name of the table where the stream should be output to.
        pub table: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: OutputTableArgs,
    ) -> OutputTableResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let batch_size_binding = args.batch_size.get_output(context);
        let columns_to_removes_binding = args.columns_to_removes.get_output(context);
        let name_binding = args.name.get_output(context);
        let partition_key_binding = args.partition_key.get_output(context);
        let resource_group_name_binding = args.resource_group_name.get_output(context);
        let row_key_binding = args.row_key.get_output(context);
        let storage_account_key_binding = args.storage_account_key.get_output(context);
        let storage_account_name_binding = args.storage_account_name.get_output(context);
        let stream_analytics_job_name_binding = args
            .stream_analytics_job_name
            .get_output(context);
        let table_binding = args.table.get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "azure:streamanalytics/outputTable:OutputTable".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "batchSize".into(),
                    value: &batch_size_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "columnsToRemoves".into(),
                    value: &columns_to_removes_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "name".into(),
                    value: &name_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "partitionKey".into(),
                    value: &partition_key_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "resourceGroupName".into(),
                    value: &resource_group_name_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "rowKey".into(),
                    value: &row_key_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "storageAccountKey".into(),
                    value: &storage_account_key_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "storageAccountName".into(),
                    value: &storage_account_name_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "streamAnalyticsJobName".into(),
                    value: &stream_analytics_job_name_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "table".into(),
                    value: &table_binding.drop_type(),
                },
            ],
        };
        let o = context.register_resource(request);
        OutputTableResult {
            batch_size: o.get_field("batchSize"),
            columns_to_removes: o.get_field("columnsToRemoves"),
            name: o.get_field("name"),
            partition_key: o.get_field("partitionKey"),
            resource_group_name: o.get_field("resourceGroupName"),
            row_key: o.get_field("rowKey"),
            storage_account_key: o.get_field("storageAccountKey"),
            storage_account_name: o.get_field("storageAccountName"),
            stream_analytics_job_name: o.get_field("streamAnalyticsJobName"),
            table: o.get_field("table"),
        }
    }
}
