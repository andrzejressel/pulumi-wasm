/// Jobs are actions that BigQuery runs on your behalf to load data, export data, query data, or copy data.
/// Once a BigQuery job is created, it cannot be changed or deleted.
///
///
/// To get more information about Job, see:
///
/// * [API documentation](https://cloud.google.com/bigquery/docs/reference/rest/v2/jobs)
/// * How-to Guides
///     * [BigQuery Jobs Intro](https://cloud.google.com/bigquery/docs/jobs-overview)
///
/// ## Example Usage
///
/// ### Bigquery Job Query
///
///
/// ```yaml
/// resources:
///   foo:
///     type: gcp:bigquery:Table
///     properties:
///       deletionProtection: false
///       datasetId: ${bar.datasetId}
///       tableId: job_query_table
///   bar:
///     type: gcp:bigquery:Dataset
///     properties:
///       datasetId: job_query_dataset
///       friendlyName: test
///       description: This is a test description
///       location: US
///   job:
///     type: gcp:bigquery:Job
///     properties:
///       jobId: job_query
///       labels:
///         example-label: example-value
///       query:
///         query: SELECT state FROM [lookerdata:cdc.project_tycho_reports]
///         destinationTable:
///           projectId: ${foo.project}
///           datasetId: ${foo.datasetId}
///           tableId: ${foo.tableId}
///         allowLargeResults: true
///         flattenResults: true
///         scriptOptions:
///           keyResultStatement: LAST
/// ```
/// ### Bigquery Job Query Table Reference
///
///
/// ```yaml
/// resources:
///   foo:
///     type: gcp:bigquery:Table
///     properties:
///       deletionProtection: false
///       datasetId: ${bar.datasetId}
///       tableId: job_query_table
///   bar:
///     type: gcp:bigquery:Dataset
///     properties:
///       datasetId: job_query_dataset
///       friendlyName: test
///       description: This is a test description
///       location: US
///   job:
///     type: gcp:bigquery:Job
///     properties:
///       jobId: job_query
///       labels:
///         example-label: example-value
///       query:
///         query: SELECT state FROM [lookerdata:cdc.project_tycho_reports]
///         destinationTable:
///           tableId: ${foo.id}
///         defaultDataset:
///           datasetId: ${bar.id}
///         allowLargeResults: true
///         flattenResults: true
///         scriptOptions:
///           keyResultStatement: LAST
/// ```
/// ### Bigquery Job Load
///
///
/// ```yaml
/// resources:
///   foo:
///     type: gcp:bigquery:Table
///     properties:
///       deletionProtection: false
///       datasetId: ${bar.datasetId}
///       tableId: job_load_table
///   bar:
///     type: gcp:bigquery:Dataset
///     properties:
///       datasetId: job_load_dataset
///       friendlyName: test
///       description: This is a test description
///       location: US
///   job:
///     type: gcp:bigquery:Job
///     properties:
///       jobId: job_load
///       labels:
///         my_job: load
///       load:
///         sourceUris:
///           - gs://cloud-samples-data/bigquery/us-states/us-states-by-date.csv
///         destinationTable:
///           projectId: ${foo.project}
///           datasetId: ${foo.datasetId}
///           tableId: ${foo.tableId}
///         skipLeadingRows: 1
///         schemaUpdateOptions:
///           - ALLOW_FIELD_RELAXATION
///           - ALLOW_FIELD_ADDITION
///         writeDisposition: WRITE_APPEND
///         autodetect: true
/// ```
/// ### Bigquery Job Load Geojson
///
///
/// ```yaml
/// resources:
///   bucket:
///     type: gcp:storage:Bucket
///     properties:
///       name: ${project}-bq-geojson
///       location: US
///       uniformBucketLevelAccess: true
///   object:
///     type: gcp:storage:BucketObject
///     properties:
///       name: geojson-data.jsonl
///       bucket: ${bucket.name}
///       content: |
///         {"type":"Feature","properties":{"continent":"Europe","region":"Scandinavia"},"geometry":{"type":"Polygon","coordinates":[[[-30.94,53.33],[33.05,53.33],[33.05,71.86],[-30.94,71.86],[-30.94,53.33]]]}}
///         {"type":"Feature","properties":{"continent":"Africa","region":"West Africa"},"geometry":{"type":"Polygon","coordinates":[[[-23.91,0],[11.95,0],[11.95,18.98],[-23.91,18.98],[-23.91,0]]]}}
///   foo:
///     type: gcp:bigquery:Table
///     properties:
///       deletionProtection: false
///       datasetId: ${bar.datasetId}
///       tableId: job_load_table
///   bar:
///     type: gcp:bigquery:Dataset
///     properties:
///       datasetId: job_load_dataset
///       friendlyName: test
///       description: This is a test description
///       location: US
///   job:
///     type: gcp:bigquery:Job
///     properties:
///       jobId: job_load
///       labels:
///         my_job: load
///       load:
///         sourceUris:
///           - gs://${object.bucket}/${object.name}
///         destinationTable:
///           projectId: ${foo.project}
///           datasetId: ${foo.datasetId}
///           tableId: ${foo.tableId}
///         writeDisposition: WRITE_TRUNCATE
///         autodetect: true
///         sourceFormat: NEWLINE_DELIMITED_JSON
///         jsonExtension: GEOJSON
///     options:
///       dependsOn:
///         - ${object}
/// variables:
///   project: my-project-name
/// ```
/// ### Bigquery Job Load Parquet
///
///
/// ```yaml
/// resources:
///   test:
///     type: gcp:storage:Bucket
///     properties:
///       name: job_load_bucket
///       location: US
///       uniformBucketLevelAccess: true
///   testBucketObject:
///     type: gcp:storage:BucketObject
///     name: test
///     properties:
///       name: job_load_bucket_object
///       source:
///         fn::FileAsset: ./test-fixtures/test.parquet.gzip
///       bucket: ${test.name}
///   testDataset:
///     type: gcp:bigquery:Dataset
///     name: test
///     properties:
///       datasetId: job_load_dataset
///       friendlyName: test
///       description: This is a test description
///       location: US
///   testTable:
///     type: gcp:bigquery:Table
///     name: test
///     properties:
///       deletionProtection: false
///       tableId: job_load_table
///       datasetId: ${testDataset.datasetId}
///   job:
///     type: gcp:bigquery:Job
///     properties:
///       jobId: job_load
///       labels:
///         my_job: load
///       load:
///         sourceUris:
///           - gs://${testBucketObject.bucket}/${testBucketObject.name}
///         destinationTable:
///           projectId: ${testTable.project}
///           datasetId: ${testTable.datasetId}
///           tableId: ${testTable.tableId}
///         schemaUpdateOptions:
///           - ALLOW_FIELD_RELAXATION
///           - ALLOW_FIELD_ADDITION
///         writeDisposition: WRITE_APPEND
///         sourceFormat: PARQUET
///         autodetect: true
///         parquetOptions:
///           enumAsString: true
///           enableListInference: true
/// ```
/// ### Bigquery Job Copy
///
///
/// ### Bigquery Job Extract
///
///
/// ```yaml
/// resources:
///   source-one:
///     type: gcp:bigquery:Table
///     properties:
///       deletionProtection: false
///       datasetId: ${["source-oneDataset"].datasetId}
///       tableId: job_extract_table
///       schema: |
///         [
///           {
///             "name": "name",
///             "type": "STRING",
///             "mode": "NULLABLE"
///           },
///           {
///             "name": "post_abbr",
///             "type": "STRING",
///             "mode": "NULLABLE"
///           },
///           {
///             "name": "date",
///             "type": "DATE",
///             "mode": "NULLABLE"
///           }
///         ]
///   source-oneDataset:
///     type: gcp:bigquery:Dataset
///     name: source-one
///     properties:
///       datasetId: job_extract_dataset
///       friendlyName: test
///       description: This is a test description
///       location: US
///   dest:
///     type: gcp:storage:Bucket
///     properties:
///       name: job_extract_bucket
///       location: US
///       forceDestroy: true
///   job:
///     type: gcp:bigquery:Job
///     properties:
///       jobId: job_extract
///       extract:
///         destinationUris:
///           - ${dest.url}/extract
///         sourceTable:
///           projectId: ${["source-one"].project}
///           datasetId: ${["source-one"].datasetId}
///           tableId: ${["source-one"].tableId}
///         destinationFormat: NEWLINE_DELIMITED_JSON
///         compression: GZIP
/// ```
///
/// ## Import
///
/// Job can be imported using any of these accepted formats:
///
/// * `projects/{{project}}/jobs/{{job_id}}/location/{{location}}`
///
/// * `projects/{{project}}/jobs/{{job_id}}`
///
/// * `{{project}}/{{job_id}}/{{location}}`
///
/// * `{{job_id}}/{{location}}`
///
/// * `{{project}}/{{job_id}}`
///
/// * `{{job_id}}`
///
/// When using the `pulumi import` command, Job can be imported using one of the formats above. For example:
///
/// ```sh
/// $ pulumi import gcp:bigquery/job:Job default projects/{{project}}/jobs/{{job_id}}/location/{{location}}
/// ```
///
/// ```sh
/// $ pulumi import gcp:bigquery/job:Job default projects/{{project}}/jobs/{{job_id}}
/// ```
///
/// ```sh
/// $ pulumi import gcp:bigquery/job:Job default {{project}}/{{job_id}}/{{location}}
/// ```
///
/// ```sh
/// $ pulumi import gcp:bigquery/job:Job default {{job_id}}/{{location}}
/// ```
///
/// ```sh
/// $ pulumi import gcp:bigquery/job:Job default {{project}}/{{job_id}}
/// ```
///
/// ```sh
/// $ pulumi import gcp:bigquery/job:Job default {{job_id}}
/// ```
///
pub mod job {
    #[derive(pulumi_wasm_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct JobArgs {
        /// Copies a table.
        #[builder(into, default)]
        pub copy: pulumi_wasm_rust::InputOrOutput<
            Option<super::super::types::bigquery::JobCopy>,
        >,
        /// Configures an extract job.
        #[builder(into, default)]
        pub extract: pulumi_wasm_rust::InputOrOutput<
            Option<super::super::types::bigquery::JobExtract>,
        >,
        /// The ID of the job. The ID must contain only letters (a-z, A-Z), numbers (0-9), underscores (_), or dashes (-). The maximum length is 1,024 characters.
        #[builder(into)]
        pub job_id: pulumi_wasm_rust::InputOrOutput<String>,
        /// Job timeout in milliseconds. If this time limit is exceeded, BigQuery may attempt to terminate the job.
        #[builder(into, default)]
        pub job_timeout_ms: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        /// The labels associated with this job. You can use these to organize and group your jobs. **Note**: This field is
        /// non-authoritative, and will only manage the labels present in your configuration. Please refer to the field
        /// 'effective_labels' for all of the labels present on the resource.
        #[builder(into, default)]
        pub labels: pulumi_wasm_rust::InputOrOutput<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// Configures a load job.
        #[builder(into, default)]
        pub load: pulumi_wasm_rust::InputOrOutput<
            Option<super::super::types::bigquery::JobLoad>,
        >,
        /// Specifies where the error occurred, if present.
        #[builder(into, default)]
        pub location: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        #[builder(into, default)]
        pub project: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        /// Configures a query job.
        #[builder(into, default)]
        pub query: pulumi_wasm_rust::InputOrOutput<
            Option<super::super::types::bigquery::JobQuery>,
        >,
    }
    #[allow(dead_code)]
    pub struct JobResult {
        /// Copies a table.
        pub copy: pulumi_wasm_rust::Output<
            Option<super::super::types::bigquery::JobCopy>,
        >,
        /// (Output)
        /// All of labels (key/value pairs) present on the resource in GCP, including the labels configured through Pulumi, other clients and services.
        pub effective_labels: pulumi_wasm_rust::Output<
            std::collections::HashMap<String, String>,
        >,
        /// Configures an extract job.
        pub extract: pulumi_wasm_rust::Output<
            Option<super::super::types::bigquery::JobExtract>,
        >,
        /// The ID of the job. The ID must contain only letters (a-z, A-Z), numbers (0-9), underscores (_), or dashes (-). The maximum length is 1,024 characters.
        pub job_id: pulumi_wasm_rust::Output<String>,
        /// Job timeout in milliseconds. If this time limit is exceeded, BigQuery may attempt to terminate the job.
        pub job_timeout_ms: pulumi_wasm_rust::Output<Option<String>>,
        /// (Output)
        /// The type of the job.
        pub job_type: pulumi_wasm_rust::Output<String>,
        /// The labels associated with this job. You can use these to organize and group your jobs. **Note**: This field is
        /// non-authoritative, and will only manage the labels present in your configuration. Please refer to the field
        /// 'effective_labels' for all of the labels present on the resource.
        pub labels: pulumi_wasm_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// Configures a load job.
        pub load: pulumi_wasm_rust::Output<
            Option<super::super::types::bigquery::JobLoad>,
        >,
        /// Specifies where the error occurred, if present.
        pub location: pulumi_wasm_rust::Output<Option<String>>,
        pub project: pulumi_wasm_rust::Output<String>,
        /// (Output)
        /// The combination of labels configured directly on the resource
        /// and default labels configured on the provider.
        pub pulumi_labels: pulumi_wasm_rust::Output<
            std::collections::HashMap<String, String>,
        >,
        /// Configures a query job.
        pub query: pulumi_wasm_rust::Output<
            Option<super::super::types::bigquery::JobQuery>,
        >,
        /// The status of this job. Examine this value when polling an asynchronous job to see if the job is complete.
        /// Structure is documented below.
        pub statuses: pulumi_wasm_rust::Output<
            Vec<super::super::types::bigquery::JobStatus>,
        >,
        /// Email address of the user who ran the job.
        pub user_email: pulumi_wasm_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_wasm_rust::PulumiContext,
        name: &str,
        args: JobArgs,
    ) -> JobResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let copy_binding = args.copy.get_output(context).get_inner();
        let extract_binding = args.extract.get_output(context).get_inner();
        let job_id_binding = args.job_id.get_output(context).get_inner();
        let job_timeout_ms_binding = args.job_timeout_ms.get_output(context).get_inner();
        let labels_binding = args.labels.get_output(context).get_inner();
        let load_binding = args.load.get_output(context).get_inner();
        let location_binding = args.location.get_output(context).get_inner();
        let project_binding = args.project.get_output(context).get_inner();
        let query_binding = args.query.get_output(context).get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "gcp:bigquery/job:Job".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "copy".into(),
                    value: &copy_binding,
                },
                register_interface::ObjectField {
                    name: "extract".into(),
                    value: &extract_binding,
                },
                register_interface::ObjectField {
                    name: "jobId".into(),
                    value: &job_id_binding,
                },
                register_interface::ObjectField {
                    name: "jobTimeoutMs".into(),
                    value: &job_timeout_ms_binding,
                },
                register_interface::ObjectField {
                    name: "labels".into(),
                    value: &labels_binding,
                },
                register_interface::ObjectField {
                    name: "load".into(),
                    value: &load_binding,
                },
                register_interface::ObjectField {
                    name: "location".into(),
                    value: &location_binding,
                },
                register_interface::ObjectField {
                    name: "project".into(),
                    value: &project_binding,
                },
                register_interface::ObjectField {
                    name: "query".into(),
                    value: &query_binding,
                },
            ]),
        };
        let o = register_interface::register(context.get_inner(), &request);
        JobResult {
            copy: pulumi_wasm_rust::__private::into_domain(o.extract_field("copy")),
            effective_labels: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("effectiveLabels"),
            ),
            extract: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("extract"),
            ),
            job_id: pulumi_wasm_rust::__private::into_domain(o.extract_field("jobId")),
            job_timeout_ms: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("jobTimeoutMs"),
            ),
            job_type: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("jobType"),
            ),
            labels: pulumi_wasm_rust::__private::into_domain(o.extract_field("labels")),
            load: pulumi_wasm_rust::__private::into_domain(o.extract_field("load")),
            location: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("location"),
            ),
            project: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("project"),
            ),
            pulumi_labels: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("pulumiLabels"),
            ),
            query: pulumi_wasm_rust::__private::into_domain(o.extract_field("query")),
            statuses: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("statuses"),
            ),
            user_email: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("userEmail"),
            ),
        }
    }
}
