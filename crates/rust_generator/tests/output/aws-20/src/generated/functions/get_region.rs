#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod get_region {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GetRegionArgs {
        /// EC2 endpoint of the region to select.
        #[builder(into, default)]
        pub endpoint: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        #[builder(into, default)]
        pub id: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Full name of the region to select.
        #[builder(into, default)]
        pub name: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
    }
    #[allow(dead_code)]
    pub struct GetRegionResult {
        /// Region's description in this format: "Location (Region name)".
        pub description: pulumi_gestalt_rust::Output<String>,
        /// EC2 endpoint for the selected region.
        pub endpoint: pulumi_gestalt_rust::Output<String>,
        pub id: pulumi_gestalt_rust::Output<String>,
        /// Name of the selected region.
        pub name: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn invoke(
        context: &pulumi_gestalt_rust::Context,
        args: GetRegionArgs,
    ) -> GetRegionResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let endpoint_binding = args.endpoint.get_output(context);
        let id_binding = args.id.get_output(context);
        let name_binding = args.name.get_output(context);
        let request = pulumi_gestalt_rust::InvokeResourceRequest {
            token: "aws:index/getRegion:getRegion".into(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "endpoint".into(),
                    value: endpoint_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "id".into(),
                    value: id_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "name".into(),
                    value: name_binding.get_id(),
                },
            ],
        };
        let o = context.invoke_resource(request);
        GetRegionResult {
            description: o.get_field("description"),
            endpoint: o.get_field("endpoint"),
            id: o.get_field("id"),
            name: o.get_field("name"),
        }
    }
}
