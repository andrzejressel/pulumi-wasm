/// Manages an Image Builder Lifecycle Policy.
///
/// ## Example Usage
///
/// ```yaml
/// resources:
///   example:
///     type: aws:iam:Role
///     properties:
///       assumeRolePolicy:
///         fn::toJSON:
///           Version: 2012-10-17
///           Statement:
///             - Action: sts:AssumeRole
///               Effect: Allow
///               Principal:
///                 Service: imagebuilder.${currentGetPartition.dnsSuffix}
///       name: example
///   exampleRolePolicyAttachment:
///     type: aws:iam:RolePolicyAttachment
///     name: example
///     properties:
///       policyArn: arn:${currentGetPartition.partition}:iam::aws:policy/service-role/EC2ImageBuilderLifecycleExecutionPolicy
///       role: ${example.name}
///   exampleLifecyclePolicy:
///     type: aws:imagebuilder:LifecyclePolicy
///     name: example
///     properties:
///       name: name
///       description: Example description
///       executionRole: ${example.arn}
///       resourceType: AMI_IMAGE
///       policyDetails:
///         - action:
///             type: DELETE
///           filter:
///             type: AGE
///             value: 6
///             retainAtLeast: 10
///             unit: YEARS
///       resourceSelection:
///         tagMap:
///           key1: value1
///           key2: value2
///     options:
///       dependsOn:
///         - ${exampleRolePolicyAttachment}
/// variables:
///   current:
///     fn::invoke:
///       function: aws:getRegion
///       arguments: {}
///   currentGetPartition:
///     fn::invoke:
///       function: aws:getPartition
///       arguments: {}
/// ```
///
/// ## Import
///
/// Using `pulumi import`, import `aws_imagebuilder_lifecycle_policy` using the Amazon Resource Name (ARN). For example:
///
/// ```sh
/// $ pulumi import aws:imagebuilder/lifecyclePolicy:LifecyclePolicy example arn:aws:imagebuilder:us-east-1:123456789012:lifecycle-policy/example
/// ```
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod lifecycle_policy {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct LifecyclePolicyArgs {
        /// description for the lifecycle policy.
        #[builder(into, default)]
        pub description: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The Amazon Resource Name (ARN) for the IAM role you create that grants Image Builder access to run lifecycle actions. More information about this role can be found [`here`](https://docs.aws.amazon.com/imagebuilder/latest/userguide/image-lifecycle-prerequisites.html#image-lifecycle-prereq-role).
        #[builder(into)]
        pub execution_role: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The name of the lifecycle policy to create.
        #[builder(into, default)]
        pub name: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Configuration block with policy details. Detailed below.
        #[builder(into, default)]
        pub policy_details: pulumi_gestalt_rust::InputOrOutput<
            Option<Vec<super::super::types::imagebuilder::LifecyclePolicyPolicyDetail>>,
        >,
        /// Selection criteria for the resources that the lifecycle policy applies to. Detailed below.
        ///
        /// The following arguments are optional:
        #[builder(into, default)]
        pub resource_selection: pulumi_gestalt_rust::InputOrOutput<
            Option<super::super::types::imagebuilder::LifecyclePolicyResourceSelection>,
        >,
        /// The type of Image Builder resource that the lifecycle policy applies to. Valid values: `AMI_IMAGE` or `CONTAINER_IMAGE`.
        #[builder(into)]
        pub resource_type: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The status of the lifecycle policy.
        #[builder(into, default)]
        pub status: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Key-value map of resource tags for the Image Builder Lifecycle Policy. If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
        #[builder(into, default)]
        pub tags: pulumi_gestalt_rust::InputOrOutput<
            Option<std::collections::HashMap<String, String>>,
        >,
    }
    #[allow(dead_code)]
    pub struct LifecyclePolicyResult {
        /// Amazon Resource Name (ARN) of the lifecycle policy.
        pub arn: pulumi_gestalt_rust::Output<String>,
        /// description for the lifecycle policy.
        pub description: pulumi_gestalt_rust::Output<Option<String>>,
        /// The Amazon Resource Name (ARN) for the IAM role you create that grants Image Builder access to run lifecycle actions. More information about this role can be found [`here`](https://docs.aws.amazon.com/imagebuilder/latest/userguide/image-lifecycle-prerequisites.html#image-lifecycle-prereq-role).
        pub execution_role: pulumi_gestalt_rust::Output<String>,
        /// The name of the lifecycle policy to create.
        pub name: pulumi_gestalt_rust::Output<String>,
        /// Configuration block with policy details. Detailed below.
        pub policy_details: pulumi_gestalt_rust::Output<
            Option<Vec<super::super::types::imagebuilder::LifecyclePolicyPolicyDetail>>,
        >,
        /// Selection criteria for the resources that the lifecycle policy applies to. Detailed below.
        ///
        /// The following arguments are optional:
        pub resource_selection: pulumi_gestalt_rust::Output<
            Option<super::super::types::imagebuilder::LifecyclePolicyResourceSelection>,
        >,
        /// The type of Image Builder resource that the lifecycle policy applies to. Valid values: `AMI_IMAGE` or `CONTAINER_IMAGE`.
        pub resource_type: pulumi_gestalt_rust::Output<String>,
        /// The status of the lifecycle policy.
        pub status: pulumi_gestalt_rust::Output<String>,
        /// Key-value map of resource tags for the Image Builder Lifecycle Policy. If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
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
        args: LifecyclePolicyArgs,
    ) -> LifecyclePolicyResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let description_binding = args.description.get_output(context);
        let execution_role_binding = args.execution_role.get_output(context);
        let name_binding = args.name.get_output(context);
        let policy_details_binding = args.policy_details.get_output(context);
        let resource_selection_binding = args.resource_selection.get_output(context);
        let resource_type_binding = args.resource_type.get_output(context);
        let status_binding = args.status.get_output(context);
        let tags_binding = args.tags.get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "aws:imagebuilder/lifecyclePolicy:LifecyclePolicy".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "description".into(),
                    value: description_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "executionRole".into(),
                    value: execution_role_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "name".into(),
                    value: name_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "policyDetails".into(),
                    value: policy_details_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "resourceSelection".into(),
                    value: resource_selection_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "resourceType".into(),
                    value: resource_type_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "status".into(),
                    value: status_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "tags".into(),
                    value: tags_binding.get_id(),
                },
            ],
        };
        let o = context.register_resource(request);
        LifecyclePolicyResult {
            arn: o.get_field("arn"),
            description: o.get_field("description"),
            execution_role: o.get_field("executionRole"),
            name: o.get_field("name"),
            policy_details: o.get_field("policyDetails"),
            resource_selection: o.get_field("resourceSelection"),
            resource_type: o.get_field("resourceType"),
            status: o.get_field("status"),
            tags: o.get_field("tags"),
            tags_all: o.get_field("tagsAll"),
        }
    }
}
