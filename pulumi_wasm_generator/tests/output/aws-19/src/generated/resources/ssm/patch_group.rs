/// Provides an SSM Patch Group resource
///
/// ## Example Usage
///
/// ```ignore
/// use pulumi_wasm_rust::Output;
/// use pulumi_wasm_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let patchgroup = patch_group::create(
///         "patchgroup",
///         PatchGroupArgs::builder()
///             .baseline_id("${production.id}")
///             .patch_group("patch-group-name")
///             .build_struct(),
///     );
///     let production = patch_baseline::create(
///         "production",
///         PatchBaselineArgs::builder()
///             .approved_patches(vec!["KB123456",])
///             .name("patch-baseline")
///             .build_struct(),
///     );
/// }
/// ```
pub mod patch_group {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct PatchGroupArgs {
        /// The ID of the patch baseline to register the patch group with.
        #[builder(into)]
        pub baseline_id: pulumi_wasm_rust::Output<String>,
        /// The name of the patch group that should be registered with the patch baseline.
        #[builder(into)]
        pub patch_group: pulumi_wasm_rust::Output<String>,
    }
    #[allow(dead_code)]
    pub struct PatchGroupResult {
        /// The ID of the patch baseline to register the patch group with.
        pub baseline_id: pulumi_wasm_rust::Output<String>,
        /// The name of the patch group that should be registered with the patch baseline.
        pub patch_group: pulumi_wasm_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(name: &str, args: PatchGroupArgs) -> PatchGroupResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let baseline_id_binding = args.baseline_id.get_inner();
        let patch_group_binding = args.patch_group.get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "aws:ssm/patchGroup:PatchGroup".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "baselineId".into(),
                    value: &baseline_id_binding,
                },
                register_interface::ObjectField {
                    name: "patchGroup".into(),
                    value: &patch_group_binding,
                },
            ]),
            results: Vec::from([
                register_interface::ResultField {
                    name: "baselineId".into(),
                },
                register_interface::ResultField {
                    name: "patchGroup".into(),
                },
            ]),
        };
        let o = register_interface::register(&request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        PatchGroupResult {
            baseline_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("baselineId").unwrap(),
            ),
            patch_group: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("patchGroup").unwrap(),
            ),
        }
    }
}
