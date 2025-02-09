/// Provides a Glue ML Transform resource.
///
/// ## Example Usage
///
/// ```yaml
/// resources:
///   test:
///     type: aws:glue:MLTransform
///     properties:
///       name: example
///       roleArn: ${testAwsIamRole.arn}
///       inputRecordTables:
///         - databaseName: ${testCatalogTable.databaseName}
///           tableName: ${testCatalogTable.name}
///       parameters:
///         transformType: FIND_MATCHES
///         findMatchesParameters:
///           primaryKeyColumnName: my_column_1
///     options:
///       dependsOn:
///         - ${testAwsIamRolePolicyAttachment}
///   testCatalogDatabase:
///     type: aws:glue:CatalogDatabase
///     name: test
///     properties:
///       name: example
///   testCatalogTable:
///     type: aws:glue:CatalogTable
///     name: test
///     properties:
///       name: example
///       databaseName: ${testCatalogDatabase.name}
///       owner: my_owner
///       retention: 1
///       tableType: VIRTUAL_VIEW
///       viewExpandedText: view_expanded_text_1
///       viewOriginalText: view_original_text_1
///       storageDescriptor:
///         bucketColumns:
///           - bucket_column_1
///         compressed: false
///         inputFormat: SequenceFileInputFormat
///         location: my_location
///         numberOfBuckets: 1
///         outputFormat: SequenceFileInputFormat
///         storedAsSubDirectories: false
///         parameters:
///           param1: param1_val
///         columns:
///           - name: my_column_1
///             type: int
///             comment: my_column1_comment
///           - name: my_column_2
///             type: string
///             comment: my_column2_comment
///         serDeInfo:
///           name: ser_de_name
///           parameters:
///             param1: param_val_1
///           serializationLibrary: org.apache.hadoop.hive.serde2.columnar.ColumnarSerDe
///         sortColumns:
///           - column: my_column_1
///             sortOrder: 1
///         skewedInfo:
///           skewedColumnNames:
///             - my_column_1
///           skewedColumnValueLocationMaps:
///             my_column_1: my_column_1_val_loc_map
///           skewedColumnValues:
///             - skewed_val_1
///       partitionKeys:
///         - name: my_column_1
///           type: int
///           comment: my_column_1_comment
///         - name: my_column_2
///           type: string
///           comment: my_column_2_comment
///       parameters:
///         param1: param1_val
/// ```
///
/// ## Import
///
/// Using `pulumi import`, import Glue ML Transforms using `id`. For example:
///
/// ```sh
/// $ pulumi import aws:glue/mLTransform:MLTransform example tfm-c2cafbe83b1c575f49eaca9939220e2fcd58e2d5
/// ```
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod ml_transform {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct MLTransformArgs {
        /// Description of the ML Transform.
        #[builder(into, default)]
        pub description: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The version of glue to use, for example "1.0". For information about available versions, see the [AWS Glue Release Notes](https://docs.aws.amazon.com/glue/latest/dg/release-notes.html).
        #[builder(into, default)]
        pub glue_version: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// A list of AWS Glue table definitions used by the transform. see Input Record Tables.
        #[builder(into)]
        pub input_record_tables: pulumi_gestalt_rust::InputOrOutput<
            Vec<super::super::types::glue::MlTransformInputRecordTable>,
        >,
        /// The number of AWS Glue data processing units (DPUs) that are allocated to task runs for this transform. You can allocate from `2` to `100` DPUs; the default is `10`. `max_capacity` is a mutually exclusive option with `number_of_workers` and `worker_type`.
        #[builder(into, default)]
        pub max_capacity: pulumi_gestalt_rust::InputOrOutput<Option<f64>>,
        /// The maximum number of times to retry this ML Transform if it fails.
        #[builder(into, default)]
        pub max_retries: pulumi_gestalt_rust::InputOrOutput<Option<i32>>,
        /// The name you assign to this ML Transform. It must be unique in your account.
        #[builder(into, default)]
        pub name: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The number of workers of a defined `worker_type` that are allocated when an ML Transform runs. Required with `worker_type`.
        #[builder(into, default)]
        pub number_of_workers: pulumi_gestalt_rust::InputOrOutput<Option<i32>>,
        /// The algorithmic parameters that are specific to the transform type used. Conditionally dependent on the transform type. see Parameters.
        #[builder(into)]
        pub parameters: pulumi_gestalt_rust::InputOrOutput<
            super::super::types::glue::MlTransformParameters,
        >,
        /// The ARN of the IAM role associated with this ML Transform.
        #[builder(into)]
        pub role_arn: pulumi_gestalt_rust::InputOrOutput<String>,
        /// Key-value map of resource tags. .If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
        #[builder(into, default)]
        pub tags: pulumi_gestalt_rust::InputOrOutput<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// The ML Transform timeout in minutes. The default is 2880 minutes (48 hours).
        #[builder(into, default)]
        pub timeout: pulumi_gestalt_rust::InputOrOutput<Option<i32>>,
        /// The type of predefined worker that is allocated when an ML Transform runs. Accepts a value of `Standard`, `G.1X`, or `G.2X`. Required with `number_of_workers`.
        #[builder(into, default)]
        pub worker_type: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
    }
    #[allow(dead_code)]
    pub struct MLTransformResult {
        /// Amazon Resource Name (ARN) of Glue ML Transform.
        pub arn: pulumi_gestalt_rust::Output<String>,
        /// Description of the ML Transform.
        pub description: pulumi_gestalt_rust::Output<Option<String>>,
        /// The version of glue to use, for example "1.0". For information about available versions, see the [AWS Glue Release Notes](https://docs.aws.amazon.com/glue/latest/dg/release-notes.html).
        pub glue_version: pulumi_gestalt_rust::Output<String>,
        /// A list of AWS Glue table definitions used by the transform. see Input Record Tables.
        pub input_record_tables: pulumi_gestalt_rust::Output<
            Vec<super::super::types::glue::MlTransformInputRecordTable>,
        >,
        /// The number of labels available for this transform.
        pub label_count: pulumi_gestalt_rust::Output<i32>,
        /// The number of AWS Glue data processing units (DPUs) that are allocated to task runs for this transform. You can allocate from `2` to `100` DPUs; the default is `10`. `max_capacity` is a mutually exclusive option with `number_of_workers` and `worker_type`.
        pub max_capacity: pulumi_gestalt_rust::Output<f64>,
        /// The maximum number of times to retry this ML Transform if it fails.
        pub max_retries: pulumi_gestalt_rust::Output<Option<i32>>,
        /// The name you assign to this ML Transform. It must be unique in your account.
        pub name: pulumi_gestalt_rust::Output<String>,
        /// The number of workers of a defined `worker_type` that are allocated when an ML Transform runs. Required with `worker_type`.
        pub number_of_workers: pulumi_gestalt_rust::Output<Option<i32>>,
        /// The algorithmic parameters that are specific to the transform type used. Conditionally dependent on the transform type. see Parameters.
        pub parameters: pulumi_gestalt_rust::Output<
            super::super::types::glue::MlTransformParameters,
        >,
        /// The ARN of the IAM role associated with this ML Transform.
        pub role_arn: pulumi_gestalt_rust::Output<String>,
        /// The object that represents the schema that this transform accepts. see Schema.
        pub schemas: pulumi_gestalt_rust::Output<
            Vec<super::super::types::glue::MlTransformSchema>,
        >,
        /// Key-value map of resource tags. .If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
        pub tags: pulumi_gestalt_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// A map of tags assigned to the resource, including those inherited from the provider `default_tags` configuration block.
        pub tags_all: pulumi_gestalt_rust::Output<
            std::collections::HashMap<String, String>,
        >,
        /// The ML Transform timeout in minutes. The default is 2880 minutes (48 hours).
        pub timeout: pulumi_gestalt_rust::Output<Option<i32>>,
        /// The type of predefined worker that is allocated when an ML Transform runs. Accepts a value of `Standard`, `G.1X`, or `G.2X`. Required with `number_of_workers`.
        pub worker_type: pulumi_gestalt_rust::Output<Option<String>>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: MLTransformArgs,
    ) -> MLTransformResult {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::register_interface;
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let description_binding = args.description.get_output(context);
        let glue_version_binding = args.glue_version.get_output(context);
        let input_record_tables_binding = args.input_record_tables.get_output(context);
        let max_capacity_binding = args.max_capacity.get_output(context);
        let max_retries_binding = args.max_retries.get_output(context);
        let name_binding = args.name.get_output(context);
        let number_of_workers_binding = args.number_of_workers.get_output(context);
        let parameters_binding = args.parameters.get_output(context);
        let role_arn_binding = args.role_arn.get_output(context);
        let tags_binding = args.tags.get_output(context);
        let timeout_binding = args.timeout.get_output(context);
        let worker_type_binding = args.worker_type.get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "aws:glue/mLTransform:MLTransform".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "description".into(),
                    value: description_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "glueVersion".into(),
                    value: glue_version_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "inputRecordTables".into(),
                    value: input_record_tables_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "maxCapacity".into(),
                    value: max_capacity_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "maxRetries".into(),
                    value: max_retries_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "name".into(),
                    value: name_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "numberOfWorkers".into(),
                    value: number_of_workers_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "parameters".into(),
                    value: parameters_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "roleArn".into(),
                    value: role_arn_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "tags".into(),
                    value: tags_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "timeout".into(),
                    value: timeout_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "workerType".into(),
                    value: worker_type_binding.get_id(),
                },
            ],
        };
        let o = context.register_resource(request);
        MLTransformResult {
            arn: o.get_field("arn"),
            description: o.get_field("description"),
            glue_version: o.get_field("glueVersion"),
            input_record_tables: o.get_field("inputRecordTables"),
            label_count: o.get_field("labelCount"),
            max_capacity: o.get_field("maxCapacity"),
            max_retries: o.get_field("maxRetries"),
            name: o.get_field("name"),
            number_of_workers: o.get_field("numberOfWorkers"),
            parameters: o.get_field("parameters"),
            role_arn: o.get_field("roleArn"),
            schemas: o.get_field("schemas"),
            tags: o.get_field("tags"),
            tags_all: o.get_field("tagsAll"),
            timeout: o.get_field("timeout"),
            worker_type: o.get_field("workerType"),
        }
    }
}
