/// Provides a resource to manage AWS Device Farm Instance Profiles.
/// ∂
/// > **NOTE:** AWS currently has limited regional support for Device Farm (e.g., `us-west-2`). See [AWS Device Farm endpoints and quotas](https://docs.aws.amazon.com/general/latest/gr/devicefarm.html) for information on supported regions.
///
/// ## Example Usage
///
/// ```ignore
/// use pulumi_wasm_rust::Output;
/// use pulumi_wasm_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let example = instance_profile::create(
///         "example",
///         InstanceProfileArgs::builder().name("example").build_struct(),
///     );
/// }
/// ```
///
/// ## Import
///
/// Using `pulumi import`, import DeviceFarm Instance Profiles using their ARN. For example:
///
/// ```sh
/// $ pulumi import aws:devicefarm/instanceProfile:InstanceProfile example arn:aws:devicefarm:us-west-2:123456789012:instanceprofile:4fa784c7-ccb4-4dbf-ba4f-02198320daa1
/// ```
pub mod instance_profile {
    #[derive(pulumi_wasm_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct InstanceProfileArgs {
        /// The description of the instance profile.
        #[builder(into, default)]
        pub description: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        /// An array of strings that specifies the list of app packages that should not be cleaned up from the device after a test run.
        #[builder(into, default)]
        pub exclude_app_packages_from_cleanups: pulumi_wasm_rust::InputOrOutput<
            Option<Vec<String>>,
        >,
        /// The name for the instance profile.
        #[builder(into, default)]
        pub name: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        /// When set to `true`, Device Farm removes app packages after a test run. The default value is `false` for private devices.
        #[builder(into, default)]
        pub package_cleanup: pulumi_wasm_rust::InputOrOutput<Option<bool>>,
        /// When set to `true`, Device Farm reboots the instance after a test run. The default value is `true`.
        #[builder(into, default)]
        pub reboot_after_use: pulumi_wasm_rust::InputOrOutput<Option<bool>>,
        /// A map of tags to assign to the resource. If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
        #[builder(into, default)]
        pub tags: pulumi_wasm_rust::InputOrOutput<
            Option<std::collections::HashMap<String, String>>,
        >,
    }
    #[allow(dead_code)]
    pub struct InstanceProfileResult {
        /// The Amazon Resource Name of this instance profile.
        pub arn: pulumi_wasm_rust::Output<String>,
        /// The description of the instance profile.
        pub description: pulumi_wasm_rust::Output<Option<String>>,
        /// An array of strings that specifies the list of app packages that should not be cleaned up from the device after a test run.
        pub exclude_app_packages_from_cleanups: pulumi_wasm_rust::Output<
            Option<Vec<String>>,
        >,
        /// The name for the instance profile.
        pub name: pulumi_wasm_rust::Output<String>,
        /// When set to `true`, Device Farm removes app packages after a test run. The default value is `false` for private devices.
        pub package_cleanup: pulumi_wasm_rust::Output<Option<bool>>,
        /// When set to `true`, Device Farm reboots the instance after a test run. The default value is `true`.
        pub reboot_after_use: pulumi_wasm_rust::Output<Option<bool>>,
        /// A map of tags to assign to the resource. If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
        pub tags: pulumi_wasm_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// A map of tags assigned to the resource, including those inherited from the provider `default_tags` configuration block.
        pub tags_all: pulumi_wasm_rust::Output<
            std::collections::HashMap<String, String>,
        >,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_wasm_rust::PulumiContext,
        name: &str,
        args: InstanceProfileArgs,
    ) -> InstanceProfileResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let description_binding = args.description.get_output(context).get_inner();
        let exclude_app_packages_from_cleanups_binding = args
            .exclude_app_packages_from_cleanups
            .get_output(context)
            .get_inner();
        let name_binding = args.name.get_output(context).get_inner();
        let package_cleanup_binding = args
            .package_cleanup
            .get_output(context)
            .get_inner();
        let reboot_after_use_binding = args
            .reboot_after_use
            .get_output(context)
            .get_inner();
        let tags_binding = args.tags.get_output(context).get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "aws:devicefarm/instanceProfile:InstanceProfile".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "description".into(),
                    value: &description_binding,
                },
                register_interface::ObjectField {
                    name: "excludeAppPackagesFromCleanups".into(),
                    value: &exclude_app_packages_from_cleanups_binding,
                },
                register_interface::ObjectField {
                    name: "name".into(),
                    value: &name_binding,
                },
                register_interface::ObjectField {
                    name: "packageCleanup".into(),
                    value: &package_cleanup_binding,
                },
                register_interface::ObjectField {
                    name: "rebootAfterUse".into(),
                    value: &reboot_after_use_binding,
                },
                register_interface::ObjectField {
                    name: "tags".into(),
                    value: &tags_binding,
                },
            ]),
        };
        let o = register_interface::register(context.get_inner(), &request);
        InstanceProfileResult {
            arn: pulumi_wasm_rust::__private::into_domain(o.extract_field("arn")),
            description: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("description"),
            ),
            exclude_app_packages_from_cleanups: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("excludeAppPackagesFromCleanups"),
            ),
            name: pulumi_wasm_rust::__private::into_domain(o.extract_field("name")),
            package_cleanup: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("packageCleanup"),
            ),
            reboot_after_use: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("rebootAfterUse"),
            ),
            tags: pulumi_wasm_rust::__private::into_domain(o.extract_field("tags")),
            tags_all: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("tagsAll"),
            ),
        }
    }
}
