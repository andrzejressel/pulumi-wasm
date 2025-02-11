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
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: ExperienceArgs,
    ) -> ExperienceResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let configuration_binding = args.configuration.get_output(context);
        let description_binding = args.description.get_output(context);
        let index_id_binding = args.index_id.get_output(context);
        let name_binding = args.name.get_output(context);
        let role_arn_binding = args.role_arn.get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "aws:kendra/experience:Experience".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "configuration".into(),
                    value: &configuration_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "description".into(),
                    value: &description_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "indexId".into(),
                    value: &index_id_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "name".into(),
                    value: &name_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "roleArn".into(),
                    value: &role_arn_binding.drop_type(),
                },
            ],
        };
        let o = context.register_resource(request);
        ExperienceResult {
            arn: o.get_field("arn"),
            configuration: o.get_field("configuration"),
            description: o.get_field("description"),
            endpoints: o.get_field("endpoints"),
            experience_id: o.get_field("experienceId"),
            index_id: o.get_field("indexId"),
            name: o.get_field("name"),
            role_arn: o.get_field("roleArn"),
            status: o.get_field("status"),
        }
    }
}
