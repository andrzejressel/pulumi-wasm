#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod get_availability_zone {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GetAvailabilityZoneArgs {
        /// Set to `true` to include all Availability Zones and Local Zones regardless of your opt in status.
        #[builder(into, default)]
        pub all_availability_zones: pulumi_gestalt_rust::InputOrOutput<Option<bool>>,
        /// Configuration block(s) for filtering. Detailed below.
        #[builder(into, default)]
        pub filters: pulumi_gestalt_rust::InputOrOutput<
            Option<Vec<super::super::types::GetAvailabilityZoneFilter>>,
        >,
        /// Full name of the availability zone to select.
        #[builder(into, default)]
        pub name: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Specific availability zone state to require. May be any of `"available"`, `"information"` or `"impaired"`.
        #[builder(into, default)]
        pub state: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Zone ID of the availability zone to select.
        #[builder(into, default)]
        pub zone_id: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
    }
    #[allow(dead_code)]
    pub struct GetAvailabilityZoneResult {
        pub all_availability_zones: pulumi_gestalt_rust::Output<Option<bool>>,
        pub filters: pulumi_gestalt_rust::Output<
            Option<Vec<super::super::types::GetAvailabilityZoneFilter>>,
        >,
        /// For Availability Zones, this is the same value as the Region name. For Local Zones, the name of the associated group, for example `us-west-2-lax-1`.
        pub group_name: pulumi_gestalt_rust::Output<String>,
        /// The provider-assigned unique ID for this managed resource.
        pub id: pulumi_gestalt_rust::Output<String>,
        pub name: pulumi_gestalt_rust::Output<String>,
        /// Part of the AZ name that appears after the region name, uniquely identifying the AZ within its region.
        /// For Availability Zones this is usually a single letter, for example `a` for the `us-west-2a` zone.
        /// For Local and Wavelength Zones this is a longer string, for example `wl1-sfo-wlz-1` for the `us-west-2-wl1-sfo-wlz-1` zone.
        pub name_suffix: pulumi_gestalt_rust::Output<String>,
        /// The name of the location from which the address is advertised.
        pub network_border_group: pulumi_gestalt_rust::Output<String>,
        /// For Availability Zones, this always has the value of `opt-in-not-required`. For Local Zones, this is the opt in status. The possible values are `opted-in` and `not-opted-in`.
        pub opt_in_status: pulumi_gestalt_rust::Output<String>,
        /// ID of the zone that handles some of the Local Zone or Wavelength Zone control plane operations, such as API calls.
        pub parent_zone_id: pulumi_gestalt_rust::Output<String>,
        /// Name of the zone that handles some of the Local Zone or Wavelength Zone control plane operations, such as API calls.
        pub parent_zone_name: pulumi_gestalt_rust::Output<String>,
        /// Region where the selected availability zone resides. This is always the region selected on the provider, since this data source searches only within that region.
        pub region: pulumi_gestalt_rust::Output<String>,
        pub state: pulumi_gestalt_rust::Output<String>,
        pub zone_id: pulumi_gestalt_rust::Output<String>,
        /// Type of zone. Values are `availability-zone`, `local-zone`, and `wavelength-zone`.
        pub zone_type: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn invoke(
        context: &pulumi_gestalt_rust::Context,
        args: GetAvailabilityZoneArgs,
    ) -> GetAvailabilityZoneResult {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::register_interface;
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let all_availability_zones_binding = args
            .all_availability_zones
            .get_output(context);
        let filters_binding = args.filters.get_output(context);
        let name_binding = args.name.get_output(context);
        let state_binding = args.state.get_output(context);
        let zone_id_binding = args.zone_id.get_output(context);
        let request = pulumi_gestalt_rust::InvokeResourceRequest {
            token: "aws:index/getAvailabilityZone:getAvailabilityZone".into(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "allAvailabilityZones".into(),
                    value: all_availability_zones_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "filters".into(),
                    value: filters_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "name".into(),
                    value: name_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "state".into(),
                    value: state_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "zoneId".into(),
                    value: zone_id_binding.get_id(),
                },
            ],
        };
        let o = context.invoke_resource(request);
        GetAvailabilityZoneResult {
            all_availability_zones: o.get_field("allAvailabilityZones"),
            filters: o.get_field("filters"),
            group_name: o.get_field("groupName"),
            id: o.get_field("id"),
            name: o.get_field("name"),
            name_suffix: o.get_field("nameSuffix"),
            network_border_group: o.get_field("networkBorderGroup"),
            opt_in_status: o.get_field("optInStatus"),
            parent_zone_id: o.get_field("parentZoneId"),
            parent_zone_name: o.get_field("parentZoneName"),
            region: o.get_field("region"),
            state: o.get_field("state"),
            zone_id: o.get_field("zoneId"),
            zone_type: o.get_field("zoneType"),
        }
    }
}
