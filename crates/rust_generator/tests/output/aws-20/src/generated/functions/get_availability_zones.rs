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
        context: &pulumi_gestalt_rust::PulumiContext,
        args: GetAvailabilityZonesArgs,
    ) -> GetAvailabilityZonesResult {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::register_interface;
        use std::collections::HashMap;
        let all_availability_zones_binding = args
            .all_availability_zones
            .get_output(context)
            .get_inner();
        let exclude_names_binding = args.exclude_names.get_output(context).get_inner();
        let exclude_zone_ids_binding = args
            .exclude_zone_ids
            .get_output(context)
            .get_inner();
        let filters_binding = args.filters.get_output(context).get_inner();
        let state_binding = args.state.get_output(context).get_inner();
        let request = register_interface::ResourceInvokeRequest {
            token: "aws:index/getAvailabilityZones:getAvailabilityZones".into(),
            version: super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "allAvailabilityZones".into(),
                    value: &all_availability_zones_binding,
                },
                register_interface::ObjectField {
                    name: "excludeNames".into(),
                    value: &exclude_names_binding,
                },
                register_interface::ObjectField {
                    name: "excludeZoneIds".into(),
                    value: &exclude_zone_ids_binding,
                },
                register_interface::ObjectField {
                    name: "filters".into(),
                    value: &filters_binding,
                },
                register_interface::ObjectField {
                    name: "state".into(),
                    value: &state_binding,
                },
            ]),
        };
        let o = register_interface::invoke(context.get_inner(), &request);
        GetAvailabilityZonesResult {
            all_availability_zones: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("allAvailabilityZones"),
            ),
            exclude_names: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("excludeNames"),
            ),
            exclude_zone_ids: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("excludeZoneIds"),
            ),
            filters: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("filters"),
            ),
            group_names: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("groupNames"),
            ),
            id: pulumi_gestalt_rust::__private::into_domain(o.extract_field("id")),
            names: pulumi_gestalt_rust::__private::into_domain(o.extract_field("names")),
            state: pulumi_gestalt_rust::__private::into_domain(o.extract_field("state")),
            zone_ids: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("zoneIds"),
            ),
        }
    }
}
