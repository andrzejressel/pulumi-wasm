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
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod target {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct TargetArgs {
        /// Optional. User annotations. These attributes can only be set and used by the user, and not by Google Cloud Deploy. See https://google.aip.dev/128#annotations for more details such as format and size limitations.
        ///
        /// **Note**: This field is non-authoritative, and will only manage the annotations present in your configuration.
        /// Please refer to the field `effective_annotations` for all of the annotations present on the resource.
        #[builder(into, default)]
        pub annotations: pulumi_gestalt_rust::InputOrOutput<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// Information specifying an Anthos Cluster.
        #[builder(into, default)]
        pub anthos_cluster: pulumi_gestalt_rust::InputOrOutput<
            Option<super::super::types::clouddeploy::TargetAnthosCluster>,
        >,
        /// Optional. Map of entity IDs to their associated entities. Associated entities allows specifying places other than the deployment target for specific features. For example, the Gateway API canary can be configured to deploy the HTTPRoute to a different cluster(s) than the deployment cluster using associated entities. An entity ID must consist of lower-case letters, numbers, and hyphens, start with a letter and end with a letter or a number, and have a max length of 63 characters. In other words, it must match the following regex: `^a-z?$`.
        #[builder(into, default)]
        pub associated_entities: pulumi_gestalt_rust::InputOrOutput<
            Option<Vec<super::super::types::clouddeploy::TargetAssociatedEntity>>,
        >,
        /// Optional. Information specifying a Custom Target.
        #[builder(into, default)]
        pub custom_target: pulumi_gestalt_rust::InputOrOutput<
            Option<super::super::types::clouddeploy::TargetCustomTarget>,
        >,
        /// Optional. The deploy parameters to use for this target.
        #[builder(into, default)]
        pub deploy_parameters: pulumi_gestalt_rust::InputOrOutput<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// Optional. Description of the `Target`. Max length is 255 characters.
        #[builder(into, default)]
        pub description: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Configurations for all execution that relates to this `Target`. Each `ExecutionEnvironmentUsage` value may only be used in a single configuration; using the same value multiple times is an error. When one or more configurations are specified, they must include the `RENDER` and `DEPLOY` `ExecutionEnvironmentUsage` values. When no configurations are specified, execution will use the default specified in `DefaultPool`.
        #[builder(into, default)]
        pub execution_configs: pulumi_gestalt_rust::InputOrOutput<
            Option<Vec<super::super::types::clouddeploy::TargetExecutionConfig>>,
        >,
        /// Information specifying a GKE Cluster.
        #[builder(into, default)]
        pub gke: pulumi_gestalt_rust::InputOrOutput<
            Option<super::super::types::clouddeploy::TargetGke>,
        >,
        /// Optional. Labels are attributes that can be set and used by both the user and by Google Cloud Deploy. Labels must meet the following constraints: * Keys and values can contain only lowercase letters, numeric characters, underscores, and dashes. * All characters must use UTF-8 encoding, and international characters are allowed. * Keys must start with a lowercase letter or international character. * Each resource is limited to a maximum of 64 labels. Both keys and values are additionally constrained to be <= 128 bytes.
        ///
        /// **Note**: This field is non-authoritative, and will only manage the labels present in your configuration.
        /// Please refer to the field `effective_labels` for all of the labels present on the resource.
        #[builder(into, default)]
        pub labels: pulumi_gestalt_rust::InputOrOutput<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// The location for the resource
        #[builder(into)]
        pub location: pulumi_gestalt_rust::InputOrOutput<String>,
        /// Information specifying a multiTarget.
        #[builder(into, default)]
        pub multi_target: pulumi_gestalt_rust::InputOrOutput<
            Option<super::super::types::clouddeploy::TargetMultiTarget>,
        >,
        /// Name of the `Target`. Format is `a-z?`.
        ///
        ///
        ///
        /// - - -
        #[builder(into, default)]
        pub name: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The project for the resource
        #[builder(into, default)]
        pub project: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Optional. Whether or not the `Target` requires approval.
        #[builder(into, default)]
        pub require_approval: pulumi_gestalt_rust::InputOrOutput<Option<bool>>,
        /// Information specifying a Cloud Run deployment target.
        #[builder(into, default)]
        pub run: pulumi_gestalt_rust::InputOrOutput<
            Option<super::super::types::clouddeploy::TargetRun>,
        >,
    }
    #[allow(dead_code)]
    pub struct TargetResult {
        /// Optional. User annotations. These attributes can only be set and used by the user, and not by Google Cloud Deploy. See https://google.aip.dev/128#annotations for more details such as format and size limitations.
        ///
        /// **Note**: This field is non-authoritative, and will only manage the annotations present in your configuration.
        /// Please refer to the field `effective_annotations` for all of the annotations present on the resource.
        pub annotations: pulumi_gestalt_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// Information specifying an Anthos Cluster.
        pub anthos_cluster: pulumi_gestalt_rust::Output<
            Option<super::super::types::clouddeploy::TargetAnthosCluster>,
        >,
        /// Optional. Map of entity IDs to their associated entities. Associated entities allows specifying places other than the deployment target for specific features. For example, the Gateway API canary can be configured to deploy the HTTPRoute to a different cluster(s) than the deployment cluster using associated entities. An entity ID must consist of lower-case letters, numbers, and hyphens, start with a letter and end with a letter or a number, and have a max length of 63 characters. In other words, it must match the following regex: `^a-z?$`.
        pub associated_entities: pulumi_gestalt_rust::Output<
            Option<Vec<super::super::types::clouddeploy::TargetAssociatedEntity>>,
        >,
        /// Output only. Time at which the `Target` was created.
        pub create_time: pulumi_gestalt_rust::Output<String>,
        /// Optional. Information specifying a Custom Target.
        pub custom_target: pulumi_gestalt_rust::Output<
            Option<super::super::types::clouddeploy::TargetCustomTarget>,
        >,
        /// Optional. The deploy parameters to use for this target.
        pub deploy_parameters: pulumi_gestalt_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// Optional. Description of the `Target`. Max length is 255 characters.
        pub description: pulumi_gestalt_rust::Output<Option<String>>,
        pub effective_annotations: pulumi_gestalt_rust::Output<
            std::collections::HashMap<String, String>,
        >,
        /// All of labels (key/value pairs) present on the resource in GCP, including the labels configured through Pulumi, other clients and services.
        pub effective_labels: pulumi_gestalt_rust::Output<
            std::collections::HashMap<String, String>,
        >,
        /// Optional. This checksum is computed by the server based on the value of other fields, and may be sent on update and delete requests to ensure the client has an up-to-date value before proceeding.
        pub etag: pulumi_gestalt_rust::Output<String>,
        /// Configurations for all execution that relates to this `Target`. Each `ExecutionEnvironmentUsage` value may only be used in a single configuration; using the same value multiple times is an error. When one or more configurations are specified, they must include the `RENDER` and `DEPLOY` `ExecutionEnvironmentUsage` values. When no configurations are specified, execution will use the default specified in `DefaultPool`.
        pub execution_configs: pulumi_gestalt_rust::Output<
            Vec<super::super::types::clouddeploy::TargetExecutionConfig>,
        >,
        /// Information specifying a GKE Cluster.
        pub gke: pulumi_gestalt_rust::Output<
            Option<super::super::types::clouddeploy::TargetGke>,
        >,
        /// Optional. Labels are attributes that can be set and used by both the user and by Google Cloud Deploy. Labels must meet the following constraints: * Keys and values can contain only lowercase letters, numeric characters, underscores, and dashes. * All characters must use UTF-8 encoding, and international characters are allowed. * Keys must start with a lowercase letter or international character. * Each resource is limited to a maximum of 64 labels. Both keys and values are additionally constrained to be <= 128 bytes.
        ///
        /// **Note**: This field is non-authoritative, and will only manage the labels present in your configuration.
        /// Please refer to the field `effective_labels` for all of the labels present on the resource.
        pub labels: pulumi_gestalt_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// The location for the resource
        pub location: pulumi_gestalt_rust::Output<String>,
        /// Information specifying a multiTarget.
        pub multi_target: pulumi_gestalt_rust::Output<
            Option<super::super::types::clouddeploy::TargetMultiTarget>,
        >,
        /// Name of the `Target`. Format is `a-z?`.
        ///
        ///
        ///
        /// - - -
        pub name: pulumi_gestalt_rust::Output<String>,
        /// The project for the resource
        pub project: pulumi_gestalt_rust::Output<String>,
        /// The combination of labels configured directly on the resource and default labels configured on the provider.
        pub pulumi_labels: pulumi_gestalt_rust::Output<
            std::collections::HashMap<String, String>,
        >,
        /// Optional. Whether or not the `Target` requires approval.
        pub require_approval: pulumi_gestalt_rust::Output<Option<bool>>,
        /// Information specifying a Cloud Run deployment target.
        pub run: pulumi_gestalt_rust::Output<
            Option<super::super::types::clouddeploy::TargetRun>,
        >,
        /// Output only. Resource id of the `Target`.
        pub target_id: pulumi_gestalt_rust::Output<String>,
        /// Output only. Unique identifier of the `Target`.
        pub uid: pulumi_gestalt_rust::Output<String>,
        /// Output only. Most recent time at which the `Target` was updated.
        pub update_time: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: TargetArgs,
    ) -> TargetResult {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::register_interface;
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let annotations_binding = args.annotations.get_output(context);
        let anthos_cluster_binding = args.anthos_cluster.get_output(context);
        let associated_entities_binding = args.associated_entities.get_output(context);
        let custom_target_binding = args.custom_target.get_output(context);
        let deploy_parameters_binding = args.deploy_parameters.get_output(context);
        let description_binding = args.description.get_output(context);
        let execution_configs_binding = args.execution_configs.get_output(context);
        let gke_binding = args.gke.get_output(context);
        let labels_binding = args.labels.get_output(context);
        let location_binding = args.location.get_output(context);
        let multi_target_binding = args.multi_target.get_output(context);
        let name_binding = args.name.get_output(context);
        let project_binding = args.project.get_output(context);
        let require_approval_binding = args.require_approval.get_output(context);
        let run_binding = args.run.get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "gcp:clouddeploy/target:Target".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "annotations".into(),
                    value: annotations_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "anthosCluster".into(),
                    value: anthos_cluster_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "associatedEntities".into(),
                    value: associated_entities_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "customTarget".into(),
                    value: custom_target_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "deployParameters".into(),
                    value: deploy_parameters_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "description".into(),
                    value: description_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "executionConfigs".into(),
                    value: execution_configs_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "gke".into(),
                    value: gke_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "labels".into(),
                    value: labels_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "location".into(),
                    value: location_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "multiTarget".into(),
                    value: multi_target_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "name".into(),
                    value: name_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "project".into(),
                    value: project_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "requireApproval".into(),
                    value: require_approval_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "run".into(),
                    value: run_binding.get_id(),
                },
            ],
        };
        let o = context.register_resource(request);
        TargetResult {
            annotations: o.get_field("annotations"),
            anthos_cluster: o.get_field("anthosCluster"),
            associated_entities: o.get_field("associatedEntities"),
            create_time: o.get_field("createTime"),
            custom_target: o.get_field("customTarget"),
            deploy_parameters: o.get_field("deployParameters"),
            description: o.get_field("description"),
            effective_annotations: o.get_field("effectiveAnnotations"),
            effective_labels: o.get_field("effectiveLabels"),
            etag: o.get_field("etag"),
            execution_configs: o.get_field("executionConfigs"),
            gke: o.get_field("gke"),
            labels: o.get_field("labels"),
            location: o.get_field("location"),
            multi_target: o.get_field("multiTarget"),
            name: o.get_field("name"),
            project: o.get_field("project"),
            pulumi_labels: o.get_field("pulumiLabels"),
            require_approval: o.get_field("requireApproval"),
            run: o.get_field("run"),
            target_id: o.get_field("targetId"),
            uid: o.get_field("uid"),
            update_time: o.get_field("updateTime"),
        }
    }
}
