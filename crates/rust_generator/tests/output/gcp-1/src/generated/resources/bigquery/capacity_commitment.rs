/// Capacity commitment is a way to purchase compute capacity for BigQuery jobs (in the form of slots) with some committed period of usage. Annual commitments renew by default. Commitments can be removed after their commitment end time passes.
///
/// In order to remove annual commitment, its plan needs to be changed to monthly or flex first.
///
///
/// To get more information about CapacityCommitment, see:
///
/// * [API documentation](https://cloud.google.com/bigquery/docs/reference/reservations/rest/v1/projects.locations.capacityCommitments)
/// * How-to Guides
///     * [Introduction to Reservations](https://cloud.google.com/bigquery/docs/reservations-intro)
///
/// ## Example Usage
///
/// ### Bigquery Reservation Capacity Commitment Docs
///
///
/// ```ignore
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let example = capacity_commitment::create(
///         "example",
///         CapacityCommitmentArgs::builder()
///             .capacity_commitment_id("example-commitment")
///             .edition("ENTERPRISE")
///             .location("us-west2")
///             .plan("FLEX_FLAT_RATE")
///             .slot_count(100)
///             .build_struct(),
///     );
/// }
/// ```
///
/// ## Import
///
/// CapacityCommitment can be imported using any of these accepted formats:
///
/// * `projects/{{project}}/locations/{{location}}/capacityCommitments/{{capacity_commitment_id}}`
///
/// * `{{project}}/{{location}}/{{capacity_commitment_id}}`
///
/// * `{{location}}/{{capacity_commitment_id}}`
///
/// When using the `pulumi import` command, CapacityCommitment can be imported using one of the formats above. For example:
///
/// ```sh
/// $ pulumi import gcp:bigquery/capacityCommitment:CapacityCommitment default projects/{{project}}/locations/{{location}}/capacityCommitments/{{capacity_commitment_id}}
/// ```
///
/// ```sh
/// $ pulumi import gcp:bigquery/capacityCommitment:CapacityCommitment default {{project}}/{{location}}/{{capacity_commitment_id}}
/// ```
///
/// ```sh
/// $ pulumi import gcp:bigquery/capacityCommitment:CapacityCommitment default {{location}}/{{capacity_commitment_id}}
/// ```
///
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod capacity_commitment {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct CapacityCommitmentArgs {
        /// The optional capacity commitment ID. Capacity commitment name will be generated automatically if this field is
        /// empty. This field must only contain lower case alphanumeric characters or dashes. The first and last character
        /// cannot be a dash. Max length is 64 characters. NOTE: this ID won't be kept if the capacity commitment is split
        /// or merged.
        #[builder(into, default)]
        pub capacity_commitment_id: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The edition type. Valid values are STANDARD, ENTERPRISE, ENTERPRISE_PLUS
        #[builder(into, default)]
        pub edition: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// If true, fail the request if another project in the organization has a capacity commitment.
        #[builder(into, default)]
        pub enforce_single_admin_project_per_org: pulumi_gestalt_rust::InputOrOutput<
            Option<String>,
        >,
        /// The geographic location where the transfer config should reside.
        /// Examples: US, EU, asia-northeast1. The default value is US.
        #[builder(into, default)]
        pub location: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Capacity commitment plan. Valid values are at https://cloud.google.com/bigquery/docs/reference/reservations/rpc/google.cloud.bigquery.reservation.v1#commitmentplan
        ///
        ///
        /// - - -
        #[builder(into)]
        pub plan: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The ID of the project in which the resource belongs.
        /// If it is not provided, the provider project is used.
        #[builder(into, default)]
        pub project: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The plan this capacity commitment is converted to after commitmentEndTime passes. Once the plan is changed, committed period is extended according to commitment plan. Only applicable for some commitment plans.
        #[builder(into, default)]
        pub renewal_plan: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Number of slots in this commitment.
        #[builder(into)]
        pub slot_count: pulumi_gestalt_rust::InputOrOutput<i32>,
    }
    #[allow(dead_code)]
    pub struct CapacityCommitmentResult {
        /// The optional capacity commitment ID. Capacity commitment name will be generated automatically if this field is
        /// empty. This field must only contain lower case alphanumeric characters or dashes. The first and last character
        /// cannot be a dash. Max length is 64 characters. NOTE: this ID won't be kept if the capacity commitment is split
        /// or merged.
        pub capacity_commitment_id: pulumi_gestalt_rust::Output<Option<String>>,
        /// The start of the current commitment period. It is applicable only for ACTIVE capacity commitments.
        pub commitment_end_time: pulumi_gestalt_rust::Output<String>,
        /// The start of the current commitment period. It is applicable only for ACTIVE capacity commitments.
        pub commitment_start_time: pulumi_gestalt_rust::Output<String>,
        /// The edition type. Valid values are STANDARD, ENTERPRISE, ENTERPRISE_PLUS
        pub edition: pulumi_gestalt_rust::Output<Option<String>>,
        /// If true, fail the request if another project in the organization has a capacity commitment.
        pub enforce_single_admin_project_per_org: pulumi_gestalt_rust::Output<
            Option<String>,
        >,
        /// The geographic location where the transfer config should reside.
        /// Examples: US, EU, asia-northeast1. The default value is US.
        pub location: pulumi_gestalt_rust::Output<Option<String>>,
        /// The resource name of the capacity commitment, e.g., projects/myproject/locations/US/capacityCommitments/123
        pub name: pulumi_gestalt_rust::Output<String>,
        /// Capacity commitment plan. Valid values are at https://cloud.google.com/bigquery/docs/reference/reservations/rpc/google.cloud.bigquery.reservation.v1#commitmentplan
        ///
        ///
        /// - - -
        pub plan: pulumi_gestalt_rust::Output<String>,
        /// The ID of the project in which the resource belongs.
        /// If it is not provided, the provider project is used.
        pub project: pulumi_gestalt_rust::Output<String>,
        /// The plan this capacity commitment is converted to after commitmentEndTime passes. Once the plan is changed, committed period is extended according to commitment plan. Only applicable for some commitment plans.
        pub renewal_plan: pulumi_gestalt_rust::Output<Option<String>>,
        /// Number of slots in this commitment.
        pub slot_count: pulumi_gestalt_rust::Output<i32>,
        /// State of the commitment
        pub state: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: CapacityCommitmentArgs,
    ) -> CapacityCommitmentResult {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::register_interface;
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let capacity_commitment_id_binding = args
            .capacity_commitment_id
            .get_output(context);
        let edition_binding = args.edition.get_output(context);
        let enforce_single_admin_project_per_org_binding = args
            .enforce_single_admin_project_per_org
            .get_output(context);
        let location_binding = args.location.get_output(context);
        let plan_binding = args.plan.get_output(context);
        let project_binding = args.project.get_output(context);
        let renewal_plan_binding = args.renewal_plan.get_output(context);
        let slot_count_binding = args.slot_count.get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "gcp:bigquery/capacityCommitment:CapacityCommitment".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "capacityCommitmentId".into(),
                    value: capacity_commitment_id_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "edition".into(),
                    value: edition_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "enforceSingleAdminProjectPerOrg".into(),
                    value: enforce_single_admin_project_per_org_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "location".into(),
                    value: location_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "plan".into(),
                    value: plan_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "project".into(),
                    value: project_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "renewalPlan".into(),
                    value: renewal_plan_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "slotCount".into(),
                    value: slot_count_binding.get_id(),
                },
            ],
        };
        let o = context.register_resource(request);
        CapacityCommitmentResult {
            capacity_commitment_id: o.get_field("capacityCommitmentId"),
            commitment_end_time: o.get_field("commitmentEndTime"),
            commitment_start_time: o.get_field("commitmentStartTime"),
            edition: o.get_field("edition"),
            enforce_single_admin_project_per_org: o
                .get_field("enforceSingleAdminProjectPerOrg"),
            location: o.get_field("location"),
            name: o.get_field("name"),
            plan: o.get_field("plan"),
            project: o.get_field("project"),
            renewal_plan: o.get_field("renewalPlan"),
            slot_count: o.get_field("slotCount"),
            state: o.get_field("state"),
        }
    }
}
