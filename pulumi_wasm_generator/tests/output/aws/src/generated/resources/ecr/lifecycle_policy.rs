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
/// use pulumi_wasm_rust::Output;
/// use pulumi_wasm_rust::{add_export, pulumi_main};
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
/// use pulumi_wasm_rust::Output;
/// use pulumi_wasm_rust::{add_export, pulumi_main};
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
pub mod lifecycle_policy {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct LifecyclePolicyArgs {
        /// The policy document. This is a JSON formatted string. See more details about [Policy Parameters](http://docs.aws.amazon.com/AmazonECR/latest/userguide/LifecyclePolicies.html#lifecycle_policy_parameters) in the official AWS docs. Consider using the `aws.ecr.getLifecyclePolicyDocument` data_source to generate/manage the JSON document used for the `policy` argument.
        #[builder(into)]
        pub policy: pulumi_wasm_rust::Output<String>,
        /// Name of the repository to apply the policy.
        #[builder(into)]
        pub repository: pulumi_wasm_rust::Output<String>,
    }
    #[allow(dead_code)]
    pub struct LifecyclePolicyResult {
        /// The policy document. This is a JSON formatted string. See more details about [Policy Parameters](http://docs.aws.amazon.com/AmazonECR/latest/userguide/LifecyclePolicies.html#lifecycle_policy_parameters) in the official AWS docs. Consider using the `aws.ecr.getLifecyclePolicyDocument` data_source to generate/manage the JSON document used for the `policy` argument.
        pub policy: pulumi_wasm_rust::Output<String>,
        /// The registry ID where the repository was created.
        pub registry_id: pulumi_wasm_rust::Output<String>,
        /// Name of the repository to apply the policy.
        pub repository: pulumi_wasm_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(name: &str, args: LifecyclePolicyArgs) -> LifecyclePolicyResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let policy_binding = args.policy.get_inner();
        let repository_binding = args.repository.get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "aws:ecr/lifecyclePolicy:LifecyclePolicy".into(),
            name: name.to_string(),
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
            results: Vec::from([
                register_interface::ResultField {
                    name: "policy".into(),
                },
                register_interface::ResultField {
                    name: "registryId".into(),
                },
                register_interface::ResultField {
                    name: "repository".into(),
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
            policy: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("policy").unwrap(),
            ),
            registry_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("registryId").unwrap(),
            ),
            repository: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("repository").unwrap(),
            ),
        }
    }
}