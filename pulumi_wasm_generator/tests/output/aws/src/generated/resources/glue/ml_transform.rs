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
///       dependson:
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
pub mod ml_transform {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct MLTransformArgs {
        /// Description of the ML Transform.
        #[builder(into, default)]
        pub description: pulumi_wasm_rust::Output<Option<String>>,
        /// The version of glue to use, for example "1.0". For information about available versions, see the [AWS Glue Release Notes](https://docs.aws.amazon.com/glue/latest/dg/release-notes.html).
        #[builder(into, default)]
        pub glue_version: pulumi_wasm_rust::Output<Option<String>>,
        /// A list of AWS Glue table definitions used by the transform. see Input Record Tables.
        #[builder(into)]
        pub input_record_tables: pulumi_wasm_rust::Output<
            Vec<super::super::types::glue::MlTransformInputRecordTable>,
        >,
        /// The number of AWS Glue data processing units (DPUs) that are allocated to task runs for this transform. You can allocate from `2` to `100` DPUs; the default is `10`. `max_capacity` is a mutually exclusive option with `number_of_workers` and `worker_type`.
        #[builder(into, default)]
        pub max_capacity: pulumi_wasm_rust::Output<Option<f64>>,
        /// The maximum number of times to retry this ML Transform if it fails.
        #[builder(into, default)]
        pub max_retries: pulumi_wasm_rust::Output<Option<i32>>,
        /// The name you assign to this ML Transform. It must be unique in your account.
        #[builder(into, default)]
        pub name: pulumi_wasm_rust::Output<Option<String>>,
        /// The number of workers of a defined `worker_type` that are allocated when an ML Transform runs. Required with `worker_type`.
        #[builder(into, default)]
        pub number_of_workers: pulumi_wasm_rust::Output<Option<i32>>,
        /// The algorithmic parameters that are specific to the transform type used. Conditionally dependent on the transform type. see Parameters.
        #[builder(into)]
        pub parameters: pulumi_wasm_rust::Output<
            super::super::types::glue::MlTransformParameters,
        >,
        /// The ARN of the IAM role associated with this ML Transform.
        #[builder(into)]
        pub role_arn: pulumi_wasm_rust::Output<String>,
        /// Key-value map of resource tags. .If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
        #[builder(into, default)]
        pub tags: pulumi_wasm_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// The ML Transform timeout in minutes. The default is 2880 minutes (48 hours).
        #[builder(into, default)]
        pub timeout: pulumi_wasm_rust::Output<Option<i32>>,
        /// The type of predefined worker that is allocated when an ML Transform runs. Accepts a value of `Standard`, `G.1X`, or `G.2X`. Required with `number_of_workers`.
        #[builder(into, default)]
        pub worker_type: pulumi_wasm_rust::Output<Option<String>>,
    }
    #[allow(dead_code)]
    pub struct MLTransformResult {
        /// Amazon Resource Name (ARN) of Glue ML Transform.
        pub arn: pulumi_wasm_rust::Output<String>,
        /// Description of the ML Transform.
        pub description: pulumi_wasm_rust::Output<Option<String>>,
        /// The version of glue to use, for example "1.0". For information about available versions, see the [AWS Glue Release Notes](https://docs.aws.amazon.com/glue/latest/dg/release-notes.html).
        pub glue_version: pulumi_wasm_rust::Output<String>,
        /// A list of AWS Glue table definitions used by the transform. see Input Record Tables.
        pub input_record_tables: pulumi_wasm_rust::Output<
            Vec<super::super::types::glue::MlTransformInputRecordTable>,
        >,
        /// The number of labels available for this transform.
        pub label_count: pulumi_wasm_rust::Output<i32>,
        /// The number of AWS Glue data processing units (DPUs) that are allocated to task runs for this transform. You can allocate from `2` to `100` DPUs; the default is `10`. `max_capacity` is a mutually exclusive option with `number_of_workers` and `worker_type`.
        pub max_capacity: pulumi_wasm_rust::Output<f64>,
        /// The maximum number of times to retry this ML Transform if it fails.
        pub max_retries: pulumi_wasm_rust::Output<Option<i32>>,
        /// The name you assign to this ML Transform. It must be unique in your account.
        pub name: pulumi_wasm_rust::Output<String>,
        /// The number of workers of a defined `worker_type` that are allocated when an ML Transform runs. Required with `worker_type`.
        pub number_of_workers: pulumi_wasm_rust::Output<Option<i32>>,
        /// The algorithmic parameters that are specific to the transform type used. Conditionally dependent on the transform type. see Parameters.
        pub parameters: pulumi_wasm_rust::Output<
            super::super::types::glue::MlTransformParameters,
        >,
        /// The ARN of the IAM role associated with this ML Transform.
        pub role_arn: pulumi_wasm_rust::Output<String>,
        /// The object that represents the schema that this transform accepts. see Schema.
        pub schemas: pulumi_wasm_rust::Output<
            Vec<super::super::types::glue::MlTransformSchema>,
        >,
        /// Key-value map of resource tags. .If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
        pub tags: pulumi_wasm_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// A map of tags assigned to the resource, including those inherited from the provider `default_tags` configuration block.
        pub tags_all: pulumi_wasm_rust::Output<
            std::collections::HashMap<String, String>,
        >,
        /// The ML Transform timeout in minutes. The default is 2880 minutes (48 hours).
        pub timeout: pulumi_wasm_rust::Output<Option<i32>>,
        /// The type of predefined worker that is allocated when an ML Transform runs. Accepts a value of `Standard`, `G.1X`, or `G.2X`. Required with `number_of_workers`.
        pub worker_type: pulumi_wasm_rust::Output<Option<String>>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(name: &str, args: MLTransformArgs) -> MLTransformResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let description_binding = args.description.get_inner();
        let glue_version_binding = args.glue_version.get_inner();
        let input_record_tables_binding = args.input_record_tables.get_inner();
        let max_capacity_binding = args.max_capacity.get_inner();
        let max_retries_binding = args.max_retries.get_inner();
        let name_binding = args.name.get_inner();
        let number_of_workers_binding = args.number_of_workers.get_inner();
        let parameters_binding = args.parameters.get_inner();
        let role_arn_binding = args.role_arn.get_inner();
        let tags_binding = args.tags.get_inner();
        let timeout_binding = args.timeout.get_inner();
        let worker_type_binding = args.worker_type.get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "aws:glue/mLTransform:MLTransform".into(),
            name: name.to_string(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "description".into(),
                    value: &description_binding,
                },
                register_interface::ObjectField {
                    name: "glueVersion".into(),
                    value: &glue_version_binding,
                },
                register_interface::ObjectField {
                    name: "inputRecordTables".into(),
                    value: &input_record_tables_binding,
                },
                register_interface::ObjectField {
                    name: "maxCapacity".into(),
                    value: &max_capacity_binding,
                },
                register_interface::ObjectField {
                    name: "maxRetries".into(),
                    value: &max_retries_binding,
                },
                register_interface::ObjectField {
                    name: "name".into(),
                    value: &name_binding,
                },
                register_interface::ObjectField {
                    name: "numberOfWorkers".into(),
                    value: &number_of_workers_binding,
                },
                register_interface::ObjectField {
                    name: "parameters".into(),
                    value: &parameters_binding,
                },
                register_interface::ObjectField {
                    name: "roleArn".into(),
                    value: &role_arn_binding,
                },
                register_interface::ObjectField {
                    name: "tags".into(),
                    value: &tags_binding,
                },
                register_interface::ObjectField {
                    name: "timeout".into(),
                    value: &timeout_binding,
                },
                register_interface::ObjectField {
                    name: "workerType".into(),
                    value: &worker_type_binding,
                },
            ]),
            results: Vec::from([
                register_interface::ResultField {
                    name: "arn".into(),
                },
                register_interface::ResultField {
                    name: "description".into(),
                },
                register_interface::ResultField {
                    name: "glueVersion".into(),
                },
                register_interface::ResultField {
                    name: "inputRecordTables".into(),
                },
                register_interface::ResultField {
                    name: "labelCount".into(),
                },
                register_interface::ResultField {
                    name: "maxCapacity".into(),
                },
                register_interface::ResultField {
                    name: "maxRetries".into(),
                },
                register_interface::ResultField {
                    name: "name".into(),
                },
                register_interface::ResultField {
                    name: "numberOfWorkers".into(),
                },
                register_interface::ResultField {
                    name: "parameters".into(),
                },
                register_interface::ResultField {
                    name: "roleArn".into(),
                },
                register_interface::ResultField {
                    name: "schemas".into(),
                },
                register_interface::ResultField {
                    name: "tags".into(),
                },
                register_interface::ResultField {
                    name: "tagsAll".into(),
                },
                register_interface::ResultField {
                    name: "timeout".into(),
                },
                register_interface::ResultField {
                    name: "workerType".into(),
                },
            ]),
        };
        let o = register_interface::register(&request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        MLTransformResult {
            arn: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("arn").unwrap(),
            ),
            description: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("description").unwrap(),
            ),
            glue_version: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("glueVersion").unwrap(),
            ),
            input_record_tables: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("inputRecordTables").unwrap(),
            ),
            label_count: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("labelCount").unwrap(),
            ),
            max_capacity: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("maxCapacity").unwrap(),
            ),
            max_retries: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("maxRetries").unwrap(),
            ),
            name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("name").unwrap(),
            ),
            number_of_workers: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("numberOfWorkers").unwrap(),
            ),
            parameters: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("parameters").unwrap(),
            ),
            role_arn: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("roleArn").unwrap(),
            ),
            schemas: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("schemas").unwrap(),
            ),
            tags: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("tags").unwrap(),
            ),
            tags_all: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("tagsAll").unwrap(),
            ),
            timeout: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("timeout").unwrap(),
            ),
            worker_type: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("workerType").unwrap(),
            ),
        }
    }
}
