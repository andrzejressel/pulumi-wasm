#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod get_location {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GetLocationArgs {
        /// Specifies the supported Azure location where the resource exists.
        #[builder(into)]
        pub location: pulumi_gestalt_rust::InputOrOutput<String>,
    }
    #[allow(dead_code)]
    pub struct GetLocationResult {
        /// The display name of the location.
        pub display_name: pulumi_gestalt_rust::Output<String>,
        /// The provider-assigned unique ID for this managed resource.
        pub id: pulumi_gestalt_rust::Output<String>,
        pub location: pulumi_gestalt_rust::Output<String>,
        /// A `zone_mappings` block as defined below.
        pub zone_mappings: pulumi_gestalt_rust::Output<
            Vec<super::super::super::types::core::GetLocationZoneMapping>,
        >,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn invoke(
        context: &pulumi_gestalt_rust::Context,
        args: GetLocationArgs,
    ) -> GetLocationResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let location_binding = args.location.get_output(context);
        let request = pulumi_gestalt_rust::InvokeResourceRequest {
            token: "azure:core/getLocation:getLocation".into(),
            version: super::super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "location".into(),
                    value: location_binding.get_id(),
                },
            ],
        };
        let o = context.invoke_resource(request);
        GetLocationResult {
            display_name: o.get_field("displayName"),
            id: o.get_field("id"),
            location: o.get_field("location"),
            zone_mappings: o.get_field("zoneMappings"),
        }
    }
}
