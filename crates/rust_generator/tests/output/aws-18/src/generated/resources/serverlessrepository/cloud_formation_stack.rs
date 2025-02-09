/// Deploys an Application CloudFormation Stack from the Serverless Application Repository.
///
/// ## Example Usage
///
/// ```yaml
/// resources:
///   postgres-rotator:
///     type: aws:serverlessrepository:CloudFormationStack
///     properties:
///       name: postgres-rotator
///       applicationId: arn:aws:serverlessrepo:us-east-1:297356227824:applications/SecretsManagerRDSPostgreSQLRotationSingleUser
///       capabilities:
///         - CAPABILITY_IAM
///         - CAPABILITY_RESOURCE_POLICY
///       parameters:
///         functionName: func-postgres-rotator
///         endpoint: secretsmanager.${currentGetRegion.name}.${current.dnsSuffix}
/// variables:
///   current:
///     fn::invoke:
///       function: aws:getPartition
///       arguments: {}
///   currentGetRegion:
///     fn::invoke:
///       function: aws:getRegion
///       arguments: {}
/// ```
///
/// ## Import
///
/// Using `pulumi import`, import Serverless Application Repository Stack using the CloudFormation Stack name (with or without the `serverlessrepo-` prefix) or the CloudFormation Stack ID. For example:
///
/// ```sh
/// $ pulumi import aws:serverlessrepository/cloudFormationStack:CloudFormationStack example serverlessrepo-postgres-rotator
/// ```
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod cloud_formation_stack {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct CloudFormationStackArgs {
        /// The ARN of the application from the Serverless Application Repository.
        #[builder(into)]
        pub application_id: pulumi_gestalt_rust::InputOrOutput<String>,
        /// A list of capabilities. Valid values are `CAPABILITY_IAM`, `CAPABILITY_NAMED_IAM`, `CAPABILITY_RESOURCE_POLICY`, or `CAPABILITY_AUTO_EXPAND`
        #[builder(into)]
        pub capabilities: pulumi_gestalt_rust::InputOrOutput<Vec<String>>,
        /// The name of the stack to create. The resource deployed in AWS will be prefixed with `serverlessrepo-`
        #[builder(into, default)]
        pub name: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// A map of Parameter structures that specify input parameters for the stack.
        #[builder(into, default)]
        pub parameters: pulumi_gestalt_rust::InputOrOutput<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// The version of the application to deploy. If not supplied, deploys the latest version.
        #[builder(into, default)]
        pub semantic_version: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// A list of tags to associate with this stack. .If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
        #[builder(into, default)]
        pub tags: pulumi_gestalt_rust::InputOrOutput<
            Option<std::collections::HashMap<String, String>>,
        >,
    }
    #[allow(dead_code)]
    pub struct CloudFormationStackResult {
        /// The ARN of the application from the Serverless Application Repository.
        pub application_id: pulumi_gestalt_rust::Output<String>,
        /// A list of capabilities. Valid values are `CAPABILITY_IAM`, `CAPABILITY_NAMED_IAM`, `CAPABILITY_RESOURCE_POLICY`, or `CAPABILITY_AUTO_EXPAND`
        pub capabilities: pulumi_gestalt_rust::Output<Vec<String>>,
        /// The name of the stack to create. The resource deployed in AWS will be prefixed with `serverlessrepo-`
        pub name: pulumi_gestalt_rust::Output<String>,
        /// A map of outputs from the stack.
        pub outputs: pulumi_gestalt_rust::Output<
            std::collections::HashMap<String, String>,
        >,
        /// A map of Parameter structures that specify input parameters for the stack.
        pub parameters: pulumi_gestalt_rust::Output<
            std::collections::HashMap<String, String>,
        >,
        /// The version of the application to deploy. If not supplied, deploys the latest version.
        pub semantic_version: pulumi_gestalt_rust::Output<String>,
        /// A list of tags to associate with this stack. .If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
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
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: CloudFormationStackArgs,
    ) -> CloudFormationStackResult {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::register_interface;
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let application_id_binding = args.application_id.get_output(context);
        let capabilities_binding = args.capabilities.get_output(context);
        let name_binding = args.name.get_output(context);
        let parameters_binding = args.parameters.get_output(context);
        let semantic_version_binding = args.semantic_version.get_output(context);
        let tags_binding = args.tags.get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "aws:serverlessrepository/cloudFormationStack:CloudFormationStack"
                .into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "applicationId".into(),
                    value: application_id_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "capabilities".into(),
                    value: capabilities_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "name".into(),
                    value: name_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "parameters".into(),
                    value: parameters_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "semanticVersion".into(),
                    value: semantic_version_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "tags".into(),
                    value: tags_binding.get_id(),
                },
            ],
        };
        let o = context.register_resource(request);
        CloudFormationStackResult {
            application_id: o.get_field("applicationId"),
            capabilities: o.get_field("capabilities"),
            name: o.get_field("name"),
            outputs: o.get_field("outputs"),
            parameters: o.get_field("parameters"),
            semantic_version: o.get_field("semanticVersion"),
            tags: o.get_field("tags"),
            tags_all: o.get_field("tagsAll"),
        }
    }
}
