/// Resource for registering an AWS Systems Manager Default Patch Baseline.
///
/// ## Example Usage
///
/// ### Basic Usage
///
/// ```ignore
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let example = default_patch_baseline::create(
///         "example",
///         DefaultPatchBaselineArgs::builder()
///             .baseline_id("${examplePatchBaseline.id}")
///             .operating_system("${examplePatchBaseline.operatingSystem}")
///             .build_struct(),
///     );
///     let examplePatchBaseline = patch_baseline::create(
///         "examplePatchBaseline",
///         PatchBaselineArgs::builder()
///             .approved_patches(vec!["KB123456",])
///             .name("example")
///             .build_struct(),
///     );
/// }
/// ```
///
/// ## Import
///
/// Using the patch baseline ARN:
///
/// Using the operating system value:
///
/// __Using `pulumi import` to import__ the Systems Manager Default Patch Baseline using the patch baseline ID, patch baseline ARN, or the operating system value. For example:
///
/// Using the patch baseline ID:
///
/// ```sh
/// $ pulumi import aws:ssm/defaultPatchBaseline:DefaultPatchBaseline example pb-1234567890abcdef1
/// ```
/// Using the patch baseline ARN:
///
/// ```sh
/// $ pulumi import aws:ssm/defaultPatchBaseline:DefaultPatchBaseline example arn:aws:ssm:us-west-2:123456789012:patchbaseline/pb-1234567890abcdef1
/// ```
/// Using the operating system value:
///
/// ```sh
/// $ pulumi import aws:ssm/defaultPatchBaseline:DefaultPatchBaseline example CENTOS
/// ```
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod default_patch_baseline {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct DefaultPatchBaselineArgs {
        /// ID of the patch baseline.
        /// Can be an ID or an ARN.
        /// When specifying an AWS-provided patch baseline, must be the ARN.
        #[builder(into)]
        pub baseline_id: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The operating system the patch baseline applies to.
        /// Valid values are
        /// `AMAZON_LINUX`,
        /// `AMAZON_LINUX_2`,
        /// `AMAZON_LINUX_2022`,
        /// `CENTOS`,
        /// `DEBIAN`,
        /// `MACOS`,
        /// `ORACLE_LINUX`,
        /// `RASPBIAN`,
        /// `REDHAT_ENTERPRISE_LINUX`,
        /// `ROCKY_LINUX`,
        /// `SUSE`,
        /// `UBUNTU`, and
        /// `WINDOWS`.
        #[builder(into)]
        pub operating_system: pulumi_gestalt_rust::InputOrOutput<String>,
    }
    #[allow(dead_code)]
    pub struct DefaultPatchBaselineResult {
        /// ID of the patch baseline.
        /// Can be an ID or an ARN.
        /// When specifying an AWS-provided patch baseline, must be the ARN.
        pub baseline_id: pulumi_gestalt_rust::Output<String>,
        /// The operating system the patch baseline applies to.
        /// Valid values are
        /// `AMAZON_LINUX`,
        /// `AMAZON_LINUX_2`,
        /// `AMAZON_LINUX_2022`,
        /// `CENTOS`,
        /// `DEBIAN`,
        /// `MACOS`,
        /// `ORACLE_LINUX`,
        /// `RASPBIAN`,
        /// `REDHAT_ENTERPRISE_LINUX`,
        /// `ROCKY_LINUX`,
        /// `SUSE`,
        /// `UBUNTU`, and
        /// `WINDOWS`.
        pub operating_system: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: DefaultPatchBaselineArgs,
    ) -> DefaultPatchBaselineResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let baseline_id_binding = args.baseline_id.get_output(context);
        let operating_system_binding = args.operating_system.get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "aws:ssm/defaultPatchBaseline:DefaultPatchBaseline".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "baselineId".into(),
                    value: &baseline_id_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "operatingSystem".into(),
                    value: &operating_system_binding.drop_type(),
                },
            ],
        };
        let o = context.register_resource(request);
        DefaultPatchBaselineResult {
            baseline_id: o.get_field("baselineId"),
            operating_system: o.get_field("operatingSystem"),
        }
    }
}
