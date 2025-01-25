pub mod get_availability_zone {
    #[derive(pulumi_wasm_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GetAvailabilityZoneArgs {
        /// Set to `true` to include all Availability Zones and Local Zones regardless of your opt in status.
        #[builder(into, default)]
        pub all_availability_zones: pulumi_wasm_rust::InputOrOutput<Option<bool>>,
        /// Configuration block(s) for filtering. Detailed below.
        #[builder(into, default)]
        pub filters: pulumi_wasm_rust::InputOrOutput<
            Option<Vec<super::super::types::GetAvailabilityZoneFilter>>,
        >,
        /// Full name of the availability zone to select.
        #[builder(into, default)]
        pub name: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        /// Specific availability zone state to require. May be any of `"available"`, `"information"` or `"impaired"`.
        #[builder(into, default)]
        pub state: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        /// Zone ID of the availability zone to select.
        #[builder(into, default)]
        pub zone_id: pulumi_wasm_rust::InputOrOutput<Option<String>>,
    }
    #[allow(dead_code)]
    pub struct GetAvailabilityZoneResult {
        pub all_availability_zones: pulumi_wasm_rust::Output<Option<bool>>,
        pub filters: pulumi_wasm_rust::Output<
            Option<Vec<super::super::types::GetAvailabilityZoneFilter>>,
        >,
        /// For Availability Zones, this is the same value as the Region name. For Local Zones, the name of the associated group, for example `us-west-2-lax-1`.
        pub group_name: pulumi_wasm_rust::Output<String>,
        /// The provider-assigned unique ID for this managed resource.
        pub id: pulumi_wasm_rust::Output<String>,
        pub name: pulumi_wasm_rust::Output<String>,
        /// Part of the AZ name that appears after the region name, uniquely identifying the AZ within its region.
        /// For Availability Zones this is usually a single letter, for example `a` for the `us-west-2a` zone.
        /// For Local and Wavelength Zones this is a longer string, for example `wl1-sfo-wlz-1` for the `us-west-2-wl1-sfo-wlz-1` zone.
        pub name_suffix: pulumi_wasm_rust::Output<String>,
        /// The name of the location from which the address is advertised.
        pub network_border_group: pulumi_wasm_rust::Output<String>,
        /// For Availability Zones, this always has the value of `opt-in-not-required`. For Local Zones, this is the opt in status. The possible values are `opted-in` and `not-opted-in`.
        pub opt_in_status: pulumi_wasm_rust::Output<String>,
        /// ID of the zone that handles some of the Local Zone or Wavelength Zone control plane operations, such as API calls.
        pub parent_zone_id: pulumi_wasm_rust::Output<String>,
        /// Name of the zone that handles some of the Local Zone or Wavelength Zone control plane operations, such as API calls.
        pub parent_zone_name: pulumi_wasm_rust::Output<String>,
        /// Region where the selected availability zone resides. This is always the region selected on the provider, since this data source searches only within that region.
        pub region: pulumi_wasm_rust::Output<String>,
        pub state: pulumi_wasm_rust::Output<String>,
        pub zone_id: pulumi_wasm_rust::Output<String>,
        /// Type of zone. Values are `availability-zone`, `local-zone`, and `wavelength-zone`.
        pub zone_type: pulumi_wasm_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn invoke(
        context: &pulumi_wasm_rust::PulumiContext,
        args: GetAvailabilityZoneArgs,
    ) -> GetAvailabilityZoneResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let all_availability_zones_binding = args
            .all_availability_zones
            .get_output(context)
            .get_inner();
        let filters_binding = args.filters.get_output(context).get_inner();
        let name_binding = args.name.get_output(context).get_inner();
        let state_binding = args.state.get_output(context).get_inner();
        let zone_id_binding = args.zone_id.get_output(context).get_inner();
        let request = register_interface::ResourceInvokeRequest {
            token: "aws:index/getAvailabilityZone:getAvailabilityZone".into(),
            version: super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "allAvailabilityZones".into(),
                    value: &all_availability_zones_binding,
                },
                register_interface::ObjectField {
                    name: "filters".into(),
                    value: &filters_binding,
                },
                register_interface::ObjectField {
                    name: "name".into(),
                    value: &name_binding,
                },
                register_interface::ObjectField {
                    name: "state".into(),
                    value: &state_binding,
                },
                register_interface::ObjectField {
                    name: "zoneId".into(),
                    value: &zone_id_binding,
                },
            ]),
            results: Vec::from([
                register_interface::ResultField {
                    name: "allAvailabilityZones".into(),
                },
                register_interface::ResultField {
                    name: "filters".into(),
                },
                register_interface::ResultField {
                    name: "groupName".into(),
                },
                register_interface::ResultField {
                    name: "id".into(),
                },
                register_interface::ResultField {
                    name: "name".into(),
                },
                register_interface::ResultField {
                    name: "nameSuffix".into(),
                },
                register_interface::ResultField {
                    name: "networkBorderGroup".into(),
                },
                register_interface::ResultField {
                    name: "optInStatus".into(),
                },
                register_interface::ResultField {
                    name: "parentZoneId".into(),
                },
                register_interface::ResultField {
                    name: "parentZoneName".into(),
                },
                register_interface::ResultField {
                    name: "region".into(),
                },
                register_interface::ResultField {
                    name: "state".into(),
                },
                register_interface::ResultField {
                    name: "zoneId".into(),
                },
                register_interface::ResultField {
                    name: "zoneType".into(),
                },
            ]),
        };
        let o = register_interface::invoke(context.get_inner(), &request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        GetAvailabilityZoneResult {
            all_availability_zones: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("allAvailabilityZones").unwrap(),
            ),
            filters: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("filters").unwrap(),
            ),
            group_name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("groupName").unwrap(),
            ),
            id: pulumi_wasm_rust::__private::into_domain(hashmap.remove("id").unwrap()),
            name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("name").unwrap(),
            ),
            name_suffix: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("nameSuffix").unwrap(),
            ),
            network_border_group: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("networkBorderGroup").unwrap(),
            ),
            opt_in_status: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("optInStatus").unwrap(),
            ),
            parent_zone_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("parentZoneId").unwrap(),
            ),
            parent_zone_name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("parentZoneName").unwrap(),
            ),
            region: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("region").unwrap(),
            ),
            state: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("state").unwrap(),
            ),
            zone_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("zoneId").unwrap(),
            ),
            zone_type: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("zoneType").unwrap(),
            ),
        }
    }
}
