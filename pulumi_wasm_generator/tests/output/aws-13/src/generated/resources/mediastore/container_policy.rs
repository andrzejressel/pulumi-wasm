/// ## Example Usage
///
/// ```yaml
/// resources:
///   exampleContainer:
///     type: aws:mediastore:Container
///     name: example
///     properties:
///       name: example
///   exampleContainerPolicy:
///     type: aws:mediastore:ContainerPolicy
///     name: example
///     properties:
///       containerName: ${exampleContainer.name}
///       policy: ${example.json}
/// variables:
///   current:
///     fn::invoke:
///       function: aws:getRegion
///       arguments: {}
///   currentGetCallerIdentity:
///     fn::invoke:
///       function: aws:getCallerIdentity
///       arguments: {}
///   example:
///     fn::invoke:
///       function: aws:iam:getPolicyDocument
///       arguments:
///         statements:
///           - sid: MediaStoreFullAccess
///             effect: Allow
///             principals:
///               - type: AWS
///                 identifiers:
///                   - arn:aws:iam::${currentGetCallerIdentity.accountId}:root
///             actions:
///               - mediastore:*
///             resources:
///               - arn:aws:mediastore:${current.name}:${currentGetCallerIdentity.accountId}:container/${exampleContainer.name}/*
///             conditions:
///               - test: Bool
///                 variable: aws:SecureTransport
///                 values:
///                   - 'true'
/// ```
///
/// ## Import
///
/// Using `pulumi import`, import MediaStore Container Policy using the MediaStore Container Name. For example:
///
/// ```sh
/// $ pulumi import aws:mediastore/containerPolicy:ContainerPolicy example example
/// ```
pub mod container_policy {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct ContainerPolicyArgs {
        /// The name of the container.
        #[builder(into)]
        pub container_name: pulumi_wasm_rust::Output<String>,
        /// The contents of the policy.
        #[builder(into)]
        pub policy: pulumi_wasm_rust::Output<String>,
    }
    #[allow(dead_code)]
    pub struct ContainerPolicyResult {
        /// The name of the container.
        pub container_name: pulumi_wasm_rust::Output<String>,
        /// The contents of the policy.
        pub policy: pulumi_wasm_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(name: &str, args: ContainerPolicyArgs) -> ContainerPolicyResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let container_name_binding = args.container_name.get_inner();
        let policy_binding = args.policy.get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "aws:mediastore/containerPolicy:ContainerPolicy".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "containerName".into(),
                    value: &container_name_binding,
                },
                register_interface::ObjectField {
                    name: "policy".into(),
                    value: &policy_binding,
                },
            ]),
            results: Vec::from([
                register_interface::ResultField {
                    name: "containerName".into(),
                },
                register_interface::ResultField {
                    name: "policy".into(),
                },
            ]),
        };
        let o = register_interface::register(&request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        ContainerPolicyResult {
            container_name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("containerName").unwrap(),
            ),
            policy: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("policy").unwrap(),
            ),
        }
    }
}
