#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod get_availability_zones {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GetAvailabilityZonesArgs {
        /// Set to `true` to include all Availability Zones and Local Zones regardless of your opt in status.
        #[builder(into, default)]
        pub all_availability_zones: pulumi_gestalt_rust::InputOrOutput<Option<bool>>,
        /// List of Availability Zone names to exclude.
        #[builder(into, default)]
        pub exclude_names: pulumi_gestalt_rust::InputOrOutput<Option<Vec<String>>>,
        /// List of Availability Zone IDs to exclude.
        #[builder(into, default)]
        pub exclude_zone_ids: pulumi_gestalt_rust::InputOrOutput<Option<Vec<String>>>,
        /// Configuration block(s) for filtering. Detailed below.
        #[builder(into, default)]
        pub filters: pulumi_gestalt_rust::InputOrOutput<
            Option<Vec<super::super::types::GetAvailabilityZonesFilter>>,
        >,
        /// Allows to filter list of Availability Zones based on their
        /// current state. Can be either `"available"`, `"information"`, `"impaired"` or
        /// `"unavailable"`. By default the list includes a complete set of Availability Zones
        /// to which the underlying AWS account has access, regardless of their state.
        #[builder(into, default)]
        pub state: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
    }
    #[allow(dead_code)]
    pub struct GetAvailabilityZonesResult {
        pub all_availability_zones: pulumi_gestalt_rust::Output<Option<bool>>,
        pub exclude_names: pulumi_gestalt_rust::Output<Option<Vec<String>>>,
        pub exclude_zone_ids: pulumi_gestalt_rust::Output<Option<Vec<String>>>,
        pub filters: pulumi_gestalt_rust::Output<
            Option<Vec<super::super::types::GetAvailabilityZonesFilter>>,
        >,
        /// A set of the Availability Zone Group names. For Availability Zones, this is the same value as the Region name. For Local Zones, the name of the associated group, for example `us-west-2-lax-1`.
        pub group_names: pulumi_gestalt_rust::Output<Vec<String>>,
        /// The provider-assigned unique ID for this managed resource.
        pub id: pulumi_gestalt_rust::Output<String>,
        /// List of the Availability Zone names available to the account.
        pub names: pulumi_gestalt_rust::Output<Vec<String>>,
        pub state: pulumi_gestalt_rust::Output<Option<String>>,
        /// List of the Availability Zone IDs available to the account.
        pub zone_ids: pulumi_gestalt_rust::Output<Vec<String>>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn invoke(
        context: &pulumi_gestalt_rust::Context,
        args: GetAvailabilityZonesArgs,
    ) -> GetAvailabilityZonesResult {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::register_interface;
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let all_availability_zones_binding = args
            .all_availability_zones
            .get_output(context);
        let exclude_names_binding = args.exclude_names.get_output(context);
        let exclude_zone_ids_binding = args.exclude_zone_ids.get_output(context);
        let filters_binding = args.filters.get_output(context);
        let state_binding = args.state.get_output(context);
        let request = pulumi_gestalt_rust::InvokeResourceRequest {
            token: "aws:index/getAvailabilityZones:getAvailabilityZones".into(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "allAvailabilityZones".into(),
                    value: all_availability_zones_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "excludeNames".into(),
                    value: exclude_names_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "excludeZoneIds".into(),
                    value: exclude_zone_ids_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "filters".into(),
                    value: filters_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "state".into(),
                    value: state_binding.get_id(),
                },
            ],
        };
        let o = context.invoke_resource(request);
        GetAvailabilityZonesResult {
            all_availability_zones: o.get_field("allAvailabilityZones"),
            exclude_names: o.get_field("excludeNames"),
            exclude_zone_ids: o.get_field("excludeZoneIds"),
            filters: o.get_field("filters"),
            group_names: o.get_field("groupNames"),
            id: o.get_field("id"),
            names: o.get_field("names"),
            state: o.get_field("state"),
            zone_ids: o.get_field("zoneIds"),
        }
    }
}
