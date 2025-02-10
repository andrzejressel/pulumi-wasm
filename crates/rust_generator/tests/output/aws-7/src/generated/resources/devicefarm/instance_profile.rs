/// Provides a resource to manage AWS Device Farm Instance Profiles.
/// ∂
/// > **NOTE:** AWS currently has limited regional support for Device Farm (e.g., `us-west-2`). See [AWS Device Farm endpoints and quotas](https://docs.aws.amazon.com/general/latest/gr/devicefarm.html) for information on supported regions.
///
/// ## Example Usage
///
/// ```ignore
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
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
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod instance_profile {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct InstanceProfileArgs {
        /// The description of the instance profile.
        #[builder(into, default)]
        pub description: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// An array of strings that specifies the list of app packages that should not be cleaned up from the device after a test run.
        #[builder(into, default)]
        pub exclude_app_packages_from_cleanups: pulumi_gestalt_rust::InputOrOutput<
            Option<Vec<String>>,
        >,
        /// The name for the instance profile.
        #[builder(into, default)]
        pub name: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// When set to `true`, Device Farm removes app packages after a test run. The default value is `false` for private devices.
        #[builder(into, default)]
        pub package_cleanup: pulumi_gestalt_rust::InputOrOutput<Option<bool>>,
        /// When set to `true`, Device Farm reboots the instance after a test run. The default value is `true`.
        #[builder(into, default)]
        pub reboot_after_use: pulumi_gestalt_rust::InputOrOutput<Option<bool>>,
        /// A map of tags to assign to the resource. If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
        #[builder(into, default)]
        pub tags: pulumi_gestalt_rust::InputOrOutput<
            Option<std::collections::HashMap<String, String>>,
        >,
    }
    #[allow(dead_code)]
    pub struct InstanceProfileResult {
        /// The Amazon Resource Name of this instance profile.
        pub arn: pulumi_gestalt_rust::Output<String>,
        /// The description of the instance profile.
        pub description: pulumi_gestalt_rust::Output<Option<String>>,
        /// An array of strings that specifies the list of app packages that should not be cleaned up from the device after a test run.
        pub exclude_app_packages_from_cleanups: pulumi_gestalt_rust::Output<
            Option<Vec<String>>,
        >,
        /// The name for the instance profile.
        pub name: pulumi_gestalt_rust::Output<String>,
        /// When set to `true`, Device Farm removes app packages after a test run. The default value is `false` for private devices.
        pub package_cleanup: pulumi_gestalt_rust::Output<Option<bool>>,
        /// When set to `true`, Device Farm reboots the instance after a test run. The default value is `true`.
        pub reboot_after_use: pulumi_gestalt_rust::Output<Option<bool>>,
        /// A map of tags to assign to the resource. If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
        pub tags: pulumi_gestalt_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// A map of tags assigned to the resource, including those inherited from the provider `default_tags` configuration block.
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
        args: InstanceProfileArgs,
    ) -> InstanceProfileResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let description_binding = args.description.get_output(context);
        let exclude_app_packages_from_cleanups_binding = args
            .exclude_app_packages_from_cleanups
            .get_output(context);
        let name_binding = args.name.get_output(context);
        let package_cleanup_binding = args.package_cleanup.get_output(context);
        let reboot_after_use_binding = args.reboot_after_use.get_output(context);
        let tags_binding = args.tags.get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "aws:devicefarm/instanceProfile:InstanceProfile".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "description".into(),
                    value: description_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "excludeAppPackagesFromCleanups".into(),
                    value: exclude_app_packages_from_cleanups_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "name".into(),
                    value: name_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "packageCleanup".into(),
                    value: package_cleanup_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "rebootAfterUse".into(),
                    value: reboot_after_use_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "tags".into(),
                    value: tags_binding.get_id(),
                },
            ],
        };
        let o = context.register_resource(request);
        InstanceProfileResult {
            arn: o.get_field("arn"),
            description: o.get_field("description"),
            exclude_app_packages_from_cleanups: o
                .get_field("excludeAppPackagesFromCleanups"),
            name: o.get_field("name"),
            package_cleanup: o.get_field("packageCleanup"),
            reboot_after_use: o.get_field("rebootAfterUse"),
            tags: o.get_field("tags"),
            tags_all: o.get_field("tagsAll"),
        }
    }
}
