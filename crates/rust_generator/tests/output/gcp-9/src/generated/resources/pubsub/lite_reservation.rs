/// A named resource representing a shared pool of capacity.
///
///
/// To get more information about Reservation, see:
///
/// * [API documentation](https://cloud.google.com/pubsub/lite/docs/reference/rest/v1/admin.projects.locations.reservations)
/// * How-to Guides
///     * [Managing Reservations](https://cloud.google.com/pubsub/lite/docs/reservations)
///
/// ## Example Usage
///
/// ### Pubsub Lite Reservation Basic
///
///
/// ```yaml
/// resources:
///   example:
///     type: gcp:pubsub:LiteReservation
///     properties:
///       name: example-reservation
///       project: ${project.number}
///       throughputCapacity: 2
/// variables:
///   project:
///     fn::invoke:
///       function: gcp:organizations:getProject
///       arguments: {}
/// ```
///
/// ## Import
///
/// Reservation can be imported using any of these accepted formats:
///
/// * `projects/{{project}}/locations/{{region}}/reservations/{{name}}`
///
/// * `{{project}}/{{region}}/{{name}}`
///
/// * `{{region}}/{{name}}`
///
/// * `{{name}}`
///
/// When using the `pulumi import` command, Reservation can be imported using one of the formats above. For example:
///
/// ```sh
/// $ pulumi import gcp:pubsub/liteReservation:LiteReservation default projects/{{project}}/locations/{{region}}/reservations/{{name}}
/// ```
///
/// ```sh
/// $ pulumi import gcp:pubsub/liteReservation:LiteReservation default {{project}}/{{region}}/{{name}}
/// ```
///
/// ```sh
/// $ pulumi import gcp:pubsub/liteReservation:LiteReservation default {{region}}/{{name}}
/// ```
///
/// ```sh
/// $ pulumi import gcp:pubsub/liteReservation:LiteReservation default {{name}}
/// ```
///
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod lite_reservation {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct LiteReservationArgs {
        /// Name of the reservation.
        ///
        ///
        /// - - -
        #[builder(into, default)]
        pub name: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The ID of the project in which the resource belongs.
        /// If it is not provided, the provider project is used.
        #[builder(into, default)]
        pub project: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The region of the pubsub lite reservation.
        #[builder(into, default)]
        pub region: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The reserved throughput capacity. Every unit of throughput capacity is
        /// equivalent to 1 MiB/s of published messages or 2 MiB/s of subscribed
        /// messages.
        #[builder(into)]
        pub throughput_capacity: pulumi_gestalt_rust::InputOrOutput<i32>,
    }
    #[allow(dead_code)]
    pub struct LiteReservationResult {
        /// Name of the reservation.
        ///
        ///
        /// - - -
        pub name: pulumi_gestalt_rust::Output<String>,
        /// The ID of the project in which the resource belongs.
        /// If it is not provided, the provider project is used.
        pub project: pulumi_gestalt_rust::Output<String>,
        /// The region of the pubsub lite reservation.
        pub region: pulumi_gestalt_rust::Output<Option<String>>,
        /// The reserved throughput capacity. Every unit of throughput capacity is
        /// equivalent to 1 MiB/s of published messages or 2 MiB/s of subscribed
        /// messages.
        pub throughput_capacity: pulumi_gestalt_rust::Output<i32>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::PulumiContext,
        name: &str,
        args: LiteReservationArgs,
    ) -> LiteReservationResult {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::register_interface;
        use std::collections::HashMap;
        let name_binding_1 = args.name.get_output(context);
        let name_binding = name_binding_1.get_inner();
        let project_binding_1 = args.project.get_output(context);
        let project_binding = project_binding_1.get_inner();
        let region_binding_1 = args.region.get_output(context);
        let region_binding = region_binding_1.get_inner();
        let throughput_capacity_binding_1 = args.throughput_capacity.get_output(context);
        let throughput_capacity_binding = throughput_capacity_binding_1.get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "gcp:pubsub/liteReservation:LiteReservation".into(),
            name: name.to_string(),
            version: super::super::get_version(),
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
                    name: "region".into(),
                    value: &region_binding,
                },
                register_interface::ObjectField {
                    name: "throughputCapacity".into(),
                    value: &throughput_capacity_binding,
                },
            ]),
        };
        let o = register_interface::register(context.get_inner(), &request);
        LiteReservationResult {
            name: pulumi_gestalt_rust::__private::into_domain(o.extract_field("name")),
            project: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("project"),
            ),
            region: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("region"),
            ),
            throughput_capacity: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("throughputCapacity"),
            ),
        }
    }
}
