/// Resource for managing an AWS FinSpace Kx User.
///
/// ## Example Usage
///
/// ### Basic Usage
///
/// ```yaml
/// resources:
///   example:
///     type: aws:kms:Key
///     properties:
///       description: Example KMS Key
///       deletionWindowInDays: 7
///   exampleKxEnvironment:
///     type: aws:finspace:KxEnvironment
///     name: example
///     properties:
///       name: my-tf-kx-environment
///       kmsKeyId: ${example.arn}
///   exampleRole:
///     type: aws:iam:Role
///     name: example
///     properties:
///       name: example-role
///       assumeRolePolicy:
///         fn::toJSON:
///           Version: 2012-10-17
///           Statement:
///             - Action: sts:AssumeRole
///               Effect: Allow
///               Sid: ""
///               Principal:
///                 Service: ec2.amazonaws.com
///   exampleKxUser:
///     type: aws:finspace:KxUser
///     name: example
///     properties:
///       name: my-tf-kx-user
///       environmentId: ${exampleKxEnvironment.id}
///       iamRole: ${exampleRole.arn}
/// ```
///
/// ## Import
///
/// Using `pulumi import`, import an AWS FinSpace Kx User using the `id` (environment ID and user name, comma-delimited). For example:
///
/// ```sh
/// $ pulumi import aws:finspace/kxUser:KxUser example n3ceo7wqxoxcti5tujqwzs,my-tf-kx-user
/// ```
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod kx_user {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct KxUserArgs {
        /// Unique identifier for the KX environment.
        #[builder(into)]
        pub environment_id: pulumi_gestalt_rust::InputOrOutput<String>,
        /// IAM role ARN to be associated with the user.
        ///
        /// The following arguments are optional:
        #[builder(into)]
        pub iam_role: pulumi_gestalt_rust::InputOrOutput<String>,
        /// A unique identifier for the user.
        #[builder(into, default)]
        pub name: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Key-value mapping of resource tags. If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
        #[builder(into, default)]
        pub tags: pulumi_gestalt_rust::InputOrOutput<
            Option<std::collections::HashMap<String, String>>,
        >,
    }
    #[allow(dead_code)]
    pub struct KxUserResult {
        /// Amazon Resource Name (ARN) identifier of the KX user.
        pub arn: pulumi_gestalt_rust::Output<String>,
        /// Unique identifier for the KX environment.
        pub environment_id: pulumi_gestalt_rust::Output<String>,
        /// IAM role ARN to be associated with the user.
        ///
        /// The following arguments are optional:
        pub iam_role: pulumi_gestalt_rust::Output<String>,
        /// A unique identifier for the user.
        pub name: pulumi_gestalt_rust::Output<String>,
        /// Key-value mapping of resource tags. If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
        pub tags: pulumi_gestalt_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// Map of tags assigned to the resource, including those inherited from the provider `default_tags` configuration block.
        pub tags_all: pulumi_gestalt_rust::Output<
            std::collections::HashMap<String, String>,
        >,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: KxUserArgs,
    ) -> KxUserResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let environment_id_binding = args.environment_id.get_output(context);
        let iam_role_binding = args.iam_role.get_output(context);
        let name_binding = args.name.get_output(context);
        let tags_binding = args.tags.get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "aws:finspace/kxUser:KxUser".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "environmentId".into(),
                    value: &environment_id_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "iamRole".into(),
                    value: &iam_role_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "name".into(),
                    value: &name_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "tags".into(),
                    value: &tags_binding.drop_type(),
                },
            ],
        };
        let o = context.register_resource(request);
        KxUserResult {
            arn: o.get_field("arn"),
            environment_id: o.get_field("environmentId"),
            iam_role: o.get_field("iamRole"),
            name: o.get_field("name"),
            tags: o.get_field("tags"),
            tags_all: o.get_field("tagsAll"),
        }
    }
}
