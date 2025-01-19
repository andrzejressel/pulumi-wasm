pub mod get_availability_zones {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GetAvailabilityZonesArgs {
        /// Set to `true` to include all Availability Zones and Local Zones regardless of your opt in status.
        #[builder(into, default)]
        pub all_availability_zones: pulumi_wasm_rust::Output<Option<bool>>,
        /// List of Availability Zone names to exclude.
        #[builder(into, default)]
        pub exclude_names: pulumi_wasm_rust::Output<Option<Vec<String>>>,
        /// List of Availability Zone IDs to exclude.
        #[builder(into, default)]
        pub exclude_zone_ids: pulumi_wasm_rust::Output<Option<Vec<String>>>,
        /// Configuration block(s) for filtering. Detailed below.
        #[builder(into, default)]
        pub filters: pulumi_wasm_rust::Output<
            Option<Vec<super::super::types::GetAvailabilityZonesFilter>>,
        >,
        /// Allows to filter list of Availability Zones based on their
        /// current state. Can be either `"available"`, `"information"`, `"impaired"` or
        /// `"unavailable"`. By default the list includes a complete set of Availability Zones
        /// to which the underlying AWS account has access, regardless of their state.
        #[builder(into, default)]
        pub state: pulumi_wasm_rust::Output<Option<String>>,
    }
    #[allow(dead_code)]
    pub struct GetAvailabilityZonesResult {
        pub all_availability_zones: pulumi_wasm_rust::Output<Option<bool>>,
        pub exclude_names: pulumi_wasm_rust::Output<Option<Vec<String>>>,
        pub exclude_zone_ids: pulumi_wasm_rust::Output<Option<Vec<String>>>,
        pub filters: pulumi_wasm_rust::Output<
            Option<Vec<super::super::types::GetAvailabilityZonesFilter>>,
        >,
        /// A set of the Availability Zone Group names. For Availability Zones, this is the same value as the Region name. For Local Zones, the name of the associated group, for example `us-west-2-lax-1`.
        pub group_names: pulumi_wasm_rust::Output<Vec<String>>,
        /// The provider-assigned unique ID for this managed resource.
        pub id: pulumi_wasm_rust::Output<String>,
        /// List of the Availability Zone names available to the account.
        pub names: pulumi_wasm_rust::Output<Vec<String>>,
        pub state: pulumi_wasm_rust::Output<Option<String>>,
        /// List of the Availability Zone IDs available to the account.
        pub zone_ids: pulumi_wasm_rust::Output<Vec<String>>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn invoke(args: GetAvailabilityZonesArgs) -> GetAvailabilityZonesResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let all_availability_zones_binding = args.all_availability_zones.get_inner();
        let exclude_names_binding = args.exclude_names.get_inner();
        let exclude_zone_ids_binding = args.exclude_zone_ids.get_inner();
        let filters_binding = args.filters.get_inner();
        let state_binding = args.state.get_inner();
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
            results: Vec::from([
                register_interface::ResultField {
                    name: "allAvailabilityZones".into(),
                },
                register_interface::ResultField {
                    name: "excludeNames".into(),
                },
                register_interface::ResultField {
                    name: "excludeZoneIds".into(),
                },
                register_interface::ResultField {
                    name: "filters".into(),
                },
                register_interface::ResultField {
                    name: "groupNames".into(),
                },
                register_interface::ResultField {
                    name: "id".into(),
                },
                register_interface::ResultField {
                    name: "names".into(),
                },
                register_interface::ResultField {
                    name: "state".into(),
                },
                register_interface::ResultField {
                    name: "zoneIds".into(),
                },
            ]),
        };
        let o = register_interface::invoke(&request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        GetAvailabilityZonesResult {
            all_availability_zones: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("allAvailabilityZones").unwrap(),
            ),
            exclude_names: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("excludeNames").unwrap(),
            ),
            exclude_zone_ids: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("excludeZoneIds").unwrap(),
            ),
            filters: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("filters").unwrap(),
            ),
            group_names: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("groupNames").unwrap(),
            ),
            id: pulumi_wasm_rust::__private::into_domain(hashmap.remove("id").unwrap()),
            names: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("names").unwrap(),
            ),
            state: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("state").unwrap(),
            ),
            zone_ids: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("zoneIds").unwrap(),
            ),
        }
    }
}
