/// Manages a job resource within a Dataproc cluster within GCE. For more information see
/// [the official dataproc documentation](https://cloud.google.com/dataproc/).
///
/// !> **Note:** This resource does not support 'update' and changing any attributes will cause the resource to be recreated.
///
/// ## Example Usage
///
/// ```yaml
/// resources:
///   mycluster:
///     type: gcp:dataproc:Cluster
///     properties:
///       name: dproc-cluster-unique-name
///       region: us-central1
///   # Submit an example spark job to a dataproc cluster
///   spark:
///     type: gcp:dataproc:Job
///     properties:
///       region: ${mycluster.region}
///       forceDelete: true
///       placement:
///         clusterName: ${mycluster.name}
///       sparkConfig:
///         mainClass: org.apache.spark.examples.SparkPi
///         jarFileUris:
///           - file:///usr/lib/spark/examples/jars/spark-examples.jar
///         args:
///           - '1000'
///         properties:
///           spark.logConf: 'true'
///         loggingConfig:
///           driverLogLevels:
///             root: INFO
///   # Submit an example pyspark job to a dataproc cluster
///   pyspark:
///     type: gcp:dataproc:Job
///     properties:
///       region: ${mycluster.region}
///       forceDelete: true
///       placement:
///         clusterName: ${mycluster.name}
///       pysparkConfig:
///         mainPythonFileUri: gs://dataproc-examples-2f10d78d114f6aaec76462e3c310f31f/src/pyspark/hello-world/hello-world.py
///         properties:
///           spark.logConf: 'true'
/// outputs:
///   # Check out current state of the jobs
///   sparkStatus: ${spark.statuses[0].state}
///   pysparkStatus: ${pyspark.statuses[0].state}
/// ```
///
/// ## Import
///
/// This resource does not support import.
///
pub mod job {
    #[derive(pulumi_wasm_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct JobArgs {
        /// By default, you can only delete inactive jobs within
        /// Dataproc. Setting this to true, and calling destroy, will ensure that the
        /// job is first cancelled before issuing the delete.
        #[builder(into, default)]
        pub force_delete: pulumi_wasm_rust::InputOrOutput<Option<bool>>,
        /// The config of Hadoop job
        #[builder(into, default)]
        pub hadoop_config: pulumi_wasm_rust::InputOrOutput<
            Option<super::super::types::dataproc::JobHadoopConfig>,
        >,
        /// The config of hive job
        #[builder(into, default)]
        pub hive_config: pulumi_wasm_rust::InputOrOutput<
            Option<super::super::types::dataproc::JobHiveConfig>,
        >,
        /// The list of labels (key/value pairs) to add to the job.
        /// **Note**: This field is non-authoritative, and will only manage the labels present in your configuration.
        /// Please refer to the field 'effective_labels' for all of the labels present on the resource.
        #[builder(into, default)]
        pub labels: pulumi_wasm_rust::InputOrOutput<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// The config of pag job.
        #[builder(into, default)]
        pub pig_config: pulumi_wasm_rust::InputOrOutput<
            Option<super::super::types::dataproc::JobPigConfig>,
        >,
        /// The config of job placement.
        #[builder(into)]
        pub placement: pulumi_wasm_rust::InputOrOutput<
            super::super::types::dataproc::JobPlacement,
        >,
        /// The config of presto job
        #[builder(into, default)]
        pub presto_config: pulumi_wasm_rust::InputOrOutput<
            Option<super::super::types::dataproc::JobPrestoConfig>,
        >,
        /// The project in which the `cluster` can be found and jobs
        /// subsequently run against. If it is not provided, the provider project is used.
        #[builder(into, default)]
        pub project: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        /// The config of pySpark job.
        #[builder(into, default)]
        pub pyspark_config: pulumi_wasm_rust::InputOrOutput<
            Option<super::super::types::dataproc::JobPysparkConfig>,
        >,
        /// The reference of the job
        #[builder(into, default)]
        pub reference: pulumi_wasm_rust::InputOrOutput<
            Option<super::super::types::dataproc::JobReference>,
        >,
        /// The Cloud Dataproc region. This essentially determines which clusters are available
        /// for this job to be submitted to. If not specified, defaults to `global`.
        #[builder(into, default)]
        pub region: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        /// Optional. Job scheduling configuration.
        #[builder(into, default)]
        pub scheduling: pulumi_wasm_rust::InputOrOutput<
            Option<super::super::types::dataproc::JobScheduling>,
        >,
        /// The config of the Spark job.
        #[builder(into, default)]
        pub spark_config: pulumi_wasm_rust::InputOrOutput<
            Option<super::super::types::dataproc::JobSparkConfig>,
        >,
        /// The config of SparkSql job
        #[builder(into, default)]
        pub sparksql_config: pulumi_wasm_rust::InputOrOutput<
            Option<super::super::types::dataproc::JobSparksqlConfig>,
        >,
    }
    #[allow(dead_code)]
    pub struct JobResult {
        /// If present, the location of miscellaneous control files which may be used as part of job setup and handling. If not present, control files may be placed in the same location as driver_output_uri.
        pub driver_controls_files_uri: pulumi_wasm_rust::Output<String>,
        /// A URI pointing to the location of the stdout of the job's driver program.
        pub driver_output_resource_uri: pulumi_wasm_rust::Output<String>,
        /// All of labels (key/value pairs) present on the resource in GCP, including the labels configured through Pulumi, other clients and services.
        ///
        /// * `scheduling.max_failures_per_hour` - (Required) Maximum number of times per hour a driver may be restarted as a result of driver exiting with non-zero code before job is reported failed.
        ///
        /// * `scheduling.max_failures_total` - (Required) Maximum number of times in total a driver may be restarted as a result of driver exiting with non-zero code before job is reported failed.
        pub effective_labels: pulumi_wasm_rust::Output<
            std::collections::HashMap<String, String>,
        >,
        /// By default, you can only delete inactive jobs within
        /// Dataproc. Setting this to true, and calling destroy, will ensure that the
        /// job is first cancelled before issuing the delete.
        pub force_delete: pulumi_wasm_rust::Output<Option<bool>>,
        /// The config of Hadoop job
        pub hadoop_config: pulumi_wasm_rust::Output<
            Option<super::super::types::dataproc::JobHadoopConfig>,
        >,
        /// The config of hive job
        pub hive_config: pulumi_wasm_rust::Output<
            Option<super::super::types::dataproc::JobHiveConfig>,
        >,
        /// The list of labels (key/value pairs) to add to the job.
        /// **Note**: This field is non-authoritative, and will only manage the labels present in your configuration.
        /// Please refer to the field 'effective_labels' for all of the labels present on the resource.
        pub labels: pulumi_wasm_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// The config of pag job.
        pub pig_config: pulumi_wasm_rust::Output<
            Option<super::super::types::dataproc::JobPigConfig>,
        >,
        /// The config of job placement.
        pub placement: pulumi_wasm_rust::Output<
            super::super::types::dataproc::JobPlacement,
        >,
        /// The config of presto job
        pub presto_config: pulumi_wasm_rust::Output<
            Option<super::super::types::dataproc::JobPrestoConfig>,
        >,
        /// The project in which the `cluster` can be found and jobs
        /// subsequently run against. If it is not provided, the provider project is used.
        pub project: pulumi_wasm_rust::Output<String>,
        /// The combination of labels configured directly on the resource and default labels configured on the provider.
        pub pulumi_labels: pulumi_wasm_rust::Output<
            std::collections::HashMap<String, String>,
        >,
        /// The config of pySpark job.
        pub pyspark_config: pulumi_wasm_rust::Output<
            Option<super::super::types::dataproc::JobPysparkConfig>,
        >,
        /// The reference of the job
        pub reference: pulumi_wasm_rust::Output<
            super::super::types::dataproc::JobReference,
        >,
        /// The Cloud Dataproc region. This essentially determines which clusters are available
        /// for this job to be submitted to. If not specified, defaults to `global`.
        pub region: pulumi_wasm_rust::Output<Option<String>>,
        /// Optional. Job scheduling configuration.
        pub scheduling: pulumi_wasm_rust::Output<
            Option<super::super::types::dataproc::JobScheduling>,
        >,
        /// The config of the Spark job.
        pub spark_config: pulumi_wasm_rust::Output<
            Option<super::super::types::dataproc::JobSparkConfig>,
        >,
        /// The config of SparkSql job
        pub sparksql_config: pulumi_wasm_rust::Output<
            Option<super::super::types::dataproc::JobSparksqlConfig>,
        >,
        /// The status of the job.
        pub statuses: pulumi_wasm_rust::Output<
            Vec<super::super::types::dataproc::JobStatus>,
        >,
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
        use pulumi_wasm_rust::__private::pulumi_gestalt_adapter_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let force_delete_binding = args.force_delete.get_output(context).get_inner();
        let hadoop_config_binding = args.hadoop_config.get_output(context).get_inner();
        let hive_config_binding = args.hive_config.get_output(context).get_inner();
        let labels_binding = args.labels.get_output(context).get_inner();
        let pig_config_binding = args.pig_config.get_output(context).get_inner();
        let placement_binding = args.placement.get_output(context).get_inner();
        let presto_config_binding = args.presto_config.get_output(context).get_inner();
        let project_binding = args.project.get_output(context).get_inner();
        let pyspark_config_binding = args.pyspark_config.get_output(context).get_inner();
        let reference_binding = args.reference.get_output(context).get_inner();
        let region_binding = args.region.get_output(context).get_inner();
        let scheduling_binding = args.scheduling.get_output(context).get_inner();
        let spark_config_binding = args.spark_config.get_output(context).get_inner();
        let sparksql_config_binding = args
            .sparksql_config
            .get_output(context)
            .get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "gcp:dataproc/job:Job".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "forceDelete".into(),
                    value: &force_delete_binding,
                },
                register_interface::ObjectField {
                    name: "hadoopConfig".into(),
                    value: &hadoop_config_binding,
                },
                register_interface::ObjectField {
                    name: "hiveConfig".into(),
                    value: &hive_config_binding,
                },
                register_interface::ObjectField {
                    name: "labels".into(),
                    value: &labels_binding,
                },
                register_interface::ObjectField {
                    name: "pigConfig".into(),
                    value: &pig_config_binding,
                },
                register_interface::ObjectField {
                    name: "placement".into(),
                    value: &placement_binding,
                },
                register_interface::ObjectField {
                    name: "prestoConfig".into(),
                    value: &presto_config_binding,
                },
                register_interface::ObjectField {
                    name: "project".into(),
                    value: &project_binding,
                },
                register_interface::ObjectField {
                    name: "pysparkConfig".into(),
                    value: &pyspark_config_binding,
                },
                register_interface::ObjectField {
                    name: "reference".into(),
                    value: &reference_binding,
                },
                register_interface::ObjectField {
                    name: "region".into(),
                    value: &region_binding,
                },
                register_interface::ObjectField {
                    name: "scheduling".into(),
                    value: &scheduling_binding,
                },
                register_interface::ObjectField {
                    name: "sparkConfig".into(),
                    value: &spark_config_binding,
                },
                register_interface::ObjectField {
                    name: "sparksqlConfig".into(),
                    value: &sparksql_config_binding,
                },
            ]),
        };
        let o = register_interface::register(context.get_inner(), &request);
        JobResult {
            driver_controls_files_uri: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("driverControlsFilesUri"),
            ),
            driver_output_resource_uri: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("driverOutputResourceUri"),
            ),
            effective_labels: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("effectiveLabels"),
            ),
            force_delete: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("forceDelete"),
            ),
            hadoop_config: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("hadoopConfig"),
            ),
            hive_config: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("hiveConfig"),
            ),
            labels: pulumi_wasm_rust::__private::into_domain(o.extract_field("labels")),
            pig_config: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("pigConfig"),
            ),
            placement: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("placement"),
            ),
            presto_config: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("prestoConfig"),
            ),
            project: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("project"),
            ),
            pulumi_labels: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("pulumiLabels"),
            ),
            pyspark_config: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("pysparkConfig"),
            ),
            reference: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("reference"),
            ),
            region: pulumi_wasm_rust::__private::into_domain(o.extract_field("region")),
            scheduling: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("scheduling"),
            ),
            spark_config: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("sparkConfig"),
            ),
            sparksql_config: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("sparksqlConfig"),
            ),
            statuses: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("statuses"),
            ),
        }
    }
}
