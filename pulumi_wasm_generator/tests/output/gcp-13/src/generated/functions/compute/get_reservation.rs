pub mod get_reservation {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GetReservationArgs {
        /// The name of the Compute Reservation.
        #[builder(into)]
        pub name: pulumi_wasm_rust::Output<String>,
        /// Project from which to list the Compute Reservation. Defaults to project declared in the provider.
        #[builder(into, default)]
        pub project: pulumi_wasm_rust::Output<Option<String>>,
        /// Zone where the Compute Reservation resides.
        #[builder(into)]
        pub zone: pulumi_wasm_rust::Output<String>,
    }
    #[allow(dead_code)]
    pub struct GetReservationResult {
        pub commitment: pulumi_wasm_rust::Output<String>,
        pub creation_timestamp: pulumi_wasm_rust::Output<String>,
        pub description: pulumi_wasm_rust::Output<String>,
        /// The provider-assigned unique ID for this managed resource.
        pub id: pulumi_wasm_rust::Output<String>,
        pub name: pulumi_wasm_rust::Output<String>,
        pub project: pulumi_wasm_rust::Output<Option<String>>,
        pub self_link: pulumi_wasm_rust::Output<String>,
        pub share_settings: pulumi_wasm_rust::Output<
            Vec<super::super::super::types::compute::GetReservationShareSetting>,
        >,
        pub specific_reservation_required: pulumi_wasm_rust::Output<bool>,
        pub specific_reservations: pulumi_wasm_rust::Output<
            Vec<super::super::super::types::compute::GetReservationSpecificReservation>,
        >,
        pub status: pulumi_wasm_rust::Output<String>,
        pub zone: pulumi_wasm_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn invoke(args: GetReservationArgs) -> GetReservationResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let name_binding = args.name.get_inner();
        let project_binding = args.project.get_inner();
        let zone_binding = args.zone.get_inner();
        let request = register_interface::ResourceInvokeRequest {
            token: "gcp:compute/getReservation:getReservation".into(),
            version: super::super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "name".into(),
                    value: &name_binding,
                },
                register_interface::ObjectField {
                    name: "project".into(),
                    value: &project_binding,
                },
                register_interface::ObjectField {
                    name: "zone".into(),
                    value: &zone_binding,
                },
            ]),
            results: Vec::from([
                register_interface::ResultField {
                    name: "commitment".into(),
                },
                register_interface::ResultField {
                    name: "creationTimestamp".into(),
                },
                register_interface::ResultField {
                    name: "description".into(),
                },
                register_interface::ResultField {
                    name: "id".into(),
                },
                register_interface::ResultField {
                    name: "name".into(),
                },
                register_interface::ResultField {
                    name: "project".into(),
                },
                register_interface::ResultField {
                    name: "selfLink".into(),
                },
                register_interface::ResultField {
                    name: "shareSettings".into(),
                },
                register_interface::ResultField {
                    name: "specificReservationRequired".into(),
                },
                register_interface::ResultField {
                    name: "specificReservations".into(),
                },
                register_interface::ResultField {
                    name: "status".into(),
                },
                register_interface::ResultField {
                    name: "zone".into(),
                },
            ]),
        };
        let o = register_interface::invoke(&request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        GetReservationResult {
            commitment: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("commitment").unwrap(),
            ),
            creation_timestamp: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("creationTimestamp").unwrap(),
            ),
            description: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("description").unwrap(),
            ),
            id: pulumi_wasm_rust::__private::into_domain(hashmap.remove("id").unwrap()),
            name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("name").unwrap(),
            ),
            project: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("project").unwrap(),
            ),
            self_link: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("selfLink").unwrap(),
            ),
            share_settings: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("shareSettings").unwrap(),
            ),
            specific_reservation_required: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("specificReservationRequired").unwrap(),
            ),
            specific_reservations: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("specificReservations").unwrap(),
            ),
            status: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("status").unwrap(),
            ),
            zone: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("zone").unwrap(),
            ),
        }
    }
}
