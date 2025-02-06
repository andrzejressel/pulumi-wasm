/// Provides an SSM Patch Group resource
///
/// ## Example Usage
///
/// ```ignore
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
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
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct PatchGroupArgs {
        /// The ID of the patch baseline to register the patch group with.
        #[builder(into)]
        pub baseline_id: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The name of the patch group that should be registered with the patch baseline.
        #[builder(into)]
        pub patch_group: pulumi_gestalt_rust::InputOrOutput<String>,
    }
    #[allow(dead_code)]
    pub struct PatchGroupResult {
        /// The ID of the patch baseline to register the patch group with.
        pub baseline_id: pulumi_gestalt_rust::Output<String>,
        /// The name of the patch group that should be registered with the patch baseline.
        pub patch_group: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::PulumiContext,
        name: &str,
        args: PatchGroupArgs,
    ) -> PatchGroupResult {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::register_interface;
        use std::collections::HashMap;
        let baseline_id_binding = args.baseline_id.get_output(context).get_inner();
        let patch_group_binding = args.patch_group.get_output(context).get_inner();
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
        };
        let o = register_interface::register(context.get_inner(), &request);
        PatchGroupResult {
            baseline_id: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("baselineId"),
            ),
            patch_group: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("patchGroup"),
            ),
        }
    }
}
