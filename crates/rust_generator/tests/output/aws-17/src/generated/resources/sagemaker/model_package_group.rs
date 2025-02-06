/// Provides a SageMaker Model Package Group resource.
///
/// ## Example Usage
///
/// ### Basic usage
///
/// ```ignore
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let example = model_package_group::create(
///         "example",
///         ModelPackageGroupArgs::builder()
///             .model_package_group_name("example")
///             .build_struct(),
///     );
/// }
/// ```
///
/// ## Import
///
/// Using `pulumi import`, import SageMaker Model Package Groups using the `name`. For example:
///
/// ```sh
/// $ pulumi import aws:sagemaker/modelPackageGroup:ModelPackageGroup test_model_package_group my-code-repo
/// ```
pub mod model_package_group {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct ModelPackageGroupArgs {
        /// A description for the model group.
        #[builder(into, default)]
        pub model_package_group_description: pulumi_gestalt_rust::InputOrOutput<
            Option<String>,
        >,
        /// The name of the model group.
        #[builder(into)]
        pub model_package_group_name: pulumi_gestalt_rust::InputOrOutput<String>,
        /// A map of tags to assign to the resource. .If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
        #[builder(into, default)]
        pub tags: pulumi_gestalt_rust::InputOrOutput<
            Option<std::collections::HashMap<String, String>>,
        >,
    }
    #[allow(dead_code)]
    pub struct ModelPackageGroupResult {
        /// The Amazon Resource Name (ARN) assigned by AWS to this Model Package Group.
        pub arn: pulumi_gestalt_rust::Output<String>,
        /// A description for the model group.
        pub model_package_group_description: pulumi_gestalt_rust::Output<Option<String>>,
        /// The name of the model group.
        pub model_package_group_name: pulumi_gestalt_rust::Output<String>,
        /// A map of tags to assign to the resource. .If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
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
        context: &pulumi_gestalt_rust::PulumiContext,
        name: &str,
        args: ModelPackageGroupArgs,
    ) -> ModelPackageGroupResult {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::register_interface;
        use std::collections::HashMap;
        let model_package_group_description_binding = args
            .model_package_group_description
            .get_output(context)
            .get_inner();
        let model_package_group_name_binding = args
            .model_package_group_name
            .get_output(context)
            .get_inner();
        let tags_binding = args.tags.get_output(context).get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "aws:sagemaker/modelPackageGroup:ModelPackageGroup".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "modelPackageGroupDescription".into(),
                    value: &model_package_group_description_binding,
                },
                register_interface::ObjectField {
                    name: "modelPackageGroupName".into(),
                    value: &model_package_group_name_binding,
                },
                register_interface::ObjectField {
                    name: "tags".into(),
                    value: &tags_binding,
                },
            ]),
        };
        let o = register_interface::register(context.get_inner(), &request);
        ModelPackageGroupResult {
            arn: pulumi_gestalt_rust::__private::into_domain(o.extract_field("arn")),
            model_package_group_description: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("modelPackageGroupDescription"),
            ),
            model_package_group_name: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("modelPackageGroupName"),
            ),
            tags: pulumi_gestalt_rust::__private::into_domain(o.extract_field("tags")),
            tags_all: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("tagsAll"),
            ),
        }
    }
}
