/// Manages an ECR repository lifecycle policy.
///
/// > **NOTE:** Only one `aws.ecr.LifecyclePolicy` resource can be used with the same ECR repository. To apply multiple rules, they must be combined in the `policy` JSON.
///
/// > **NOTE:** The AWS ECR API seems to reorder rules based on `rulePriority`. If you define multiple rules that are not sorted in ascending `rulePriority` order in the this provider code, the resource will be flagged for recreation every deployment.
///
/// ## Example Usage
///
/// ### Policy on untagged image
///
/// ```ignore
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let example = repository::create(
///         "example",
///         RepositoryArgs::builder().name("example-repo").build_struct(),
///     );
///     let exampleLifecyclePolicy = lifecycle_policy::create(
///         "exampleLifecyclePolicy",
///         LifecyclePolicyArgs::builder()
///             .policy(
///                 "{\n    \"rules\": [\n        {\n            \"rulePriority\": 1,\n            \"description\": \"Expire images older than 14 days\",\n            \"selection\": {\n                \"tagStatus\": \"untagged\",\n                \"countType\": \"sinceImagePushed\",\n                \"countUnit\": \"days\",\n                \"countNumber\": 14\n            },\n            \"action\": {\n                \"type\": \"expire\"\n            }\n        }\n    ]\n}",
///             )
///             .repository("${example.name}")
///             .build_struct(),
///     );
/// }
/// ```
///
/// ### Policy on tagged image
///
/// ```ignore
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let example = repository::create(
///         "example",
///         RepositoryArgs::builder().name("example-repo").build_struct(),
///     );
///     let exampleLifecyclePolicy = lifecycle_policy::create(
///         "exampleLifecyclePolicy",
///         LifecyclePolicyArgs::builder()
///             .policy(
///                 "{\n    \"rules\": [\n        {\n            \"rulePriority\": 1,\n            \"description\": \"Keep last 30 images\",\n            \"selection\": {\n                \"tagStatus\": \"tagged\",\n                \"tagPrefixList\": [\"v\"],\n                \"countType\": \"imageCountMoreThan\",\n                \"countNumber\": 30\n            },\n            \"action\": {\n                \"type\": \"expire\"\n            }\n        }\n    ]\n}",
///             )
///             .repository("${example.name}")
///             .build_struct(),
///     );
/// }
/// ```
///
/// ## Import
///
/// Using `pulumi import`, import ECR Lifecycle Policy using the name of the repository. For example:
///
/// ```sh
/// $ pulumi import aws:ecr/lifecyclePolicy:LifecyclePolicy example tf-example
/// ```
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod lifecycle_policy {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct LifecyclePolicyArgs {
        /// The policy document. This is a JSON formatted string. See more details about [Policy Parameters](http://docs.aws.amazon.com/AmazonECR/latest/userguide/LifecyclePolicies.html#lifecycle_policy_parameters) in the official AWS docs. Consider using the `aws.ecr.getLifecyclePolicyDocument` data_source to generate/manage the JSON document used for the `policy` argument.
        #[builder(into)]
        pub policy: pulumi_gestalt_rust::InputOrOutput<String>,
        /// Name of the repository to apply the policy.
        #[builder(into)]
        pub repository: pulumi_gestalt_rust::InputOrOutput<String>,
    }
    #[allow(dead_code)]
    pub struct LifecyclePolicyResult {
        /// The policy document. This is a JSON formatted string. See more details about [Policy Parameters](http://docs.aws.amazon.com/AmazonECR/latest/userguide/LifecyclePolicies.html#lifecycle_policy_parameters) in the official AWS docs. Consider using the `aws.ecr.getLifecyclePolicyDocument` data_source to generate/manage the JSON document used for the `policy` argument.
        pub policy: pulumi_gestalt_rust::Output<String>,
        /// The registry ID where the repository was created.
        pub registry_id: pulumi_gestalt_rust::Output<String>,
        /// Name of the repository to apply the policy.
        pub repository: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::PulumiContext,
        name: &str,
        args: LifecyclePolicyArgs,
    ) -> LifecyclePolicyResult {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::register_interface;
        use std::collections::HashMap;
        let policy_binding_1 = args.policy.get_output(context);
        let policy_binding = policy_binding_1.get_inner();
        let repository_binding_1 = args.repository.get_output(context);
        let repository_binding = repository_binding_1.get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "aws:ecr/lifecyclePolicy:LifecyclePolicy".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "policy".into(),
                    value: &policy_binding,
                },
                register_interface::ObjectField {
                    name: "repository".into(),
                    value: &repository_binding,
                },
            ]),
        };
        let o = register_interface::register(context.get_inner(), &request);
        LifecyclePolicyResult {
            policy: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("policy"),
            ),
            registry_id: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("registryId"),
            ),
            repository: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("repository"),
            ),
        }
    }
}
