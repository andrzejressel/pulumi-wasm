/// Provides an SSM Patch Baseline resource.
///
/// > **NOTE on Patch Baselines:** The `approved_patches` and `approval_rule` are
/// both marked as optional fields, but the Patch Baseline requires that at least one
/// of them is specified.
///
/// ## Example Usage
///
/// ### Basic Usage
///
/// Using `approved_patches` only.
///
/// ```ignore
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let production = patch_baseline::create(
///         "production",
///         PatchBaselineArgs::builder()
///             .approved_patches(vec!["KB123456",])
///             .name("patch-baseline")
///             .build_struct(),
///     );
/// }
/// ```
///
/// ### Advanced Usage, specifying patch filters
///
/// ```ignore
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let production = patch_baseline::create(
///         "production",
///         PatchBaselineArgs::builder()
///             .approval_rules(
///                 vec![
///                     PatchBaselineApprovalRule::builder().approveAfterDays(7)
///                     .complianceLevel("HIGH")
///                     .patchFilters(vec![PatchBaselineApprovalRulePatchFilter::builder()
///                     .key("PRODUCT").values(vec!["WindowsServer2016",]).build_struct(),
///                     PatchBaselineApprovalRulePatchFilter::builder().key("CLASSIFICATION")
///                     .values(vec!["CriticalUpdates", "SecurityUpdates", "Updates",])
///                     .build_struct(), PatchBaselineApprovalRulePatchFilter::builder()
///                     .key("MSRC_SEVERITY").values(vec!["Critical", "Important",
///                     "Moderate",]).build_struct(),]).build_struct(),
///                     PatchBaselineApprovalRule::builder().approveAfterDays(7)
///                     .patchFilters(vec![PatchBaselineApprovalRulePatchFilter::builder()
///                     .key("PRODUCT").values(vec!["WindowsServer2012",]).build_struct(),])
///                     .build_struct(),
///                 ],
///             )
///             .approved_patches(vec!["KB123456", "KB456789",])
///             .description("Patch Baseline Description")
///             .global_filters(
///                 vec![
///                     PatchBaselineGlobalFilter::builder().key("PRODUCT")
///                     .values(vec!["WindowsServer2008",]).build_struct(),
///                     PatchBaselineGlobalFilter::builder().key("CLASSIFICATION")
///                     .values(vec!["ServicePacks",]).build_struct(),
///                     PatchBaselineGlobalFilter::builder().key("MSRC_SEVERITY")
///                     .values(vec!["Low",]).build_struct(),
///                 ],
///             )
///             .name("patch-baseline")
///             .rejected_patches(vec!["KB987654",])
///             .build_struct(),
///     );
/// }
/// ```
///
/// ### Advanced usage, specifying Microsoft application and Windows patch rules
///
/// ```ignore
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let windowsOsApps = patch_baseline::create(
///         "windowsOsApps",
///         PatchBaselineArgs::builder()
///             .approval_rules(
///                 vec![
///                     PatchBaselineApprovalRule::builder().approveAfterDays(7)
///                     .patchFilters(vec![PatchBaselineApprovalRulePatchFilter::builder()
///                     .key("CLASSIFICATION").values(vec!["CriticalUpdates",
///                     "SecurityUpdates",]).build_struct(),
///                     PatchBaselineApprovalRulePatchFilter::builder().key("MSRC_SEVERITY")
///                     .values(vec!["Critical", "Important",]).build_struct(),])
///                     .build_struct(), PatchBaselineApprovalRule::builder()
///                     .approveAfterDays(7)
///                     .patchFilters(vec![PatchBaselineApprovalRulePatchFilter::builder()
///                     .key("PATCH_SET").values(vec!["APPLICATION",]).build_struct(),
///                     PatchBaselineApprovalRulePatchFilter::builder().key("PRODUCT")
///                     .values(vec!["Office 2013", "Office 2016",]).build_struct(),])
///                     .build_struct(),
///                 ],
///             )
///             .description("Patch both Windows and Microsoft apps")
///             .name("WindowsOSAndMicrosoftApps")
///             .operating_system("WINDOWS")
///             .build_struct(),
///     );
/// }
/// ```
///
/// ### Advanced usage, specifying alternate patch source repository
///
/// ```ignore
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let al201709 = patch_baseline::create(
///         "al201709",
///         PatchBaselineArgs::builder()
///             .approval_rules(vec![PatchBaselineApprovalRule::builder().build_struct(),])
///             .description("My patch repository for Amazon Linux 2017.09")
///             .name("Amazon-Linux-2017.09")
///             .operating_system("AMAZON_LINUX")
///             .sources(
///                 vec![
///                     PatchBaselineSource::builder()
///                     .configuration("[amzn-main]\nname=amzn-main-Base\nmirrorlist=http://repo./$awsregion./$awsdomain//$releasever/main/mirror.list\nmirrorlist_expire=300\nmetadata_expire=300\npriority=10\nfailovermethod=priority\nfastestmirror_enabled=0\ngpgcheck=1\ngpgkey=file:///etc/pki/rpm-gpg/RPM-GPG-KEY-amazon-ga\nenabled=1\nretries=3\ntimeout=5\nreport_instanceid=yes")
///                     .name("My-AL2017.09").products(vec!["AmazonLinux2017.09",])
///                     .build_struct(),
///                 ],
///             )
///             .build_struct(),
///     );
/// }
/// ```
///
/// ## Import
///
/// Using `pulumi import`, import SSM Patch Baselines using their baseline ID. For example:
///
/// ```sh
/// $ pulumi import aws:ssm/patchBaseline:PatchBaseline example pb-12345678
/// ```
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod patch_baseline {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct PatchBaselineArgs {
        /// Set of rules used to include patches in the baseline. Up to 10 approval rules can be specified. See `approval_rule` below.
        #[builder(into, default)]
        pub approval_rules: pulumi_gestalt_rust::InputOrOutput<
            Option<Vec<super::super::types::ssm::PatchBaselineApprovalRule>>,
        >,
        /// List of explicitly approved patches for the baseline. Cannot be specified with `approval_rule`.
        #[builder(into, default)]
        pub approved_patches: pulumi_gestalt_rust::InputOrOutput<Option<Vec<String>>>,
        /// Compliance level for approved patches. This means that if an approved patch is reported as missing, this is the severity of the compliance violation. Valid values are `CRITICAL`, `HIGH`, `MEDIUM`, `LOW`, `INFORMATIONAL`, `UNSPECIFIED`. The default value is `UNSPECIFIED`.
        #[builder(into, default)]
        pub approved_patches_compliance_level: pulumi_gestalt_rust::InputOrOutput<
            Option<String>,
        >,
        /// Whether the list of approved patches includes non-security updates that should be applied to the instances. Applies to Linux instances only.
        #[builder(into, default)]
        pub approved_patches_enable_non_security: pulumi_gestalt_rust::InputOrOutput<
            Option<bool>,
        >,
        /// Description of the patch baseline.
        #[builder(into, default)]
        pub description: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Set of global filters used to exclude patches from the baseline. Up to 4 global filters can be specified using Key/Value pairs. Valid Keys are `PRODUCT`, `CLASSIFICATION`, `MSRC_SEVERITY`, and `PATCH_ID`.
        #[builder(into, default)]
        pub global_filters: pulumi_gestalt_rust::InputOrOutput<
            Option<Vec<super::super::types::ssm::PatchBaselineGlobalFilter>>,
        >,
        /// Name of the patch baseline.
        ///
        /// The following arguments are optional:
        #[builder(into, default)]
        pub name: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Operating system the patch baseline applies to. Valid values are `ALMA_LINUX`, `AMAZON_LINUX`, `AMAZON_LINUX_2`, `AMAZON_LINUX_2022`, `AMAZON_LINUX_2023`, `CENTOS`, `DEBIAN`, `MACOS`, `ORACLE_LINUX`, `RASPBIAN`, `REDHAT_ENTERPRISE_LINUX`, `ROCKY_LINUX`, `SUSE`, `UBUNTU`, and `WINDOWS`. The default value is `WINDOWS`.
        #[builder(into, default)]
        pub operating_system: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// List of rejected patches.
        #[builder(into, default)]
        pub rejected_patches: pulumi_gestalt_rust::InputOrOutput<Option<Vec<String>>>,
        /// Action for Patch Manager to take on patches included in the `rejected_patches` list. Valid values are `ALLOW_AS_DEPENDENCY` and `BLOCK`.
        #[builder(into, default)]
        pub rejected_patches_action: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Configuration block with alternate sources for patches. Applies to Linux instances only. See `source` below.
        #[builder(into, default)]
        pub sources: pulumi_gestalt_rust::InputOrOutput<
            Option<Vec<super::super::types::ssm::PatchBaselineSource>>,
        >,
        /// Map of tags to assign to the resource. If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
        #[builder(into, default)]
        pub tags: pulumi_gestalt_rust::InputOrOutput<
            Option<std::collections::HashMap<String, String>>,
        >,
    }
    #[allow(dead_code)]
    pub struct PatchBaselineResult {
        /// Set of rules used to include patches in the baseline. Up to 10 approval rules can be specified. See `approval_rule` below.
        pub approval_rules: pulumi_gestalt_rust::Output<
            Option<Vec<super::super::types::ssm::PatchBaselineApprovalRule>>,
        >,
        /// List of explicitly approved patches for the baseline. Cannot be specified with `approval_rule`.
        pub approved_patches: pulumi_gestalt_rust::Output<Option<Vec<String>>>,
        /// Compliance level for approved patches. This means that if an approved patch is reported as missing, this is the severity of the compliance violation. Valid values are `CRITICAL`, `HIGH`, `MEDIUM`, `LOW`, `INFORMATIONAL`, `UNSPECIFIED`. The default value is `UNSPECIFIED`.
        pub approved_patches_compliance_level: pulumi_gestalt_rust::Output<
            Option<String>,
        >,
        /// Whether the list of approved patches includes non-security updates that should be applied to the instances. Applies to Linux instances only.
        pub approved_patches_enable_non_security: pulumi_gestalt_rust::Output<
            Option<bool>,
        >,
        /// ARN of the baseline.
        pub arn: pulumi_gestalt_rust::Output<String>,
        /// Description of the patch baseline.
        pub description: pulumi_gestalt_rust::Output<Option<String>>,
        /// Set of global filters used to exclude patches from the baseline. Up to 4 global filters can be specified using Key/Value pairs. Valid Keys are `PRODUCT`, `CLASSIFICATION`, `MSRC_SEVERITY`, and `PATCH_ID`.
        pub global_filters: pulumi_gestalt_rust::Output<
            Option<Vec<super::super::types::ssm::PatchBaselineGlobalFilter>>,
        >,
        /// JSON definition of the baseline.
        pub json: pulumi_gestalt_rust::Output<String>,
        /// Name of the patch baseline.
        ///
        /// The following arguments are optional:
        pub name: pulumi_gestalt_rust::Output<String>,
        /// Operating system the patch baseline applies to. Valid values are `ALMA_LINUX`, `AMAZON_LINUX`, `AMAZON_LINUX_2`, `AMAZON_LINUX_2022`, `AMAZON_LINUX_2023`, `CENTOS`, `DEBIAN`, `MACOS`, `ORACLE_LINUX`, `RASPBIAN`, `REDHAT_ENTERPRISE_LINUX`, `ROCKY_LINUX`, `SUSE`, `UBUNTU`, and `WINDOWS`. The default value is `WINDOWS`.
        pub operating_system: pulumi_gestalt_rust::Output<Option<String>>,
        /// List of rejected patches.
        pub rejected_patches: pulumi_gestalt_rust::Output<Option<Vec<String>>>,
        /// Action for Patch Manager to take on patches included in the `rejected_patches` list. Valid values are `ALLOW_AS_DEPENDENCY` and `BLOCK`.
        pub rejected_patches_action: pulumi_gestalt_rust::Output<String>,
        /// Configuration block with alternate sources for patches. Applies to Linux instances only. See `source` below.
        pub sources: pulumi_gestalt_rust::Output<
            Option<Vec<super::super::types::ssm::PatchBaselineSource>>,
        >,
        /// Map of tags to assign to the resource. If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
        pub tags: pulumi_gestalt_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// Map of tags assigned to the resource, including those inherited from the provider `default_tags` configuration block.
        pub tags_all: pulumi_gestalt_rust::Output<
            std::collections::HashMap<String, String>,
        >,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: PatchBaselineArgs,
    ) -> PatchBaselineResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let approval_rules_binding = args.approval_rules.get_output(context);
        let approved_patches_binding = args.approved_patches.get_output(context);
        let approved_patches_compliance_level_binding = args
            .approved_patches_compliance_level
            .get_output(context);
        let approved_patches_enable_non_security_binding = args
            .approved_patches_enable_non_security
            .get_output(context);
        let description_binding = args.description.get_output(context);
        let global_filters_binding = args.global_filters.get_output(context);
        let name_binding = args.name.get_output(context);
        let operating_system_binding = args.operating_system.get_output(context);
        let rejected_patches_binding = args.rejected_patches.get_output(context);
        let rejected_patches_action_binding = args
            .rejected_patches_action
            .get_output(context);
        let sources_binding = args.sources.get_output(context);
        let tags_binding = args.tags.get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "aws:ssm/patchBaseline:PatchBaseline".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "approvalRules".into(),
                    value: &approval_rules_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "approvedPatches".into(),
                    value: &approved_patches_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "approvedPatchesComplianceLevel".into(),
                    value: &approved_patches_compliance_level_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "approvedPatchesEnableNonSecurity".into(),
                    value: &approved_patches_enable_non_security_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "description".into(),
                    value: &description_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "globalFilters".into(),
                    value: &global_filters_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "name".into(),
                    value: &name_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "operatingSystem".into(),
                    value: &operating_system_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "rejectedPatches".into(),
                    value: &rejected_patches_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "rejectedPatchesAction".into(),
                    value: &rejected_patches_action_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "sources".into(),
                    value: &sources_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "tags".into(),
                    value: &tags_binding.drop_type(),
                },
            ],
        };
        let o = context.register_resource(request);
        PatchBaselineResult {
            approval_rules: o.get_field("approvalRules"),
            approved_patches: o.get_field("approvedPatches"),
            approved_patches_compliance_level: o
                .get_field("approvedPatchesComplianceLevel"),
            approved_patches_enable_non_security: o
                .get_field("approvedPatchesEnableNonSecurity"),
            arn: o.get_field("arn"),
            description: o.get_field("description"),
            global_filters: o.get_field("globalFilters"),
            json: o.get_field("json"),
            name: o.get_field("name"),
            operating_system: o.get_field("operatingSystem"),
            rejected_patches: o.get_field("rejectedPatches"),
            rejected_patches_action: o.get_field("rejectedPatchesAction"),
            sources: o.get_field("sources"),
            tags: o.get_field("tags"),
            tags_all: o.get_field("tagsAll"),
        }
    }
}
