#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod get_patch_baseline {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GetPatchBaselineArgs {
        /// Filters the results against the baselines default_baseline field.
        #[builder(into, default)]
        pub default_baseline: pulumi_gestalt_rust::InputOrOutput<Option<bool>>,
        /// Filter results by the baseline name prefix.
        #[builder(into, default)]
        pub name_prefix: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Specified OS for the baseline. Valid values: `AMAZON_LINUX`, `AMAZON_LINUX_2`, `UBUNTU`, `REDHAT_ENTERPRISE_LINUX`, `SUSE`, `CENTOS`, `ORACLE_LINUX`, `DEBIAN`, `MACOS`, `RASPBIAN` and `ROCKY_LINUX`.
        #[builder(into, default)]
        pub operating_system: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Owner of the baseline. Valid values: `All`, `AWS`, `Self` (the current account).
        ///
        /// The following arguments are optional:
        #[builder(into)]
        pub owner: pulumi_gestalt_rust::InputOrOutput<String>,
    }
    #[allow(dead_code)]
    pub struct GetPatchBaselineResult {
        /// List of rules used to include patches in the baseline.
        pub approval_rules: pulumi_gestalt_rust::Output<
            Vec<super::super::super::types::ssm::GetPatchBaselineApprovalRule>,
        >,
        /// List of explicitly approved patches for the baseline.
        pub approved_patches: pulumi_gestalt_rust::Output<Vec<String>>,
        /// Compliance level for approved patches.
        pub approved_patches_compliance_level: pulumi_gestalt_rust::Output<String>,
        /// Indicates whether the list of approved patches includes non-security updates that should be applied to the instances.
        pub approved_patches_enable_non_security: pulumi_gestalt_rust::Output<bool>,
        pub default_baseline: pulumi_gestalt_rust::Output<Option<bool>>,
        /// Description of the baseline.
        pub description: pulumi_gestalt_rust::Output<String>,
        /// Set of global filters used to exclude patches from the baseline.
        pub global_filters: pulumi_gestalt_rust::Output<
            Vec<super::super::super::types::ssm::GetPatchBaselineGlobalFilter>,
        >,
        /// The provider-assigned unique ID for this managed resource.
        pub id: pulumi_gestalt_rust::Output<String>,
        /// JSON representation of the baseline.
        pub json: pulumi_gestalt_rust::Output<String>,
        /// Name specified to identify the patch source.
        pub name: pulumi_gestalt_rust::Output<String>,
        pub name_prefix: pulumi_gestalt_rust::Output<Option<String>>,
        pub operating_system: pulumi_gestalt_rust::Output<Option<String>>,
        pub owner: pulumi_gestalt_rust::Output<String>,
        /// List of rejected patches.
        pub rejected_patches: pulumi_gestalt_rust::Output<Vec<String>>,
        /// Action specified to take on patches included in the `rejected_patches` list.
        pub rejected_patches_action: pulumi_gestalt_rust::Output<String>,
        /// Information about the patches to use to update the managed nodes, including target operating systems and source repositories.
        pub sources: pulumi_gestalt_rust::Output<
            Vec<super::super::super::types::ssm::GetPatchBaselineSource>,
        >,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn invoke(
        context: &pulumi_gestalt_rust::Context,
        args: GetPatchBaselineArgs,
    ) -> GetPatchBaselineResult {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::register_interface;
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let default_baseline_binding = args.default_baseline.get_output(context);
        let name_prefix_binding = args.name_prefix.get_output(context);
        let operating_system_binding = args.operating_system.get_output(context);
        let owner_binding = args.owner.get_output(context);
        let request = pulumi_gestalt_rust::InvokeResourceRequest {
            token: "aws:ssm/getPatchBaseline:getPatchBaseline".into(),
            version: super::super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "defaultBaseline".into(),
                    value: default_baseline_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "namePrefix".into(),
                    value: name_prefix_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "operatingSystem".into(),
                    value: operating_system_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "owner".into(),
                    value: owner_binding.get_id(),
                },
            ],
        };
        let o = context.invoke_resource(request);
        GetPatchBaselineResult {
            approval_rules: o.get_field("approvalRules"),
            approved_patches: o.get_field("approvedPatches"),
            approved_patches_compliance_level: o
                .get_field("approvedPatchesComplianceLevel"),
            approved_patches_enable_non_security: o
                .get_field("approvedPatchesEnableNonSecurity"),
            default_baseline: o.get_field("defaultBaseline"),
            description: o.get_field("description"),
            global_filters: o.get_field("globalFilters"),
            id: o.get_field("id"),
            json: o.get_field("json"),
            name: o.get_field("name"),
            name_prefix: o.get_field("namePrefix"),
            operating_system: o.get_field("operatingSystem"),
            owner: o.get_field("owner"),
            rejected_patches: o.get_field("rejectedPatches"),
            rejected_patches_action: o.get_field("rejectedPatchesAction"),
            sources: o.get_field("sources"),
        }
    }
}
