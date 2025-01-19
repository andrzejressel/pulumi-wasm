/// Resource for registering an AWS Systems Manager Default Patch Baseline.
///
/// ## Example Usage
///
/// ### Basic Usage
///
/// ```ignore
/// use pulumi_wasm_rust::Output;
/// use pulumi_wasm_rust::{add_export, pulumi_main};
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
pub mod default_patch_baseline {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct DefaultPatchBaselineArgs {
        /// ID of the patch baseline.
        /// Can be an ID or an ARN.
        /// When specifying an AWS-provided patch baseline, must be the ARN.
        #[builder(into)]
        pub baseline_id: pulumi_wasm_rust::Output<String>,
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
        pub operating_system: pulumi_wasm_rust::Output<String>,
    }
    #[allow(dead_code)]
    pub struct DefaultPatchBaselineResult {
        /// ID of the patch baseline.
        /// Can be an ID or an ARN.
        /// When specifying an AWS-provided patch baseline, must be the ARN.
        pub baseline_id: pulumi_wasm_rust::Output<String>,
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
        pub operating_system: pulumi_wasm_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        name: &str,
        args: DefaultPatchBaselineArgs,
    ) -> DefaultPatchBaselineResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let baseline_id_binding = args.baseline_id.get_inner();
        let operating_system_binding = args.operating_system.get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "aws:ssm/defaultPatchBaseline:DefaultPatchBaseline".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "baselineId".into(),
                    value: &baseline_id_binding,
                },
                register_interface::ObjectField {
                    name: "operatingSystem".into(),
                    value: &operating_system_binding,
                },
            ]),
            results: Vec::from([
                register_interface::ResultField {
                    name: "baselineId".into(),
                },
                register_interface::ResultField {
                    name: "operatingSystem".into(),
                },
            ]),
        };
        let o = register_interface::register(&request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        DefaultPatchBaselineResult {
            baseline_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("baselineId").unwrap(),
            ),
            operating_system: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("operatingSystem").unwrap(),
            ),
        }
    }
}
