#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod get_zones {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GetZonesArgs {
        /// Project from which to list available zones. Defaults to project declared in the provider.
        #[builder(into, default)]
        pub project: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Region from which to list available zones. Defaults to region declared in the provider.
        #[builder(into, default)]
        pub region: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Allows to filter list of zones based on their current status. Status can be either `UP` or `DOWN`.
        /// Defaults to no filtering (all available zones - both `UP` and `DOWN`).
        #[builder(into, default)]
        pub status: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
    }
    #[allow(dead_code)]
    pub struct GetZonesResult {
        /// The provider-assigned unique ID for this managed resource.
        pub id: pulumi_gestalt_rust::Output<String>,
        /// A list of zones available in the given region
        pub names: pulumi_gestalt_rust::Output<Vec<String>>,
        pub project: pulumi_gestalt_rust::Output<String>,
        pub region: pulumi_gestalt_rust::Output<Option<String>>,
        pub status: pulumi_gestalt_rust::Output<Option<String>>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn invoke(
        context: &pulumi_gestalt_rust::Context,
        args: GetZonesArgs,
    ) -> GetZonesResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let project_binding = args.project.get_output(context);
        let region_binding = args.region.get_output(context);
        let status_binding = args.status.get_output(context);
        let request = pulumi_gestalt_rust::InvokeResourceRequest {
            token: "gcp:compute/getZones:getZones".into(),
            version: super::super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "project".into(),
                    value: &project_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "region".into(),
                    value: &region_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "status".into(),
                    value: &status_binding.drop_type(),
                },
            ],
        };
        let o = context.invoke_resource(request);
        GetZonesResult {
            id: o.get_field("id"),
            names: o.get_field("names"),
            project: o.get_field("project"),
            region: o.get_field("region"),
            status: o.get_field("status"),
        }
    }
}
