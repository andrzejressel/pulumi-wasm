/// Represents a regional Commitment resource.
///
/// Creating a commitment resource means that you are purchasing a committed
/// use contract with an explicit start and end time. You can create commitments
/// based on vCPUs and memory usage and receive discounted rates.
///
///
/// To get more information about RegionCommitment, see:
///
/// * [API documentation](https://cloud.google.com/compute/docs/reference/rest/v1/regionCommitments)
/// * How-to Guides
///     * [Committed use discounts for Compute Engine](https://cloud.google.com/compute/docs/instances/committed-use-discounts-overview)
///
/// ## Example Usage
///
/// ### Compute Region Commitment Basic
///
///
/// ```ignore
/// use pulumi_wasm_rust::Output;
/// use pulumi_wasm_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let foobar = region_commitment::create(
///         "foobar",
///         RegionCommitmentArgs::builder()
///             .name("my-region-commitment")
///             .plan("THIRTY_SIX_MONTH")
///             .resources(
///                 vec![
///                     RegionCommitmentResource::builder().amount("4"). type ("VCPU")
///                     .build_struct(), RegionCommitmentResource::builder().amount("9").
///                     type ("MEMORY").build_struct(),
///                 ],
///             )
///             .build_struct(),
///     );
/// }
/// ```
/// ### Compute Region Commitment Full
///
///
/// ```ignore
/// use pulumi_wasm_rust::Output;
/// use pulumi_wasm_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let foobar = region_commitment::create(
///         "foobar",
///         RegionCommitmentArgs::builder()
///             .auto_renew(true)
///             .category("MACHINE")
///             .description("some description")
///             .name("my-full-commitment")
///             .plan("THIRTY_SIX_MONTH")
///             .resources(
///                 vec![
///                     RegionCommitmentResource::builder().amount("4"). type ("VCPU")
///                     .build_struct(), RegionCommitmentResource::builder().amount("9").
///                     type ("MEMORY").build_struct(),
///                 ],
///             )
///             .type_("MEMORY_OPTIMIZED")
///             .build_struct(),
///     );
/// }
/// ```
///
/// ## Import
///
/// RegionCommitment can be imported using any of these accepted formats:
///
/// * `projects/{{project}}/regions/{{region}}/commitments/{{name}}`
///
/// * `{{project}}/{{region}}/{{name}}`
///
/// * `{{region}}/{{name}}`
///
/// * `{{name}}`
///
/// When using the `pulumi import` command, RegionCommitment can be imported using one of the formats above. For example:
///
/// ```sh
/// $ pulumi import gcp:compute/regionCommitment:RegionCommitment default projects/{{project}}/regions/{{region}}/commitments/{{name}}
/// ```
///
/// ```sh
/// $ pulumi import gcp:compute/regionCommitment:RegionCommitment default {{project}}/{{region}}/{{name}}
/// ```
///
/// ```sh
/// $ pulumi import gcp:compute/regionCommitment:RegionCommitment default {{region}}/{{name}}
/// ```
///
/// ```sh
/// $ pulumi import gcp:compute/regionCommitment:RegionCommitment default {{name}}
/// ```
///
pub mod region_commitment {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct RegionCommitmentArgs {
        /// Specifies whether to enable automatic renewal for the commitment.
        /// The default value is false if not specified.
        /// If the field is set to true, the commitment will be automatically renewed for either
        /// one or three years according to the terms of the existing commitment.
        #[builder(into, default)]
        pub auto_renew: pulumi_wasm_rust::Output<Option<bool>>,
        /// The category of the commitment. Category MACHINE specifies commitments composed of
        /// machine resources such as VCPU or MEMORY, listed in resources. Category LICENSE
        /// specifies commitments composed of software licenses, listed in licenseResources.
        /// Note that only MACHINE commitments should have a Type specified.
        /// Possible values are: `LICENSE`, `MACHINE`.
        #[builder(into, default)]
        pub category: pulumi_wasm_rust::Output<Option<String>>,
        /// An optional description of this resource.
        #[builder(into, default)]
        pub description: pulumi_wasm_rust::Output<Option<String>>,
        /// Specifies the already existing reservations to attach to the Commitment.
        #[builder(into, default)]
        pub existing_reservations: pulumi_wasm_rust::Output<Option<String>>,
        /// The license specification required as part of a license commitment.
        /// Structure is documented below.
        #[builder(into, default)]
        pub license_resource: pulumi_wasm_rust::Output<
            Option<super::super::types::compute::RegionCommitmentLicenseResource>,
        >,
        /// Name of the resource. The name must be 1-63 characters long and match
        /// the regular expression `a-z?` which means the
        /// first character must be a lowercase letter, and all following
        /// characters must be a dash, lowercase letter, or digit, except the last
        /// character, which cannot be a dash.
        #[builder(into, default)]
        pub name: pulumi_wasm_rust::Output<Option<String>>,
        /// The plan for this commitment, which determines duration and discount rate.
        /// The currently supported plans are TWELVE_MONTH (1 year), and THIRTY_SIX_MONTH (3 years).
        /// Possible values are: `TWELVE_MONTH`, `THIRTY_SIX_MONTH`.
        ///
        ///
        /// - - -
        #[builder(into)]
        pub plan: pulumi_wasm_rust::Output<String>,
        /// The ID of the project in which the resource belongs.
        /// If it is not provided, the provider project is used.
        #[builder(into, default)]
        pub project: pulumi_wasm_rust::Output<Option<String>>,
        /// URL of the region where this commitment may be used.
        #[builder(into, default)]
        pub region: pulumi_wasm_rust::Output<Option<String>>,
        /// A list of commitment amounts for particular resources.
        /// Note that VCPU and MEMORY resource commitments must occur together.
        /// Structure is documented below.
        #[builder(into, default)]
        pub resources: pulumi_wasm_rust::Output<
            Option<Vec<super::super::types::compute::RegionCommitmentResource>>,
        >,
        /// The type of commitment, which affects the discount rate and the eligible resources.
        /// The type could be one of the following value: `MEMORY_OPTIMIZED`, `ACCELERATOR_OPTIMIZED`,
        /// `GENERAL_PURPOSE_N1`, `GENERAL_PURPOSE_N2`, `GENERAL_PURPOSE_N2D`, `GENERAL_PURPOSE_E2`,
        /// `GENERAL_PURPOSE_T2D`, `GENERAL_PURPOSE_C3`, `COMPUTE_OPTIMIZED_C2`, `COMPUTE_OPTIMIZED_C2D` and
        /// `GRAPHICS_OPTIMIZED_G2`
        #[builder(into, default)]
        pub type_: pulumi_wasm_rust::Output<Option<String>>,
    }
    #[allow(dead_code)]
    pub struct RegionCommitmentResult {
        /// Specifies whether to enable automatic renewal for the commitment.
        /// The default value is false if not specified.
        /// If the field is set to true, the commitment will be automatically renewed for either
        /// one or three years according to the terms of the existing commitment.
        pub auto_renew: pulumi_wasm_rust::Output<bool>,
        /// The category of the commitment. Category MACHINE specifies commitments composed of
        /// machine resources such as VCPU or MEMORY, listed in resources. Category LICENSE
        /// specifies commitments composed of software licenses, listed in licenseResources.
        /// Note that only MACHINE commitments should have a Type specified.
        /// Possible values are: `LICENSE`, `MACHINE`.
        pub category: pulumi_wasm_rust::Output<String>,
        /// Unique identifier for the resource.
        pub commitment_id: pulumi_wasm_rust::Output<i32>,
        /// Creation timestamp in RFC3339 text format.
        pub creation_timestamp: pulumi_wasm_rust::Output<String>,
        /// An optional description of this resource.
        pub description: pulumi_wasm_rust::Output<Option<String>>,
        /// Commitment end time in RFC3339 text format.
        pub end_timestamp: pulumi_wasm_rust::Output<String>,
        /// Specifies the already existing reservations to attach to the Commitment.
        pub existing_reservations: pulumi_wasm_rust::Output<String>,
        /// The license specification required as part of a license commitment.
        /// Structure is documented below.
        pub license_resource: pulumi_wasm_rust::Output<
            Option<super::super::types::compute::RegionCommitmentLicenseResource>,
        >,
        /// Name of the resource. The name must be 1-63 characters long and match
        /// the regular expression `a-z?` which means the
        /// first character must be a lowercase letter, and all following
        /// characters must be a dash, lowercase letter, or digit, except the last
        /// character, which cannot be a dash.
        pub name: pulumi_wasm_rust::Output<String>,
        /// The plan for this commitment, which determines duration and discount rate.
        /// The currently supported plans are TWELVE_MONTH (1 year), and THIRTY_SIX_MONTH (3 years).
        /// Possible values are: `TWELVE_MONTH`, `THIRTY_SIX_MONTH`.
        ///
        ///
        /// - - -
        pub plan: pulumi_wasm_rust::Output<String>,
        /// The ID of the project in which the resource belongs.
        /// If it is not provided, the provider project is used.
        pub project: pulumi_wasm_rust::Output<String>,
        /// URL of the region where this commitment may be used.
        pub region: pulumi_wasm_rust::Output<String>,
        /// A list of commitment amounts for particular resources.
        /// Note that VCPU and MEMORY resource commitments must occur together.
        /// Structure is documented below.
        pub resources: pulumi_wasm_rust::Output<
            Option<Vec<super::super::types::compute::RegionCommitmentResource>>,
        >,
        /// The URI of the created resource.
        pub self_link: pulumi_wasm_rust::Output<String>,
        /// Commitment start time in RFC3339 text format.
        pub start_timestamp: pulumi_wasm_rust::Output<String>,
        /// Status of the commitment with regards to eventual expiration
        /// (each commitment has an end date defined).
        pub status: pulumi_wasm_rust::Output<String>,
        /// A human-readable explanation of the status.
        pub status_message: pulumi_wasm_rust::Output<String>,
        /// The type of commitment, which affects the discount rate and the eligible resources.
        /// The type could be one of the following value: `MEMORY_OPTIMIZED`, `ACCELERATOR_OPTIMIZED`,
        /// `GENERAL_PURPOSE_N1`, `GENERAL_PURPOSE_N2`, `GENERAL_PURPOSE_N2D`, `GENERAL_PURPOSE_E2`,
        /// `GENERAL_PURPOSE_T2D`, `GENERAL_PURPOSE_C3`, `COMPUTE_OPTIMIZED_C2`, `COMPUTE_OPTIMIZED_C2D` and
        /// `GRAPHICS_OPTIMIZED_G2`
        pub type_: pulumi_wasm_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(name: &str, args: RegionCommitmentArgs) -> RegionCommitmentResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let auto_renew_binding = args.auto_renew.get_inner();
        let category_binding = args.category.get_inner();
        let description_binding = args.description.get_inner();
        let existing_reservations_binding = args.existing_reservations.get_inner();
        let license_resource_binding = args.license_resource.get_inner();
        let name_binding = args.name.get_inner();
        let plan_binding = args.plan.get_inner();
        let project_binding = args.project.get_inner();
        let region_binding = args.region.get_inner();
        let resources_binding = args.resources.get_inner();
        let type__binding = args.type_.get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "gcp:compute/regionCommitment:RegionCommitment".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "autoRenew".into(),
                    value: &auto_renew_binding,
                },
                register_interface::ObjectField {
                    name: "category".into(),
                    value: &category_binding,
                },
                register_interface::ObjectField {
                    name: "description".into(),
                    value: &description_binding,
                },
                register_interface::ObjectField {
                    name: "existingReservations".into(),
                    value: &existing_reservations_binding,
                },
                register_interface::ObjectField {
                    name: "licenseResource".into(),
                    value: &license_resource_binding,
                },
                register_interface::ObjectField {
                    name: "name".into(),
                    value: &name_binding,
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
                    name: "region".into(),
                    value: &region_binding,
                },
                register_interface::ObjectField {
                    name: "resources".into(),
                    value: &resources_binding,
                },
                register_interface::ObjectField {
                    name: "type".into(),
                    value: &type__binding,
                },
            ]),
            results: Vec::from([
                register_interface::ResultField {
                    name: "autoRenew".into(),
                },
                register_interface::ResultField {
                    name: "category".into(),
                },
                register_interface::ResultField {
                    name: "commitmentId".into(),
                },
                register_interface::ResultField {
                    name: "creationTimestamp".into(),
                },
                register_interface::ResultField {
                    name: "description".into(),
                },
                register_interface::ResultField {
                    name: "endTimestamp".into(),
                },
                register_interface::ResultField {
                    name: "existingReservations".into(),
                },
                register_interface::ResultField {
                    name: "licenseResource".into(),
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
                    name: "region".into(),
                },
                register_interface::ResultField {
                    name: "resources".into(),
                },
                register_interface::ResultField {
                    name: "selfLink".into(),
                },
                register_interface::ResultField {
                    name: "startTimestamp".into(),
                },
                register_interface::ResultField {
                    name: "status".into(),
                },
                register_interface::ResultField {
                    name: "statusMessage".into(),
                },
                register_interface::ResultField {
                    name: "type".into(),
                },
            ]),
        };
        let o = register_interface::register(&request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        RegionCommitmentResult {
            auto_renew: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("autoRenew").unwrap(),
            ),
            category: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("category").unwrap(),
            ),
            commitment_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("commitmentId").unwrap(),
            ),
            creation_timestamp: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("creationTimestamp").unwrap(),
            ),
            description: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("description").unwrap(),
            ),
            end_timestamp: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("endTimestamp").unwrap(),
            ),
            existing_reservations: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("existingReservations").unwrap(),
            ),
            license_resource: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("licenseResource").unwrap(),
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
            region: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("region").unwrap(),
            ),
            resources: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("resources").unwrap(),
            ),
            self_link: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("selfLink").unwrap(),
            ),
            start_timestamp: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("startTimestamp").unwrap(),
            ),
            status: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("status").unwrap(),
            ),
            status_message: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("statusMessage").unwrap(),
            ),
            type_: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("type").unwrap(),
            ),
        }
    }
}
