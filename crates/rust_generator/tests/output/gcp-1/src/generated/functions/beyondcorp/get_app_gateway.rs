#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod get_app_gateway {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GetAppGatewayArgs {
        /// The name of the App Gateway.
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
    pub struct GetAppGatewayResult {
        pub allocated_connections: pulumi_gestalt_rust::Output<
            Vec<super::super::super::types::beyondcorp::GetAppGatewayAllocatedConnection>,
        >,
        pub display_name: pulumi_gestalt_rust::Output<String>,
        pub effective_labels: pulumi_gestalt_rust::Output<
            std::collections::HashMap<String, String>,
        >,
        pub host_type: pulumi_gestalt_rust::Output<String>,
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
        pub state: pulumi_gestalt_rust::Output<String>,
        pub type_: pulumi_gestalt_rust::Output<String>,
        pub uri: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn invoke(
        context: &pulumi_gestalt_rust::Context,
        args: GetAppGatewayArgs,
    ) -> GetAppGatewayResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let name_binding = args.name.get_output(context);
        let project_binding = args.project.get_output(context);
        let region_binding = args.region.get_output(context);
        let request = pulumi_gestalt_rust::InvokeResourceRequest {
            token: "gcp:beyondcorp/getAppGateway:getAppGateway".into(),
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
        GetAppGatewayResult {
            allocated_connections: o.get_field("allocatedConnections"),
            display_name: o.get_field("displayName"),
            effective_labels: o.get_field("effectiveLabels"),
            host_type: o.get_field("hostType"),
            id: o.get_field("id"),
            labels: o.get_field("labels"),
            name: o.get_field("name"),
            project: o.get_field("project"),
            pulumi_labels: o.get_field("pulumiLabels"),
            region: o.get_field("region"),
            state: o.get_field("state"),
            type_: o.get_field("type"),
            uri: o.get_field("uri"),
        }
    }
}
