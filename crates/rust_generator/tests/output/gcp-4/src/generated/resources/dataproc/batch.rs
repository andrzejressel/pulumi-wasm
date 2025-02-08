/// Dataproc Serverless Batches lets you run Spark workloads without requiring you to
/// provision and manage your own Dataproc cluster.
///
///
/// To get more information about Batch, see:
///
/// * [API documentation](https://cloud.google.com/dataproc-serverless/docs/reference/rest/v1/projects.locations.batches)
/// * How-to Guides
///     * [Dataproc Serverless Batches Intro](https://cloud.google.com/dataproc-serverless/docs/overview)
///
/// ## Example Usage
///
/// ### Dataproc Batch Spark
///
///
/// ```yaml
/// resources:
///   exampleBatchSpark:
///     type: gcp:dataproc:Batch
///     name: example_batch_spark
///     properties:
///       batchId: tf-test-batch_75125
///       location: us-central1
///       labels:
///         batch_test: terraform
///       runtimeConfig:
///         properties:
///           spark.dynamicAllocation.enabled: 'false'
///           spark.executor.instances: '2'
///       environmentConfig:
///         executionConfig:
///           subnetworkUri: default
///           ttl: 3600s
///           networkTags:
///             - tag1
///       sparkBatch:
///         mainClass: org.apache.spark.examples.SparkPi
///         args:
///           - '10'
///         jarFileUris:
///           - file:///usr/lib/spark/examples/jars/spark-examples.jar
/// ```
/// ### Dataproc Batch Spark Full
///
///
/// ```yaml
/// resources:
///   exampleBatchSpark:
///     type: gcp:dataproc:Batch
///     name: example_batch_spark
///     properties:
///       batchId: dataproc-batch
///       location: us-central1
///       labels:
///         batch_test: terraform
///       runtimeConfig:
///         properties:
///           spark.dynamicAllocation.enabled: 'false'
///           spark.executor.instances: '2'
///         version: '2.2'
///       environmentConfig:
///         executionConfig:
///           ttl: 3600s
///           networkTags:
///             - tag1
///           kmsKey: ${cryptoKey.id}
///           networkUri: default
///           serviceAccount: ${project.number}-compute@developer.gserviceaccount.com
///           stagingBucket: ${bucket.name}
///         peripheralsConfig:
///           metastoreService: ${ms.name}
///           sparkHistoryServerConfig:
///             dataprocCluster: ${basic.id}
///       sparkBatch:
///         mainClass: org.apache.spark.examples.SparkPi
///         args:
///           - '10'
///         jarFileUris:
///           - file:///usr/lib/spark/examples/jars/spark-examples.jar
///     options:
///       dependsOn:
///         - ${cryptoKeyMember1}
///   bucket:
///     type: gcp:storage:Bucket
///     properties:
///       uniformBucketLevelAccess: true
///       name: dataproc-bucket
///       location: US
///       forceDestroy: true
///   cryptoKey:
///     type: gcp:kms:CryptoKey
///     name: crypto_key
///     properties:
///       name: example-key
///       keyRing: ${keyRing.id}
///       purpose: ENCRYPT_DECRYPT
///   keyRing:
///     type: gcp:kms:KeyRing
///     name: key_ring
///     properties:
///       name: example-keyring
///       location: us-central1
///   cryptoKeyMember1:
///     type: gcp:kms:CryptoKeyIAMMember
///     name: crypto_key_member_1
///     properties:
///       cryptoKeyId: ${cryptoKey.id}
///       role: roles/cloudkms.cryptoKeyEncrypterDecrypter
///       member: serviceAccount:service-${project.number}@dataproc-accounts.iam.gserviceaccount.com
///   basic:
///     type: gcp:dataproc:Cluster
///     properties:
///       name: dataproc-batch
///       region: us-central1
///       clusterConfig:
///         softwareConfig:
///           overrideProperties:
///             dataproc:dataproc.allow.zero.workers: 'true'
///             spark:spark.history.fs.logDirectory: gs://${bucket.name}/*/spark-job-history
///         endpointConfig:
///           enableHttpPortAccess: true
///         masterConfig:
///           numInstances: 1
///           machineType: e2-standard-2
///           diskConfig:
///             bootDiskSizeGb: 35
///         metastoreConfig:
///           dataprocMetastoreService: ${ms.name}
///   ms:
///     type: gcp:dataproc:MetastoreService
///     properties:
///       serviceId: dataproc-batch
///       location: us-central1
///       port: 9080
///       tier: DEVELOPER
///       maintenanceWindow:
///         hourOfDay: 2
///         dayOfWeek: SUNDAY
///       hiveMetastoreConfig:
///         version: 3.1.2
/// variables:
///   project:
///     fn::invoke:
///       function: gcp:organizations:getProject
///       arguments: {}
///   gcsAccount:
///     fn::invoke:
///       function: gcp:storage:getProjectServiceAccount
///       arguments: {}
/// ```
/// ### Dataproc Batch Sparksql
///
///
/// ```yaml
/// resources:
///   exampleBatchSparsql:
///     type: gcp:dataproc:Batch
///     name: example_batch_sparsql
///     properties:
///       batchId: tf-test-batch_88722
///       location: us-central1
///       runtimeConfig:
///         properties:
///           spark.dynamicAllocation.enabled: 'false'
///           spark.executor.instances: '2'
///       environmentConfig:
///         executionConfig:
///           subnetworkUri: default
///       sparkSqlBatch:
///         queryFileUri: gs://dataproc-examples/spark-sql/natality/cigarette_correlations.sql
///         jarFileUris:
///           - file:///usr/lib/spark/examples/jars/spark-examples.jar
///         queryVariables:
///           name: value
/// ```
/// ### Dataproc Batch Pyspark
///
///
/// ```yaml
/// resources:
///   exampleBatchPyspark:
///     type: gcp:dataproc:Batch
///     name: example_batch_pyspark
///     properties:
///       batchId: tf-test-batch_39249
///       location: us-central1
///       runtimeConfig:
///         properties:
///           spark.dynamicAllocation.enabled: 'false'
///           spark.executor.instances: '2'
///       environmentConfig:
///         executionConfig:
///           subnetworkUri: default
///       pysparkBatch:
///         mainPythonFileUri: https://storage.googleapis.com/terraform-batches/test_util.py
///         args:
///           - '10'
///         jarFileUris:
///           - file:///usr/lib/spark/examples/jars/spark-examples.jar
///         pythonFileUris:
///           - gs://dataproc-examples/pyspark/hello-world/hello-world.py
///         archiveUris:
///           - https://storage.googleapis.com/terraform-batches/animals.txt.tar.gz#unpacked
///           - https://storage.googleapis.com/terraform-batches/animals.txt.jar
///           - https://storage.googleapis.com/terraform-batches/animals.txt
///         fileUris:
///           - https://storage.googleapis.com/terraform-batches/people.txt
/// ```
/// ### Dataproc Batch Sparkr
///
///
/// ```yaml
/// resources:
///   exampleBatchSparkr:
///     type: gcp:dataproc:Batch
///     name: example_batch_sparkr
///     properties:
///       batchId: tf-test-batch_74391
///       location: us-central1
///       labels:
///         batch_test: terraform
///       runtimeConfig:
///         properties:
///           spark.dynamicAllocation.enabled: 'false'
///           spark.executor.instances: '2'
///       environmentConfig:
///         executionConfig:
///           subnetworkUri: default
///           ttl: 3600s
///           networkTags:
///             - tag1
///       sparkRBatch:
///         mainRFileUri: https://storage.googleapis.com/terraform-batches/spark-r-flights.r
///         args:
///           - https://storage.googleapis.com/terraform-batches/flights.csv
/// ```
/// ### Dataproc Batch Autotuning
///
///
/// ```yaml
/// resources:
///   exampleBatchAutotuning:
///     type: gcp:dataproc:Batch
///     name: example_batch_autotuning
///     properties:
///       batchId: tf-test-batch_16511
///       location: us-central1
///       labels:
///         batch_test: terraform
///       runtimeConfig:
///         version: '2.2'
///         properties:
///           spark.dynamicAllocation.enabled: 'false'
///           spark.executor.instances: '2'
///         cohort: tf-dataproc-batch-example
///         autotuningConfig:
///           scenarios:
///             - SCALING
///             - MEMORY
///       environmentConfig:
///         executionConfig:
///           subnetworkUri: default
///           ttl: 3600s
///       sparkBatch:
///         mainClass: org.apache.spark.examples.SparkPi
///         args:
///           - '10'
///         jarFileUris:
///           - file:///usr/lib/spark/examples/jars/spark-examples.jar
/// ```
///
/// ## Import
///
/// Batch can be imported using any of these accepted formats:
///
/// * `projects/{{project}}/locations/{{location}}/batches/{{batch_id}}`
///
/// * `{{project}}/{{location}}/{{batch_id}}`
///
/// * `{{location}}/{{batch_id}}`
///
/// When using the `pulumi import` command, Batch can be imported using one of the formats above. For example:
///
/// ```sh
/// $ pulumi import gcp:dataproc/batch:Batch default projects/{{project}}/locations/{{location}}/batches/{{batch_id}}
/// ```
///
/// ```sh
/// $ pulumi import gcp:dataproc/batch:Batch default {{project}}/{{location}}/{{batch_id}}
/// ```
///
/// ```sh
/// $ pulumi import gcp:dataproc/batch:Batch default {{location}}/{{batch_id}}
/// ```
///
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod batch {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct BatchArgs {
        /// The ID to use for the batch, which will become the final component of the batch's resource name.
        /// This value must be 4-63 characters. Valid characters are /[a-z][0-9]-/.
        #[builder(into, default)]
        pub batch_id: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Environment configuration for the batch execution.
        /// Structure is documented below.
        #[builder(into, default)]
        pub environment_config: pulumi_gestalt_rust::InputOrOutput<
            Option<super::super::types::dataproc::BatchEnvironmentConfig>,
        >,
        /// The labels to associate with this batch.
        ///
        /// **Note**: This field is non-authoritative, and will only manage the labels present in your configuration.
        /// Please refer to the field `effective_labels` for all of the labels present on the resource.
        #[builder(into, default)]
        pub labels: pulumi_gestalt_rust::InputOrOutput<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// The location in which the batch will be created in.
        #[builder(into, default)]
        pub location: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The ID of the project in which the resource belongs.
        /// If it is not provided, the provider project is used.
        #[builder(into, default)]
        pub project: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// PySpark batch config.
        /// Structure is documented below.
        #[builder(into, default)]
        pub pyspark_batch: pulumi_gestalt_rust::InputOrOutput<
            Option<super::super::types::dataproc::BatchPysparkBatch>,
        >,
        /// Runtime configuration for the batch execution.
        /// Structure is documented below.
        #[builder(into, default)]
        pub runtime_config: pulumi_gestalt_rust::InputOrOutput<
            Option<super::super::types::dataproc::BatchRuntimeConfig>,
        >,
        /// Spark batch config.
        /// Structure is documented below.
        #[builder(into, default)]
        pub spark_batch: pulumi_gestalt_rust::InputOrOutput<
            Option<super::super::types::dataproc::BatchSparkBatch>,
        >,
        /// SparkR batch config.
        /// Structure is documented below.
        #[builder(into, default)]
        pub spark_r_batch: pulumi_gestalt_rust::InputOrOutput<
            Option<super::super::types::dataproc::BatchSparkRBatch>,
        >,
        /// Spark SQL batch config.
        /// Structure is documented below.
        #[builder(into, default)]
        pub spark_sql_batch: pulumi_gestalt_rust::InputOrOutput<
            Option<super::super::types::dataproc::BatchSparkSqlBatch>,
        >,
    }
    #[allow(dead_code)]
    pub struct BatchResult {
        /// The ID to use for the batch, which will become the final component of the batch's resource name.
        /// This value must be 4-63 characters. Valid characters are /[a-z][0-9]-/.
        pub batch_id: pulumi_gestalt_rust::Output<Option<String>>,
        /// The time when the batch was created.
        pub create_time: pulumi_gestalt_rust::Output<String>,
        /// The email address of the user who created the batch.
        pub creator: pulumi_gestalt_rust::Output<String>,
        /// All of labels (key/value pairs) present on the resource in GCP, including the labels configured through Pulumi, other clients and services.
        pub effective_labels: pulumi_gestalt_rust::Output<
            std::collections::HashMap<String, String>,
        >,
        /// Environment configuration for the batch execution.
        /// Structure is documented below.
        pub environment_config: pulumi_gestalt_rust::Output<
            Option<super::super::types::dataproc::BatchEnvironmentConfig>,
        >,
        /// The labels to associate with this batch.
        ///
        /// **Note**: This field is non-authoritative, and will only manage the labels present in your configuration.
        /// Please refer to the field `effective_labels` for all of the labels present on the resource.
        pub labels: pulumi_gestalt_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// The location in which the batch will be created in.
        pub location: pulumi_gestalt_rust::Output<Option<String>>,
        /// The resource name of the batch.
        pub name: pulumi_gestalt_rust::Output<String>,
        /// The resource name of the operation associated with this batch.
        pub operation: pulumi_gestalt_rust::Output<String>,
        /// The ID of the project in which the resource belongs.
        /// If it is not provided, the provider project is used.
        pub project: pulumi_gestalt_rust::Output<String>,
        /// The combination of labels configured directly on the resource
        /// and default labels configured on the provider.
        pub pulumi_labels: pulumi_gestalt_rust::Output<
            std::collections::HashMap<String, String>,
        >,
        /// PySpark batch config.
        /// Structure is documented below.
        pub pyspark_batch: pulumi_gestalt_rust::Output<
            Option<super::super::types::dataproc::BatchPysparkBatch>,
        >,
        /// Runtime configuration for the batch execution.
        /// Structure is documented below.
        pub runtime_config: pulumi_gestalt_rust::Output<
            Option<super::super::types::dataproc::BatchRuntimeConfig>,
        >,
        /// Runtime information about batch execution.
        /// Structure is documented below.
        pub runtime_infos: pulumi_gestalt_rust::Output<
            Vec<super::super::types::dataproc::BatchRuntimeInfo>,
        >,
        /// Spark batch config.
        /// Structure is documented below.
        pub spark_batch: pulumi_gestalt_rust::Output<
            Option<super::super::types::dataproc::BatchSparkBatch>,
        >,
        /// SparkR batch config.
        /// Structure is documented below.
        pub spark_r_batch: pulumi_gestalt_rust::Output<
            Option<super::super::types::dataproc::BatchSparkRBatch>,
        >,
        /// Spark SQL batch config.
        /// Structure is documented below.
        pub spark_sql_batch: pulumi_gestalt_rust::Output<
            Option<super::super::types::dataproc::BatchSparkSqlBatch>,
        >,
        /// (Output)
        /// The state of the batch at this point in history. For possible values, see the [API documentation](https://cloud.google.com/dataproc-serverless/docs/reference/rest/v1/projects.locations.batches#State).
        pub state: pulumi_gestalt_rust::Output<String>,
        /// Historical state information for the batch.
        /// Structure is documented below.
        pub state_histories: pulumi_gestalt_rust::Output<
            Vec<super::super::types::dataproc::BatchStateHistory>,
        >,
        /// (Output)
        /// Details about the state at this point in history.
        pub state_message: pulumi_gestalt_rust::Output<String>,
        /// Batch state details, such as a failure description if the state is FAILED.
        pub state_time: pulumi_gestalt_rust::Output<String>,
        /// A batch UUID (Unique Universal Identifier). The service generates this value when it creates the batch.
        pub uuid: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::PulumiContext,
        name: &str,
        args: BatchArgs,
    ) -> BatchResult {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::register_interface;
        use std::collections::HashMap;
        let batch_id_binding = args.batch_id.get_output(context).get_inner();
        let environment_config_binding = args
            .environment_config
            .get_output(context)
            .get_inner();
        let labels_binding = args.labels.get_output(context).get_inner();
        let location_binding = args.location.get_output(context).get_inner();
        let project_binding = args.project.get_output(context).get_inner();
        let pyspark_batch_binding = args.pyspark_batch.get_output(context).get_inner();
        let runtime_config_binding = args.runtime_config.get_output(context).get_inner();
        let spark_batch_binding = args.spark_batch.get_output(context).get_inner();
        let spark_r_batch_binding = args.spark_r_batch.get_output(context).get_inner();
        let spark_sql_batch_binding = args
            .spark_sql_batch
            .get_output(context)
            .get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "gcp:dataproc/batch:Batch".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "batchId".into(),
                    value: &batch_id_binding,
                },
                register_interface::ObjectField {
                    name: "environmentConfig".into(),
                    value: &environment_config_binding,
                },
                register_interface::ObjectField {
                    name: "labels".into(),
                    value: &labels_binding,
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
                    name: "pysparkBatch".into(),
                    value: &pyspark_batch_binding,
                },
                register_interface::ObjectField {
                    name: "runtimeConfig".into(),
                    value: &runtime_config_binding,
                },
                register_interface::ObjectField {
                    name: "sparkBatch".into(),
                    value: &spark_batch_binding,
                },
                register_interface::ObjectField {
                    name: "sparkRBatch".into(),
                    value: &spark_r_batch_binding,
                },
                register_interface::ObjectField {
                    name: "sparkSqlBatch".into(),
                    value: &spark_sql_batch_binding,
                },
            ]),
        };
        let o = register_interface::register(context.get_inner(), &request);
        BatchResult {
            batch_id: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("batchId"),
            ),
            create_time: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("createTime"),
            ),
            creator: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("creator"),
            ),
            effective_labels: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("effectiveLabels"),
            ),
            environment_config: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("environmentConfig"),
            ),
            labels: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("labels"),
            ),
            location: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("location"),
            ),
            name: pulumi_gestalt_rust::__private::into_domain(o.extract_field("name")),
            operation: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("operation"),
            ),
            project: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("project"),
            ),
            pulumi_labels: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("pulumiLabels"),
            ),
            pyspark_batch: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("pysparkBatch"),
            ),
            runtime_config: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("runtimeConfig"),
            ),
            runtime_infos: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("runtimeInfos"),
            ),
            spark_batch: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("sparkBatch"),
            ),
            spark_r_batch: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("sparkRBatch"),
            ),
            spark_sql_batch: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("sparkSqlBatch"),
            ),
            state: pulumi_gestalt_rust::__private::into_domain(o.extract_field("state")),
            state_histories: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("stateHistories"),
            ),
            state_message: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("stateMessage"),
            ),
            state_time: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("stateTime"),
            ),
            uuid: pulumi_gestalt_rust::__private::into_domain(o.extract_field("uuid")),
        }
    }
}
