#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod get_mesh_istio_service {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GetMeshIstioServiceArgs {
        /// Identifier for the mesh in which this Istio service is defined.
        /// Corresponds to the meshUid metric label in Istio metrics.
        #[builder(into)]
        pub mesh_uid: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The ID of the project in which the resource belongs.
        /// If it is not provided, the provider project is used.
        #[builder(into, default)]
        pub project: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The name of the Istio service underlying this service.
        /// Corresponds to the destination_service_name metric label in Istio metrics.
        ///
        /// - - -
        ///
        /// Other optional fields include:
        #[builder(into)]
        pub service_name: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The namespace of the Istio service underlying this service.
        /// Corresponds to the destination_service_namespace metric label in Istio metrics.
        #[builder(into)]
        pub service_namespace: pulumi_gestalt_rust::InputOrOutput<String>,
    }
    #[allow(dead_code)]
    pub struct GetMeshIstioServiceResult {
        /// Name used for UI elements listing this (Monitoring) Service.
        pub display_name: pulumi_gestalt_rust::Output<String>,
        /// The provider-assigned unique ID for this managed resource.
        pub id: pulumi_gestalt_rust::Output<String>,
        pub mesh_uid: pulumi_gestalt_rust::Output<String>,
        /// The full REST resource name for this channel. The syntax is:
        /// `projects/[PROJECT_ID]/services/[SERVICE_ID]`.
        pub name: pulumi_gestalt_rust::Output<String>,
        pub project: pulumi_gestalt_rust::Output<Option<String>>,
        pub service_id: pulumi_gestalt_rust::Output<String>,
        pub service_name: pulumi_gestalt_rust::Output<String>,
        pub service_namespace: pulumi_gestalt_rust::Output<String>,
        /// Configuration for how to query telemetry on the Service. Structure is documented below.
        pub telemetries: pulumi_gestalt_rust::Output<
            Vec<super::super::super::types::monitoring::GetMeshIstioServiceTelemetry>,
        >,
        pub user_labels: pulumi_gestalt_rust::Output<
            std::collections::HashMap<String, String>,
        >,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn invoke(
        context: &pulumi_gestalt_rust::PulumiContext,
        args: GetMeshIstioServiceArgs,
    ) -> GetMeshIstioServiceResult {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::register_interface;
        use std::collections::HashMap;
        let mesh_uid_binding_1 = args.mesh_uid.get_output(context);
        let mesh_uid_binding = mesh_uid_binding_1.get_inner();
        let project_binding_1 = args.project.get_output(context);
        let project_binding = project_binding_1.get_inner();
        let service_name_binding_1 = args.service_name.get_output(context);
        let service_name_binding = service_name_binding_1.get_inner();
        let service_namespace_binding_1 = args.service_namespace.get_output(context);
        let service_namespace_binding = service_namespace_binding_1.get_inner();
        let request = register_interface::ResourceInvokeRequest {
            token: "gcp:monitoring/getMeshIstioService:getMeshIstioService".into(),
            version: super::super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "meshUid".into(),
                    value: &mesh_uid_binding,
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
        GetMeshIstioServiceResult {
            display_name: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("displayName"),
            ),
            id: pulumi_gestalt_rust::__private::into_domain(o.extract_field("id")),
            mesh_uid: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("meshUid"),
            ),
            name: pulumi_gestalt_rust::__private::into_domain(o.extract_field("name")),
            project: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("project"),
            ),
            service_id: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("serviceId"),
            ),
            service_name: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("serviceName"),
            ),
            service_namespace: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("serviceNamespace"),
            ),
            telemetries: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("telemetries"),
            ),
            user_labels: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("userLabels"),
            ),
        }
    }
}
