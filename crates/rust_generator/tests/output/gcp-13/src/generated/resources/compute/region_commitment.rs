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
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
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
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
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
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod region_commitment {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct RegionCommitmentArgs {
        /// Specifies whether to enable automatic renewal for the commitment.
        /// The default value is false if not specified.
        /// If the field is set to true, the commitment will be automatically renewed for either
        /// one or three years according to the terms of the existing commitment.
        #[builder(into, default)]
        pub auto_renew: pulumi_gestalt_rust::InputOrOutput<Option<bool>>,
        /// The category of the commitment. Category MACHINE specifies commitments composed of
        /// machine resources such as VCPU or MEMORY, listed in resources. Category LICENSE
        /// specifies commitments composed of software licenses, listed in licenseResources.
        /// Note that only MACHINE commitments should have a Type specified.
        /// Possible values are: `LICENSE`, `MACHINE`.
        #[builder(into, default)]
        pub category: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// An optional description of this resource.
        #[builder(into, default)]
        pub description: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Specifies the already existing reservations to attach to the Commitment.
        #[builder(into, default)]
        pub existing_reservations: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The license specification required as part of a license commitment.
        /// Structure is documented below.
        #[builder(into, default)]
        pub license_resource: pulumi_gestalt_rust::InputOrOutput<
            Option<super::super::types::compute::RegionCommitmentLicenseResource>,
        >,
        /// Name of the resource. The name must be 1-63 characters long and match
        /// the regular expression `a-z?` which means the
        /// first character must be a lowercase letter, and all following
        /// characters must be a dash, lowercase letter, or digit, except the last
        /// character, which cannot be a dash.
        #[builder(into, default)]
        pub name: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The plan for this commitment, which determines duration and discount rate.
        /// The currently supported plans are TWELVE_MONTH (1 year), and THIRTY_SIX_MONTH (3 years).
        /// Possible values are: `TWELVE_MONTH`, `THIRTY_SIX_MONTH`.
        ///
        ///
        /// - - -
        #[builder(into)]
        pub plan: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The ID of the project in which the resource belongs.
        /// If it is not provided, the provider project is used.
        #[builder(into, default)]
        pub project: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// URL of the region where this commitment may be used.
        #[builder(into, default)]
        pub region: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// A list of commitment amounts for particular resources.
        /// Note that VCPU and MEMORY resource commitments must occur together.
        /// Structure is documented below.
        #[builder(into, default)]
        pub resources: pulumi_gestalt_rust::InputOrOutput<
            Option<Vec<super::super::types::compute::RegionCommitmentResource>>,
        >,
        /// The type of commitment, which affects the discount rate and the eligible resources.
        /// The type could be one of the following value: `MEMORY_OPTIMIZED`, `ACCELERATOR_OPTIMIZED`,
        /// `GENERAL_PURPOSE_N1`, `GENERAL_PURPOSE_N2`, `GENERAL_PURPOSE_N2D`, `GENERAL_PURPOSE_E2`,
        /// `GENERAL_PURPOSE_T2D`, `GENERAL_PURPOSE_C3`, `COMPUTE_OPTIMIZED_C2`, `COMPUTE_OPTIMIZED_C2D` and
        /// `GRAPHICS_OPTIMIZED_G2`
        #[builder(into, default)]
        pub type_: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
    }
    #[allow(dead_code)]
    pub struct RegionCommitmentResult {
        /// Specifies whether to enable automatic renewal for the commitment.
        /// The default value is false if not specified.
        /// If the field is set to true, the commitment will be automatically renewed for either
        /// one or three years according to the terms of the existing commitment.
        pub auto_renew: pulumi_gestalt_rust::Output<bool>,
        /// The category of the commitment. Category MACHINE specifies commitments composed of
        /// machine resources such as VCPU or MEMORY, listed in resources. Category LICENSE
        /// specifies commitments composed of software licenses, listed in licenseResources.
        /// Note that only MACHINE commitments should have a Type specified.
        /// Possible values are: `LICENSE`, `MACHINE`.
        pub category: pulumi_gestalt_rust::Output<String>,
        /// Unique identifier for the resource.
        pub commitment_id: pulumi_gestalt_rust::Output<i32>,
        /// Creation timestamp in RFC3339 text format.
        pub creation_timestamp: pulumi_gestalt_rust::Output<String>,
        /// An optional description of this resource.
        pub description: pulumi_gestalt_rust::Output<Option<String>>,
        /// Commitment end time in RFC3339 text format.
        pub end_timestamp: pulumi_gestalt_rust::Output<String>,
        /// Specifies the already existing reservations to attach to the Commitment.
        pub existing_reservations: pulumi_gestalt_rust::Output<String>,
        /// The license specification required as part of a license commitment.
        /// Structure is documented below.
        pub license_resource: pulumi_gestalt_rust::Output<
            Option<super::super::types::compute::RegionCommitmentLicenseResource>,
        >,
        /// Name of the resource. The name must be 1-63 characters long and match
        /// the regular expression `a-z?` which means the
        /// first character must be a lowercase letter, and all following
        /// characters must be a dash, lowercase letter, or digit, except the last
        /// character, which cannot be a dash.
        pub name: pulumi_gestalt_rust::Output<String>,
        /// The plan for this commitment, which determines duration and discount rate.
        /// The currently supported plans are TWELVE_MONTH (1 year), and THIRTY_SIX_MONTH (3 years).
        /// Possible values are: `TWELVE_MONTH`, `THIRTY_SIX_MONTH`.
        ///
        ///
        /// - - -
        pub plan: pulumi_gestalt_rust::Output<String>,
        /// The ID of the project in which the resource belongs.
        /// If it is not provided, the provider project is used.
        pub project: pulumi_gestalt_rust::Output<String>,
        /// URL of the region where this commitment may be used.
        pub region: pulumi_gestalt_rust::Output<String>,
        /// A list of commitment amounts for particular resources.
        /// Note that VCPU and MEMORY resource commitments must occur together.
        /// Structure is documented below.
        pub resources: pulumi_gestalt_rust::Output<
            Option<Vec<super::super::types::compute::RegionCommitmentResource>>,
        >,
        /// The URI of the created resource.
        pub self_link: pulumi_gestalt_rust::Output<String>,
        /// Commitment start time in RFC3339 text format.
        pub start_timestamp: pulumi_gestalt_rust::Output<String>,
        /// Status of the commitment with regards to eventual expiration
        /// (each commitment has an end date defined).
        pub status: pulumi_gestalt_rust::Output<String>,
        /// A human-readable explanation of the status.
        pub status_message: pulumi_gestalt_rust::Output<String>,
        /// The type of commitment, which affects the discount rate and the eligible resources.
        /// The type could be one of the following value: `MEMORY_OPTIMIZED`, `ACCELERATOR_OPTIMIZED`,
        /// `GENERAL_PURPOSE_N1`, `GENERAL_PURPOSE_N2`, `GENERAL_PURPOSE_N2D`, `GENERAL_PURPOSE_E2`,
        /// `GENERAL_PURPOSE_T2D`, `GENERAL_PURPOSE_C3`, `COMPUTE_OPTIMIZED_C2`, `COMPUTE_OPTIMIZED_C2D` and
        /// `GRAPHICS_OPTIMIZED_G2`
        pub type_: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: RegionCommitmentArgs,
    ) -> RegionCommitmentResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let auto_renew_binding = args.auto_renew.get_output(context);
        let category_binding = args.category.get_output(context);
        let description_binding = args.description.get_output(context);
        let existing_reservations_binding = args
            .existing_reservations
            .get_output(context);
        let license_resource_binding = args.license_resource.get_output(context);
        let name_binding = args.name.get_output(context);
        let plan_binding = args.plan.get_output(context);
        let project_binding = args.project.get_output(context);
        let region_binding = args.region.get_output(context);
        let resources_binding = args.resources.get_output(context);
        let type__binding = args.type_.get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "gcp:compute/regionCommitment:RegionCommitment".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "autoRenew".into(),
                    value: auto_renew_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "category".into(),
                    value: category_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "description".into(),
                    value: description_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "existingReservations".into(),
                    value: existing_reservations_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "licenseResource".into(),
                    value: license_resource_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "name".into(),
                    value: name_binding.get_id(),
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
                    name: "region".into(),
                    value: region_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "resources".into(),
                    value: resources_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "type".into(),
                    value: type__binding.get_id(),
                },
            ],
        };
        let o = context.register_resource(request);
        RegionCommitmentResult {
            auto_renew: o.get_field("autoRenew"),
            category: o.get_field("category"),
            commitment_id: o.get_field("commitmentId"),
            creation_timestamp: o.get_field("creationTimestamp"),
            description: o.get_field("description"),
            end_timestamp: o.get_field("endTimestamp"),
            existing_reservations: o.get_field("existingReservations"),
            license_resource: o.get_field("licenseResource"),
            name: o.get_field("name"),
            plan: o.get_field("plan"),
            project: o.get_field("project"),
            region: o.get_field("region"),
            resources: o.get_field("resources"),
            self_link: o.get_field("selfLink"),
            start_timestamp: o.get_field("startTimestamp"),
            status: o.get_field("status"),
            status_message: o.get_field("statusMessage"),
            type_: o.get_field("type"),
        }
    }
}
