#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod get_machine_types {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GetMachineTypesArgs {
        /// A filter expression that filters machine types listed in the response.
        #[builder(into, default)]
        pub filter: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Project from which to list available zones. Defaults to project declared in the provider.
        #[builder(into, default)]
        pub project: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Zone from which to list machine types.
        #[builder(into, default)]
        pub zone: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
    }
    #[allow(dead_code)]
    pub struct GetMachineTypesResult {
        pub filter: pulumi_gestalt_rust::Output<Option<String>>,
        /// The provider-assigned unique ID for this managed resource.
        pub id: pulumi_gestalt_rust::Output<String>,
        /// The list of machine types matching the provided filter. Structure is documented below.
        pub machine_types: pulumi_gestalt_rust::Output<
            Vec<super::super::super::types::compute::GetMachineTypesMachineType>,
        >,
        pub project: pulumi_gestalt_rust::Output<String>,
        pub zone: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn invoke(
        context: &pulumi_gestalt_rust::Context,
        args: GetMachineTypesArgs,
    ) -> GetMachineTypesResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let filter_binding = args.filter.get_output(context);
        let project_binding = args.project.get_output(context);
        let zone_binding = args.zone.get_output(context);
        let request = pulumi_gestalt_rust::InvokeResourceRequest {
            token: "gcp:compute/getMachineTypes:getMachineTypes".into(),
            version: super::super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "filter".into(),
                    value: filter_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "project".into(),
                    value: project_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "zone".into(),
                    value: zone_binding.get_id(),
                },
            ],
        };
        let o = context.invoke_resource(request);
        GetMachineTypesResult {
            filter: o.get_field("filter"),
            id: o.get_field("id"),
            machine_types: o.get_field("machineTypes"),
            project: o.get_field("project"),
            zone: o.get_field("zone"),
        }
    }
}
