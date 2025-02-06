/// Provides an Elastic Beanstalk Application Resource. Elastic Beanstalk allows
/// you to deploy and manage applications in the AWS cloud without worrying about
/// the infrastructure that runs those applications.
///
/// This resource creates an application that has one configuration template named
/// `default`, and no application versions
///
/// ## Example Usage
///
/// ```ignore
/// use pulumi_wasm_rust::Output;
/// use pulumi_wasm_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let tftest = application::create(
///         "tftest",
///         ApplicationArgs::builder()
///             .appversion_lifecycle(
///                 ApplicationAppversionLifecycle::builder()
///                     .deleteSourceFromS3(true)
///                     .maxCount(128)
///                     .serviceRole("${beanstalkService.arn}")
///                     .build_struct(),
///             )
///             .description("tf-test-desc")
///             .name("tf-test-name")
///             .build_struct(),
///     );
/// }
/// ```
///
/// ## Import
///
/// Using `pulumi import`, import Elastic Beanstalk Applications using the `name`. For example:
///
/// ```sh
/// $ pulumi import aws:elasticbeanstalk/application:Application tf_test tf-test-name
/// ```
pub mod application {
    #[derive(pulumi_wasm_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct ApplicationArgs {
        #[builder(into, default)]
        pub appversion_lifecycle: pulumi_wasm_rust::InputOrOutput<
            Option<super::super::types::elasticbeanstalk::ApplicationAppversionLifecycle>,
        >,
        /// Short description of the application
        #[builder(into, default)]
        pub description: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        /// The name of the application, must be unique within your account
        #[builder(into, default)]
        pub name: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        /// Key-value map of tags for the Elastic Beanstalk Application. If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
        #[builder(into, default)]
        pub tags: pulumi_wasm_rust::InputOrOutput<
            Option<std::collections::HashMap<String, String>>,
        >,
    }
    #[allow(dead_code)]
    pub struct ApplicationResult {
        pub appversion_lifecycle: pulumi_wasm_rust::Output<
            Option<super::super::types::elasticbeanstalk::ApplicationAppversionLifecycle>,
        >,
        /// The ARN assigned by AWS for this Elastic Beanstalk Application.
        pub arn: pulumi_wasm_rust::Output<String>,
        /// Short description of the application
        pub description: pulumi_wasm_rust::Output<Option<String>>,
        /// The name of the application, must be unique within your account
        pub name: pulumi_wasm_rust::Output<String>,
        /// Key-value map of tags for the Elastic Beanstalk Application. If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
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
        args: ApplicationArgs,
    ) -> ApplicationResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let appversion_lifecycle_binding = args
            .appversion_lifecycle
            .get_output(context)
            .get_inner();
        let description_binding = args.description.get_output(context).get_inner();
        let name_binding = args.name.get_output(context).get_inner();
        let tags_binding = args.tags.get_output(context).get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "aws:elasticbeanstalk/application:Application".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "appversionLifecycle".into(),
                    value: &appversion_lifecycle_binding,
                },
                register_interface::ObjectField {
                    name: "description".into(),
                    value: &description_binding,
                },
                register_interface::ObjectField {
                    name: "name".into(),
                    value: &name_binding,
                },
                register_interface::ObjectField {
                    name: "tags".into(),
                    value: &tags_binding,
                },
            ]),
        };
        let o = register_interface::register(context.get_inner(), &request);
        ApplicationResult {
            appversion_lifecycle: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("appversionLifecycle"),
            ),
            arn: pulumi_wasm_rust::__private::into_domain(o.extract_field("arn")),
            description: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("description"),
            ),
            name: pulumi_wasm_rust::__private::into_domain(o.extract_field("name")),
            tags: pulumi_wasm_rust::__private::into_domain(o.extract_field("tags")),
            tags_all: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("tagsAll"),
            ),
        }
    }
}
