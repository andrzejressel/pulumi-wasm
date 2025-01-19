/// Resource for managing an AWS Kendra Experience.
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
///     let example = experience::create(
///         "example",
///         ExperienceArgs::builder()
///             .configuration(
///                 ExperienceConfiguration::builder()
///                     .contentSourceConfiguration(
///                         ExperienceConfigurationContentSourceConfiguration::builder()
///                             .directPutContent(true)
///                             .faqIds(vec!["${exampleAwsKendraFaq.faqId}",])
///                             .build_struct(),
///                     )
///                     .userIdentityConfiguration(
///                         ExperienceConfigurationUserIdentityConfiguration::builder()
///                             .identityAttributeName(
///                                 "12345ec453-1546651e-79c4-4554-91fa-00b43ccfa245",
///                             )
///                             .build_struct(),
///                     )
///                     .build_struct(),
///             )
///             .description("My Kendra Experience")
///             .index_id("${exampleAwsKendraIndex.id}")
///             .name("example")
///             .role_arn("${exampleAwsIamRole.arn}")
///             .build_struct(),
///     );
/// }
/// ```
///
/// ## Import
///
/// Using `pulumi import`, import Kendra Experience using the unique identifiers of the experience and index separated by a slash (`/`). For example:
///
/// ```sh
/// $ pulumi import aws:kendra/experience:Experience example 1045d08d-66ef-4882-b3ed-dfb7df183e90/b34dfdf7-1f2b-4704-9581-79e00296845f
/// ```
pub mod experience {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct ExperienceArgs {
        /// Configuration information for your Amazon Kendra experience. The provider will only perform drift detection of its value when present in a configuration. Detailed below.
        #[builder(into, default)]
        pub configuration: pulumi_wasm_rust::Output<
            Option<super::super::types::kendra::ExperienceConfiguration>,
        >,
        /// A description for your Amazon Kendra experience.
        #[builder(into, default)]
        pub description: pulumi_wasm_rust::Output<Option<String>>,
        /// The identifier of the index for your Amazon Kendra experience.
        #[builder(into)]
        pub index_id: pulumi_wasm_rust::Output<String>,
        /// A name for your Amazon Kendra experience.
        #[builder(into, default)]
        pub name: pulumi_wasm_rust::Output<Option<String>>,
        /// The Amazon Resource Name (ARN) of a role with permission to access `Query API`, `QuerySuggestions API`, `SubmitFeedback API`, and `AWS SSO` that stores your user and group information. For more information, see [IAM roles for Amazon Kendra](https://docs.aws.amazon.com/kendra/latest/dg/iam-roles.html).
        ///
        /// The following arguments are optional:
        #[builder(into)]
        pub role_arn: pulumi_wasm_rust::Output<String>,
    }
    #[allow(dead_code)]
    pub struct ExperienceResult {
        /// ARN of the Experience.
        pub arn: pulumi_wasm_rust::Output<String>,
        /// Configuration information for your Amazon Kendra experience. The provider will only perform drift detection of its value when present in a configuration. Detailed below.
        pub configuration: pulumi_wasm_rust::Output<
            super::super::types::kendra::ExperienceConfiguration,
        >,
        /// A description for your Amazon Kendra experience.
        pub description: pulumi_wasm_rust::Output<Option<String>>,
        /// Shows the endpoint URLs for your Amazon Kendra experiences. The URLs are unique and fully hosted by AWS.
        pub endpoints: pulumi_wasm_rust::Output<
            Vec<super::super::types::kendra::ExperienceEndpoint>,
        >,
        /// The unique identifier of the experience.
        pub experience_id: pulumi_wasm_rust::Output<String>,
        /// The identifier of the index for your Amazon Kendra experience.
        pub index_id: pulumi_wasm_rust::Output<String>,
        /// A name for your Amazon Kendra experience.
        pub name: pulumi_wasm_rust::Output<String>,
        /// The Amazon Resource Name (ARN) of a role with permission to access `Query API`, `QuerySuggestions API`, `SubmitFeedback API`, and `AWS SSO` that stores your user and group information. For more information, see [IAM roles for Amazon Kendra](https://docs.aws.amazon.com/kendra/latest/dg/iam-roles.html).
        ///
        /// The following arguments are optional:
        pub role_arn: pulumi_wasm_rust::Output<String>,
        /// The current processing status of your Amazon Kendra experience.
        pub status: pulumi_wasm_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(name: &str, args: ExperienceArgs) -> ExperienceResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let configuration_binding = args.configuration.get_inner();
        let description_binding = args.description.get_inner();
        let index_id_binding = args.index_id.get_inner();
        let name_binding = args.name.get_inner();
        let role_arn_binding = args.role_arn.get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "aws:kendra/experience:Experience".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "configuration".into(),
                    value: &configuration_binding,
                },
                register_interface::ObjectField {
                    name: "description".into(),
                    value: &description_binding,
                },
                register_interface::ObjectField {
                    name: "indexId".into(),
                    value: &index_id_binding,
                },
                register_interface::ObjectField {
                    name: "name".into(),
                    value: &name_binding,
                },
                register_interface::ObjectField {
                    name: "roleArn".into(),
                    value: &role_arn_binding,
                },
            ]),
            results: Vec::from([
                register_interface::ResultField {
                    name: "arn".into(),
                },
                register_interface::ResultField {
                    name: "configuration".into(),
                },
                register_interface::ResultField {
                    name: "description".into(),
                },
                register_interface::ResultField {
                    name: "endpoints".into(),
                },
                register_interface::ResultField {
                    name: "experienceId".into(),
                },
                register_interface::ResultField {
                    name: "indexId".into(),
                },
                register_interface::ResultField {
                    name: "name".into(),
                },
                register_interface::ResultField {
                    name: "roleArn".into(),
                },
                register_interface::ResultField {
                    name: "status".into(),
                },
            ]),
        };
        let o = register_interface::register(&request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        ExperienceResult {
            arn: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("arn").unwrap(),
            ),
            configuration: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("configuration").unwrap(),
            ),
            description: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("description").unwrap(),
            ),
            endpoints: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("endpoints").unwrap(),
            ),
            experience_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("experienceId").unwrap(),
            ),
            index_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("indexId").unwrap(),
            ),
            name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("name").unwrap(),
            ),
            role_arn: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("roleArn").unwrap(),
            ),
            status: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("status").unwrap(),
            ),
        }
    }
}
