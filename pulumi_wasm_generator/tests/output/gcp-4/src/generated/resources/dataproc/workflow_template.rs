/// A Workflow Template is a reusable workflow configuration. It defines a graph of jobs with information on where to run those jobs.
///
/// ## Example Usage
///
/// ```ignore
/// use pulumi_wasm_rust::Output;
/// use pulumi_wasm_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let template = workflow_template::create(
///         "template",
///         WorkflowTemplateArgs::builder()
///             .jobs(
///                 vec![
///                     WorkflowTemplateJob::builder()
///                     .sparkJob(WorkflowTemplateJobSparkJob::builder()
///                     .mainClass("SomeClass").build_struct()).stepId("someJob")
///                     .build_struct(), WorkflowTemplateJob::builder()
///                     .prerequisiteStepIds(vec!["someJob",])
///                     .prestoJob(WorkflowTemplateJobPrestoJob::builder()
///                     .queryFileUri("someuri").build_struct()).stepId("otherJob")
///                     .build_struct(),
///                 ],
///             )
///             .location("us-central1")
///             .name("template-example")
///             .placement(
///                 WorkflowTemplatePlacement::builder()
///                     .managedCluster(
///                         WorkflowTemplatePlacementManagedCluster::builder()
///                             .clusterName("my-cluster")
///                             .config(
///                                 WorkflowTemplatePlacementManagedClusterConfig::builder()
///                                     .gceClusterConfig(
///                                         WorkflowTemplatePlacementManagedClusterConfigGceClusterConfig::builder()
///                                             .tags(vec!["foo", "bar",])
///                                             .zone("us-central1-a")
///                                             .build_struct(),
///                                     )
///                                     .masterConfig(
///                                         WorkflowTemplatePlacementManagedClusterConfigMasterConfig::builder()
///                                             .diskConfig(
///                                                 WorkflowTemplatePlacementManagedClusterConfigMasterConfigDiskConfig::builder()
///                                                     .bootDiskSizeGb(15)
///                                                     .bootDiskType("pd-ssd")
///                                                     .build_struct(),
///                                             )
///                                             .machineType("n1-standard-1")
///                                             .numInstances(1)
///                                             .build_struct(),
///                                     )
///                                     .secondaryWorkerConfig(
///                                         WorkflowTemplatePlacementManagedClusterConfigSecondaryWorkerConfig::builder()
///                                             .numInstances(2)
///                                             .build_struct(),
///                                     )
///                                     .softwareConfig(
///                                         WorkflowTemplatePlacementManagedClusterConfigSoftwareConfig::builder()
///                                             .imageVersion("2.0.35-debian10")
///                                             .build_struct(),
///                                     )
///                                     .workerConfig(
///                                         WorkflowTemplatePlacementManagedClusterConfigWorkerConfig::builder()
///                                             .diskConfig(
///                                                 WorkflowTemplatePlacementManagedClusterConfigWorkerConfigDiskConfig::builder()
///                                                     .bootDiskSizeGb(10)
///                                                     .numLocalSsds(2)
///                                                     .build_struct(),
///                                             )
///                                             .machineType("n1-standard-2")
///                                             .numInstances(3)
///                                             .build_struct(),
///                                     )
///                                     .build_struct(),
///                             )
///                             .build_struct(),
///                     )
///                     .build_struct(),
///             )
///             .build_struct(),
///     );
/// }
/// ```
///
/// ## Import
///
/// WorkflowTemplate can be imported using any of these accepted formats:
///
/// * `projects/{{project}}/locations/{{location}}/workflowTemplates/{{name}}`
///
/// * `{{project}}/{{location}}/{{name}}`
///
/// * `{{location}}/{{name}}`
///
/// When using the `pulumi import` command, WorkflowTemplate can be imported using one of the formats above. For example:
///
/// ```sh
/// $ pulumi import gcp:dataproc/workflowTemplate:WorkflowTemplate default projects/{{project}}/locations/{{location}}/workflowTemplates/{{name}}
/// ```
///
/// ```sh
/// $ pulumi import gcp:dataproc/workflowTemplate:WorkflowTemplate default {{project}}/{{location}}/{{name}}
/// ```
///
/// ```sh
/// $ pulumi import gcp:dataproc/workflowTemplate:WorkflowTemplate default {{location}}/{{name}}
/// ```
///
pub mod workflow_template {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct WorkflowTemplateArgs {
        /// Optional. Timeout duration for the DAG of jobs, expressed in seconds (see [JSON representation of
        /// duration](https://developers.google.com/protocol-buffers/docs/proto3#json)). The timeout duration must be from 10
        /// minutes ("600s") to 24 hours ("86400s"). The timer begins when the first job is submitted. If the workflow is running at
        /// the end of the timeout period, any remaining jobs are cancelled, the workflow is ended, and if the workflow was running
        /// on a [managed
        /// cluster](https://www.terraform.io/dataproc/docs/concepts/workflows/using-workflows#configuring_or_selecting_a_cluster),
        /// the cluster is deleted.
        #[builder(into, default)]
        pub dag_timeout: pulumi_wasm_rust::Output<Option<String>>,
        /// Required. The Directed Acyclic Graph of Jobs to submit.
        #[builder(into)]
        pub jobs: pulumi_wasm_rust::Output<
            Vec<super::super::types::dataproc::WorkflowTemplateJob>,
        >,
        /// Optional. The labels to associate with this template. These labels will be propagated to all jobs and clusters created
        /// by the workflow instance. Label **keys** must contain 1 to 63 characters, and must conform to [RFC
        /// 1035](https://www.ietf.org/rfc/rfc1035.txt). Label **values** may be empty, but, if present, must contain 1 to 63
        /// characters, and must conform to [RFC 1035](https://www.ietf.org/rfc/rfc1035.txt). No more than 32 labels can be
        /// associated with a template. **Note**: This field is non-authoritative, and will only manage the labels present in your
        /// configuration. Please refer to the field `effective_labels` for all of the labels present on the resource.
        #[builder(into, default)]
        pub labels: pulumi_wasm_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// The location for the resource
        #[builder(into)]
        pub location: pulumi_wasm_rust::Output<String>,
        /// Output only. The resource name of the workflow template, as described in https://cloud.google.com/apis/design/resource_names. * For `projects.regions.workflowTemplates`, the resource name of the template has the following format: `projects/{project_id}/regions/{region}/workflowTemplates/{template_id}` * For `projects.locations.workflowTemplates`, the resource name of the template has the following format: `projects/{project_id}/locations/{location}/workflowTemplates/{template_id}`
        #[builder(into, default)]
        pub name: pulumi_wasm_rust::Output<Option<String>>,
        /// Optional. Template parameters whose values are substituted into the template. Values for parameters must be provided
        /// when the template is instantiated.
        #[builder(into, default)]
        pub parameters: pulumi_wasm_rust::Output<
            Option<Vec<super::super::types::dataproc::WorkflowTemplateParameter>>,
        >,
        /// Required. WorkflowTemplate scheduling information.
        #[builder(into)]
        pub placement: pulumi_wasm_rust::Output<
            super::super::types::dataproc::WorkflowTemplatePlacement,
        >,
        /// The project for the resource
        #[builder(into, default)]
        pub project: pulumi_wasm_rust::Output<Option<String>>,
        /// Output only. The current version of this workflow template.
        #[builder(into, default)]
        pub version: pulumi_wasm_rust::Output<Option<i32>>,
    }
    #[allow(dead_code)]
    pub struct WorkflowTemplateResult {
        /// Output only. The time template was created.
        pub create_time: pulumi_wasm_rust::Output<String>,
        /// Optional. Timeout duration for the DAG of jobs, expressed in seconds (see [JSON representation of
        /// duration](https://developers.google.com/protocol-buffers/docs/proto3#json)). The timeout duration must be from 10
        /// minutes ("600s") to 24 hours ("86400s"). The timer begins when the first job is submitted. If the workflow is running at
        /// the end of the timeout period, any remaining jobs are cancelled, the workflow is ended, and if the workflow was running
        /// on a [managed
        /// cluster](https://www.terraform.io/dataproc/docs/concepts/workflows/using-workflows#configuring_or_selecting_a_cluster),
        /// the cluster is deleted.
        pub dag_timeout: pulumi_wasm_rust::Output<Option<String>>,
        pub effective_labels: pulumi_wasm_rust::Output<
            std::collections::HashMap<String, String>,
        >,
        /// Required. The Directed Acyclic Graph of Jobs to submit.
        pub jobs: pulumi_wasm_rust::Output<
            Vec<super::super::types::dataproc::WorkflowTemplateJob>,
        >,
        /// Optional. The labels to associate with this template. These labels will be propagated to all jobs and clusters created
        /// by the workflow instance. Label **keys** must contain 1 to 63 characters, and must conform to [RFC
        /// 1035](https://www.ietf.org/rfc/rfc1035.txt). Label **values** may be empty, but, if present, must contain 1 to 63
        /// characters, and must conform to [RFC 1035](https://www.ietf.org/rfc/rfc1035.txt). No more than 32 labels can be
        /// associated with a template. **Note**: This field is non-authoritative, and will only manage the labels present in your
        /// configuration. Please refer to the field `effective_labels` for all of the labels present on the resource.
        pub labels: pulumi_wasm_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// The location for the resource
        pub location: pulumi_wasm_rust::Output<String>,
        /// Output only. The resource name of the workflow template, as described in https://cloud.google.com/apis/design/resource_names. * For `projects.regions.workflowTemplates`, the resource name of the template has the following format: `projects/{project_id}/regions/{region}/workflowTemplates/{template_id}` * For `projects.locations.workflowTemplates`, the resource name of the template has the following format: `projects/{project_id}/locations/{location}/workflowTemplates/{template_id}`
        pub name: pulumi_wasm_rust::Output<String>,
        /// Optional. Template parameters whose values are substituted into the template. Values for parameters must be provided
        /// when the template is instantiated.
        pub parameters: pulumi_wasm_rust::Output<
            Option<Vec<super::super::types::dataproc::WorkflowTemplateParameter>>,
        >,
        /// Required. WorkflowTemplate scheduling information.
        pub placement: pulumi_wasm_rust::Output<
            super::super::types::dataproc::WorkflowTemplatePlacement,
        >,
        /// The project for the resource
        pub project: pulumi_wasm_rust::Output<String>,
        /// The combination of labels configured directly on the resource and default labels configured on the provider.
        pub pulumi_labels: pulumi_wasm_rust::Output<
            std::collections::HashMap<String, String>,
        >,
        /// Output only. The time template was last updated.
        pub update_time: pulumi_wasm_rust::Output<String>,
        /// Output only. The current version of this workflow template.
        pub version: pulumi_wasm_rust::Output<i32>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(name: &str, args: WorkflowTemplateArgs) -> WorkflowTemplateResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let dag_timeout_binding = args.dag_timeout.get_inner();
        let jobs_binding = args.jobs.get_inner();
        let labels_binding = args.labels.get_inner();
        let location_binding = args.location.get_inner();
        let name_binding = args.name.get_inner();
        let parameters_binding = args.parameters.get_inner();
        let placement_binding = args.placement.get_inner();
        let project_binding = args.project.get_inner();
        let version_binding = args.version.get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "gcp:dataproc/workflowTemplate:WorkflowTemplate".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "dagTimeout".into(),
                    value: &dag_timeout_binding,
                },
                register_interface::ObjectField {
                    name: "jobs".into(),
                    value: &jobs_binding,
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
                    name: "name".into(),
                    value: &name_binding,
                },
                register_interface::ObjectField {
                    name: "parameters".into(),
                    value: &parameters_binding,
                },
                register_interface::ObjectField {
                    name: "placement".into(),
                    value: &placement_binding,
                },
                register_interface::ObjectField {
                    name: "project".into(),
                    value: &project_binding,
                },
                register_interface::ObjectField {
                    name: "version".into(),
                    value: &version_binding,
                },
            ]),
            results: Vec::from([
                register_interface::ResultField {
                    name: "createTime".into(),
                },
                register_interface::ResultField {
                    name: "dagTimeout".into(),
                },
                register_interface::ResultField {
                    name: "effectiveLabels".into(),
                },
                register_interface::ResultField {
                    name: "jobs".into(),
                },
                register_interface::ResultField {
                    name: "labels".into(),
                },
                register_interface::ResultField {
                    name: "location".into(),
                },
                register_interface::ResultField {
                    name: "name".into(),
                },
                register_interface::ResultField {
                    name: "parameters".into(),
                },
                register_interface::ResultField {
                    name: "placement".into(),
                },
                register_interface::ResultField {
                    name: "project".into(),
                },
                register_interface::ResultField {
                    name: "pulumiLabels".into(),
                },
                register_interface::ResultField {
                    name: "updateTime".into(),
                },
                register_interface::ResultField {
                    name: "version".into(),
                },
            ]),
        };
        let o = register_interface::register(&request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        WorkflowTemplateResult {
            create_time: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("createTime").unwrap(),
            ),
            dag_timeout: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("dagTimeout").unwrap(),
            ),
            effective_labels: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("effectiveLabels").unwrap(),
            ),
            jobs: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("jobs").unwrap(),
            ),
            labels: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("labels").unwrap(),
            ),
            location: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("location").unwrap(),
            ),
            name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("name").unwrap(),
            ),
            parameters: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("parameters").unwrap(),
            ),
            placement: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("placement").unwrap(),
            ),
            project: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("project").unwrap(),
            ),
            pulumi_labels: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("pulumiLabels").unwrap(),
            ),
            update_time: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("updateTime").unwrap(),
            ),
            version: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("version").unwrap(),
            ),
        }
    }
}
