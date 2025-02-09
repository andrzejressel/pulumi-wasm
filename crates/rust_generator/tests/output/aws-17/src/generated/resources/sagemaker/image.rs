/// Provides a SageMaker Image resource.
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
///     let example = image::create(
///         "example",
///         ImageArgs::builder().image_name("example").role_arn("${test.arn}").build_struct(),
///     );
/// }
/// ```
///
/// ## Import
///
/// Using `pulumi import`, import SageMaker Code Images using the `name`. For example:
///
/// ```sh
/// $ pulumi import aws:sagemaker/image:Image test_image my-code-repo
/// ```
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod image {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct ImageArgs {
        /// The description of the image.
        #[builder(into, default)]
        pub description: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The display name of the image. When the image is added to a domain (must be unique to the domain).
        #[builder(into, default)]
        pub display_name: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The name of the image. Must be unique to your account.
        #[builder(into)]
        pub image_name: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The Amazon Resource Name (ARN) of an IAM role that enables Amazon SageMaker to perform tasks on your behalf.
        #[builder(into)]
        pub role_arn: pulumi_gestalt_rust::InputOrOutput<String>,
        /// A map of tags to assign to the resource. .If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
        #[builder(into, default)]
        pub tags: pulumi_gestalt_rust::InputOrOutput<
            Option<std::collections::HashMap<String, String>>,
        >,
    }
    #[allow(dead_code)]
    pub struct ImageResult {
        /// The Amazon Resource Name (ARN) assigned by AWS to this Image.
        pub arn: pulumi_gestalt_rust::Output<String>,
        /// The description of the image.
        pub description: pulumi_gestalt_rust::Output<Option<String>>,
        /// The display name of the image. When the image is added to a domain (must be unique to the domain).
        pub display_name: pulumi_gestalt_rust::Output<Option<String>>,
        /// The name of the image. Must be unique to your account.
        pub image_name: pulumi_gestalt_rust::Output<String>,
        /// The Amazon Resource Name (ARN) of an IAM role that enables Amazon SageMaker to perform tasks on your behalf.
        pub role_arn: pulumi_gestalt_rust::Output<String>,
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
        args: ImageArgs,
    ) -> ImageResult {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::register_interface;
        use std::collections::HashMap;
        let description_binding_1 = args.description.get_output(context);
        let description_binding = description_binding_1.get_inner();
        let display_name_binding_1 = args.display_name.get_output(context);
        let display_name_binding = display_name_binding_1.get_inner();
        let image_name_binding_1 = args.image_name.get_output(context);
        let image_name_binding = image_name_binding_1.get_inner();
        let role_arn_binding_1 = args.role_arn.get_output(context);
        let role_arn_binding = role_arn_binding_1.get_inner();
        let tags_binding_1 = args.tags.get_output(context);
        let tags_binding = tags_binding_1.get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "aws:sagemaker/image:Image".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "description".into(),
                    value: &description_binding,
                },
                register_interface::ObjectField {
                    name: "displayName".into(),
                    value: &display_name_binding,
                },
                register_interface::ObjectField {
                    name: "imageName".into(),
                    value: &image_name_binding,
                },
                register_interface::ObjectField {
                    name: "roleArn".into(),
                    value: &role_arn_binding,
                },
                register_interface::ObjectField {
                    name: "tags".into(),
                    value: &tags_binding,
                },
            ]),
        };
        let o = register_interface::register(context.get_inner(), &request);
        ImageResult {
            arn: pulumi_gestalt_rust::__private::into_domain(o.extract_field("arn")),
            description: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("description"),
            ),
            display_name: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("displayName"),
            ),
            image_name: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("imageName"),
            ),
            role_arn: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("roleArn"),
            ),
            tags: pulumi_gestalt_rust::__private::into_domain(o.extract_field("tags")),
            tags_all: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("tagsAll"),
            ),
        }
    }
}
