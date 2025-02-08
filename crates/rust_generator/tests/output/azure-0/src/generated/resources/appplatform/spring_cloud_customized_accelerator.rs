/// Manages a Spring Cloud Customized Accelerator.
///
/// ## Example Usage
///
/// ```ignore
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let example = resource_group::create(
///         "example",
///         ResourceGroupArgs::builder()
///             .location("west europe")
///             .name("example-resources")
///             .build_struct(),
///     );
///     let exampleSpringCloudAccelerator = spring_cloud_accelerator::create(
///         "exampleSpringCloudAccelerator",
///         SpringCloudAcceleratorArgs::builder()
///             .name("default")
///             .spring_cloud_service_id("${exampleSpringCloudService.id}")
///             .build_struct(),
///     );
///     let exampleSpringCloudCustomizedAccelerator = spring_cloud_customized_accelerator::create(
///         "exampleSpringCloudCustomizedAccelerator",
///         SpringCloudCustomizedAcceleratorArgs::builder()
///             .accelerator_tags(vec!["tag-a", "tag-b",])
///             .description("example description")
///             .display_name("example name")
///             .git_repository(
///                 SpringCloudCustomizedAcceleratorGitRepository::builder()
///                     .gitTag("spring.version.2.0.3")
///                     .intervalInSeconds(100)
///                     .url("https://github.com/Azure-Samples/piggymetrics")
///                     .build_struct(),
///             )
///             .icon_url(
///                 "https://images.freecreatives.com/wp-content/uploads/2015/05/smiley-559124_640.jpg",
///             )
///             .name("example")
///             .spring_cloud_accelerator_id("${exampleSpringCloudAccelerator.id}")
///             .build_struct(),
///     );
///     let exampleSpringCloudService = spring_cloud_service::create(
///         "exampleSpringCloudService",
///         SpringCloudServiceArgs::builder()
///             .location("${example.location}")
///             .name("example")
///             .resource_group_name("${example.name}")
///             .sku_name("E0")
///             .build_struct(),
///     );
/// }
/// ```
///
/// ## Import
///
/// Spring Cloud Customized Accelerators can be imported using the `resource id`, e.g.
///
/// ```sh
/// $ pulumi import azure:appplatform/springCloudCustomizedAccelerator:SpringCloudCustomizedAccelerator example /subscriptions/12345678-1234-9876-4563-123456789012/resourceGroups/resGroup1/providers/Microsoft.AppPlatform/spring/spring1/applicationAccelerators/default/customizedAccelerators/customizedAccelerator1
/// ```
///
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod spring_cloud_customized_accelerator {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct SpringCloudCustomizedAcceleratorArgs {
        /// Specifies a list of accelerator tags.
        #[builder(into, default)]
        pub accelerator_tags: pulumi_gestalt_rust::InputOrOutput<Option<Vec<String>>>,
        /// Specifies the type of the Spring Cloud Customized Accelerator. Possible values are `Accelerator` and `Fragment`. Defaults to `Accelerator`.
        #[builder(into, default)]
        pub accelerator_type: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Specifies the description of the Spring Cloud Customized Accelerator.
        #[builder(into, default)]
        pub description: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Specifies the display name of the Spring Cloud Customized Accelerator..
        #[builder(into, default)]
        pub display_name: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// A `git_repository` block as defined below.
        #[builder(into)]
        pub git_repository: pulumi_gestalt_rust::InputOrOutput<
            super::super::types::appplatform::SpringCloudCustomizedAcceleratorGitRepository,
        >,
        /// Specifies the icon URL of the Spring Cloud Customized Accelerator..
        #[builder(into, default)]
        pub icon_url: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The name which should be used for this Spring Cloud Customized Accelerator. Changing this forces a new Spring Cloud Customized Accelerator to be created.
        #[builder(into, default)]
        pub name: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The ID of the Spring Cloud Accelerator. Changing this forces a new Spring Cloud Customized Accelerator to be created.
        #[builder(into)]
        pub spring_cloud_accelerator_id: pulumi_gestalt_rust::InputOrOutput<String>,
    }
    #[allow(dead_code)]
    pub struct SpringCloudCustomizedAcceleratorResult {
        /// Specifies a list of accelerator tags.
        pub accelerator_tags: pulumi_gestalt_rust::Output<Option<Vec<String>>>,
        /// Specifies the type of the Spring Cloud Customized Accelerator. Possible values are `Accelerator` and `Fragment`. Defaults to `Accelerator`.
        pub accelerator_type: pulumi_gestalt_rust::Output<Option<String>>,
        /// Specifies the description of the Spring Cloud Customized Accelerator.
        pub description: pulumi_gestalt_rust::Output<Option<String>>,
        /// Specifies the display name of the Spring Cloud Customized Accelerator..
        pub display_name: pulumi_gestalt_rust::Output<Option<String>>,
        /// A `git_repository` block as defined below.
        pub git_repository: pulumi_gestalt_rust::Output<
            super::super::types::appplatform::SpringCloudCustomizedAcceleratorGitRepository,
        >,
        /// Specifies the icon URL of the Spring Cloud Customized Accelerator..
        pub icon_url: pulumi_gestalt_rust::Output<Option<String>>,
        /// The name which should be used for this Spring Cloud Customized Accelerator. Changing this forces a new Spring Cloud Customized Accelerator to be created.
        pub name: pulumi_gestalt_rust::Output<String>,
        /// The ID of the Spring Cloud Accelerator. Changing this forces a new Spring Cloud Customized Accelerator to be created.
        pub spring_cloud_accelerator_id: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::PulumiContext,
        name: &str,
        args: SpringCloudCustomizedAcceleratorArgs,
    ) -> SpringCloudCustomizedAcceleratorResult {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::register_interface;
        use std::collections::HashMap;
        let accelerator_tags_binding = args
            .accelerator_tags
            .get_output(context)
            .get_inner();
        let accelerator_type_binding = args
            .accelerator_type
            .get_output(context)
            .get_inner();
        let description_binding = args.description.get_output(context).get_inner();
        let display_name_binding = args.display_name.get_output(context).get_inner();
        let git_repository_binding = args.git_repository.get_output(context).get_inner();
        let icon_url_binding = args.icon_url.get_output(context).get_inner();
        let name_binding = args.name.get_output(context).get_inner();
        let spring_cloud_accelerator_id_binding = args
            .spring_cloud_accelerator_id
            .get_output(context)
            .get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "azure:appplatform/springCloudCustomizedAccelerator:SpringCloudCustomizedAccelerator"
                .into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "acceleratorTags".into(),
                    value: &accelerator_tags_binding,
                },
                register_interface::ObjectField {
                    name: "acceleratorType".into(),
                    value: &accelerator_type_binding,
                },
                register_interface::ObjectField {
                    name: "description".into(),
                    value: &description_binding,
                },
                register_interface::ObjectField {
                    name: "displayName".into(),
                    value: &display_name_binding,
                },
                register_interface::ObjectField {
                    name: "gitRepository".into(),
                    value: &git_repository_binding,
                },
                register_interface::ObjectField {
                    name: "iconUrl".into(),
                    value: &icon_url_binding,
                },
                register_interface::ObjectField {
                    name: "name".into(),
                    value: &name_binding,
                },
                register_interface::ObjectField {
                    name: "springCloudAcceleratorId".into(),
                    value: &spring_cloud_accelerator_id_binding,
                },
            ]),
        };
        let o = register_interface::register(context.get_inner(), &request);
        SpringCloudCustomizedAcceleratorResult {
            accelerator_tags: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("acceleratorTags"),
            ),
            accelerator_type: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("acceleratorType"),
            ),
            description: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("description"),
            ),
            display_name: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("displayName"),
            ),
            git_repository: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("gitRepository"),
            ),
            icon_url: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("iconUrl"),
            ),
            name: pulumi_gestalt_rust::__private::into_domain(o.extract_field("name")),
            spring_cloud_accelerator_id: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("springCloudAcceleratorId"),
            ),
        }
    }
}
