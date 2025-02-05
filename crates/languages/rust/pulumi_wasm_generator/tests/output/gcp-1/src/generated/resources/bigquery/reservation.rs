/// A reservation is a mechanism used to guarantee BigQuery slots to users.
///
///
/// To get more information about Reservation, see:
///
/// * [API documentation](https://cloud.google.com/bigquery/docs/reference/reservations/rest/v1/projects.locations.reservations/create)
/// * How-to Guides
///     * [Introduction to Reservations](https://cloud.google.com/bigquery/docs/reservations-intro)
///
/// ## Example Usage
///
/// ### Bigquery Reservation Basic
///
///
/// ```ignore
/// use pulumi_wasm_rust::Output;
/// use pulumi_wasm_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let reservation = reservation::create(
///         "reservation",
///         ReservationArgs::builder()
///             .autoscale(ReservationAutoscale::builder().maxSlots(100).build_struct())
///             .concurrency(0)
///             .edition("STANDARD")
///             .ignore_idle_slots(true)
///             .location("us-west2")
///             .name("my-reservation")
///             .slot_capacity(0)
///             .build_struct(),
///     );
/// }
/// ```
///
/// ## Import
///
/// Reservation can be imported using any of these accepted formats:
///
/// * `projects/{{project}}/locations/{{location}}/reservations/{{name}}`
///
/// * `{{project}}/{{location}}/{{name}}`
///
/// * `{{location}}/{{name}}`
///
/// When using the `pulumi import` command, Reservation can be imported using one of the formats above. For example:
///
/// ```sh
/// $ pulumi import gcp:bigquery/reservation:Reservation default projects/{{project}}/locations/{{location}}/reservations/{{name}}
/// ```
///
/// ```sh
/// $ pulumi import gcp:bigquery/reservation:Reservation default {{project}}/{{location}}/{{name}}
/// ```
///
/// ```sh
/// $ pulumi import gcp:bigquery/reservation:Reservation default {{location}}/{{name}}
/// ```
///
pub mod reservation {
    #[derive(pulumi_wasm_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct ReservationArgs {
        /// The configuration parameters for the auto scaling feature.
        /// Structure is documented below.
        #[builder(into, default)]
        pub autoscale: pulumi_wasm_rust::InputOrOutput<
            Option<super::super::types::bigquery::ReservationAutoscale>,
        >,
        /// Maximum number of queries that are allowed to run concurrently in this reservation. This is a soft limit due to asynchronous nature of the system and various optimizations for small queries. Default value is 0 which means that concurrency will be automatically set based on the reservation size.
        #[builder(into, default)]
        pub concurrency: pulumi_wasm_rust::InputOrOutput<Option<i32>>,
        /// The edition type. Valid values are STANDARD, ENTERPRISE, ENTERPRISE_PLUS
        #[builder(into, default)]
        pub edition: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        /// If false, any query using this reservation will use idle slots from other reservations within
        /// the same admin project. If true, a query using this reservation will execute with the slot
        /// capacity specified above at most.
        #[builder(into, default)]
        pub ignore_idle_slots: pulumi_wasm_rust::InputOrOutput<Option<bool>>,
        /// The geographic location where the transfer config should reside.
        /// Examples: US, EU, asia-northeast1. The default value is US.
        #[builder(into, default)]
        pub location: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        /// The name of the reservation. This field must only contain alphanumeric characters or dash.
        ///
        ///
        /// - - -
        #[builder(into, default)]
        pub name: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        /// The ID of the project in which the resource belongs.
        /// If it is not provided, the provider project is used.
        #[builder(into, default)]
        pub project: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        /// Minimum slots available to this reservation. A slot is a unit of computational power in BigQuery, and serves as the
        /// unit of parallelism. Queries using this reservation might use more slots during runtime if ignoreIdleSlots is set to false.
        #[builder(into)]
        pub slot_capacity: pulumi_wasm_rust::InputOrOutput<i32>,
    }
    #[allow(dead_code)]
    pub struct ReservationResult {
        /// The configuration parameters for the auto scaling feature.
        /// Structure is documented below.
        pub autoscale: pulumi_wasm_rust::Output<
            Option<super::super::types::bigquery::ReservationAutoscale>,
        >,
        /// Maximum number of queries that are allowed to run concurrently in this reservation. This is a soft limit due to asynchronous nature of the system and various optimizations for small queries. Default value is 0 which means that concurrency will be automatically set based on the reservation size.
        pub concurrency: pulumi_wasm_rust::Output<Option<i32>>,
        /// The edition type. Valid values are STANDARD, ENTERPRISE, ENTERPRISE_PLUS
        pub edition: pulumi_wasm_rust::Output<String>,
        /// If false, any query using this reservation will use idle slots from other reservations within
        /// the same admin project. If true, a query using this reservation will execute with the slot
        /// capacity specified above at most.
        pub ignore_idle_slots: pulumi_wasm_rust::Output<Option<bool>>,
        /// The geographic location where the transfer config should reside.
        /// Examples: US, EU, asia-northeast1. The default value is US.
        pub location: pulumi_wasm_rust::Output<Option<String>>,
        /// The name of the reservation. This field must only contain alphanumeric characters or dash.
        ///
        ///
        /// - - -
        pub name: pulumi_wasm_rust::Output<String>,
        /// The ID of the project in which the resource belongs.
        /// If it is not provided, the provider project is used.
        pub project: pulumi_wasm_rust::Output<String>,
        /// Minimum slots available to this reservation. A slot is a unit of computational power in BigQuery, and serves as the
        /// unit of parallelism. Queries using this reservation might use more slots during runtime if ignoreIdleSlots is set to false.
        pub slot_capacity: pulumi_wasm_rust::Output<i32>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_wasm_rust::PulumiContext,
        name: &str,
        args: ReservationArgs,
    ) -> ReservationResult {
        use pulumi_wasm_rust::__private::pulumi_gestalt_adapter_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let autoscale_binding = args.autoscale.get_output(context).get_inner();
        let concurrency_binding = args.concurrency.get_output(context).get_inner();
        let edition_binding = args.edition.get_output(context).get_inner();
        let ignore_idle_slots_binding = args
            .ignore_idle_slots
            .get_output(context)
            .get_inner();
        let location_binding = args.location.get_output(context).get_inner();
        let name_binding = args.name.get_output(context).get_inner();
        let project_binding = args.project.get_output(context).get_inner();
        let slot_capacity_binding = args.slot_capacity.get_output(context).get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "gcp:bigquery/reservation:Reservation".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "autoscale".into(),
                    value: &autoscale_binding,
                },
                register_interface::ObjectField {
                    name: "concurrency".into(),
                    value: &concurrency_binding,
                },
                register_interface::ObjectField {
                    name: "edition".into(),
                    value: &edition_binding,
                },
                register_interface::ObjectField {
                    name: "ignoreIdleSlots".into(),
                    value: &ignore_idle_slots_binding,
                },
                register_interface::ObjectField {
                    name: "location".into(),
                    value: &location_binding,
                },
                register_interface::ObjectField {
                    name: "name".into(),
                    value: &name_binding,
                },
                register_interface::ObjectField {
                    name: "project".into(),
                    value: &project_binding,
                },
                register_interface::ObjectField {
                    name: "slotCapacity".into(),
                    value: &slot_capacity_binding,
                },
            ]),
        };
        let o = register_interface::register(context.get_inner(), &request);
        ReservationResult {
            autoscale: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("autoscale"),
            ),
            concurrency: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("concurrency"),
            ),
            edition: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("edition"),
            ),
            ignore_idle_slots: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("ignoreIdleSlots"),
            ),
            location: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("location"),
            ),
            name: pulumi_wasm_rust::__private::into_domain(o.extract_field("name")),
            project: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("project"),
            ),
            slot_capacity: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("slotCapacity"),
            ),
        }
    }
}
