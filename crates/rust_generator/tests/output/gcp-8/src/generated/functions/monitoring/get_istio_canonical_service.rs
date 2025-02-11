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
        context: &pulumi_gestalt_rust::Context,
        args: GetIstioCanonicalServiceArgs,
    ) -> GetIstioCanonicalServiceResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let canonical_service_binding = args.canonical_service.get_output(context);
        let canonical_service_namespace_binding = args
            .canonical_service_namespace
            .get_output(context);
        let mesh_uid_binding = args.mesh_uid.get_output(context);
        let project_binding = args.project.get_output(context);
        let request = pulumi_gestalt_rust::InvokeResourceRequest {
            token: "gcp:monitoring/getIstioCanonicalService:getIstioCanonicalService"
                .into(),
            version: super::super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "canonicalService".into(),
                    value: &canonical_service_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "canonicalServiceNamespace".into(),
                    value: &canonical_service_namespace_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "meshUid".into(),
                    value: &mesh_uid_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "project".into(),
                    value: &project_binding.drop_type(),
                },
            ],
        };
        let o = context.invoke_resource(request);
        GetIstioCanonicalServiceResult {
            canonical_service: o.get_field("canonicalService"),
            canonical_service_namespace: o.get_field("canonicalServiceNamespace"),
            display_name: o.get_field("displayName"),
            id: o.get_field("id"),
            mesh_uid: o.get_field("meshUid"),
            name: o.get_field("name"),
            project: o.get_field("project"),
            service_id: o.get_field("serviceId"),
            telemetries: o.get_field("telemetries"),
            user_labels: o.get_field("userLabels"),
        }
    }
}
