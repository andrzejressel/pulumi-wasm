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
pub mod gdc_service_instance {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GdcServiceInstanceArgs {
        /// User-provided human-readable name to be used in user interfaces.
        #[builder(into, default)]
        pub display_name: pulumi_wasm_rust::Output<Option<String>>,
        /// Gdce cluster information.
        /// Structure is documented below.
        #[builder(into, default)]
        pub gdce_cluster: pulumi_wasm_rust::Output<
            Option<super::super::types::dataproc::GdcServiceInstanceGdceCluster>,
        >,
        /// The labels to associate with this service instance. Labels may be used for filtering and billing tracking.
        /// **Note**: This field is non-authoritative, and will only manage the labels present in your configuration.
        /// Please refer to the field `effective_labels` for all of the labels present on the resource.
        #[builder(into, default)]
        pub labels: pulumi_wasm_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// Location of the resource.
        #[builder(into)]
        pub location: pulumi_wasm_rust::Output<String>,
        /// The ID of the project in which the resource belongs.
        /// If it is not provided, the provider project is used.
        #[builder(into, default)]
        pub project: pulumi_wasm_rust::Output<Option<String>>,
        /// Requested service account to associate with ServiceInstance.
        #[builder(into, default)]
        pub service_account: pulumi_wasm_rust::Output<Option<String>>,
        /// Id of the service instance.
        ///
        ///
        /// - - -
        #[builder(into)]
        pub service_instance_id: pulumi_wasm_rust::Output<String>,
        /// Spark-specific service instance configuration.
        #[builder(into, default)]
        pub spark_service_instance_config: pulumi_wasm_rust::Output<
            Option<
                super::super::types::dataproc::GdcServiceInstanceSparkServiceInstanceConfig,
            >,
        >,
    }
    #[allow(dead_code)]
    pub struct GdcServiceInstanceResult {
        /// The timestamp when the resource was created.
        pub create_time: pulumi_wasm_rust::Output<String>,
        /// User-provided human-readable name to be used in user interfaces.
        pub display_name: pulumi_wasm_rust::Output<Option<String>>,
        /// All of labels (key/value pairs) present on the resource in GCP, including the labels configured through Pulumi, other clients and services.
        pub effective_labels: pulumi_wasm_rust::Output<
            std::collections::HashMap<String, String>,
        >,
        /// Effective service account associated with ServiceInstance. This will be the service_account if specified. Otherwise, it will be an automatically created per-resource P4SA that also automatically has Fleet Workload. Identity bindings applied.
        pub effective_service_account: pulumi_wasm_rust::Output<String>,
        /// Gdce cluster information.
        /// Structure is documented below.
        pub gdce_cluster: pulumi_wasm_rust::Output<
            Option<super::super::types::dataproc::GdcServiceInstanceGdceCluster>,
        >,
        /// The labels to associate with this service instance. Labels may be used for filtering and billing tracking.
        /// **Note**: This field is non-authoritative, and will only manage the labels present in your configuration.
        /// Please refer to the field `effective_labels` for all of the labels present on the resource.
        pub labels: pulumi_wasm_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// Location of the resource.
        pub location: pulumi_wasm_rust::Output<String>,
        /// Identifier. The name of the service instance.
        pub name: pulumi_wasm_rust::Output<String>,
        /// The ID of the project in which the resource belongs.
        /// If it is not provided, the provider project is used.
        pub project: pulumi_wasm_rust::Output<String>,
        /// The combination of labels configured directly on the resource
        /// and default labels configured on the provider.
        pub pulumi_labels: pulumi_wasm_rust::Output<
            std::collections::HashMap<String, String>,
        >,
        /// Whether the service instance is currently reconciling. True if the current state of the resource does not match the intended state, and the system is working to reconcile them, whether or not the change was user initiated.
        pub reconciling: pulumi_wasm_rust::Output<bool>,
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
        pub requested_state: pulumi_wasm_rust::Output<String>,
        /// Requested service account to associate with ServiceInstance.
        pub service_account: pulumi_wasm_rust::Output<Option<String>>,
        /// Id of the service instance.
        ///
        ///
        /// - - -
        pub service_instance_id: pulumi_wasm_rust::Output<String>,
        /// Spark-specific service instance configuration.
        pub spark_service_instance_config: pulumi_wasm_rust::Output<
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
        pub state: pulumi_wasm_rust::Output<String>,
        /// A message explaining the current state.
        pub state_message: pulumi_wasm_rust::Output<String>,
        /// System generated unique identifier for this service instance, formatted as UUID4.
        pub uid: pulumi_wasm_rust::Output<String>,
        /// The timestamp when the resource was most recently updated.
        pub update_time: pulumi_wasm_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(name: &str, args: GdcServiceInstanceArgs) -> GdcServiceInstanceResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let display_name_binding = args.display_name.get_inner();
        let gdce_cluster_binding = args.gdce_cluster.get_inner();
        let labels_binding = args.labels.get_inner();
        let location_binding = args.location.get_inner();
        let project_binding = args.project.get_inner();
        let service_account_binding = args.service_account.get_inner();
        let service_instance_id_binding = args.service_instance_id.get_inner();
        let spark_service_instance_config_binding = args
            .spark_service_instance_config
            .get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "gcp:dataproc/gdcServiceInstance:GdcServiceInstance".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "displayName".into(),
                    value: &display_name_binding,
                },
                register_interface::ObjectField {
                    name: "gdceCluster".into(),
                    value: &gdce_cluster_binding,
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
                    name: "serviceAccount".into(),
                    value: &service_account_binding,
                },
                register_interface::ObjectField {
                    name: "serviceInstanceId".into(),
                    value: &service_instance_id_binding,
                },
                register_interface::ObjectField {
                    name: "sparkServiceInstanceConfig".into(),
                    value: &spark_service_instance_config_binding,
                },
            ]),
            results: Vec::from([
                register_interface::ResultField {
                    name: "createTime".into(),
                },
                register_interface::ResultField {
                    name: "displayName".into(),
                },
                register_interface::ResultField {
                    name: "effectiveLabels".into(),
                },
                register_interface::ResultField {
                    name: "effectiveServiceAccount".into(),
                },
                register_interface::ResultField {
                    name: "gdceCluster".into(),
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
                    name: "project".into(),
                },
                register_interface::ResultField {
                    name: "pulumiLabels".into(),
                },
                register_interface::ResultField {
                    name: "reconciling".into(),
                },
                register_interface::ResultField {
                    name: "requestedState".into(),
                },
                register_interface::ResultField {
                    name: "serviceAccount".into(),
                },
                register_interface::ResultField {
                    name: "serviceInstanceId".into(),
                },
                register_interface::ResultField {
                    name: "sparkServiceInstanceConfig".into(),
                },
                register_interface::ResultField {
                    name: "state".into(),
                },
                register_interface::ResultField {
                    name: "stateMessage".into(),
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
        GdcServiceInstanceResult {
            create_time: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("createTime").unwrap(),
            ),
            display_name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("displayName").unwrap(),
            ),
            effective_labels: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("effectiveLabels").unwrap(),
            ),
            effective_service_account: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("effectiveServiceAccount").unwrap(),
            ),
            gdce_cluster: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("gdceCluster").unwrap(),
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
            project: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("project").unwrap(),
            ),
            pulumi_labels: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("pulumiLabels").unwrap(),
            ),
            reconciling: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("reconciling").unwrap(),
            ),
            requested_state: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("requestedState").unwrap(),
            ),
            service_account: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("serviceAccount").unwrap(),
            ),
            service_instance_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("serviceInstanceId").unwrap(),
            ),
            spark_service_instance_config: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("sparkServiceInstanceConfig").unwrap(),
            ),
            state: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("state").unwrap(),
            ),
            state_message: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("stateMessage").unwrap(),
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
