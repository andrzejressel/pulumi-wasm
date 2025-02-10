#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod get_app_connection {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GetAppConnectionArgs {
        /// The name of the App Connection.
        ///
        /// - - -
        #[builder(into)]
        pub name: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The project in which the resource belongs. If it
        /// is not provided, the provider project is used.
        #[builder(into, default)]
        pub project: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The region in which the resource belongs. If it
        /// is not provided, the provider region is used.
        #[builder(into, default)]
        pub region: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
    }
    #[allow(dead_code)]
    pub struct GetAppConnectionResult {
        pub application_endpoints: pulumi_gestalt_rust::Output<
            Vec<
                super::super::super::types::beyondcorp::GetAppConnectionApplicationEndpoint,
            >,
        >,
        pub connectors: pulumi_gestalt_rust::Output<Vec<String>>,
        pub display_name: pulumi_gestalt_rust::Output<String>,
        pub effective_labels: pulumi_gestalt_rust::Output<
            std::collections::HashMap<String, String>,
        >,
        pub gateways: pulumi_gestalt_rust::Output<
            Vec<super::super::super::types::beyondcorp::GetAppConnectionGateway>,
        >,
        /// The provider-assigned unique ID for this managed resource.
        pub id: pulumi_gestalt_rust::Output<String>,
        pub labels: pulumi_gestalt_rust::Output<
            std::collections::HashMap<String, String>,
        >,
        pub name: pulumi_gestalt_rust::Output<String>,
        pub project: pulumi_gestalt_rust::Output<Option<String>>,
        pub pulumi_labels: pulumi_gestalt_rust::Output<
            std::collections::HashMap<String, String>,
        >,
        pub region: pulumi_gestalt_rust::Output<Option<String>>,
        pub type_: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn invoke(
        context: &pulumi_gestalt_rust::Context,
        args: GetAppConnectionArgs,
    ) -> GetAppConnectionResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let name_binding = args.name.get_output(context);
        let project_binding = args.project.get_output(context);
        let region_binding = args.region.get_output(context);
        let request = pulumi_gestalt_rust::InvokeResourceRequest {
            token: "gcp:beyondcorp/getAppConnection:getAppConnection".into(),
            version: super::super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "name".into(),
                    value: name_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "project".into(),
                    value: project_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "region".into(),
                    value: region_binding.get_id(),
                },
            ],
        };
        let o = context.invoke_resource(request);
        GetAppConnectionResult {
            application_endpoints: o.get_field("applicationEndpoints"),
            connectors: o.get_field("connectors"),
            display_name: o.get_field("displayName"),
            effective_labels: o.get_field("effectiveLabels"),
            gateways: o.get_field("gateways"),
            id: o.get_field("id"),
            labels: o.get_field("labels"),
            name: o.get_field("name"),
            project: o.get_field("project"),
            pulumi_labels: o.get_field("pulumiLabels"),
            region: o.get_field("region"),
            type_: o.get_field("type"),
        }
    }
}
