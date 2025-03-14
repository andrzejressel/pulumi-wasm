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
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
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
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod reservation {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct ReservationArgs {
        /// The configuration parameters for the auto scaling feature.
        /// Structure is documented below.
        #[builder(into, default)]
        pub autoscale: pulumi_gestalt_rust::InputOrOutput<
            Option<super::super::types::bigquery::ReservationAutoscale>,
        >,
        /// Maximum number of queries that are allowed to run concurrently in this reservation. This is a soft limit due to asynchronous nature of the system and various optimizations for small queries. Default value is 0 which means that concurrency will be automatically set based on the reservation size.
        #[builder(into, default)]
        pub concurrency: pulumi_gestalt_rust::InputOrOutput<Option<i32>>,
        /// The edition type. Valid values are STANDARD, ENTERPRISE, ENTERPRISE_PLUS
        #[builder(into, default)]
        pub edition: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// If false, any query using this reservation will use idle slots from other reservations within
        /// the same admin project. If true, a query using this reservation will execute with the slot
        /// capacity specified above at most.
        #[builder(into, default)]
        pub ignore_idle_slots: pulumi_gestalt_rust::InputOrOutput<Option<bool>>,
        /// The geographic location where the transfer config should reside.
        /// Examples: US, EU, asia-northeast1. The default value is US.
        #[builder(into, default)]
        pub location: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The name of the reservation. This field must only contain alphanumeric characters or dash.
        ///
        ///
        /// - - -
        #[builder(into, default)]
        pub name: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The ID of the project in which the resource belongs.
        /// If it is not provided, the provider project is used.
        #[builder(into, default)]
        pub project: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Minimum slots available to this reservation. A slot is a unit of computational power in BigQuery, and serves as the
        /// unit of parallelism. Queries using this reservation might use more slots during runtime if ignoreIdleSlots is set to false.
        #[builder(into)]
        pub slot_capacity: pulumi_gestalt_rust::InputOrOutput<i32>,
    }
    #[allow(dead_code)]
    pub struct ReservationResult {
        /// The configuration parameters for the auto scaling feature.
        /// Structure is documented below.
        pub autoscale: pulumi_gestalt_rust::Output<
            Option<super::super::types::bigquery::ReservationAutoscale>,
        >,
        /// Maximum number of queries that are allowed to run concurrently in this reservation. This is a soft limit due to asynchronous nature of the system and various optimizations for small queries. Default value is 0 which means that concurrency will be automatically set based on the reservation size.
        pub concurrency: pulumi_gestalt_rust::Output<Option<i32>>,
        /// The edition type. Valid values are STANDARD, ENTERPRISE, ENTERPRISE_PLUS
        pub edition: pulumi_gestalt_rust::Output<String>,
        /// If false, any query using this reservation will use idle slots from other reservations within
        /// the same admin project. If true, a query using this reservation will execute with the slot
        /// capacity specified above at most.
        pub ignore_idle_slots: pulumi_gestalt_rust::Output<Option<bool>>,
        /// The geographic location where the transfer config should reside.
        /// Examples: US, EU, asia-northeast1. The default value is US.
        pub location: pulumi_gestalt_rust::Output<Option<String>>,
        /// The name of the reservation. This field must only contain alphanumeric characters or dash.
        ///
        ///
        /// - - -
        pub name: pulumi_gestalt_rust::Output<String>,
        /// The ID of the project in which the resource belongs.
        /// If it is not provided, the provider project is used.
        pub project: pulumi_gestalt_rust::Output<String>,
        /// Minimum slots available to this reservation. A slot is a unit of computational power in BigQuery, and serves as the
        /// unit of parallelism. Queries using this reservation might use more slots during runtime if ignoreIdleSlots is set to false.
        pub slot_capacity: pulumi_gestalt_rust::Output<i32>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: ReservationArgs,
    ) -> ReservationResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let autoscale_binding = args.autoscale.get_output(context);
        let concurrency_binding = args.concurrency.get_output(context);
        let edition_binding = args.edition.get_output(context);
        let ignore_idle_slots_binding = args.ignore_idle_slots.get_output(context);
        let location_binding = args.location.get_output(context);
        let name_binding = args.name.get_output(context);
        let project_binding = args.project.get_output(context);
        let slot_capacity_binding = args.slot_capacity.get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "gcp:bigquery/reservation:Reservation".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "autoscale".into(),
                    value: &autoscale_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "concurrency".into(),
                    value: &concurrency_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "edition".into(),
                    value: &edition_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "ignoreIdleSlots".into(),
                    value: &ignore_idle_slots_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "location".into(),
                    value: &location_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "name".into(),
                    value: &name_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "project".into(),
                    value: &project_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "slotCapacity".into(),
                    value: &slot_capacity_binding.drop_type(),
                },
            ],
        };
        let o = context.register_resource(request);
        ReservationResult {
            autoscale: o.get_field("autoscale"),
            concurrency: o.get_field("concurrency"),
            edition: o.get_field("edition"),
            ignore_idle_slots: o.get_field("ignoreIdleSlots"),
            location: o.get_field("location"),
            name: o.get_field("name"),
            project: o.get_field("project"),
            slot_capacity: o.get_field("slotCapacity"),
        }
    }
}
