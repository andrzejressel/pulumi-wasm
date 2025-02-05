pub mod get_cluster_istio_service {
    #[derive(pulumi_wasm_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GetClusterIstioServiceArgs {
        /// The name of the Kubernetes cluster in which this Istio service
        /// is defined. Corresponds to the clusterName resource label in k8s_cluster resources.
        #[builder(into)]
        pub cluster_name: pulumi_wasm_rust::InputOrOutput<String>,
        /// The location of the Kubernetes cluster in which this Istio service
        /// is defined. Corresponds to the location resource label in k8s_cluster resources.
        #[builder(into)]
        pub location: pulumi_wasm_rust::InputOrOutput<String>,
        /// The ID of the project in which the resource belongs.
        /// If it is not provided, the provider project is used.
        #[builder(into, default)]
        pub project: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        /// The name of the Istio service underlying this service.
        /// Corresponds to the destination_service_name metric label in Istio metrics.
        ///
        /// - - -
        ///
        /// Other optional fields include:
        #[builder(into)]
        pub service_name: pulumi_wasm_rust::InputOrOutput<String>,
        /// The namespace of the Istio service underlying this service.
        /// Corresponds to the destination_service_namespace metric label in Istio metrics.
        #[builder(into)]
        pub service_namespace: pulumi_wasm_rust::InputOrOutput<String>,
    }
    #[allow(dead_code)]
    pub struct GetClusterIstioServiceResult {
        pub cluster_name: pulumi_wasm_rust::Output<String>,
        /// Name used for UI elements listing this (Monitoring) Service.
        pub display_name: pulumi_wasm_rust::Output<String>,
        /// The provider-assigned unique ID for this managed resource.
        pub id: pulumi_wasm_rust::Output<String>,
        pub location: pulumi_wasm_rust::Output<String>,
        /// The full REST resource name for this channel. The syntax is:
        /// `projects/[PROJECT_ID]/services/[SERVICE_ID]`.
        pub name: pulumi_wasm_rust::Output<String>,
        pub project: pulumi_wasm_rust::Output<Option<String>>,
        pub service_id: pulumi_wasm_rust::Output<String>,
        pub service_name: pulumi_wasm_rust::Output<String>,
        pub service_namespace: pulumi_wasm_rust::Output<String>,
        /// Configuration for how to query telemetry on the Service. Structure is documented below.
        pub telemetries: pulumi_wasm_rust::Output<
            Vec<super::super::super::types::monitoring::GetClusterIstioServiceTelemetry>,
        >,
        pub user_labels: pulumi_wasm_rust::Output<
            std::collections::HashMap<String, String>,
        >,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn invoke(
        context: &pulumi_wasm_rust::PulumiContext,
        args: GetClusterIstioServiceArgs,
    ) -> GetClusterIstioServiceResult {
        use pulumi_wasm_rust::__private::pulumi_gestalt_adapter_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let cluster_name_binding = args.cluster_name.get_output(context).get_inner();
        let location_binding = args.location.get_output(context).get_inner();
        let project_binding = args.project.get_output(context).get_inner();
        let service_name_binding = args.service_name.get_output(context).get_inner();
        let service_namespace_binding = args
            .service_namespace
            .get_output(context)
            .get_inner();
        let request = register_interface::ResourceInvokeRequest {
            token: "gcp:monitoring/getClusterIstioService:getClusterIstioService".into(),
            version: super::super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "clusterName".into(),
                    value: &cluster_name_binding,
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
                    name: "serviceName".into(),
                    value: &service_name_binding,
                },
                register_interface::ObjectField {
                    name: "serviceNamespace".into(),
                    value: &service_namespace_binding,
                },
            ]),
        };
        let o = register_interface::invoke(context.get_inner(), &request);
        GetClusterIstioServiceResult {
            cluster_name: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("clusterName"),
            ),
            display_name: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("displayName"),
            ),
            id: pulumi_wasm_rust::__private::into_domain(o.extract_field("id")),
            location: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("location"),
            ),
            name: pulumi_wasm_rust::__private::into_domain(o.extract_field("name")),
            project: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("project"),
            ),
            service_id: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("serviceId"),
            ),
            service_name: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("serviceName"),
            ),
            service_namespace: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("serviceNamespace"),
            ),
            telemetries: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("telemetries"),
            ),
            user_labels: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("userLabels"),
            ),
        }
    }
}
