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
///       dependson:
///         - ${exampleRolePolicyAttachment}
/// variables:
///   current:
///     fn::invoke:
///       Function: aws:getRegion
///       Arguments: {}
///   currentGetPartition:
///     fn::invoke:
///       Function: aws:getPartition
///       Arguments: {}
/// ```
///
/// ## Import
///
/// Using `pulumi import`, import `aws_imagebuilder_lifecycle_policy` using the Amazon Resource Name (ARN). For example:
///
/// ```sh
/// $ pulumi import aws:imagebuilder/lifecyclePolicy:LifecyclePolicy example arn:aws:imagebuilder:us-east-1:123456789012:lifecycle-policy/example
/// ```
pub mod lifecycle_policy {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct LifecyclePolicyArgs {
        /// description for the lifecycle policy.
        #[builder(into, default)]
        pub description: pulumi_wasm_rust::Output<Option<String>>,
        /// The Amazon Resource Name (ARN) for the IAM role you create that grants Image Builder access to run lifecycle actions. More information about this role can be found [`here`](https://docs.aws.amazon.com/imagebuilder/latest/userguide/image-lifecycle-prerequisites.html#image-lifecycle-prereq-role).
        #[builder(into)]
        pub execution_role: pulumi_wasm_rust::Output<String>,
        /// The name of the lifecycle policy to create.
        #[builder(into, default)]
        pub name: pulumi_wasm_rust::Output<Option<String>>,
        /// Configuration block with policy details. Detailed below.
        #[builder(into, default)]
        pub policy_details: pulumi_wasm_rust::Output<
            Option<Vec<super::super::types::imagebuilder::LifecyclePolicyPolicyDetail>>,
        >,
        /// Selection criteria for the resources that the lifecycle policy applies to. Detailed below.
        ///
        /// The following arguments are optional:
        #[builder(into, default)]
        pub resource_selection: pulumi_wasm_rust::Output<
            Option<super::super::types::imagebuilder::LifecyclePolicyResourceSelection>,
        >,
        /// The type of Image Builder resource that the lifecycle policy applies to. Valid values: `AMI_IMAGE` or `CONTAINER_IMAGE`.
        #[builder(into)]
        pub resource_type: pulumi_wasm_rust::Output<String>,
        /// The status of the lifecycle policy.
        #[builder(into, default)]
        pub status: pulumi_wasm_rust::Output<Option<String>>,
        /// Key-value map of resource tags for the Image Builder Lifecycle Policy. If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
        #[builder(into, default)]
        pub tags: pulumi_wasm_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
    }
    #[allow(dead_code)]
    pub struct LifecyclePolicyResult {
        /// Amazon Resource Name (ARN) of the lifecycle policy.
        pub arn: pulumi_wasm_rust::Output<String>,
        /// description for the lifecycle policy.
        pub description: pulumi_wasm_rust::Output<Option<String>>,
        /// The Amazon Resource Name (ARN) for the IAM role you create that grants Image Builder access to run lifecycle actions. More information about this role can be found [`here`](https://docs.aws.amazon.com/imagebuilder/latest/userguide/image-lifecycle-prerequisites.html#image-lifecycle-prereq-role).
        pub execution_role: pulumi_wasm_rust::Output<String>,
        /// The name of the lifecycle policy to create.
        pub name: pulumi_wasm_rust::Output<String>,
        /// Configuration block with policy details. Detailed below.
        pub policy_details: pulumi_wasm_rust::Output<
            Option<Vec<super::super::types::imagebuilder::LifecyclePolicyPolicyDetail>>,
        >,
        /// Selection criteria for the resources that the lifecycle policy applies to. Detailed below.
        ///
        /// The following arguments are optional:
        pub resource_selection: pulumi_wasm_rust::Output<
            Option<super::super::types::imagebuilder::LifecyclePolicyResourceSelection>,
        >,
        /// The type of Image Builder resource that the lifecycle policy applies to. Valid values: `AMI_IMAGE` or `CONTAINER_IMAGE`.
        pub resource_type: pulumi_wasm_rust::Output<String>,
        /// The status of the lifecycle policy.
        pub status: pulumi_wasm_rust::Output<String>,
        /// Key-value map of resource tags for the Image Builder Lifecycle Policy. If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
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
    pub fn create(name: &str, args: LifecyclePolicyArgs) -> LifecyclePolicyResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let description_binding = args.description.get_inner();
        let execution_role_binding = args.execution_role.get_inner();
        let name_binding = args.name.get_inner();
        let policy_details_binding = args.policy_details.get_inner();
        let resource_selection_binding = args.resource_selection.get_inner();
        let resource_type_binding = args.resource_type.get_inner();
        let status_binding = args.status.get_inner();
        let tags_binding = args.tags.get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "aws:imagebuilder/lifecyclePolicy:LifecyclePolicy".into(),
            name: name.to_string(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "description".into(),
                    value: &description_binding,
                },
                register_interface::ObjectField {
                    name: "executionRole".into(),
                    value: &execution_role_binding,
                },
                register_interface::ObjectField {
                    name: "name".into(),
                    value: &name_binding,
                },
                register_interface::ObjectField {
                    name: "policyDetails".into(),
                    value: &policy_details_binding,
                },
                register_interface::ObjectField {
                    name: "resourceSelection".into(),
                    value: &resource_selection_binding,
                },
                register_interface::ObjectField {
                    name: "resourceType".into(),
                    value: &resource_type_binding,
                },
                register_interface::ObjectField {
                    name: "status".into(),
                    value: &status_binding,
                },
                register_interface::ObjectField {
                    name: "tags".into(),
                    value: &tags_binding,
                },
            ]),
            results: Vec::from([
                register_interface::ResultField {
                    name: "arn".into(),
                },
                register_interface::ResultField {
                    name: "description".into(),
                },
                register_interface::ResultField {
                    name: "executionRole".into(),
                },
                register_interface::ResultField {
                    name: "name".into(),
                },
                register_interface::ResultField {
                    name: "policyDetails".into(),
                },
                register_interface::ResultField {
                    name: "resourceSelection".into(),
                },
                register_interface::ResultField {
                    name: "resourceType".into(),
                },
                register_interface::ResultField {
                    name: "status".into(),
                },
                register_interface::ResultField {
                    name: "tags".into(),
                },
                register_interface::ResultField {
                    name: "tagsAll".into(),
                },
            ]),
        };
        let o = register_interface::register(&request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        LifecyclePolicyResult {
            arn: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("arn").unwrap(),
            ),
            description: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("description").unwrap(),
            ),
            execution_role: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("executionRole").unwrap(),
            ),
            name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("name").unwrap(),
            ),
            policy_details: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("policyDetails").unwrap(),
            ),
            resource_selection: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("resourceSelection").unwrap(),
            ),
            resource_type: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("resourceType").unwrap(),
            ),
            status: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("status").unwrap(),
            ),
            tags: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("tags").unwrap(),
            ),
            tags_all: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("tagsAll").unwrap(),
            ),
        }
    }
}