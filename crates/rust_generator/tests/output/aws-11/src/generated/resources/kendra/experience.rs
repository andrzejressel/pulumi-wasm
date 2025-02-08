/// Resource for managing an AWS Kendra Experience.
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
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod experience {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct ExperienceArgs {
        /// Configuration information for your Amazon Kendra experience. The provider will only perform drift detection of its value when present in a configuration. Detailed below.
        #[builder(into, default)]
        pub configuration: pulumi_gestalt_rust::InputOrOutput<
            Option<super::super::types::kendra::ExperienceConfiguration>,
        >,
        /// A description for your Amazon Kendra experience.
        #[builder(into, default)]
        pub description: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The identifier of the index for your Amazon Kendra experience.
        #[builder(into)]
        pub index_id: pulumi_gestalt_rust::InputOrOutput<String>,
        /// A name for your Amazon Kendra experience.
        #[builder(into, default)]
        pub name: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The Amazon Resource Name (ARN) of a role with permission to access `Query API`, `QuerySuggestions API`, `SubmitFeedback API`, and `AWS SSO` that stores your user and group information. For more information, see [IAM roles for Amazon Kendra](https://docs.aws.amazon.com/kendra/latest/dg/iam-roles.html).
        ///
        /// The following arguments are optional:
        #[builder(into)]
        pub role_arn: pulumi_gestalt_rust::InputOrOutput<String>,
    }
    #[allow(dead_code)]
    pub struct ExperienceResult {
        /// ARN of the Experience.
        pub arn: pulumi_gestalt_rust::Output<String>,
        /// Configuration information for your Amazon Kendra experience. The provider will only perform drift detection of its value when present in a configuration. Detailed below.
        pub configuration: pulumi_gestalt_rust::Output<
            super::super::types::kendra::ExperienceConfiguration,
        >,
        /// A description for your Amazon Kendra experience.
        pub description: pulumi_gestalt_rust::Output<Option<String>>,
        /// Shows the endpoint URLs for your Amazon Kendra experiences. The URLs are unique and fully hosted by AWS.
        pub endpoints: pulumi_gestalt_rust::Output<
            Vec<super::super::types::kendra::ExperienceEndpoint>,
        >,
        /// The unique identifier of the experience.
        pub experience_id: pulumi_gestalt_rust::Output<String>,
        /// The identifier of the index for your Amazon Kendra experience.
        pub index_id: pulumi_gestalt_rust::Output<String>,
        /// A name for your Amazon Kendra experience.
        pub name: pulumi_gestalt_rust::Output<String>,
        /// The Amazon Resource Name (ARN) of a role with permission to access `Query API`, `QuerySuggestions API`, `SubmitFeedback API`, and `AWS SSO` that stores your user and group information. For more information, see [IAM roles for Amazon Kendra](https://docs.aws.amazon.com/kendra/latest/dg/iam-roles.html).
        ///
        /// The following arguments are optional:
        pub role_arn: pulumi_gestalt_rust::Output<String>,
        /// The current processing status of your Amazon Kendra experience.
        pub status: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::PulumiContext,
        name: &str,
        args: ExperienceArgs,
    ) -> ExperienceResult {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::register_interface;
        use std::collections::HashMap;
        let configuration_binding = args.configuration.get_output(context).get_inner();
        let description_binding = args.description.get_output(context).get_inner();
        let index_id_binding = args.index_id.get_output(context).get_inner();
        let name_binding = args.name.get_output(context).get_inner();
        let role_arn_binding = args.role_arn.get_output(context).get_inner();
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
        };
        let o = register_interface::register(context.get_inner(), &request);
        ExperienceResult {
            arn: pulumi_gestalt_rust::__private::into_domain(o.extract_field("arn")),
            configuration: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("configuration"),
            ),
            description: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("description"),
            ),
            endpoints: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("endpoints"),
            ),
            experience_id: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("experienceId"),
            ),
            index_id: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("indexId"),
            ),
            name: pulumi_gestalt_rust::__private::into_domain(o.extract_field("name")),
            role_arn: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("roleArn"),
            ),
            status: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("status"),
            ),
        }
    }
}
