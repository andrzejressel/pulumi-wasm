#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod get_service {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GetServiceArgs {
        /// The location of the cloud run instance. eg us-central1
        ///
        /// - - -
        #[builder(into)]
        pub location: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The name of the Cloud Run Service.
        #[builder(into)]
        pub name: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The project in which the resource belongs. If it
        /// is not provided, the provider project is used.
        #[builder(into, default)]
        pub project: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
    }
    #[allow(dead_code)]
    pub struct GetServiceResult {
        pub autogenerate_revision_name: pulumi_gestalt_rust::Output<bool>,
        /// The provider-assigned unique ID for this managed resource.
        pub id: pulumi_gestalt_rust::Output<String>,
        pub location: pulumi_gestalt_rust::Output<String>,
        pub metadatas: pulumi_gestalt_rust::Output<
            Vec<super::super::super::types::cloudrun::GetServiceMetadata>,
        >,
        pub name: pulumi_gestalt_rust::Output<String>,
        pub project: pulumi_gestalt_rust::Output<Option<String>>,
        pub statuses: pulumi_gestalt_rust::Output<
            Vec<super::super::super::types::cloudrun::GetServiceStatus>,
        >,
        pub templates: pulumi_gestalt_rust::Output<
            Vec<super::super::super::types::cloudrun::GetServiceTemplate>,
        >,
        pub traffics: pulumi_gestalt_rust::Output<
            Vec<super::super::super::types::cloudrun::GetServiceTraffic>,
        >,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn invoke(
        context: &pulumi_gestalt_rust::Context,
        args: GetServiceArgs,
    ) -> GetServiceResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let location_binding = args.location.get_output(context);
        let name_binding = args.name.get_output(context);
        let project_binding = args.project.get_output(context);
        let request = pulumi_gestalt_rust::InvokeResourceRequest {
            token: "gcp:cloudrun/getService:getService".into(),
            version: super::super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "location".into(),
                    value: location_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "name".into(),
                    value: name_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "project".into(),
                    value: project_binding.get_id(),
                },
            ],
        };
        let o = context.invoke_resource(request);
        GetServiceResult {
            autogenerate_revision_name: o.get_field("autogenerateRevisionName"),
            id: o.get_field("id"),
            location: o.get_field("location"),
            metadatas: o.get_field("metadatas"),
            name: o.get_field("name"),
            project: o.get_field("project"),
            statuses: o.get_field("statuses"),
            templates: o.get_field("templates"),
            traffics: o.get_field("traffics"),
        }
    }
}
