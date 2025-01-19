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
/// use pulumi_wasm_rust::Output;
/// use pulumi_wasm_rust::{add_export, pulumi_main};
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
pub mod capacity_commitment {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct CapacityCommitmentArgs {
        /// The optional capacity commitment ID. Capacity commitment name will be generated automatically if this field is
        /// empty. This field must only contain lower case alphanumeric characters or dashes. The first and last character
        /// cannot be a dash. Max length is 64 characters. NOTE: this ID won't be kept if the capacity commitment is split
        /// or merged.
        #[builder(into, default)]
        pub capacity_commitment_id: pulumi_wasm_rust::Output<Option<String>>,
        /// The edition type. Valid values are STANDARD, ENTERPRISE, ENTERPRISE_PLUS
        #[builder(into, default)]
        pub edition: pulumi_wasm_rust::Output<Option<String>>,
        /// If true, fail the request if another project in the organization has a capacity commitment.
        #[builder(into, default)]
        pub enforce_single_admin_project_per_org: pulumi_wasm_rust::Output<
            Option<String>,
        >,
        /// The geographic location where the transfer config should reside.
        /// Examples: US, EU, asia-northeast1. The default value is US.
        #[builder(into, default)]
        pub location: pulumi_wasm_rust::Output<Option<String>>,
        /// Capacity commitment plan. Valid values are at https://cloud.google.com/bigquery/docs/reference/reservations/rpc/google.cloud.bigquery.reservation.v1#commitmentplan
        ///
        ///
        /// - - -
        #[builder(into)]
        pub plan: pulumi_wasm_rust::Output<String>,
        /// The ID of the project in which the resource belongs.
        /// If it is not provided, the provider project is used.
        #[builder(into, default)]
        pub project: pulumi_wasm_rust::Output<Option<String>>,
        /// The plan this capacity commitment is converted to after commitmentEndTime passes. Once the plan is changed, committed period is extended according to commitment plan. Only applicable for some commitment plans.
        #[builder(into, default)]
        pub renewal_plan: pulumi_wasm_rust::Output<Option<String>>,
        /// Number of slots in this commitment.
        #[builder(into)]
        pub slot_count: pulumi_wasm_rust::Output<i32>,
    }
    #[allow(dead_code)]
    pub struct CapacityCommitmentResult {
        /// The optional capacity commitment ID. Capacity commitment name will be generated automatically if this field is
        /// empty. This field must only contain lower case alphanumeric characters or dashes. The first and last character
        /// cannot be a dash. Max length is 64 characters. NOTE: this ID won't be kept if the capacity commitment is split
        /// or merged.
        pub capacity_commitment_id: pulumi_wasm_rust::Output<Option<String>>,
        /// The start of the current commitment period. It is applicable only for ACTIVE capacity commitments.
        pub commitment_end_time: pulumi_wasm_rust::Output<String>,
        /// The start of the current commitment period. It is applicable only for ACTIVE capacity commitments.
        pub commitment_start_time: pulumi_wasm_rust::Output<String>,
        /// The edition type. Valid values are STANDARD, ENTERPRISE, ENTERPRISE_PLUS
        pub edition: pulumi_wasm_rust::Output<Option<String>>,
        /// If true, fail the request if another project in the organization has a capacity commitment.
        pub enforce_single_admin_project_per_org: pulumi_wasm_rust::Output<
            Option<String>,
        >,
        /// The geographic location where the transfer config should reside.
        /// Examples: US, EU, asia-northeast1. The default value is US.
        pub location: pulumi_wasm_rust::Output<Option<String>>,
        /// The resource name of the capacity commitment, e.g., projects/myproject/locations/US/capacityCommitments/123
        pub name: pulumi_wasm_rust::Output<String>,
        /// Capacity commitment plan. Valid values are at https://cloud.google.com/bigquery/docs/reference/reservations/rpc/google.cloud.bigquery.reservation.v1#commitmentplan
        ///
        ///
        /// - - -
        pub plan: pulumi_wasm_rust::Output<String>,
        /// The ID of the project in which the resource belongs.
        /// If it is not provided, the provider project is used.
        pub project: pulumi_wasm_rust::Output<String>,
        /// The plan this capacity commitment is converted to after commitmentEndTime passes. Once the plan is changed, committed period is extended according to commitment plan. Only applicable for some commitment plans.
        pub renewal_plan: pulumi_wasm_rust::Output<Option<String>>,
        /// Number of slots in this commitment.
        pub slot_count: pulumi_wasm_rust::Output<i32>,
        /// State of the commitment
        pub state: pulumi_wasm_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(name: &str, args: CapacityCommitmentArgs) -> CapacityCommitmentResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let capacity_commitment_id_binding = args.capacity_commitment_id.get_inner();
        let edition_binding = args.edition.get_inner();
        let enforce_single_admin_project_per_org_binding = args
            .enforce_single_admin_project_per_org
            .get_inner();
        let location_binding = args.location.get_inner();
        let plan_binding = args.plan.get_inner();
        let project_binding = args.project.get_inner();
        let renewal_plan_binding = args.renewal_plan.get_inner();
        let slot_count_binding = args.slot_count.get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "gcp:bigquery/capacityCommitment:CapacityCommitment".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "capacityCommitmentId".into(),
                    value: &capacity_commitment_id_binding,
                },
                register_interface::ObjectField {
                    name: "edition".into(),
                    value: &edition_binding,
                },
                register_interface::ObjectField {
                    name: "enforceSingleAdminProjectPerOrg".into(),
                    value: &enforce_single_admin_project_per_org_binding,
                },
                register_interface::ObjectField {
                    name: "location".into(),
                    value: &location_binding,
                },
                register_interface::ObjectField {
                    name: "plan".into(),
                    value: &plan_binding,
                },
                register_interface::ObjectField {
                    name: "project".into(),
                    value: &project_binding,
                },
                register_interface::ObjectField {
                    name: "renewalPlan".into(),
                    value: &renewal_plan_binding,
                },
                register_interface::ObjectField {
                    name: "slotCount".into(),
                    value: &slot_count_binding,
                },
            ]),
            results: Vec::from([
                register_interface::ResultField {
                    name: "capacityCommitmentId".into(),
                },
                register_interface::ResultField {
                    name: "commitmentEndTime".into(),
                },
                register_interface::ResultField {
                    name: "commitmentStartTime".into(),
                },
                register_interface::ResultField {
                    name: "edition".into(),
                },
                register_interface::ResultField {
                    name: "enforceSingleAdminProjectPerOrg".into(),
                },
                register_interface::ResultField {
                    name: "location".into(),
                },
                register_interface::ResultField {
                    name: "name".into(),
                },
                register_interface::ResultField {
                    name: "plan".into(),
                },
                register_interface::ResultField {
                    name: "project".into(),
                },
                register_interface::ResultField {
                    name: "renewalPlan".into(),
                },
                register_interface::ResultField {
                    name: "slotCount".into(),
                },
                register_interface::ResultField {
                    name: "state".into(),
                },
            ]),
        };
        let o = register_interface::register(&request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        CapacityCommitmentResult {
            capacity_commitment_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("capacityCommitmentId").unwrap(),
            ),
            commitment_end_time: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("commitmentEndTime").unwrap(),
            ),
            commitment_start_time: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("commitmentStartTime").unwrap(),
            ),
            edition: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("edition").unwrap(),
            ),
            enforce_single_admin_project_per_org: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("enforceSingleAdminProjectPerOrg").unwrap(),
            ),
            location: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("location").unwrap(),
            ),
            name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("name").unwrap(),
            ),
            plan: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("plan").unwrap(),
            ),
            project: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("project").unwrap(),
            ),
            renewal_plan: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("renewalPlan").unwrap(),
            ),
            slot_count: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("slotCount").unwrap(),
            ),
            state: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("state").unwrap(),
            ),
        }
    }
}
