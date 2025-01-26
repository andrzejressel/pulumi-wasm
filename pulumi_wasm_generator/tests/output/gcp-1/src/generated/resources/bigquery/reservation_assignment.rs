/// The BigqueryReservation Assignment resource.
///
///
/// To get more information about ReservationAssignment, see:
///
/// * [API documentation](https://cloud.google.com/bigquery/docs/reference/reservations/rest/v1/projects.locations.reservations.assignments)
/// * How-to Guides
///     * [Work with reservation assignments](https://cloud.google.com/bigquery/docs/reservations-assignments)
///
/// ## Example Usage
///
/// ### Bigquery Reservation Assignment Basic
///
///
/// ```ignore
/// use pulumi_wasm_rust::Output;
/// use pulumi_wasm_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let assignment = reservation_assignment::create(
///         "assignment",
///         ReservationAssignmentArgs::builder()
///             .assignee("projects/my-project-name")
///             .job_type("PIPELINE")
///             .reservation("${basic.id}")
///             .build_struct(),
///     );
///     let basic = reservation::create(
///         "basic",
///         ReservationArgs::builder()
///             .ignore_idle_slots(false)
///             .location("us-central1")
///             .name("example-reservation")
///             .project("my-project-name")
///             .slot_capacity(0)
///             .build_struct(),
///     );
/// }
/// ```
///
/// ## Import
///
/// ReservationAssignment can be imported using any of these accepted formats:
///
/// * `projects/{{project}}/locations/{{location}}/reservations/{{reservation}}/assignments/{{name}}`
///
/// * `{{project}}/{{location}}/{{reservation}}/{{name}}`
///
/// * `{{location}}/{{reservation}}/{{name}}`
///
/// When using the `pulumi import` command, ReservationAssignment can be imported using one of the formats above. For example:
///
/// ```sh
/// $ pulumi import gcp:bigquery/reservationAssignment:ReservationAssignment default projects/{{project}}/locations/{{location}}/reservations/{{reservation}}/assignments/{{name}}
/// ```
///
/// ```sh
/// $ pulumi import gcp:bigquery/reservationAssignment:ReservationAssignment default {{project}}/{{location}}/{{reservation}}/{{name}}
/// ```
///
/// ```sh
/// $ pulumi import gcp:bigquery/reservationAssignment:ReservationAssignment default {{location}}/{{reservation}}/{{name}}
/// ```
///
pub mod reservation_assignment {
    #[derive(pulumi_wasm_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct ReservationAssignmentArgs {
        /// The resource which will use the reservation. E.g. projects/myproject, folders/123, organizations/456.
        #[builder(into)]
        pub assignee: pulumi_wasm_rust::InputOrOutput<String>,
        /// Types of job, which could be specified when using the reservation. Possible values: JOB_TYPE_UNSPECIFIED, PIPELINE, QUERY
        #[builder(into)]
        pub job_type: pulumi_wasm_rust::InputOrOutput<String>,
        /// The location for the resource
        #[builder(into, default)]
        pub location: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        /// The ID of the project in which the resource belongs.
        /// If it is not provided, the provider project is used.
        #[builder(into, default)]
        pub project: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        /// The reservation for the resource
        ///
        ///
        /// - - -
        #[builder(into)]
        pub reservation: pulumi_wasm_rust::InputOrOutput<String>,
    }
    #[allow(dead_code)]
    pub struct ReservationAssignmentResult {
        /// The resource which will use the reservation. E.g. projects/myproject, folders/123, organizations/456.
        pub assignee: pulumi_wasm_rust::Output<String>,
        /// Types of job, which could be specified when using the reservation. Possible values: JOB_TYPE_UNSPECIFIED, PIPELINE, QUERY
        pub job_type: pulumi_wasm_rust::Output<String>,
        /// The location for the resource
        pub location: pulumi_wasm_rust::Output<String>,
        /// Output only. The resource name of the assignment.
        pub name: pulumi_wasm_rust::Output<String>,
        /// The ID of the project in which the resource belongs.
        /// If it is not provided, the provider project is used.
        pub project: pulumi_wasm_rust::Output<String>,
        /// The reservation for the resource
        ///
        ///
        /// - - -
        pub reservation: pulumi_wasm_rust::Output<String>,
        /// Assignment will remain in PENDING state if no active capacity commitment is present. It will become ACTIVE when some capacity commitment becomes active.
        /// Possible values: STATE_UNSPECIFIED, PENDING, ACTIVE
        pub state: pulumi_wasm_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_wasm_rust::PulumiContext,
        name: &str,
        args: ReservationAssignmentArgs,
    ) -> ReservationAssignmentResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let assignee_binding = args.assignee.get_output(context).get_inner();
        let job_type_binding = args.job_type.get_output(context).get_inner();
        let location_binding = args.location.get_output(context).get_inner();
        let project_binding = args.project.get_output(context).get_inner();
        let reservation_binding = args.reservation.get_output(context).get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "gcp:bigquery/reservationAssignment:ReservationAssignment".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "assignee".into(),
                    value: &assignee_binding,
                },
                register_interface::ObjectField {
                    name: "jobType".into(),
                    value: &job_type_binding,
                },
                register_interface::ObjectField {
                    name: "location".into(),
                    value: &location_binding,
                },
                register_interface::ObjectField {
                    name: "project".into(),
                    value: &project_binding,
                },
                register_interface::ObjectField {
                    name: "reservation".into(),
                    value: &reservation_binding,
                },
            ]),
        };
        let o = register_interface::register(context.get_inner(), &request);
        ReservationAssignmentResult {
            assignee: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("assignee"),
            ),
            job_type: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("jobType"),
            ),
            location: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("location"),
            ),
            name: pulumi_wasm_rust::__private::into_domain(o.extract_field("name")),
            project: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("project"),
            ),
            reservation: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("reservation"),
            ),
            state: pulumi_wasm_rust::__private::into_domain(o.extract_field("state")),
        }
    }
}
