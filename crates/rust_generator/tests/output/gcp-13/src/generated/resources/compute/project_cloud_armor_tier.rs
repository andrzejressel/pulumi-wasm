/// Sets the Cloud Armor tier of the project.
///
///
/// To get more information about ProjectCloudArmorTier, see:
///
/// * [API documentation](https://cloud.google.com/compute/docs/reference/rest/v1/projects/setCloudArmorTier)
/// * How-to Guides
///     * [Subscribing to Cloud Armor Enterprise](https://cloud.google.com/armor/docs/managed-protection-overview#subscribing_to_plus)
///
/// ## Example Usage
///
/// ### Compute Project Cloud Armor Tier Basic
///
///
/// ```ignore
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let cloudArmorTierConfig = project_cloud_armor_tier::create(
///         "cloudArmorTierConfig",
///         ProjectCloudArmorTierArgs::builder()
///             .cloud_armor_tier("CA_STANDARD")
///             .build_struct(),
///     );
/// }
/// ```
/// ### Compute Project Cloud Armor Tier Project Set
///
///
/// ```ignore
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let cloudArmorTierConfig = project_cloud_armor_tier::create(
///         "cloudArmorTierConfig",
///         ProjectCloudArmorTierArgs::builder()
///             .cloud_armor_tier("CA_STANDARD")
///             .project("${project.projectId}")
///             .build_struct(),
///     );
///     let compute = service::create(
///         "compute",
///         ServiceArgs::builder()
///             .project("${project.projectId}")
///             .service("compute.googleapis.com")
///             .build_struct(),
///     );
///     let project = project::create(
///         "project",
///         ProjectArgs::builder()
///             .billing_account("000000-0000000-0000000-000000")
///             .deletion_policy("DELETE")
///             .name("your_project_id")
///             .org_id("123456789")
///             .project_id("your_project_id")
///             .build_struct(),
///     );
/// }
/// ```
///
/// ## Import
///
/// ProjectCloudArmorTier can be imported using any of these accepted formats:
///
/// * `projects/{{project}}`
///
/// * `{{project}}`
///
/// When using the `pulumi import` command, ProjectCloudArmorTier can be imported using one of the formats above. For example:
///
/// ```sh
/// $ pulumi import gcp:compute/projectCloudArmorTier:ProjectCloudArmorTier default projects/{{project}}
/// ```
///
/// ```sh
/// $ pulumi import gcp:compute/projectCloudArmorTier:ProjectCloudArmorTier default {{project}}
/// ```
///
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod project_cloud_armor_tier {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct ProjectCloudArmorTierArgs {
        /// Managed protection tier to be set.
        /// Possible values are: `CA_STANDARD`, `CA_ENTERPRISE_PAYGO`, `CA_ENTERPRISE_ANNUAL`.
        ///
        ///
        /// - - -
        #[builder(into)]
        pub cloud_armor_tier: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The ID of the project in which the resource belongs.
        /// If it is not provided, the provider project is used.
        #[builder(into, default)]
        pub project: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
    }
    #[allow(dead_code)]
    pub struct ProjectCloudArmorTierResult {
        /// Managed protection tier to be set.
        /// Possible values are: `CA_STANDARD`, `CA_ENTERPRISE_PAYGO`, `CA_ENTERPRISE_ANNUAL`.
        ///
        ///
        /// - - -
        pub cloud_armor_tier: pulumi_gestalt_rust::Output<String>,
        /// The ID of the project in which the resource belongs.
        /// If it is not provided, the provider project is used.
        pub project: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: ProjectCloudArmorTierArgs,
    ) -> ProjectCloudArmorTierResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let cloud_armor_tier_binding = args.cloud_armor_tier.get_output(context);
        let project_binding = args.project.get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "gcp:compute/projectCloudArmorTier:ProjectCloudArmorTier".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "cloudArmorTier".into(),
                    value: cloud_armor_tier_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "project".into(),
                    value: project_binding.get_id(),
                },
            ],
        };
        let o = context.register_resource(request);
        ProjectCloudArmorTierResult {
            cloud_armor_tier: o.get_field("cloudArmorTier"),
            project: o.get_field("project"),
        }
    }
}
