#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod get_reservation {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GetReservationArgs {
        /// The name of the Compute Reservation.
        #[builder(into)]
        pub name: pulumi_gestalt_rust::InputOrOutput<String>,
        /// Project from which to list the Compute Reservation. Defaults to project declared in the provider.
        #[builder(into, default)]
        pub project: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Zone where the Compute Reservation resides.
        #[builder(into)]
        pub zone: pulumi_gestalt_rust::InputOrOutput<String>,
    }
    #[allow(dead_code)]
    pub struct GetReservationResult {
        pub commitment: pulumi_gestalt_rust::Output<String>,
        pub creation_timestamp: pulumi_gestalt_rust::Output<String>,
        pub description: pulumi_gestalt_rust::Output<String>,
        /// The provider-assigned unique ID for this managed resource.
        pub id: pulumi_gestalt_rust::Output<String>,
        pub name: pulumi_gestalt_rust::Output<String>,
        pub project: pulumi_gestalt_rust::Output<Option<String>>,
        pub self_link: pulumi_gestalt_rust::Output<String>,
        pub share_settings: pulumi_gestalt_rust::Output<
            Vec<super::super::super::types::compute::GetReservationShareSetting>,
        >,
        pub specific_reservation_required: pulumi_gestalt_rust::Output<bool>,
        pub specific_reservations: pulumi_gestalt_rust::Output<
            Vec<super::super::super::types::compute::GetReservationSpecificReservation>,
        >,
        pub status: pulumi_gestalt_rust::Output<String>,
        pub zone: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn invoke(
        context: &pulumi_gestalt_rust::Context,
        args: GetReservationArgs,
    ) -> GetReservationResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let name_binding = args.name.get_output(context);
        let project_binding = args.project.get_output(context);
        let zone_binding = args.zone.get_output(context);
        let request = pulumi_gestalt_rust::InvokeResourceRequest {
            token: "gcp:compute/getReservation:getReservation".into(),
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
                    name: "zone".into(),
                    value: zone_binding.get_id(),
                },
            ],
        };
        let o = context.invoke_resource(request);
        GetReservationResult {
            commitment: o.get_field("commitment"),
            creation_timestamp: o.get_field("creationTimestamp"),
            description: o.get_field("description"),
            id: o.get_field("id"),
            name: o.get_field("name"),
            project: o.get_field("project"),
            self_link: o.get_field("selfLink"),
            share_settings: o.get_field("shareSettings"),
            specific_reservation_required: o.get_field("specificReservationRequired"),
            specific_reservations: o.get_field("specificReservations"),
            status: o.get_field("status"),
            zone: o.get_field("zone"),
        }
    }
}
