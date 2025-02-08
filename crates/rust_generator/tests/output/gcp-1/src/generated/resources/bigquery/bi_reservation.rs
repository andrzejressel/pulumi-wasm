/// Represents a BI Reservation.
///
///
/// To get more information about BiReservation, see:
///
/// * [API documentation](https://cloud.google.com/bigquery/docs/reference/reservations/rest/v1/BiReservation)
/// * How-to Guides
///     * [Introduction to Reservations](https://cloud.google.com/bigquery/docs/reservations-intro)
///
/// ## Example Usage
///
/// ### Bigquery Reservation Bi Reservation Basic
///
///
/// ```yaml
/// resources:
///   reservation:
///     type: gcp:bigquery:BiReservation
///     properties:
///       location: us-west2
///       size: '3000000000'
/// ```
///
/// ## Import
///
/// BiReservation can be imported using any of these accepted formats:
///
/// * `projects/{{project}}/locations/{{location}}/biReservation`
///
/// * `{{project}}/{{location}}`
///
/// * `{{location}}`
///
/// When using the `pulumi import` command, BiReservation can be imported using one of the formats above. For example:
///
/// ```sh
/// $ pulumi import gcp:bigquery/biReservation:BiReservation default projects/{{project}}/locations/{{location}}/biReservation
/// ```
///
/// ```sh
/// $ pulumi import gcp:bigquery/biReservation:BiReservation default {{project}}/{{location}}
/// ```
///
/// ```sh
/// $ pulumi import gcp:bigquery/biReservation:BiReservation default {{location}}
/// ```
///
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod bi_reservation {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct BiReservationArgs {
        /// LOCATION_DESCRIPTION
        ///
        ///
        /// - - -
        #[builder(into)]
        pub location: pulumi_gestalt_rust::InputOrOutput<String>,
        /// Preferred tables to use BI capacity for.
        /// Structure is documented below.
        #[builder(into, default)]
        pub preferred_tables: pulumi_gestalt_rust::InputOrOutput<
            Option<Vec<super::super::types::bigquery::BiReservationPreferredTable>>,
        >,
        /// The ID of the project in which the resource belongs.
        /// If it is not provided, the provider project is used.
        #[builder(into, default)]
        pub project: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Size of a reservation, in bytes.
        #[builder(into, default)]
        pub size: pulumi_gestalt_rust::InputOrOutput<Option<i32>>,
    }
    #[allow(dead_code)]
    pub struct BiReservationResult {
        /// LOCATION_DESCRIPTION
        ///
        ///
        /// - - -
        pub location: pulumi_gestalt_rust::Output<String>,
        /// The resource name of the singleton BI reservation. Reservation names have the form `projects/{projectId}/locations/{locationId}/biReservation`.
        pub name: pulumi_gestalt_rust::Output<String>,
        /// Preferred tables to use BI capacity for.
        /// Structure is documented below.
        pub preferred_tables: pulumi_gestalt_rust::Output<
            Option<Vec<super::super::types::bigquery::BiReservationPreferredTable>>,
        >,
        /// The ID of the project in which the resource belongs.
        /// If it is not provided, the provider project is used.
        pub project: pulumi_gestalt_rust::Output<String>,
        /// Size of a reservation, in bytes.
        pub size: pulumi_gestalt_rust::Output<Option<i32>>,
        /// The last update timestamp of a reservation.
        /// A timestamp in RFC3339 UTC "Zulu" format, with nanosecond resolution and up to nine fractional digits. Examples: "2014-10-02T15:01:23Z" and "2014-10-02T15:01:23.045123456Z".
        pub update_time: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::PulumiContext,
        name: &str,
        args: BiReservationArgs,
    ) -> BiReservationResult {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::register_interface;
        use std::collections::HashMap;
        let location_binding = args.location.get_output(context).get_inner();
        let preferred_tables_binding = args
            .preferred_tables
            .get_output(context)
            .get_inner();
        let project_binding = args.project.get_output(context).get_inner();
        let size_binding = args.size.get_output(context).get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "gcp:bigquery/biReservation:BiReservation".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "location".into(),
                    value: &location_binding,
                },
                register_interface::ObjectField {
                    name: "preferredTables".into(),
                    value: &preferred_tables_binding,
                },
                register_interface::ObjectField {
                    name: "project".into(),
                    value: &project_binding,
                },
                register_interface::ObjectField {
                    name: "size".into(),
                    value: &size_binding,
                },
            ]),
        };
        let o = register_interface::register(context.get_inner(), &request);
        BiReservationResult {
            location: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("location"),
            ),
            name: pulumi_gestalt_rust::__private::into_domain(o.extract_field("name")),
            preferred_tables: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("preferredTables"),
            ),
            project: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("project"),
            ),
            size: pulumi_gestalt_rust::__private::into_domain(o.extract_field("size")),
            update_time: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("updateTime"),
            ),
        }
    }
}
