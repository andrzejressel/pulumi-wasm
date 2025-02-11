#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod get_location {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GetLocationArgs {
        /// Code for the location to retrieve.
        #[builder(into)]
        pub location_code: pulumi_gestalt_rust::InputOrOutput<String>,
    }
    #[allow(dead_code)]
    pub struct GetLocationResult {
        /// The available MAC Security (MACsec) port speeds for the location.
        pub available_macsec_port_speeds: pulumi_gestalt_rust::Output<Vec<String>>,
        /// The available port speeds for the location.
        pub available_port_speeds: pulumi_gestalt_rust::Output<Vec<String>>,
        /// Names of the service providers for the location.
        pub available_providers: pulumi_gestalt_rust::Output<Vec<String>>,
        /// The provider-assigned unique ID for this managed resource.
        pub id: pulumi_gestalt_rust::Output<String>,
        pub location_code: pulumi_gestalt_rust::Output<String>,
        /// Name of the location. This includes the name of the colocation partner and the physical site of the building.
        pub location_name: pulumi_gestalt_rust::Output<String>,
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
        let location_code_binding = args.location_code.get_output(context);
        let request = pulumi_gestalt_rust::InvokeResourceRequest {
            token: "aws:directconnect/getLocation:getLocation".into(),
            version: super::super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "locationCode".into(),
                    value: &location_code_binding.drop_type(),
                },
            ],
        };
        let o = context.invoke_resource(request);
        GetLocationResult {
            available_macsec_port_speeds: o.get_field("availableMacsecPortSpeeds"),
            available_port_speeds: o.get_field("availablePortSpeeds"),
            available_providers: o.get_field("availableProviders"),
            id: o.get_field("id"),
            location_code: o.get_field("locationCode"),
            location_name: o.get_field("locationName"),
        }
    }
}
