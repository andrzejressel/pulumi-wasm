pub mod get_patch_baseline {
    #[derive(pulumi_wasm_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GetPatchBaselineArgs {
        /// Filters the results against the baselines default_baseline field.
        #[builder(into, default)]
        pub default_baseline: pulumi_wasm_rust::InputOrOutput<Option<bool>>,
        /// Filter results by the baseline name prefix.
        #[builder(into, default)]
        pub name_prefix: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        /// Specified OS for the baseline. Valid values: `AMAZON_LINUX`, `AMAZON_LINUX_2`, `UBUNTU`, `REDHAT_ENTERPRISE_LINUX`, `SUSE`, `CENTOS`, `ORACLE_LINUX`, `DEBIAN`, `MACOS`, `RASPBIAN` and `ROCKY_LINUX`.
        #[builder(into, default)]
        pub operating_system: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        /// Owner of the baseline. Valid values: `All`, `AWS`, `Self` (the current account).
        ///
        /// The following arguments are optional:
        #[builder(into)]
        pub owner: pulumi_wasm_rust::InputOrOutput<String>,
    }
    #[allow(dead_code)]
    pub struct GetPatchBaselineResult {
        /// List of rules used to include patches in the baseline.
        pub approval_rules: pulumi_wasm_rust::Output<
            Vec<super::super::super::types::ssm::GetPatchBaselineApprovalRule>,
        >,
        /// List of explicitly approved patches for the baseline.
        pub approved_patches: pulumi_wasm_rust::Output<Vec<String>>,
        /// Compliance level for approved patches.
        pub approved_patches_compliance_level: pulumi_wasm_rust::Output<String>,
        /// Indicates whether the list of approved patches includes non-security updates that should be applied to the instances.
        pub approved_patches_enable_non_security: pulumi_wasm_rust::Output<bool>,
        pub default_baseline: pulumi_wasm_rust::Output<Option<bool>>,
        /// Description of the baseline.
        pub description: pulumi_wasm_rust::Output<String>,
        /// Set of global filters used to exclude patches from the baseline.
        pub global_filters: pulumi_wasm_rust::Output<
            Vec<super::super::super::types::ssm::GetPatchBaselineGlobalFilter>,
        >,
        /// The provider-assigned unique ID for this managed resource.
        pub id: pulumi_wasm_rust::Output<String>,
        /// JSON representation of the baseline.
        pub json: pulumi_wasm_rust::Output<String>,
        /// Name specified to identify the patch source.
        pub name: pulumi_wasm_rust::Output<String>,
        pub name_prefix: pulumi_wasm_rust::Output<Option<String>>,
        pub operating_system: pulumi_wasm_rust::Output<Option<String>>,
        pub owner: pulumi_wasm_rust::Output<String>,
        /// List of rejected patches.
        pub rejected_patches: pulumi_wasm_rust::Output<Vec<String>>,
        /// Action specified to take on patches included in the `rejected_patches` list.
        pub rejected_patches_action: pulumi_wasm_rust::Output<String>,
        /// Information about the patches to use to update the managed nodes, including target operating systems and source repositories.
        pub sources: pulumi_wasm_rust::Output<
            Vec<super::super::super::types::ssm::GetPatchBaselineSource>,
        >,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn invoke(
        context: &pulumi_wasm_rust::PulumiContext,
        args: GetPatchBaselineArgs,
    ) -> GetPatchBaselineResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let default_baseline_binding = args
            .default_baseline
            .get_output(context)
            .get_inner();
        let name_prefix_binding = args.name_prefix.get_output(context).get_inner();
        let operating_system_binding = args
            .operating_system
            .get_output(context)
            .get_inner();
        let owner_binding = args.owner.get_output(context).get_inner();
        let request = register_interface::ResourceInvokeRequest {
            token: "aws:ssm/getPatchBaseline:getPatchBaseline".into(),
            version: super::super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "defaultBaseline".into(),
                    value: &default_baseline_binding,
                },
                register_interface::ObjectField {
                    name: "namePrefix".into(),
                    value: &name_prefix_binding,
                },
                register_interface::ObjectField {
                    name: "operatingSystem".into(),
                    value: &operating_system_binding,
                },
                register_interface::ObjectField {
                    name: "owner".into(),
                    value: &owner_binding,
                },
            ]),
            results: Vec::from([
                register_interface::ResultField {
                    name: "approvalRules".into(),
                },
                register_interface::ResultField {
                    name: "approvedPatches".into(),
                },
                register_interface::ResultField {
                    name: "approvedPatchesComplianceLevel".into(),
                },
                register_interface::ResultField {
                    name: "approvedPatchesEnableNonSecurity".into(),
                },
                register_interface::ResultField {
                    name: "defaultBaseline".into(),
                },
                register_interface::ResultField {
                    name: "description".into(),
                },
                register_interface::ResultField {
                    name: "globalFilters".into(),
                },
                register_interface::ResultField {
                    name: "id".into(),
                },
                register_interface::ResultField {
                    name: "json".into(),
                },
                register_interface::ResultField {
                    name: "name".into(),
                },
                register_interface::ResultField {
                    name: "namePrefix".into(),
                },
                register_interface::ResultField {
                    name: "operatingSystem".into(),
                },
                register_interface::ResultField {
                    name: "owner".into(),
                },
                register_interface::ResultField {
                    name: "rejectedPatches".into(),
                },
                register_interface::ResultField {
                    name: "rejectedPatchesAction".into(),
                },
                register_interface::ResultField {
                    name: "sources".into(),
                },
            ]),
        };
        let o = register_interface::invoke(context.get_inner(), &request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        GetPatchBaselineResult {
            approval_rules: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("approvalRules").unwrap(),
            ),
            approved_patches: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("approvedPatches").unwrap(),
            ),
            approved_patches_compliance_level: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("approvedPatchesComplianceLevel").unwrap(),
            ),
            approved_patches_enable_non_security: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("approvedPatchesEnableNonSecurity").unwrap(),
            ),
            default_baseline: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("defaultBaseline").unwrap(),
            ),
            description: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("description").unwrap(),
            ),
            global_filters: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("globalFilters").unwrap(),
            ),
            id: pulumi_wasm_rust::__private::into_domain(hashmap.remove("id").unwrap()),
            json: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("json").unwrap(),
            ),
            name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("name").unwrap(),
            ),
            name_prefix: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("namePrefix").unwrap(),
            ),
            operating_system: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("operatingSystem").unwrap(),
            ),
            owner: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("owner").unwrap(),
            ),
            rejected_patches: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("rejectedPatches").unwrap(),
            ),
            rejected_patches_action: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("rejectedPatchesAction").unwrap(),
            ),
            sources: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("sources").unwrap(),
            ),
        }
    }
}
