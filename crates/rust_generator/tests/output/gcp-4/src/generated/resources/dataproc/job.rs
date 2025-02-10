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
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod job {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct JobArgs {
        /// By default, you can only delete inactive jobs within
        /// Dataproc. Setting this to true, and calling destroy, will ensure that the
        /// job is first cancelled before issuing the delete.
        #[builder(into, default)]
        pub force_delete: pulumi_gestalt_rust::InputOrOutput<Option<bool>>,
        /// The config of Hadoop job
        #[builder(into, default)]
        pub hadoop_config: pulumi_gestalt_rust::InputOrOutput<
            Option<super::super::types::dataproc::JobHadoopConfig>,
        >,
        /// The config of hive job
        #[builder(into, default)]
        pub hive_config: pulumi_gestalt_rust::InputOrOutput<
            Option<super::super::types::dataproc::JobHiveConfig>,
        >,
        /// The list of labels (key/value pairs) to add to the job.
        /// **Note**: This field is non-authoritative, and will only manage the labels present in your configuration.
        /// Please refer to the field 'effective_labels' for all of the labels present on the resource.
        #[builder(into, default)]
        pub labels: pulumi_gestalt_rust::InputOrOutput<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// The config of pag job.
        #[builder(into, default)]
        pub pig_config: pulumi_gestalt_rust::InputOrOutput<
            Option<super::super::types::dataproc::JobPigConfig>,
        >,
        /// The config of job placement.
        #[builder(into)]
        pub placement: pulumi_gestalt_rust::InputOrOutput<
            super::super::types::dataproc::JobPlacement,
        >,
        /// The config of presto job
        #[builder(into, default)]
        pub presto_config: pulumi_gestalt_rust::InputOrOutput<
            Option<super::super::types::dataproc::JobPrestoConfig>,
        >,
        /// The project in which the `cluster` can be found and jobs
        /// subsequently run against. If it is not provided, the provider project is used.
        #[builder(into, default)]
        pub project: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The config of pySpark job.
        #[builder(into, default)]
        pub pyspark_config: pulumi_gestalt_rust::InputOrOutput<
            Option<super::super::types::dataproc::JobPysparkConfig>,
        >,
        /// The reference of the job
        #[builder(into, default)]
        pub reference: pulumi_gestalt_rust::InputOrOutput<
            Option<super::super::types::dataproc::JobReference>,
        >,
        /// The Cloud Dataproc region. This essentially determines which clusters are available
        /// for this job to be submitted to. If not specified, defaults to `global`.
        #[builder(into, default)]
        pub region: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Optional. Job scheduling configuration.
        #[builder(into, default)]
        pub scheduling: pulumi_gestalt_rust::InputOrOutput<
            Option<super::super::types::dataproc::JobScheduling>,
        >,
        /// The config of the Spark job.
        #[builder(into, default)]
        pub spark_config: pulumi_gestalt_rust::InputOrOutput<
            Option<super::super::types::dataproc::JobSparkConfig>,
        >,
        /// The config of SparkSql job
        #[builder(into, default)]
        pub sparksql_config: pulumi_gestalt_rust::InputOrOutput<
            Option<super::super::types::dataproc::JobSparksqlConfig>,
        >,
    }
    #[allow(dead_code)]
    pub struct JobResult {
        /// If present, the location of miscellaneous control files which may be used as part of job setup and handling. If not present, control files may be placed in the same location as driver_output_uri.
        pub driver_controls_files_uri: pulumi_gestalt_rust::Output<String>,
        /// A URI pointing to the location of the stdout of the job's driver program.
        pub driver_output_resource_uri: pulumi_gestalt_rust::Output<String>,
        /// All of labels (key/value pairs) present on the resource in GCP, including the labels configured through Pulumi, other clients and services.
        ///
        /// * `scheduling.max_failures_per_hour` - (Required) Maximum number of times per hour a driver may be restarted as a result of driver exiting with non-zero code before job is reported failed.
        ///
        /// * `scheduling.max_failures_total` - (Required) Maximum number of times in total a driver may be restarted as a result of driver exiting with non-zero code before job is reported failed.
        pub effective_labels: pulumi_gestalt_rust::Output<
            std::collections::HashMap<String, String>,
        >,
        /// By default, you can only delete inactive jobs within
        /// Dataproc. Setting this to true, and calling destroy, will ensure that the
        /// job is first cancelled before issuing the delete.
        pub force_delete: pulumi_gestalt_rust::Output<Option<bool>>,
        /// The config of Hadoop job
        pub hadoop_config: pulumi_gestalt_rust::Output<
            Option<super::super::types::dataproc::JobHadoopConfig>,
        >,
        /// The config of hive job
        pub hive_config: pulumi_gestalt_rust::Output<
            Option<super::super::types::dataproc::JobHiveConfig>,
        >,
        /// The list of labels (key/value pairs) to add to the job.
        /// **Note**: This field is non-authoritative, and will only manage the labels present in your configuration.
        /// Please refer to the field 'effective_labels' for all of the labels present on the resource.
        pub labels: pulumi_gestalt_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// The config of pag job.
        pub pig_config: pulumi_gestalt_rust::Output<
            Option<super::super::types::dataproc::JobPigConfig>,
        >,
        /// The config of job placement.
        pub placement: pulumi_gestalt_rust::Output<
            super::super::types::dataproc::JobPlacement,
        >,
        /// The config of presto job
        pub presto_config: pulumi_gestalt_rust::Output<
            Option<super::super::types::dataproc::JobPrestoConfig>,
        >,
        /// The project in which the `cluster` can be found and jobs
        /// subsequently run against. If it is not provided, the provider project is used.
        pub project: pulumi_gestalt_rust::Output<String>,
        /// The combination of labels configured directly on the resource and default labels configured on the provider.
        pub pulumi_labels: pulumi_gestalt_rust::Output<
            std::collections::HashMap<String, String>,
        >,
        /// The config of pySpark job.
        pub pyspark_config: pulumi_gestalt_rust::Output<
            Option<super::super::types::dataproc::JobPysparkConfig>,
        >,
        /// The reference of the job
        pub reference: pulumi_gestalt_rust::Output<
            super::super::types::dataproc::JobReference,
        >,
        /// The Cloud Dataproc region. This essentially determines which clusters are available
        /// for this job to be submitted to. If not specified, defaults to `global`.
        pub region: pulumi_gestalt_rust::Output<Option<String>>,
        /// Optional. Job scheduling configuration.
        pub scheduling: pulumi_gestalt_rust::Output<
            Option<super::super::types::dataproc::JobScheduling>,
        >,
        /// The config of the Spark job.
        pub spark_config: pulumi_gestalt_rust::Output<
            Option<super::super::types::dataproc::JobSparkConfig>,
        >,
        /// The config of SparkSql job
        pub sparksql_config: pulumi_gestalt_rust::Output<
            Option<super::super::types::dataproc::JobSparksqlConfig>,
        >,
        /// The status of the job.
        pub statuses: pulumi_gestalt_rust::Output<
            Vec<super::super::types::dataproc::JobStatus>,
        >,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: JobArgs,
    ) -> JobResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let force_delete_binding = args.force_delete.get_output(context);
        let hadoop_config_binding = args.hadoop_config.get_output(context);
        let hive_config_binding = args.hive_config.get_output(context);
        let labels_binding = args.labels.get_output(context);
        let pig_config_binding = args.pig_config.get_output(context);
        let placement_binding = args.placement.get_output(context);
        let presto_config_binding = args.presto_config.get_output(context);
        let project_binding = args.project.get_output(context);
        let pyspark_config_binding = args.pyspark_config.get_output(context);
        let reference_binding = args.reference.get_output(context);
        let region_binding = args.region.get_output(context);
        let scheduling_binding = args.scheduling.get_output(context);
        let spark_config_binding = args.spark_config.get_output(context);
        let sparksql_config_binding = args.sparksql_config.get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "gcp:dataproc/job:Job".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "forceDelete".into(),
                    value: force_delete_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "hadoopConfig".into(),
                    value: hadoop_config_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "hiveConfig".into(),
                    value: hive_config_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "labels".into(),
                    value: labels_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "pigConfig".into(),
                    value: pig_config_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "placement".into(),
                    value: placement_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "prestoConfig".into(),
                    value: presto_config_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "project".into(),
                    value: project_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "pysparkConfig".into(),
                    value: pyspark_config_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "reference".into(),
                    value: reference_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "region".into(),
                    value: region_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "scheduling".into(),
                    value: scheduling_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "sparkConfig".into(),
                    value: spark_config_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "sparksqlConfig".into(),
                    value: sparksql_config_binding.get_id(),
                },
            ],
        };
        let o = context.register_resource(request);
        JobResult {
            driver_controls_files_uri: o.get_field("driverControlsFilesUri"),
            driver_output_resource_uri: o.get_field("driverOutputResourceUri"),
            effective_labels: o.get_field("effectiveLabels"),
            force_delete: o.get_field("forceDelete"),
            hadoop_config: o.get_field("hadoopConfig"),
            hive_config: o.get_field("hiveConfig"),
            labels: o.get_field("labels"),
            pig_config: o.get_field("pigConfig"),
            placement: o.get_field("placement"),
            presto_config: o.get_field("prestoConfig"),
            project: o.get_field("project"),
            pulumi_labels: o.get_field("pulumiLabels"),
            pyspark_config: o.get_field("pysparkConfig"),
            reference: o.get_field("reference"),
            region: o.get_field("region"),
            scheduling: o.get_field("scheduling"),
            spark_config: o.get_field("sparkConfig"),
            sparksql_config: o.get_field("sparksqlConfig"),
            statuses: o.get_field("statuses"),
        }
    }
}
