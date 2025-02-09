/// Provides a resource to manage AWS Device Farm Device Pools.
///
/// ## Example Usage
///
/// ```ignore
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let example = device_pool::create(
///         "example",
///         DevicePoolArgs::builder()
///             .name("example")
///             .project_arn("${exampleAwsDevicefarmProject.arn}")
///             .rules(
///                 vec![
///                     DevicePoolRule::builder().attribute("OS_VERSION").operator("EQUALS")
///                     .value("\"AVAILABLE\"").build_struct(),
///                 ],
///             )
///             .build_struct(),
///     );
/// }
/// ```
///
/// ## Import
///
/// Using `pulumi import`, import DeviceFarm Device Pools using their ARN. For example:
///
/// ```sh
/// $ pulumi import aws:devicefarm/devicePool:DevicePool example arn:aws:devicefarm:us-west-2:123456789012:devicepool:4fa784c7-ccb4-4dbf-ba4f-02198320daa1/4fa784c7-ccb4-4dbf-ba4f-02198320daa1
/// ```
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod device_pool {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct DevicePoolArgs {
        /// The device pool's description.
        #[builder(into, default)]
        pub description: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The number of devices that Device Farm can add to your device pool.
        #[builder(into, default)]
        pub max_devices: pulumi_gestalt_rust::InputOrOutput<Option<i32>>,
        /// The name of the Device Pool
        #[builder(into, default)]
        pub name: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The ARN of the project for the device pool.
        #[builder(into)]
        pub project_arn: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The device pool's rules. See Rule.
        #[builder(into)]
        pub rules: pulumi_gestalt_rust::InputOrOutput<
            Vec<super::super::types::devicefarm::DevicePoolRule>,
        >,
        /// A map of tags to assign to the resource. If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
        #[builder(into, default)]
        pub tags: pulumi_gestalt_rust::InputOrOutput<
            Option<std::collections::HashMap<String, String>>,
        >,
    }
    #[allow(dead_code)]
    pub struct DevicePoolResult {
        /// The Amazon Resource Name of this Device Pool
        pub arn: pulumi_gestalt_rust::Output<String>,
        /// The device pool's description.
        pub description: pulumi_gestalt_rust::Output<Option<String>>,
        /// The number of devices that Device Farm can add to your device pool.
        pub max_devices: pulumi_gestalt_rust::Output<Option<i32>>,
        /// The name of the Device Pool
        pub name: pulumi_gestalt_rust::Output<String>,
        /// The ARN of the project for the device pool.
        pub project_arn: pulumi_gestalt_rust::Output<String>,
        /// The device pool's rules. See Rule.
        pub rules: pulumi_gestalt_rust::Output<
            Vec<super::super::types::devicefarm::DevicePoolRule>,
        >,
        /// A map of tags to assign to the resource. If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
        pub tags: pulumi_gestalt_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// A map of tags assigned to the resource, including those inherited from the provider `default_tags` configuration block.
        pub tags_all: pulumi_gestalt_rust::Output<
            std::collections::HashMap<String, String>,
        >,
        pub type_: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::PulumiContext,
        name: &str,
        args: DevicePoolArgs,
    ) -> DevicePoolResult {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::register_interface;
        use std::collections::HashMap;
        let description_binding_1 = args.description.get_output(context);
        let description_binding = description_binding_1.get_inner();
        let max_devices_binding_1 = args.max_devices.get_output(context);
        let max_devices_binding = max_devices_binding_1.get_inner();
        let name_binding_1 = args.name.get_output(context);
        let name_binding = name_binding_1.get_inner();
        let project_arn_binding_1 = args.project_arn.get_output(context);
        let project_arn_binding = project_arn_binding_1.get_inner();
        let rules_binding_1 = args.rules.get_output(context);
        let rules_binding = rules_binding_1.get_inner();
        let tags_binding_1 = args.tags.get_output(context);
        let tags_binding = tags_binding_1.get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "aws:devicefarm/devicePool:DevicePool".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "description".into(),
                    value: &description_binding,
                },
                register_interface::ObjectField {
                    name: "maxDevices".into(),
                    value: &max_devices_binding,
                },
                register_interface::ObjectField {
                    name: "name".into(),
                    value: &name_binding,
                },
                register_interface::ObjectField {
                    name: "projectArn".into(),
                    value: &project_arn_binding,
                },
                register_interface::ObjectField {
                    name: "rules".into(),
                    value: &rules_binding,
                },
                register_interface::ObjectField {
                    name: "tags".into(),
                    value: &tags_binding,
                },
            ]),
        };
        let o = register_interface::register(context.get_inner(), &request);
        DevicePoolResult {
            arn: pulumi_gestalt_rust::__private::into_domain(o.extract_field("arn")),
            description: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("description"),
            ),
            max_devices: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("maxDevices"),
            ),
            name: pulumi_gestalt_rust::__private::into_domain(o.extract_field("name")),
            project_arn: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("projectArn"),
            ),
            rules: pulumi_gestalt_rust::__private::into_domain(o.extract_field("rules")),
            tags: pulumi_gestalt_rust::__private::into_domain(o.extract_field("tags")),
            tags_all: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("tagsAll"),
            ),
            type_: pulumi_gestalt_rust::__private::into_domain(o.extract_field("type")),
        }
    }
}
