/// Manages a Capacity Reservation within a Capacity Reservation Group.
///
/// ## Example Usage
///
/// ```ignore
/// use pulumi_wasm_rust::Output;
/// use pulumi_wasm_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let example = resource_group::create(
///         "example",
///         ResourceGroupArgs::builder()
///             .location("West Europe")
///             .name("example-rg")
///             .build_struct(),
///     );
///     let exampleCapacityReservation = capacity_reservation::create(
///         "exampleCapacityReservation",
///         CapacityReservationArgs::builder()
///             .capacity_reservation_group_id("${exampleCapacityReservationGroup.id}")
///             .name("example-capacity-reservation")
///             .sku(
///                 CapacityReservationSku::builder()
///                     .capacity(1)
///                     .name("Standard_D2s_v3")
///                     .build_struct(),
///             )
///             .build_struct(),
///     );
///     let exampleCapacityReservationGroup = capacity_reservation_group::create(
///         "exampleCapacityReservationGroup",
///         CapacityReservationGroupArgs::builder()
///             .location("${example.location}")
///             .name("example-capacity-reservation-group")
///             .resource_group_name("${example.name}")
///             .build_struct(),
///     );
/// }
/// ```
///
/// ## Import
///
/// Capacity Reservations can be imported using the `resource id`, e.g.
///
/// ```sh
/// $ pulumi import azure:compute/capacityReservation:CapacityReservation example /subscriptions/00000000-0000-0000-0000-000000000000/resourceGroups/group1/providers/Microsoft.Compute/capacityReservationGroups/capacityReservationGroup1/capacityReservations/capacityReservation1
/// ```
///
pub mod capacity_reservation {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct CapacityReservationArgs {
        /// The ID of the Capacity Reservation Group where the Capacity Reservation exists. Changing this forces a new resource to be created.
        #[builder(into)]
        pub capacity_reservation_group_id: pulumi_wasm_rust::Output<String>,
        /// Specifies the name of this Capacity Reservation. Changing this forces a new resource to be created.
        #[builder(into, default)]
        pub name: pulumi_wasm_rust::Output<Option<String>>,
        /// A `sku` block as defined below.
        #[builder(into)]
        pub sku: pulumi_wasm_rust::Output<
            super::super::types::compute::CapacityReservationSku,
        >,
        /// A mapping of tags to assign to the resource.
        #[builder(into, default)]
        pub tags: pulumi_wasm_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// Specifies the Availability Zone for this Capacity Reservation. Changing this forces a new resource to be created.
        #[builder(into, default)]
        pub zone: pulumi_wasm_rust::Output<Option<String>>,
    }
    #[allow(dead_code)]
    pub struct CapacityReservationResult {
        /// The ID of the Capacity Reservation Group where the Capacity Reservation exists. Changing this forces a new resource to be created.
        pub capacity_reservation_group_id: pulumi_wasm_rust::Output<String>,
        /// Specifies the name of this Capacity Reservation. Changing this forces a new resource to be created.
        pub name: pulumi_wasm_rust::Output<String>,
        /// A `sku` block as defined below.
        pub sku: pulumi_wasm_rust::Output<
            super::super::types::compute::CapacityReservationSku,
        >,
        /// A mapping of tags to assign to the resource.
        pub tags: pulumi_wasm_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// Specifies the Availability Zone for this Capacity Reservation. Changing this forces a new resource to be created.
        pub zone: pulumi_wasm_rust::Output<Option<String>>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        name: &str,
        args: CapacityReservationArgs,
    ) -> CapacityReservationResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let capacity_reservation_group_id_binding = args
            .capacity_reservation_group_id
            .get_inner();
        let name_binding = args.name.get_inner();
        let sku_binding = args.sku.get_inner();
        let tags_binding = args.tags.get_inner();
        let zone_binding = args.zone.get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "azure:compute/capacityReservation:CapacityReservation".into(),
            name: name.to_string(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "capacityReservationGroupId".into(),
                    value: &capacity_reservation_group_id_binding,
                },
                register_interface::ObjectField {
                    name: "name".into(),
                    value: &name_binding,
                },
                register_interface::ObjectField {
                    name: "sku".into(),
                    value: &sku_binding,
                },
                register_interface::ObjectField {
                    name: "tags".into(),
                    value: &tags_binding,
                },
                register_interface::ObjectField {
                    name: "zone".into(),
                    value: &zone_binding,
                },
            ]),
            results: Vec::from([
                register_interface::ResultField {
                    name: "capacityReservationGroupId".into(),
                },
                register_interface::ResultField {
                    name: "name".into(),
                },
                register_interface::ResultField {
                    name: "sku".into(),
                },
                register_interface::ResultField {
                    name: "tags".into(),
                },
                register_interface::ResultField {
                    name: "zone".into(),
                },
            ]),
        };
        let o = register_interface::register(&request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        CapacityReservationResult {
            capacity_reservation_group_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("capacityReservationGroupId").unwrap(),
            ),
            name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("name").unwrap(),
            ),
            sku: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("sku").unwrap(),
            ),
            tags: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("tags").unwrap(),
            ),
            zone: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("zone").unwrap(),
            ),
        }
    }
}