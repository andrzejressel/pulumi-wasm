/// Represents a reservation resource. A reservation ensures that capacity is
/// held in a specific zone even if the reserved VMs are not running.
///
/// Reservations apply only to Compute Engine, Cloud Dataproc, and Google
/// Kubernetes Engine VM usage.Reservations do not apply to `f1-micro` or
/// `g1-small` machine types, preemptible VMs, sole tenant nodes, or other
/// services not listed above
/// like Cloud SQL and Dataflow.
///
///
/// To get more information about Reservation, see:
///
/// * [API documentation](https://cloud.google.com/compute/docs/reference/rest/v1/reservations)
/// * How-to Guides
///     * [Reserving zonal resources](https://cloud.google.com/compute/docs/instances/reserving-zonal-resources)
///
/// ## Example Usage
///
/// ### Reservation Basic
///
///
/// ```ignore
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let gceReservation = reservation::create(
///         "gceReservation",
///         ReservationArgs::builder()
///             .name("gce-reservation")
///             .specific_reservation(
///                 ReservationSpecificReservation::builder()
///                     .count(1)
///                     .instanceProperties(
///                         ReservationSpecificReservationInstanceProperties::builder()
///                             .machineType("n2-standard-2")
///                             .minCpuPlatform("Intel Cascade Lake")
///                             .build_struct(),
///                     )
///                     .build_struct(),
///             )
///             .zone("us-central1-a")
///             .build_struct(),
///     );
/// }
/// ```
///
/// ## Import
///
/// Reservation can be imported using any of these accepted formats:
///
/// * `projects/{{project}}/zones/{{zone}}/reservations/{{name}}`
///
/// * `{{project}}/{{zone}}/{{name}}`
///
/// * `{{zone}}/{{name}}`
///
/// * `{{name}}`
///
/// When using the `pulumi import` command, Reservation can be imported using one of the formats above. For example:
///
/// ```sh
/// $ pulumi import gcp:compute/reservation:Reservation default projects/{{project}}/zones/{{zone}}/reservations/{{name}}
/// ```
///
/// ```sh
/// $ pulumi import gcp:compute/reservation:Reservation default {{project}}/{{zone}}/{{name}}
/// ```
///
/// ```sh
/// $ pulumi import gcp:compute/reservation:Reservation default {{zone}}/{{name}}
/// ```
///
/// ```sh
/// $ pulumi import gcp:compute/reservation:Reservation default {{name}}
/// ```
///
pub mod reservation {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct ReservationArgs {
        /// An optional description of this resource.
        #[builder(into, default)]
        pub description: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Name of the resource. Provided by the client when the resource is
        /// created. The name must be 1-63 characters long, and comply with
        /// RFC1035. Specifically, the name must be 1-63 characters long and match
        /// the regular expression `a-z?` which means the
        /// first character must be a lowercase letter, and all following
        /// characters must be a dash, lowercase letter, or digit, except the last
        /// character, which cannot be a dash.
        #[builder(into, default)]
        pub name: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        #[builder(into, default)]
        pub project: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The share setting for reservations.
        #[builder(into, default)]
        pub share_settings: pulumi_gestalt_rust::InputOrOutput<
            Option<super::super::types::compute::ReservationShareSettings>,
        >,
        /// Reservation for instances with specific machine shapes.
        /// Structure is documented below.
        #[builder(into)]
        pub specific_reservation: pulumi_gestalt_rust::InputOrOutput<
            super::super::types::compute::ReservationSpecificReservation,
        >,
        /// When set to true, only VMs that target this reservation by name can consume this reservation. Otherwise, it can be
        /// consumed by VMs with affinity for any reservation. Defaults to false.
        #[builder(into, default)]
        pub specific_reservation_required: pulumi_gestalt_rust::InputOrOutput<
            Option<bool>,
        >,
        /// The zone where the reservation is made.
        #[builder(into)]
        pub zone: pulumi_gestalt_rust::InputOrOutput<String>,
    }
    #[allow(dead_code)]
    pub struct ReservationResult {
        /// Full or partial URL to a parent commitment. This field displays for
        /// reservations that are tied to a commitment.
        pub commitment: pulumi_gestalt_rust::Output<String>,
        /// Creation timestamp in RFC3339 text format.
        pub creation_timestamp: pulumi_gestalt_rust::Output<String>,
        /// An optional description of this resource.
        pub description: pulumi_gestalt_rust::Output<Option<String>>,
        /// Name of the resource. Provided by the client when the resource is
        /// created. The name must be 1-63 characters long, and comply with
        /// RFC1035. Specifically, the name must be 1-63 characters long and match
        /// the regular expression `a-z?` which means the
        /// first character must be a lowercase letter, and all following
        /// characters must be a dash, lowercase letter, or digit, except the last
        /// character, which cannot be a dash.
        pub name: pulumi_gestalt_rust::Output<String>,
        pub project: pulumi_gestalt_rust::Output<String>,
        /// The URI of the created resource.
        pub self_link: pulumi_gestalt_rust::Output<String>,
        /// The share setting for reservations.
        pub share_settings: pulumi_gestalt_rust::Output<
            super::super::types::compute::ReservationShareSettings,
        >,
        /// Reservation for instances with specific machine shapes.
        /// Structure is documented below.
        pub specific_reservation: pulumi_gestalt_rust::Output<
            super::super::types::compute::ReservationSpecificReservation,
        >,
        /// When set to true, only VMs that target this reservation by name can consume this reservation. Otherwise, it can be
        /// consumed by VMs with affinity for any reservation. Defaults to false.
        pub specific_reservation_required: pulumi_gestalt_rust::Output<Option<bool>>,
        /// The status of the reservation.
        pub status: pulumi_gestalt_rust::Output<String>,
        /// The zone where the reservation is made.
        pub zone: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::PulumiContext,
        name: &str,
        args: ReservationArgs,
    ) -> ReservationResult {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::register_interface;
        use std::collections::HashMap;
        let description_binding = args.description.get_output(context).get_inner();
        let name_binding = args.name.get_output(context).get_inner();
        let project_binding = args.project.get_output(context).get_inner();
        let share_settings_binding = args.share_settings.get_output(context).get_inner();
        let specific_reservation_binding = args
            .specific_reservation
            .get_output(context)
            .get_inner();
        let specific_reservation_required_binding = args
            .specific_reservation_required
            .get_output(context)
            .get_inner();
        let zone_binding = args.zone.get_output(context).get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "gcp:compute/reservation:Reservation".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "description".into(),
                    value: &description_binding,
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
                    name: "shareSettings".into(),
                    value: &share_settings_binding,
                },
                register_interface::ObjectField {
                    name: "specificReservation".into(),
                    value: &specific_reservation_binding,
                },
                register_interface::ObjectField {
                    name: "specificReservationRequired".into(),
                    value: &specific_reservation_required_binding,
                },
                register_interface::ObjectField {
                    name: "zone".into(),
                    value: &zone_binding,
                },
            ]),
        };
        let o = register_interface::register(context.get_inner(), &request);
        ReservationResult {
            commitment: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("commitment"),
            ),
            creation_timestamp: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("creationTimestamp"),
            ),
            description: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("description"),
            ),
            name: pulumi_gestalt_rust::__private::into_domain(o.extract_field("name")),
            project: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("project"),
            ),
            self_link: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("selfLink"),
            ),
            share_settings: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("shareSettings"),
            ),
            specific_reservation: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("specificReservation"),
            ),
            specific_reservation_required: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("specificReservationRequired"),
            ),
            status: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("status"),
            ),
            zone: pulumi_gestalt_rust::__private::into_domain(o.extract_field("zone")),
        }
    }
}
