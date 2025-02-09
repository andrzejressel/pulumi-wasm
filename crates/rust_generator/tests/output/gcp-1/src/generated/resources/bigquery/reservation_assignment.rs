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
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
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
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod reservation_assignment {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct ReservationAssignmentArgs {
        /// The resource which will use the reservation. E.g. projects/myproject, folders/123, organizations/456.
        #[builder(into)]
        pub assignee: pulumi_gestalt_rust::InputOrOutput<String>,
        /// Types of job, which could be specified when using the reservation. Possible values: JOB_TYPE_UNSPECIFIED, PIPELINE, QUERY
        #[builder(into)]
        pub job_type: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The location for the resource
        #[builder(into, default)]
        pub location: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The ID of the project in which the resource belongs.
        /// If it is not provided, the provider project is used.
        #[builder(into, default)]
        pub project: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The reservation for the resource
        ///
        ///
        /// - - -
        #[builder(into)]
        pub reservation: pulumi_gestalt_rust::InputOrOutput<String>,
    }
    #[allow(dead_code)]
    pub struct ReservationAssignmentResult {
        /// The resource which will use the reservation. E.g. projects/myproject, folders/123, organizations/456.
        pub assignee: pulumi_gestalt_rust::Output<String>,
        /// Types of job, which could be specified when using the reservation. Possible values: JOB_TYPE_UNSPECIFIED, PIPELINE, QUERY
        pub job_type: pulumi_gestalt_rust::Output<String>,
        /// The location for the resource
        pub location: pulumi_gestalt_rust::Output<String>,
        /// Output only. The resource name of the assignment.
        pub name: pulumi_gestalt_rust::Output<String>,
        /// The ID of the project in which the resource belongs.
        /// If it is not provided, the provider project is used.
        pub project: pulumi_gestalt_rust::Output<String>,
        /// The reservation for the resource
        ///
        ///
        /// - - -
        pub reservation: pulumi_gestalt_rust::Output<String>,
        /// Assignment will remain in PENDING state if no active capacity commitment is present. It will become ACTIVE when some capacity commitment becomes active.
        /// Possible values: STATE_UNSPECIFIED, PENDING, ACTIVE
        pub state: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: ReservationAssignmentArgs,
    ) -> ReservationAssignmentResult {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::register_interface;
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let assignee_binding = args.assignee.get_output(context);
        let job_type_binding = args.job_type.get_output(context);
        let location_binding = args.location.get_output(context);
        let project_binding = args.project.get_output(context);
        let reservation_binding = args.reservation.get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "gcp:bigquery/reservationAssignment:ReservationAssignment".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "assignee".into(),
                    value: assignee_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "jobType".into(),
                    value: job_type_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "location".into(),
                    value: location_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "project".into(),
                    value: project_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "reservation".into(),
                    value: reservation_binding.get_id(),
                },
            ],
        };
        let o = context.register_resource(request);
        ReservationAssignmentResult {
            assignee: o.get_field("assignee"),
            job_type: o.get_field("jobType"),
            location: o.get_field("location"),
            name: o.get_field("name"),
            project: o.get_field("project"),
            reservation: o.get_field("reservation"),
            state: o.get_field("state"),
        }
    }
}
