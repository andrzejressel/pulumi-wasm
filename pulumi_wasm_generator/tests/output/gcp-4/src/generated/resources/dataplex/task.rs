/// A Dataplex task represents the work that you want Dataplex to do on a schedule. It encapsulates code, parameters, and the schedule.
///
///
/// To get more information about Task, see:
///
/// * [API documentation](https://cloud.google.com/dataplex/docs/reference/rest/v1/projects.locations.lakes.tasks)
/// * How-to Guides
///     * [Official Documentation](https://cloud.google.com/dataplex/docs)
///
/// ## Example Usage
///
/// ### Dataplex Task Basic
///
///
/// ```yaml
/// resources:
///   example:
///     type: gcp:dataplex:Lake
///     properties:
///       name: tf-test-lake_91042
///       location: us-central1
///       project: my-project-name
///   exampleTask:
///     type: gcp:dataplex:Task
///     name: example
///     properties:
///       taskId: tf-test-task_72490
///       location: us-central1
///       lake: ${example.name}
///       description: Test Task Basic
///       displayName: task-basic
///       labels:
///         count: '3'
///       triggerSpec:
///         type: RECURRING
///         disabled: false
///         maxRetries: 3
///         startTime: 2023-10-02T15:01:23Z
///         schedule: 1 * * * *
///       executionSpec:
///         serviceAccount: ${project.number}-compute@developer.gserviceaccount.com
///         project: my-project-name
///         maxJobExecutionLifetime: 100s
///         kmsKey: 234jn2kjn42k3n423
///       spark:
///         pythonScriptFile: gs://dataproc-examples/pyspark/hello-world/hello-world.py
///       project: my-project-name
/// variables:
///   project:
///     fn::invoke:
///       function: gcp:organizations:getProject
///       arguments: {}
/// ```
/// ### Dataplex Task Spark
///
///
/// ```yaml
/// resources:
///   # VPC network
///   default:
///     type: gcp:compute:Network
///     properties:
///       name: tf-test-workstation-cluster_89605
///       autoCreateSubnetworks: true
///   exampleSpark:
///     type: gcp:dataplex:Lake
///     name: example_spark
///     properties:
///       name: tf-test-lake_56730
///       location: us-central1
///       project: my-project-name
///   exampleSparkTask:
///     type: gcp:dataplex:Task
///     name: example_spark
///     properties:
///       taskId: tf-test-task_95154
///       location: us-central1
///       lake: ${exampleSpark.name}
///       triggerSpec:
///         type: ON_DEMAND
///       description: task-spark-terraform
///       executionSpec:
///         serviceAccount: ${project.number}-compute@developer.gserviceaccount.com
///         args:
///           TASK_ARGS: --output_location,gs://spark-job/task-result, --output_format, json
///       spark:
///         infrastructureSpec:
///           batch:
///             executorsCount: 2
///             maxExecutorsCount: 100
///           containerImage:
///             image: test-image
///             javaJars:
///               - test-java-jars.jar
///             pythonPackages:
///               - gs://bucket-name/my/path/to/lib.tar.gz
///             properties:
///               name: wrench
///               mass: 1.3kg
///               count: '3'
///           vpcNetwork:
///             networkTags:
///               - test-network-tag
///             subNetwork: ${default.id}
///         fileUris:
///           - gs://terrafrom-test/test.csv
///         archiveUris:
///           - gs://terraform-test/test.csv
///         sqlScript: show databases
///       project: my-project-name
/// variables:
///   project:
///     fn::invoke:
///       function: gcp:organizations:getProject
///       arguments: {}
/// ```
/// ### Dataplex Task Notebook
///
///
/// ```yaml
/// resources:
///   # VPC network
///   default:
///     type: gcp:compute:Network
///     properties:
///       name: tf-test-workstation-cluster_64336
///       autoCreateSubnetworks: true
///   exampleNotebook:
///     type: gcp:dataplex:Lake
///     name: example_notebook
///     properties:
///       name: tf-test-lake_34962
///       location: us-central1
///       project: my-project-name
///   exampleNotebookTask:
///     type: gcp:dataplex:Task
///     name: example_notebook
///     properties:
///       taskId: tf-test-task_74000
///       location: us-central1
///       lake: ${exampleNotebook.name}
///       triggerSpec:
///         type: RECURRING
///         schedule: 1 * * * *
///       executionSpec:
///         serviceAccount: ${project.number}-compute@developer.gserviceaccount.com
///         args:
///           TASK_ARGS: --output_location,gs://spark-job-jars-anrajitha/task-result, --output_format, json
///       notebook:
///         notebook: gs://terraform-test/test-notebook.ipynb
///         infrastructureSpec:
///           batch:
///             executorsCount: 2
///             maxExecutorsCount: 100
///           containerImage:
///             image: test-image
///             javaJars:
///               - test-java-jars.jar
///             pythonPackages:
///               - gs://bucket-name/my/path/to/lib.tar.gz
///             properties:
///               name: wrench
///               mass: 1.3kg
///               count: '3'
///           vpcNetwork:
///             networkTags:
///               - test-network-tag
///             network: ${default.id}
///         fileUris:
///           - gs://terraform-test/test.csv
///         archiveUris:
///           - gs://terraform-test/test.csv
///       project: my-project-name
/// variables:
///   project:
///     fn::invoke:
///       function: gcp:organizations:getProject
///       arguments: {}
/// ```
///
/// ## Import
///
/// Task can be imported using any of these accepted formats:
///
/// * `projects/{{project}}/locations/{{location}}/lakes/{{lake}}/tasks/{{task_id}}`
///
/// * `{{project}}/{{location}}/{{lake}}/{{task_id}}`
///
/// * `{{location}}/{{lake}}/{{task_id}}`
///
/// When using the `pulumi import` command, Task can be imported using one of the formats above. For example:
///
/// ```sh
/// $ pulumi import gcp:dataplex/task:Task default projects/{{project}}/locations/{{location}}/lakes/{{lake}}/tasks/{{task_id}}
/// ```
///
/// ```sh
/// $ pulumi import gcp:dataplex/task:Task default {{project}}/{{location}}/{{lake}}/{{task_id}}
/// ```
///
/// ```sh
/// $ pulumi import gcp:dataplex/task:Task default {{location}}/{{lake}}/{{task_id}}
/// ```
///
pub mod task {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct TaskArgs {
        /// User-provided description of the task.
        #[builder(into, default)]
        pub description: pulumi_wasm_rust::Output<Option<String>>,
        /// User friendly display name.
        #[builder(into, default)]
        pub display_name: pulumi_wasm_rust::Output<Option<String>>,
        /// Configuration for the cluster
        /// Structure is documented below.
        #[builder(into)]
        pub execution_spec: pulumi_wasm_rust::Output<
            super::super::types::dataplex::TaskExecutionSpec,
        >,
        /// User-defined labels for the task. **Note**: This field is non-authoritative, and will only manage the labels present in
        /// your configuration. Please refer to the field 'effective_labels' for all of the labels present on the resource.
        #[builder(into, default)]
        pub labels: pulumi_wasm_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// The lake in which the task will be created in.
        #[builder(into, default)]
        pub lake: pulumi_wasm_rust::Output<Option<String>>,
        /// The location in which the task will be created in.
        #[builder(into, default)]
        pub location: pulumi_wasm_rust::Output<Option<String>>,
        /// A service with manual scaling runs continuously, allowing you to perform complex initialization and rely on the state of
        /// its memory over time.
        #[builder(into, default)]
        pub notebook: pulumi_wasm_rust::Output<
            Option<super::super::types::dataplex::TaskNotebook>,
        >,
        #[builder(into, default)]
        pub project: pulumi_wasm_rust::Output<Option<String>>,
        /// A service with manual scaling runs continuously, allowing you to perform complex initialization and rely on the state of
        /// its memory over time.
        #[builder(into, default)]
        pub spark: pulumi_wasm_rust::Output<
            Option<super::super::types::dataplex::TaskSpark>,
        >,
        /// The task Id of the task.
        #[builder(into, default)]
        pub task_id: pulumi_wasm_rust::Output<Option<String>>,
        /// Configuration for the cluster
        /// Structure is documented below.
        #[builder(into)]
        pub trigger_spec: pulumi_wasm_rust::Output<
            super::super::types::dataplex::TaskTriggerSpec,
        >,
    }
    #[allow(dead_code)]
    pub struct TaskResult {
        /// The time when the task was created.
        pub create_time: pulumi_wasm_rust::Output<String>,
        /// User-provided description of the task.
        pub description: pulumi_wasm_rust::Output<Option<String>>,
        /// User friendly display name.
        pub display_name: pulumi_wasm_rust::Output<Option<String>>,
        /// All of labels (key/value pairs) present on the resource in GCP, including the labels configured through Pulumi, other clients and services.
        pub effective_labels: pulumi_wasm_rust::Output<
            std::collections::HashMap<String, String>,
        >,
        /// Configuration for the cluster
        /// Structure is documented below.
        pub execution_spec: pulumi_wasm_rust::Output<
            super::super::types::dataplex::TaskExecutionSpec,
        >,
        /// Configuration for the cluster
        /// Structure is documented below.
        pub execution_statuses: pulumi_wasm_rust::Output<
            Vec<super::super::types::dataplex::TaskExecutionStatus>,
        >,
        /// User-defined labels for the task. **Note**: This field is non-authoritative, and will only manage the labels present in
        /// your configuration. Please refer to the field 'effective_labels' for all of the labels present on the resource.
        pub labels: pulumi_wasm_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// The lake in which the task will be created in.
        pub lake: pulumi_wasm_rust::Output<Option<String>>,
        /// The location in which the task will be created in.
        pub location: pulumi_wasm_rust::Output<Option<String>>,
        /// (Output)
        /// The relative resource name of the job, of the form: projects/{project_number}/locations/{locationId}/lakes/{lakeId}/tasks/{taskId}/jobs/{jobId}.
        pub name: pulumi_wasm_rust::Output<String>,
        /// A service with manual scaling runs continuously, allowing you to perform complex initialization and rely on the state of
        /// its memory over time.
        pub notebook: pulumi_wasm_rust::Output<
            Option<super::super::types::dataplex::TaskNotebook>,
        >,
        pub project: pulumi_wasm_rust::Output<String>,
        /// The combination of labels configured directly on the resource
        /// and default labels configured on the provider.
        pub pulumi_labels: pulumi_wasm_rust::Output<
            std::collections::HashMap<String, String>,
        >,
        /// A service with manual scaling runs continuously, allowing you to perform complex initialization and rely on the state of
        /// its memory over time.
        pub spark: pulumi_wasm_rust::Output<
            Option<super::super::types::dataplex::TaskSpark>,
        >,
        /// (Output)
        /// Execution state for the job.
        pub state: pulumi_wasm_rust::Output<String>,
        /// The task Id of the task.
        pub task_id: pulumi_wasm_rust::Output<Option<String>>,
        /// Configuration for the cluster
        /// Structure is documented below.
        pub trigger_spec: pulumi_wasm_rust::Output<
            super::super::types::dataplex::TaskTriggerSpec,
        >,
        /// (Output)
        /// System generated globally unique ID for the job.
        pub uid: pulumi_wasm_rust::Output<String>,
        /// (Output)
        /// Last update time of the status.
        pub update_time: pulumi_wasm_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(name: &str, args: TaskArgs) -> TaskResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let description_binding = args.description.get_inner();
        let display_name_binding = args.display_name.get_inner();
        let execution_spec_binding = args.execution_spec.get_inner();
        let labels_binding = args.labels.get_inner();
        let lake_binding = args.lake.get_inner();
        let location_binding = args.location.get_inner();
        let notebook_binding = args.notebook.get_inner();
        let project_binding = args.project.get_inner();
        let spark_binding = args.spark.get_inner();
        let task_id_binding = args.task_id.get_inner();
        let trigger_spec_binding = args.trigger_spec.get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "gcp:dataplex/task:Task".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "description".into(),
                    value: &description_binding,
                },
                register_interface::ObjectField {
                    name: "displayName".into(),
                    value: &display_name_binding,
                },
                register_interface::ObjectField {
                    name: "executionSpec".into(),
                    value: &execution_spec_binding,
                },
                register_interface::ObjectField {
                    name: "labels".into(),
                    value: &labels_binding,
                },
                register_interface::ObjectField {
                    name: "lake".into(),
                    value: &lake_binding,
                },
                register_interface::ObjectField {
                    name: "location".into(),
                    value: &location_binding,
                },
                register_interface::ObjectField {
                    name: "notebook".into(),
                    value: &notebook_binding,
                },
                register_interface::ObjectField {
                    name: "project".into(),
                    value: &project_binding,
                },
                register_interface::ObjectField {
                    name: "spark".into(),
                    value: &spark_binding,
                },
                register_interface::ObjectField {
                    name: "taskId".into(),
                    value: &task_id_binding,
                },
                register_interface::ObjectField {
                    name: "triggerSpec".into(),
                    value: &trigger_spec_binding,
                },
            ]),
            results: Vec::from([
                register_interface::ResultField {
                    name: "createTime".into(),
                },
                register_interface::ResultField {
                    name: "description".into(),
                },
                register_interface::ResultField {
                    name: "displayName".into(),
                },
                register_interface::ResultField {
                    name: "effectiveLabels".into(),
                },
                register_interface::ResultField {
                    name: "executionSpec".into(),
                },
                register_interface::ResultField {
                    name: "executionStatuses".into(),
                },
                register_interface::ResultField {
                    name: "labels".into(),
                },
                register_interface::ResultField {
                    name: "lake".into(),
                },
                register_interface::ResultField {
                    name: "location".into(),
                },
                register_interface::ResultField {
                    name: "name".into(),
                },
                register_interface::ResultField {
                    name: "notebook".into(),
                },
                register_interface::ResultField {
                    name: "project".into(),
                },
                register_interface::ResultField {
                    name: "pulumiLabels".into(),
                },
                register_interface::ResultField {
                    name: "spark".into(),
                },
                register_interface::ResultField {
                    name: "state".into(),
                },
                register_interface::ResultField {
                    name: "taskId".into(),
                },
                register_interface::ResultField {
                    name: "triggerSpec".into(),
                },
                register_interface::ResultField {
                    name: "uid".into(),
                },
                register_interface::ResultField {
                    name: "updateTime".into(),
                },
            ]),
        };
        let o = register_interface::register(&request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        TaskResult {
            create_time: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("createTime").unwrap(),
            ),
            description: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("description").unwrap(),
            ),
            display_name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("displayName").unwrap(),
            ),
            effective_labels: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("effectiveLabels").unwrap(),
            ),
            execution_spec: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("executionSpec").unwrap(),
            ),
            execution_statuses: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("executionStatuses").unwrap(),
            ),
            labels: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("labels").unwrap(),
            ),
            lake: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("lake").unwrap(),
            ),
            location: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("location").unwrap(),
            ),
            name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("name").unwrap(),
            ),
            notebook: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("notebook").unwrap(),
            ),
            project: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("project").unwrap(),
            ),
            pulumi_labels: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("pulumiLabels").unwrap(),
            ),
            spark: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("spark").unwrap(),
            ),
            state: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("state").unwrap(),
            ),
            task_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("taskId").unwrap(),
            ),
            trigger_spec: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("triggerSpec").unwrap(),
            ),
            uid: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("uid").unwrap(),
            ),
            update_time: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("updateTime").unwrap(),
            ),
        }
    }
}
