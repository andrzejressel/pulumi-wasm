/// Resource for managing an [AWS Mainframe Modernization Application](https://docs.aws.amazon.com/m2/latest/userguide/applications-m2.html).
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
pub mod application {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct ApplicationArgs {
        /// The application definition for this application. You can specify either inline JSON or an S3 bucket location.
        #[builder(into, default)]
        pub definition: pulumi_wasm_rust::Output<
            Option<super::super::types::m2::ApplicationDefinition>,
        >,
        /// Description of the application.
        #[builder(into, default)]
        pub description: pulumi_wasm_rust::Output<Option<String>>,
        /// Engine type must be `microfocus | bluage`.
        #[builder(into)]
        pub engine_type: pulumi_wasm_rust::Output<String>,
        /// KMS Key to use for the Application.
        #[builder(into, default)]
        pub kms_key_id: pulumi_wasm_rust::Output<Option<String>>,
        /// Unique identifier of the application.
        ///
        /// The following arguments are optional:
        #[builder(into, default)]
        pub name: pulumi_wasm_rust::Output<Option<String>>,
        /// ARN of role for application to use to access AWS resources.
        #[builder(into, default)]
        pub role_arn: pulumi_wasm_rust::Output<Option<String>>,
        /// Map of tags assigned to the resource. If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
        #[builder(into, default)]
        pub tags: pulumi_wasm_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        #[builder(into, default)]
        pub timeouts: pulumi_wasm_rust::Output<
            Option<super::super::types::m2::ApplicationTimeouts>,
        >,
    }
    #[allow(dead_code)]
    pub struct ApplicationResult {
        /// Id of the Application.
        pub application_id: pulumi_wasm_rust::Output<String>,
        /// ARN of the Application.
        pub arn: pulumi_wasm_rust::Output<String>,
        /// Current version of the application deployed.
        pub current_version: pulumi_wasm_rust::Output<i32>,
        /// The application definition for this application. You can specify either inline JSON or an S3 bucket location.
        pub definition: pulumi_wasm_rust::Output<
            Option<super::super::types::m2::ApplicationDefinition>,
        >,
        /// Description of the application.
        pub description: pulumi_wasm_rust::Output<Option<String>>,
        /// Engine type must be `microfocus | bluage`.
        pub engine_type: pulumi_wasm_rust::Output<String>,
        /// KMS Key to use for the Application.
        pub kms_key_id: pulumi_wasm_rust::Output<Option<String>>,
        /// Unique identifier of the application.
        ///
        /// The following arguments are optional:
        pub name: pulumi_wasm_rust::Output<String>,
        /// ARN of role for application to use to access AWS resources.
        pub role_arn: pulumi_wasm_rust::Output<Option<String>>,
        /// Map of tags assigned to the resource. If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
        pub tags: pulumi_wasm_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// A map of tags assigned to the resource, including those inherited from the provider `default_tags` configuration block.
        pub tags_all: pulumi_wasm_rust::Output<
            std::collections::HashMap<String, String>,
        >,
        pub timeouts: pulumi_wasm_rust::Output<
            Option<super::super::types::m2::ApplicationTimeouts>,
        >,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(name: &str, args: ApplicationArgs) -> ApplicationResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let definition_binding = args.definition.get_inner();
        let description_binding = args.description.get_inner();
        let engine_type_binding = args.engine_type.get_inner();
        let kms_key_id_binding = args.kms_key_id.get_inner();
        let name_binding = args.name.get_inner();
        let role_arn_binding = args.role_arn.get_inner();
        let tags_binding = args.tags.get_inner();
        let timeouts_binding = args.timeouts.get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "aws:m2/application:Application".into(),
            name: name.to_string(),
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
            results: Vec::from([
                register_interface::ResultField {
                    name: "applicationId".into(),
                },
                register_interface::ResultField {
                    name: "arn".into(),
                },
                register_interface::ResultField {
                    name: "currentVersion".into(),
                },
                register_interface::ResultField {
                    name: "definition".into(),
                },
                register_interface::ResultField {
                    name: "description".into(),
                },
                register_interface::ResultField {
                    name: "engineType".into(),
                },
                register_interface::ResultField {
                    name: "kmsKeyId".into(),
                },
                register_interface::ResultField {
                    name: "name".into(),
                },
                register_interface::ResultField {
                    name: "roleArn".into(),
                },
                register_interface::ResultField {
                    name: "tags".into(),
                },
                register_interface::ResultField {
                    name: "tagsAll".into(),
                },
                register_interface::ResultField {
                    name: "timeouts".into(),
                },
            ]),
        };
        let o = register_interface::register(&request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        ApplicationResult {
            application_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("applicationId").unwrap(),
            ),
            arn: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("arn").unwrap(),
            ),
            current_version: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("currentVersion").unwrap(),
            ),
            definition: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("definition").unwrap(),
            ),
            description: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("description").unwrap(),
            ),
            engine_type: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("engineType").unwrap(),
            ),
            kms_key_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("kmsKeyId").unwrap(),
            ),
            name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("name").unwrap(),
            ),
            role_arn: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("roleArn").unwrap(),
            ),
            tags: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("tags").unwrap(),
            ),
            tags_all: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("tagsAll").unwrap(),
            ),
            timeouts: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("timeouts").unwrap(),
            ),
        }
    }
}
