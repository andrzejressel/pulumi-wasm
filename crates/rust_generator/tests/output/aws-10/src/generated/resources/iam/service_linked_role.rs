/// Provides an [IAM service-linked role](https://docs.aws.amazon.com/IAM/latest/UserGuide/using-service-linked-roles.html).
///
/// ## Example Usage
///
/// ```ignore
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let elasticbeanstalk = service_linked_role::create(
///         "elasticbeanstalk",
///         ServiceLinkedRoleArgs::builder()
///             .aws_service_name("elasticbeanstalk.amazonaws.com")
///             .build_struct(),
///     );
/// }
/// ```
///
/// ## Import
///
/// Using `pulumi import`, import IAM service-linked roles using role ARN. For example:
///
/// ```sh
/// $ pulumi import aws:iam/serviceLinkedRole:ServiceLinkedRole elasticbeanstalk arn:aws:iam::123456789012:role/aws-service-role/elasticbeanstalk.amazonaws.com/AWSServiceRoleForElasticBeanstalk
/// ```
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod service_linked_role {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct ServiceLinkedRoleArgs {
        /// The AWS service to which this role is attached. You use a string similar to a URL but without the `http://` in front. For example: `elasticbeanstalk.amazonaws.com`. To find the full list of services that support service-linked roles, check [the docs](https://docs.aws.amazon.com/IAM/latest/UserGuide/reference_aws-services-that-work-with-iam.html).
        #[builder(into)]
        pub aws_service_name: pulumi_gestalt_rust::InputOrOutput<String>,
        /// Additional string appended to the role name. Not all AWS services support custom suffixes.
        #[builder(into, default)]
        pub custom_suffix: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The description of the role.
        #[builder(into, default)]
        pub description: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Key-value mapping of tags for the IAM role. If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
        #[builder(into, default)]
        pub tags: pulumi_gestalt_rust::InputOrOutput<
            Option<std::collections::HashMap<String, String>>,
        >,
    }
    #[allow(dead_code)]
    pub struct ServiceLinkedRoleResult {
        /// The Amazon Resource Name (ARN) specifying the role.
        pub arn: pulumi_gestalt_rust::Output<String>,
        /// The AWS service to which this role is attached. You use a string similar to a URL but without the `http://` in front. For example: `elasticbeanstalk.amazonaws.com`. To find the full list of services that support service-linked roles, check [the docs](https://docs.aws.amazon.com/IAM/latest/UserGuide/reference_aws-services-that-work-with-iam.html).
        pub aws_service_name: pulumi_gestalt_rust::Output<String>,
        /// The creation date of the IAM role.
        pub create_date: pulumi_gestalt_rust::Output<String>,
        /// Additional string appended to the role name. Not all AWS services support custom suffixes.
        pub custom_suffix: pulumi_gestalt_rust::Output<Option<String>>,
        /// The description of the role.
        pub description: pulumi_gestalt_rust::Output<Option<String>>,
        /// The name of the role.
        pub name: pulumi_gestalt_rust::Output<String>,
        /// The path of the role.
        pub path: pulumi_gestalt_rust::Output<String>,
        /// Key-value mapping of tags for the IAM role. If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
        pub tags: pulumi_gestalt_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// A map of tags assigned to the resource, including those inherited from the provider `default_tags` configuration block.
        pub tags_all: pulumi_gestalt_rust::Output<
            std::collections::HashMap<String, String>,
        >,
        /// The stable and unique string identifying the role.
        pub unique_id: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: ServiceLinkedRoleArgs,
    ) -> ServiceLinkedRoleResult {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::register_interface;
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let aws_service_name_binding = args.aws_service_name.get_output(context);
        let custom_suffix_binding = args.custom_suffix.get_output(context);
        let description_binding = args.description.get_output(context);
        let tags_binding = args.tags.get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "aws:iam/serviceLinkedRole:ServiceLinkedRole".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "awsServiceName".into(),
                    value: aws_service_name_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "customSuffix".into(),
                    value: custom_suffix_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "description".into(),
                    value: description_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "tags".into(),
                    value: tags_binding.get_id(),
                },
            ],
        };
        let o = context.register_resource(request);
        ServiceLinkedRoleResult {
            arn: o.get_field("arn"),
            aws_service_name: o.get_field("awsServiceName"),
            create_date: o.get_field("createDate"),
            custom_suffix: o.get_field("customSuffix"),
            description: o.get_field("description"),
            name: o.get_field("name"),
            path: o.get_field("path"),
            tags: o.get_field("tags"),
            tags_all: o.get_field("tagsAll"),
            unique_id: o.get_field("uniqueId"),
        }
    }
}
