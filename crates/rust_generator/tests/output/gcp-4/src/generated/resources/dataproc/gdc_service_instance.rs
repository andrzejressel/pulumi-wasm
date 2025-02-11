/// A service instance is an instance of the Dataproc operator running on a GDC cluster.
///
///
/// To get more information about ServiceInstance, see:
///
/// * [API documentation](https://cloud.google.com/dataproc-gdc/docs/reference/rest/v1/projects.locations.serviceInstances)
/// * How-to Guides
///     * [Dataproc Intro](https://cloud.google.com/dataproc/)
///
/// ## Example Usage
///
/// ### Dataprocgdc Serviceinstance
///
///
/// ```yaml
/// resources:
///   service-instance:
///     type: gcp:dataproc:GdcServiceInstance
///     properties:
///       serviceInstanceId: tf-e2e-service-instance
///       project: my-project
///       location: us-west2
///       gdceCluster:
///         gdceCluster: projects/gdce-cluster-monitoring/locations/us-west2/clusters/gdce-prism-prober-ord106
///       displayName: A service instance
///       labels:
///         test-label: label-value
///       serviceAccount: dataprocgdc-cep-workflows@gdce-cluster-monitoring.iam.gserviceaccount.com
/// ```
///
/// ## Import
///
/// ServiceInstance can be imported using any of these accepted formats:
///
/// * `projects/{{project}}/locations/{{location}}/serviceInstances/{{service_instance_id}}`
///
/// * `{{project}}/{{location}}/{{service_instance_id}}`
///
/// * `{{location}}/{{service_instance_id}}`
///
/// When using the `pulumi import` command, ServiceInstance can be imported using one of the formats above. For example:
///
/// ```sh
/// $ pulumi import gcp:dataproc/gdcServiceInstance:GdcServiceInstance default projects/{{project}}/locations/{{location}}/serviceInstances/{{service_instance_id}}
/// ```
///
/// ```sh
/// $ pulumi import gcp:dataproc/gdcServiceInstance:GdcServiceInstance default {{project}}/{{location}}/{{service_instance_id}}
/// ```
///
/// ```sh
/// $ pulumi import gcp:dataproc/gdcServiceInstance:GdcServiceInstance default {{location}}/{{service_instance_id}}
/// ```
///
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod gdc_service_instance {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GdcServiceInstanceArgs {
        /// User-provided human-readable name to be used in user interfaces.
        #[builder(into, default)]
        pub display_name: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Gdce cluster information.
        /// Structure is documented below.
        #[builder(into, default)]
        pub gdce_cluster: pulumi_gestalt_rust::InputOrOutput<
            Option<super::super::types::dataproc::GdcServiceInstanceGdceCluster>,
        >,
        /// The labels to associate with this service instance. Labels may be used for filtering and billing tracking.
        /// **Note**: This field is non-authoritative, and will only manage the labels present in your configuration.
        /// Please refer to the field `effective_labels` for all of the labels present on the resource.
        #[builder(into, default)]
        pub labels: pulumi_gestalt_rust::InputOrOutput<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// Location of the resource.
        #[builder(into)]
        pub location: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The ID of the project in which the resource belongs.
        /// If it is not provided, the provider project is used.
        #[builder(into, default)]
        pub project: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Requested service account to associate with ServiceInstance.
        #[builder(into, default)]
        pub service_account: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Id of the service instance.
        ///
        ///
        /// - - -
        #[builder(into)]
        pub service_instance_id: pulumi_gestalt_rust::InputOrOutput<String>,
        /// Spark-specific service instance configuration.
        #[builder(into, default)]
        pub spark_service_instance_config: pulumi_gestalt_rust::InputOrOutput<
            Option<
                super::super::types::dataproc::GdcServiceInstanceSparkServiceInstanceConfig,
            >,
        >,
    }
    #[allow(dead_code)]
    pub struct GdcServiceInstanceResult {
        /// The timestamp when the resource was created.
        pub create_time: pulumi_gestalt_rust::Output<String>,
        /// User-provided human-readable name to be used in user interfaces.
        pub display_name: pulumi_gestalt_rust::Output<Option<String>>,
        /// All of labels (key/value pairs) present on the resource in GCP, including the labels configured through Pulumi, other clients and services.
        pub effective_labels: pulumi_gestalt_rust::Output<
            std::collections::HashMap<String, String>,
        >,
        /// Effective service account associated with ServiceInstance. This will be the service_account if specified. Otherwise, it will be an automatically created per-resource P4SA that also automatically has Fleet Workload. Identity bindings applied.
        pub effective_service_account: pulumi_gestalt_rust::Output<String>,
        /// Gdce cluster information.
        /// Structure is documented below.
        pub gdce_cluster: pulumi_gestalt_rust::Output<
            Option<super::super::types::dataproc::GdcServiceInstanceGdceCluster>,
        >,
        /// The labels to associate with this service instance. Labels may be used for filtering and billing tracking.
        /// **Note**: This field is non-authoritative, and will only manage the labels present in your configuration.
        /// Please refer to the field `effective_labels` for all of the labels present on the resource.
        pub labels: pulumi_gestalt_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// Location of the resource.
        pub location: pulumi_gestalt_rust::Output<String>,
        /// Identifier. The name of the service instance.
        pub name: pulumi_gestalt_rust::Output<String>,
        /// The ID of the project in which the resource belongs.
        /// If it is not provided, the provider project is used.
        pub project: pulumi_gestalt_rust::Output<String>,
        /// The combination of labels configured directly on the resource
        /// and default labels configured on the provider.
        pub pulumi_labels: pulumi_gestalt_rust::Output<
            std::collections::HashMap<String, String>,
        >,
        /// Whether the service instance is currently reconciling. True if the current state of the resource does not match the intended state, and the system is working to reconcile them, whether or not the change was user initiated.
        pub reconciling: pulumi_gestalt_rust::Output<bool>,
        /// The intended state to which the service instance is reconciling. Possible values:
        /// * `CREATING`
        /// * `ACTIVE`
        /// * `DISCONNECTED`
        /// * `DELETING`
        /// * `STOPPING`
        /// * `STOPPED`
        /// * `STARTING`
        /// * `UPDATING`
        /// * `FAILED`
        pub requested_state: pulumi_gestalt_rust::Output<String>,
        /// Requested service account to associate with ServiceInstance.
        pub service_account: pulumi_gestalt_rust::Output<Option<String>>,
        /// Id of the service instance.
        ///
        ///
        /// - - -
        pub service_instance_id: pulumi_gestalt_rust::Output<String>,
        /// Spark-specific service instance configuration.
        pub spark_service_instance_config: pulumi_gestalt_rust::Output<
            Option<
                super::super::types::dataproc::GdcServiceInstanceSparkServiceInstanceConfig,
            >,
        >,
        /// The current state. Possible values:
        /// * `CREATING`
        /// * `ACTIVE`
        /// * `DISCONNECTED`
        /// * `DELETING`
        /// * `STOPPING`
        /// * `STOPPED`
        /// * `STARTING`
        /// * `UPDATING`
        /// * `FAILED`
        pub state: pulumi_gestalt_rust::Output<String>,
        /// A message explaining the current state.
        pub state_message: pulumi_gestalt_rust::Output<String>,
        /// System generated unique identifier for this service instance, formatted as UUID4.
        pub uid: pulumi_gestalt_rust::Output<String>,
        /// The timestamp when the resource was most recently updated.
        pub update_time: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: GdcServiceInstanceArgs,
    ) -> GdcServiceInstanceResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let display_name_binding = args.display_name.get_output(context);
        let gdce_cluster_binding = args.gdce_cluster.get_output(context);
        let labels_binding = args.labels.get_output(context);
        let location_binding = args.location.get_output(context);
        let project_binding = args.project.get_output(context);
        let service_account_binding = args.service_account.get_output(context);
        let service_instance_id_binding = args.service_instance_id.get_output(context);
        let spark_service_instance_config_binding = args
            .spark_service_instance_config
            .get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "gcp:dataproc/gdcServiceInstance:GdcServiceInstance".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "displayName".into(),
                    value: &display_name_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "gdceCluster".into(),
                    value: &gdce_cluster_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "labels".into(),
                    value: &labels_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "location".into(),
                    value: &location_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "project".into(),
                    value: &project_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "serviceAccount".into(),
                    value: &service_account_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "serviceInstanceId".into(),
                    value: &service_instance_id_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "sparkServiceInstanceConfig".into(),
                    value: &spark_service_instance_config_binding.drop_type(),
                },
            ],
        };
        let o = context.register_resource(request);
        GdcServiceInstanceResult {
            create_time: o.get_field("createTime"),
            display_name: o.get_field("displayName"),
            effective_labels: o.get_field("effectiveLabels"),
            effective_service_account: o.get_field("effectiveServiceAccount"),
            gdce_cluster: o.get_field("gdceCluster"),
            labels: o.get_field("labels"),
            location: o.get_field("location"),
            name: o.get_field("name"),
            project: o.get_field("project"),
            pulumi_labels: o.get_field("pulumiLabels"),
            reconciling: o.get_field("reconciling"),
            requested_state: o.get_field("requestedState"),
            service_account: o.get_field("serviceAccount"),
            service_instance_id: o.get_field("serviceInstanceId"),
            spark_service_instance_config: o.get_field("sparkServiceInstanceConfig"),
            state: o.get_field("state"),
            state_message: o.get_field("stateMessage"),
            uid: o.get_field("uid"),
            update_time: o.get_field("updateTime"),
        }
    }
}
