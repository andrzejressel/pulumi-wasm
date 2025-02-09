/// Resource for managing an [AWS Mainframe Modernization Application](https://docs.aws.amazon.com/m2/latest/userguide/applications-m2.html).
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
///     let example = application::create(
///         "example",
///         ApplicationArgs::builder()
///             .definition(
///                 ApplicationDefinition::builder()
///                     .content(
///                         "{\n  \"definition\": {\n    \"listeners\": [\n      {\n        \"port\": 8196,\n        \"type\": \"http\"\n      }\n    ],\n    \"ba-application\": {\n      \"app-location\": \"${[\"s3-source\"]}/PlanetsDemo-v1.zip\"\n    }\n  },\n  \"source-locations\": [\n    {\n      \"source-id\": \"s3-source\",\n      \"source-type\": \"s3\",\n      \"properties\": {\n        \"s3-bucket\": \"example-bucket\",\n        \"s3-key-prefix\": \"v1\"\n      }\n    }\n  ],\n  \"template-version\": \"2.0\"\n}",
///                     )
///                     .build_struct(),
///             )
///             .engine_type("bluage")
///             .name("Example")
///             .build_struct(),
///     );
/// }
/// ```
///
/// ## Import
///
/// Using `pulumi import`, import Mainframe Modernization Application using the `01234567890abcdef012345678`. For example:
///
/// ```sh
/// $ pulumi import aws:m2/application:Application example 01234567890abcdef012345678
/// ```
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod application {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct ApplicationArgs {
        /// The application definition for this application. You can specify either inline JSON or an S3 bucket location.
        #[builder(into, default)]
        pub definition: pulumi_gestalt_rust::InputOrOutput<
            Option<super::super::types::m2::ApplicationDefinition>,
        >,
        /// Description of the application.
        #[builder(into, default)]
        pub description: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Engine type must be `microfocus | bluage`.
        #[builder(into)]
        pub engine_type: pulumi_gestalt_rust::InputOrOutput<String>,
        /// KMS Key to use for the Application.
        #[builder(into, default)]
        pub kms_key_id: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Unique identifier of the application.
        ///
        /// The following arguments are optional:
        #[builder(into, default)]
        pub name: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// ARN of role for application to use to access AWS resources.
        #[builder(into, default)]
        pub role_arn: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Map of tags assigned to the resource. If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
        #[builder(into, default)]
        pub tags: pulumi_gestalt_rust::InputOrOutput<
            Option<std::collections::HashMap<String, String>>,
        >,
        #[builder(into, default)]
        pub timeouts: pulumi_gestalt_rust::InputOrOutput<
            Option<super::super::types::m2::ApplicationTimeouts>,
        >,
    }
    #[allow(dead_code)]
    pub struct ApplicationResult {
        /// Id of the Application.
        pub application_id: pulumi_gestalt_rust::Output<String>,
        /// ARN of the Application.
        pub arn: pulumi_gestalt_rust::Output<String>,
        /// Current version of the application deployed.
        pub current_version: pulumi_gestalt_rust::Output<i32>,
        /// The application definition for this application. You can specify either inline JSON or an S3 bucket location.
        pub definition: pulumi_gestalt_rust::Output<
            Option<super::super::types::m2::ApplicationDefinition>,
        >,
        /// Description of the application.
        pub description: pulumi_gestalt_rust::Output<Option<String>>,
        /// Engine type must be `microfocus | bluage`.
        pub engine_type: pulumi_gestalt_rust::Output<String>,
        /// KMS Key to use for the Application.
        pub kms_key_id: pulumi_gestalt_rust::Output<Option<String>>,
        /// Unique identifier of the application.
        ///
        /// The following arguments are optional:
        pub name: pulumi_gestalt_rust::Output<String>,
        /// ARN of role for application to use to access AWS resources.
        pub role_arn: pulumi_gestalt_rust::Output<Option<String>>,
        /// Map of tags assigned to the resource. If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
        pub tags: pulumi_gestalt_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// A map of tags assigned to the resource, including those inherited from the provider `default_tags` configuration block.
        pub tags_all: pulumi_gestalt_rust::Output<
            std::collections::HashMap<String, String>,
        >,
        pub timeouts: pulumi_gestalt_rust::Output<
            Option<super::super::types::m2::ApplicationTimeouts>,
        >,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::PulumiContext,
        name: &str,
        args: ApplicationArgs,
    ) -> ApplicationResult {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::register_interface;
        use std::collections::HashMap;
        let definition_binding_1 = args.definition.get_output(context);
        let definition_binding = definition_binding_1.get_inner();
        let description_binding_1 = args.description.get_output(context);
        let description_binding = description_binding_1.get_inner();
        let engine_type_binding_1 = args.engine_type.get_output(context);
        let engine_type_binding = engine_type_binding_1.get_inner();
        let kms_key_id_binding_1 = args.kms_key_id.get_output(context);
        let kms_key_id_binding = kms_key_id_binding_1.get_inner();
        let name_binding_1 = args.name.get_output(context);
        let name_binding = name_binding_1.get_inner();
        let role_arn_binding_1 = args.role_arn.get_output(context);
        let role_arn_binding = role_arn_binding_1.get_inner();
        let tags_binding_1 = args.tags.get_output(context);
        let tags_binding = tags_binding_1.get_inner();
        let timeouts_binding_1 = args.timeouts.get_output(context);
        let timeouts_binding = timeouts_binding_1.get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "aws:m2/application:Application".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "definition".into(),
                    value: &definition_binding,
                },
                register_interface::ObjectField {
                    name: "description".into(),
                    value: &description_binding,
                },
                register_interface::ObjectField {
                    name: "engineType".into(),
                    value: &engine_type_binding,
                },
                register_interface::ObjectField {
                    name: "kmsKeyId".into(),
                    value: &kms_key_id_binding,
                },
                register_interface::ObjectField {
                    name: "name".into(),
                    value: &name_binding,
                },
                register_interface::ObjectField {
                    name: "roleArn".into(),
                    value: &role_arn_binding,
                },
                register_interface::ObjectField {
                    name: "tags".into(),
                    value: &tags_binding,
                },
                register_interface::ObjectField {
                    name: "timeouts".into(),
                    value: &timeouts_binding,
                },
            ]),
        };
        let o = register_interface::register(context.get_inner(), &request);
        ApplicationResult {
            application_id: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("applicationId"),
            ),
            arn: pulumi_gestalt_rust::__private::into_domain(o.extract_field("arn")),
            current_version: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("currentVersion"),
            ),
            definition: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("definition"),
            ),
            description: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("description"),
            ),
            engine_type: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("engineType"),
            ),
            kms_key_id: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("kmsKeyId"),
            ),
            name: pulumi_gestalt_rust::__private::into_domain(o.extract_field("name")),
            role_arn: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("roleArn"),
            ),
            tags: pulumi_gestalt_rust::__private::into_domain(o.extract_field("tags")),
            tags_all: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("tagsAll"),
            ),
            timeouts: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("timeouts"),
            ),
        }
    }
}
