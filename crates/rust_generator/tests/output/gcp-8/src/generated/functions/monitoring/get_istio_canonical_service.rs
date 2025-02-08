#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod get_istio_canonical_service {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GetIstioCanonicalServiceArgs {
        /// The name of the canonical service underlying this service.
        /// Corresponds to the destination_canonical_service_name metric label in label in Istio metrics.
        ///
        /// - - -
        ///
        /// Other optional fields include:
        #[builder(into)]
        pub canonical_service: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The namespace of the canonical service underlying this service.
        /// Corresponds to the destination_canonical_service_namespace metric label in Istio metrics.
        #[builder(into)]
        pub canonical_service_namespace: pulumi_gestalt_rust::InputOrOutput<String>,
        /// Identifier for the mesh in which this Istio service is defined.
        /// Corresponds to the meshUid metric label in Istio metrics.
        #[builder(into)]
        pub mesh_uid: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The ID of the project in which the resource belongs.
        /// If it is not provided, the provider project is used.
        #[builder(into, default)]
        pub project: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
    }
    #[allow(dead_code)]
    pub struct GetIstioCanonicalServiceResult {
        pub canonical_service: pulumi_gestalt_rust::Output<String>,
        pub canonical_service_namespace: pulumi_gestalt_rust::Output<String>,
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
        /// Configuration for how to query telemetry on the Service. Structure is documented below.
        pub telemetries: pulumi_gestalt_rust::Output<
            Vec<
                super::super::super::types::monitoring::GetIstioCanonicalServiceTelemetry,
            >,
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
        args: GetIstioCanonicalServiceArgs,
    ) -> GetIstioCanonicalServiceResult {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::register_interface;
        use std::collections::HashMap;
        let canonical_service_binding = args
            .canonical_service
            .get_output(context)
            .get_inner();
        let canonical_service_namespace_binding = args
            .canonical_service_namespace
            .get_output(context)
            .get_inner();
        let mesh_uid_binding = args.mesh_uid.get_output(context).get_inner();
        let project_binding = args.project.get_output(context).get_inner();
        let request = register_interface::ResourceInvokeRequest {
            token: "gcp:monitoring/getIstioCanonicalService:getIstioCanonicalService"
                .into(),
            version: super::super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "canonicalService".into(),
                    value: &canonical_service_binding,
                },
                register_interface::ObjectField {
                    name: "canonicalServiceNamespace".into(),
                    value: &canonical_service_namespace_binding,
                },
                register_interface::ObjectField {
                    name: "meshUid".into(),
                    value: &mesh_uid_binding,
                },
                register_interface::ObjectField {
                    name: "project".into(),
                    value: &project_binding,
                },
            ]),
        };
        let o = register_interface::invoke(context.get_inner(), &request);
        GetIstioCanonicalServiceResult {
            canonical_service: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("canonicalService"),
            ),
            canonical_service_namespace: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("canonicalServiceNamespace"),
            ),
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
            telemetries: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("telemetries"),
            ),
            user_labels: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("userLabels"),
            ),
        }
    }
}
