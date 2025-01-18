/// The Cloud Deploy `Target` resource
///
/// ## Example Usage
///
/// ### Multi_target
/// tests creating and updating a multi-target
/// ```yaml
/// resources:
///   primary:
///     type: gcp:clouddeploy:Target
///     properties:
///       location: us-west1
///       name: target
///       deployParameters: {}
///       description: multi-target description
///       executionConfigs:
///         - usages:
///             - RENDER
///             - DEPLOY
///           executionTimeout: 3600s
///       multiTarget:
///         targetIds:
///           - '1'
///           - '2'
///       project: my-project-name
///       requireApproval: false
///       annotations:
///         my_first_annotation: example-annotation-1
///         my_second_annotation: example-annotation-2
///       labels:
///         my_first_label: example-label-1
///         my_second_label: example-label-2
/// ```
/// ### Run_target
/// tests creating and updating a cloud run target
/// ```yaml
/// resources:
///   primary:
///     type: gcp:clouddeploy:Target
///     properties:
///       location: us-west1
///       name: target
///       deployParameters: {}
///       description: basic description
///       executionConfigs:
///         - usages:
///             - RENDER
///             - DEPLOY
///           executionTimeout: 3600s
///       project: my-project-name
///       requireApproval: false
///       run:
///         location: projects/my-project-name/locations/us-west1
///       annotations:
///         my_first_annotation: example-annotation-1
///         my_second_annotation: example-annotation-2
///       labels:
///         my_first_label: example-label-1
///         my_second_label: example-label-2
/// ```
/// ### Target
/// Creates a basic Cloud Deploy target
/// ```yaml
/// resources:
///   primary:
///     type: gcp:clouddeploy:Target
///     properties:
///       location: us-west1
///       name: target
///       deployParameters:
///         deployParameterKey: deployParameterValue
///       description: basic description
///       gke:
///         cluster: projects/my-project-name/locations/us-west1/clusters/example-cluster-name
///       project: my-project-name
///       requireApproval: false
///       annotations:
///         my_first_annotation: example-annotation-1
///         my_second_annotation: example-annotation-2
///       labels:
///         my_first_label: example-label-1
///         my_second_label: example-label-2
/// ```
///
/// ## Import
///
/// Target can be imported using any of these accepted formats:
///
/// * `projects/{{project}}/locations/{{location}}/targets/{{name}}`
///
/// * `{{project}}/{{location}}/{{name}}`
///
/// * `{{location}}/{{name}}`
///
/// When using the `pulumi import` command, Target can be imported using one of the formats above. For example:
///
/// ```sh
/// $ pulumi import gcp:clouddeploy/target:Target default projects/{{project}}/locations/{{location}}/targets/{{name}}
/// ```
///
/// ```sh
/// $ pulumi import gcp:clouddeploy/target:Target default {{project}}/{{location}}/{{name}}
/// ```
///
/// ```sh
/// $ pulumi import gcp:clouddeploy/target:Target default {{location}}/{{name}}
/// ```
///
pub mod target {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct TargetArgs {
        /// Optional. User annotations. These attributes can only be set and used by the user, and not by Google Cloud Deploy. See https://google.aip.dev/128#annotations for more details such as format and size limitations.
        ///
        /// **Note**: This field is non-authoritative, and will only manage the annotations present in your configuration.
        /// Please refer to the field `effective_annotations` for all of the annotations present on the resource.
        #[builder(into, default)]
        pub annotations: pulumi_wasm_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// Information specifying an Anthos Cluster.
        #[builder(into, default)]
        pub anthos_cluster: pulumi_wasm_rust::Output<
            Option<super::super::types::clouddeploy::TargetAnthosCluster>,
        >,
        /// Optional. Map of entity IDs to their associated entities. Associated entities allows specifying places other than the deployment target for specific features. For example, the Gateway API canary can be configured to deploy the HTTPRoute to a different cluster(s) than the deployment cluster using associated entities. An entity ID must consist of lower-case letters, numbers, and hyphens, start with a letter and end with a letter or a number, and have a max length of 63 characters. In other words, it must match the following regex: `^a-z?$`.
        #[builder(into, default)]
        pub associated_entities: pulumi_wasm_rust::Output<
            Option<Vec<super::super::types::clouddeploy::TargetAssociatedEntity>>,
        >,
        /// Optional. Information specifying a Custom Target.
        #[builder(into, default)]
        pub custom_target: pulumi_wasm_rust::Output<
            Option<super::super::types::clouddeploy::TargetCustomTarget>,
        >,
        /// Optional. The deploy parameters to use for this target.
        #[builder(into, default)]
        pub deploy_parameters: pulumi_wasm_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// Optional. Description of the `Target`. Max length is 255 characters.
        #[builder(into, default)]
        pub description: pulumi_wasm_rust::Output<Option<String>>,
        /// Configurations for all execution that relates to this `Target`. Each `ExecutionEnvironmentUsage` value may only be used in a single configuration; using the same value multiple times is an error. When one or more configurations are specified, they must include the `RENDER` and `DEPLOY` `ExecutionEnvironmentUsage` values. When no configurations are specified, execution will use the default specified in `DefaultPool`.
        #[builder(into, default)]
        pub execution_configs: pulumi_wasm_rust::Output<
            Option<Vec<super::super::types::clouddeploy::TargetExecutionConfig>>,
        >,
        /// Information specifying a GKE Cluster.
        #[builder(into, default)]
        pub gke: pulumi_wasm_rust::Output<
            Option<super::super::types::clouddeploy::TargetGke>,
        >,
        /// Optional. Labels are attributes that can be set and used by both the user and by Google Cloud Deploy. Labels must meet the following constraints: * Keys and values can contain only lowercase letters, numeric characters, underscores, and dashes. * All characters must use UTF-8 encoding, and international characters are allowed. * Keys must start with a lowercase letter or international character. * Each resource is limited to a maximum of 64 labels. Both keys and values are additionally constrained to be <= 128 bytes.
        ///
        /// **Note**: This field is non-authoritative, and will only manage the labels present in your configuration.
        /// Please refer to the field `effective_labels` for all of the labels present on the resource.
        #[builder(into, default)]
        pub labels: pulumi_wasm_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// The location for the resource
        #[builder(into)]
        pub location: pulumi_wasm_rust::Output<String>,
        /// Information specifying a multiTarget.
        #[builder(into, default)]
        pub multi_target: pulumi_wasm_rust::Output<
            Option<super::super::types::clouddeploy::TargetMultiTarget>,
        >,
        /// Name of the `Target`. Format is `a-z?`.
        ///
        ///
        ///
        /// - - -
        #[builder(into, default)]
        pub name: pulumi_wasm_rust::Output<Option<String>>,
        /// The project for the resource
        #[builder(into, default)]
        pub project: pulumi_wasm_rust::Output<Option<String>>,
        /// Optional. Whether or not the `Target` requires approval.
        #[builder(into, default)]
        pub require_approval: pulumi_wasm_rust::Output<Option<bool>>,
        /// Information specifying a Cloud Run deployment target.
        #[builder(into, default)]
        pub run: pulumi_wasm_rust::Output<
            Option<super::super::types::clouddeploy::TargetRun>,
        >,
    }
    #[allow(dead_code)]
    pub struct TargetResult {
        /// Optional. User annotations. These attributes can only be set and used by the user, and not by Google Cloud Deploy. See https://google.aip.dev/128#annotations for more details such as format and size limitations.
        ///
        /// **Note**: This field is non-authoritative, and will only manage the annotations present in your configuration.
        /// Please refer to the field `effective_annotations` for all of the annotations present on the resource.
        pub annotations: pulumi_wasm_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// Information specifying an Anthos Cluster.
        pub anthos_cluster: pulumi_wasm_rust::Output<
            Option<super::super::types::clouddeploy::TargetAnthosCluster>,
        >,
        /// Optional. Map of entity IDs to their associated entities. Associated entities allows specifying places other than the deployment target for specific features. For example, the Gateway API canary can be configured to deploy the HTTPRoute to a different cluster(s) than the deployment cluster using associated entities. An entity ID must consist of lower-case letters, numbers, and hyphens, start with a letter and end with a letter or a number, and have a max length of 63 characters. In other words, it must match the following regex: `^a-z?$`.
        pub associated_entities: pulumi_wasm_rust::Output<
            Option<Vec<super::super::types::clouddeploy::TargetAssociatedEntity>>,
        >,
        /// Output only. Time at which the `Target` was created.
        pub create_time: pulumi_wasm_rust::Output<String>,
        /// Optional. Information specifying a Custom Target.
        pub custom_target: pulumi_wasm_rust::Output<
            Option<super::super::types::clouddeploy::TargetCustomTarget>,
        >,
        /// Optional. The deploy parameters to use for this target.
        pub deploy_parameters: pulumi_wasm_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// Optional. Description of the `Target`. Max length is 255 characters.
        pub description: pulumi_wasm_rust::Output<Option<String>>,
        pub effective_annotations: pulumi_wasm_rust::Output<
            std::collections::HashMap<String, String>,
        >,
        /// All of labels (key/value pairs) present on the resource in GCP, including the labels configured through Pulumi, other clients and services.
        pub effective_labels: pulumi_wasm_rust::Output<
            std::collections::HashMap<String, String>,
        >,
        /// Optional. This checksum is computed by the server based on the value of other fields, and may be sent on update and delete requests to ensure the client has an up-to-date value before proceeding.
        pub etag: pulumi_wasm_rust::Output<String>,
        /// Configurations for all execution that relates to this `Target`. Each `ExecutionEnvironmentUsage` value may only be used in a single configuration; using the same value multiple times is an error. When one or more configurations are specified, they must include the `RENDER` and `DEPLOY` `ExecutionEnvironmentUsage` values. When no configurations are specified, execution will use the default specified in `DefaultPool`.
        pub execution_configs: pulumi_wasm_rust::Output<
            Vec<super::super::types::clouddeploy::TargetExecutionConfig>,
        >,
        /// Information specifying a GKE Cluster.
        pub gke: pulumi_wasm_rust::Output<
            Option<super::super::types::clouddeploy::TargetGke>,
        >,
        /// Optional. Labels are attributes that can be set and used by both the user and by Google Cloud Deploy. Labels must meet the following constraints: * Keys and values can contain only lowercase letters, numeric characters, underscores, and dashes. * All characters must use UTF-8 encoding, and international characters are allowed. * Keys must start with a lowercase letter or international character. * Each resource is limited to a maximum of 64 labels. Both keys and values are additionally constrained to be <= 128 bytes.
        ///
        /// **Note**: This field is non-authoritative, and will only manage the labels present in your configuration.
        /// Please refer to the field `effective_labels` for all of the labels present on the resource.
        pub labels: pulumi_wasm_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// The location for the resource
        pub location: pulumi_wasm_rust::Output<String>,
        /// Information specifying a multiTarget.
        pub multi_target: pulumi_wasm_rust::Output<
            Option<super::super::types::clouddeploy::TargetMultiTarget>,
        >,
        /// Name of the `Target`. Format is `a-z?`.
        ///
        ///
        ///
        /// - - -
        pub name: pulumi_wasm_rust::Output<String>,
        /// The project for the resource
        pub project: pulumi_wasm_rust::Output<String>,
        /// The combination of labels configured directly on the resource and default labels configured on the provider.
        pub pulumi_labels: pulumi_wasm_rust::Output<
            std::collections::HashMap<String, String>,
        >,
        /// Optional. Whether or not the `Target` requires approval.
        pub require_approval: pulumi_wasm_rust::Output<Option<bool>>,
        /// Information specifying a Cloud Run deployment target.
        pub run: pulumi_wasm_rust::Output<
            Option<super::super::types::clouddeploy::TargetRun>,
        >,
        /// Output only. Resource id of the `Target`.
        pub target_id: pulumi_wasm_rust::Output<String>,
        /// Output only. Unique identifier of the `Target`.
        pub uid: pulumi_wasm_rust::Output<String>,
        /// Output only. Most recent time at which the `Target` was updated.
        pub update_time: pulumi_wasm_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(name: &str, args: TargetArgs) -> TargetResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let annotations_binding = args.annotations.get_inner();
        let anthos_cluster_binding = args.anthos_cluster.get_inner();
        let associated_entities_binding = args.associated_entities.get_inner();
        let custom_target_binding = args.custom_target.get_inner();
        let deploy_parameters_binding = args.deploy_parameters.get_inner();
        let description_binding = args.description.get_inner();
        let execution_configs_binding = args.execution_configs.get_inner();
        let gke_binding = args.gke.get_inner();
        let labels_binding = args.labels.get_inner();
        let location_binding = args.location.get_inner();
        let multi_target_binding = args.multi_target.get_inner();
        let name_binding = args.name.get_inner();
        let project_binding = args.project.get_inner();
        let require_approval_binding = args.require_approval.get_inner();
        let run_binding = args.run.get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "gcp:clouddeploy/target:Target".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "annotations".into(),
                    value: &annotations_binding,
                },
                register_interface::ObjectField {
                    name: "anthosCluster".into(),
                    value: &anthos_cluster_binding,
                },
                register_interface::ObjectField {
                    name: "associatedEntities".into(),
                    value: &associated_entities_binding,
                },
                register_interface::ObjectField {
                    name: "customTarget".into(),
                    value: &custom_target_binding,
                },
                register_interface::ObjectField {
                    name: "deployParameters".into(),
                    value: &deploy_parameters_binding,
                },
                register_interface::ObjectField {
                    name: "description".into(),
                    value: &description_binding,
                },
                register_interface::ObjectField {
                    name: "executionConfigs".into(),
                    value: &execution_configs_binding,
                },
                register_interface::ObjectField {
                    name: "gke".into(),
                    value: &gke_binding,
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
                    name: "multiTarget".into(),
                    value: &multi_target_binding,
                },
                register_interface::ObjectField {
                    name: "name".into(),
                    value: &name_binding,
                },
                register_interface::ObjectField {
                    name: "project".into(),
                    value: &project_binding,
                },
                register_interface::ObjectField {
                    name: "requireApproval".into(),
                    value: &require_approval_binding,
                },
                register_interface::ObjectField {
                    name: "run".into(),
                    value: &run_binding,
                },
            ]),
            results: Vec::from([
                register_interface::ResultField {
                    name: "annotations".into(),
                },
                register_interface::ResultField {
                    name: "anthosCluster".into(),
                },
                register_interface::ResultField {
                    name: "associatedEntities".into(),
                },
                register_interface::ResultField {
                    name: "createTime".into(),
                },
                register_interface::ResultField {
                    name: "customTarget".into(),
                },
                register_interface::ResultField {
                    name: "deployParameters".into(),
                },
                register_interface::ResultField {
                    name: "description".into(),
                },
                register_interface::ResultField {
                    name: "effectiveAnnotations".into(),
                },
                register_interface::ResultField {
                    name: "effectiveLabels".into(),
                },
                register_interface::ResultField {
                    name: "etag".into(),
                },
                register_interface::ResultField {
                    name: "executionConfigs".into(),
                },
                register_interface::ResultField {
                    name: "gke".into(),
                },
                register_interface::ResultField {
                    name: "labels".into(),
                },
                register_interface::ResultField {
                    name: "location".into(),
                },
                register_interface::ResultField {
                    name: "multiTarget".into(),
                },
                register_interface::ResultField {
                    name: "name".into(),
                },
                register_interface::ResultField {
                    name: "project".into(),
                },
                register_interface::ResultField {
                    name: "pulumiLabels".into(),
                },
                register_interface::ResultField {
                    name: "requireApproval".into(),
                },
                register_interface::ResultField {
                    name: "run".into(),
                },
                register_interface::ResultField {
                    name: "targetId".into(),
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
        TargetResult {
            annotations: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("annotations").unwrap(),
            ),
            anthos_cluster: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("anthosCluster").unwrap(),
            ),
            associated_entities: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("associatedEntities").unwrap(),
            ),
            create_time: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("createTime").unwrap(),
            ),
            custom_target: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("customTarget").unwrap(),
            ),
            deploy_parameters: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("deployParameters").unwrap(),
            ),
            description: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("description").unwrap(),
            ),
            effective_annotations: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("effectiveAnnotations").unwrap(),
            ),
            effective_labels: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("effectiveLabels").unwrap(),
            ),
            etag: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("etag").unwrap(),
            ),
            execution_configs: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("executionConfigs").unwrap(),
            ),
            gke: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("gke").unwrap(),
            ),
            labels: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("labels").unwrap(),
            ),
            location: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("location").unwrap(),
            ),
            multi_target: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("multiTarget").unwrap(),
            ),
            name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("name").unwrap(),
            ),
            project: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("project").unwrap(),
            ),
            pulumi_labels: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("pulumiLabels").unwrap(),
            ),
            require_approval: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("requireApproval").unwrap(),
            ),
            run: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("run").unwrap(),
            ),
            target_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("targetId").unwrap(),
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
