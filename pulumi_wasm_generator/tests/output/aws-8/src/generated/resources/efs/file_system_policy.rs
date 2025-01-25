/// Provides an Elastic File System (EFS) File System Policy resource.
///
/// ## Example Usage
///
/// ```yaml
/// resources:
///   fs:
///     type: aws:efs:FileSystem
///     properties:
///       creationToken: my-product
///   policyFileSystemPolicy:
///     type: aws:efs:FileSystemPolicy
///     name: policy
///     properties:
///       fileSystemId: ${fs.id}
///       policy: ${policy.json}
/// variables:
///   policy:
///     fn::invoke:
///       function: aws:iam:getPolicyDocument
///       arguments:
///         statements:
///           - sid: ExampleStatement01
///             effect: Allow
///             principals:
///               - type: AWS
///                 identifiers:
///                   - '*'
///             actions:
///               - elasticfilesystem:ClientMount
///               - elasticfilesystem:ClientWrite
///             resources:
///               - ${fs.arn}
///             conditions:
///               - test: Bool
///                 variable: aws:SecureTransport
///                 values:
///                   - 'true'
/// ```
///
/// ## Import
///
/// Using `pulumi import`, import the EFS file system policies using the `id`. For example:
///
/// ```sh
/// $ pulumi import aws:efs/fileSystemPolicy:FileSystemPolicy foo fs-6fa144c6
/// ```
pub mod file_system_policy {
    #[derive(pulumi_wasm_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct FileSystemPolicyArgs {
        /// A flag to indicate whether to bypass the `aws.efs.FileSystemPolicy` lockout safety check. The policy lockout safety check determines whether the policy in the request will prevent the principal making the request will be locked out from making future `PutFileSystemPolicy` requests on the file system. Set `bypass_policy_lockout_safety_check` to `true` only when you intend to prevent the principal that is making the request from making a subsequent `PutFileSystemPolicy` request on the file system. The default value is `false`.
        #[builder(into, default)]
        pub bypass_policy_lockout_safety_check: pulumi_wasm_rust::InputOrOutput<
            Option<bool>,
        >,
        /// The ID of the EFS file system.
        #[builder(into)]
        pub file_system_id: pulumi_wasm_rust::InputOrOutput<String>,
        /// The JSON formatted file system policy for the EFS file system. see [Docs](https://docs.aws.amazon.com/efs/latest/ug/access-control-overview.html#access-control-manage-access-intro-resource-policies) for more info.
        ///
        /// The following arguments are optional:
        #[builder(into)]
        pub policy: pulumi_wasm_rust::InputOrOutput<String>,
    }
    #[allow(dead_code)]
    pub struct FileSystemPolicyResult {
        /// A flag to indicate whether to bypass the `aws.efs.FileSystemPolicy` lockout safety check. The policy lockout safety check determines whether the policy in the request will prevent the principal making the request will be locked out from making future `PutFileSystemPolicy` requests on the file system. Set `bypass_policy_lockout_safety_check` to `true` only when you intend to prevent the principal that is making the request from making a subsequent `PutFileSystemPolicy` request on the file system. The default value is `false`.
        pub bypass_policy_lockout_safety_check: pulumi_wasm_rust::Output<Option<bool>>,
        /// The ID of the EFS file system.
        pub file_system_id: pulumi_wasm_rust::Output<String>,
        /// The JSON formatted file system policy for the EFS file system. see [Docs](https://docs.aws.amazon.com/efs/latest/ug/access-control-overview.html#access-control-manage-access-intro-resource-policies) for more info.
        ///
        /// The following arguments are optional:
        pub policy: pulumi_wasm_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_wasm_rust::PulumiContext,
        name: &str,
        args: FileSystemPolicyArgs,
    ) -> FileSystemPolicyResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let bypass_policy_lockout_safety_check_binding = args
            .bypass_policy_lockout_safety_check
            .get_output(context)
            .get_inner();
        let file_system_id_binding = args.file_system_id.get_output(context).get_inner();
        let policy_binding = args.policy.get_output(context).get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "aws:efs/fileSystemPolicy:FileSystemPolicy".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "bypassPolicyLockoutSafetyCheck".into(),
                    value: &bypass_policy_lockout_safety_check_binding,
                },
                register_interface::ObjectField {
                    name: "fileSystemId".into(),
                    value: &file_system_id_binding,
                },
                register_interface::ObjectField {
                    name: "policy".into(),
                    value: &policy_binding,
                },
            ]),
            results: Vec::from([
                register_interface::ResultField {
                    name: "bypassPolicyLockoutSafetyCheck".into(),
                },
                register_interface::ResultField {
                    name: "fileSystemId".into(),
                },
                register_interface::ResultField {
                    name: "policy".into(),
                },
            ]),
        };
        let o = register_interface::register(context.get_inner(), &request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        FileSystemPolicyResult {
            bypass_policy_lockout_safety_check: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("bypassPolicyLockoutSafetyCheck").unwrap(),
            ),
            file_system_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("fileSystemId").unwrap(),
            ),
            policy: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("policy").unwrap(),
            ),
        }
    }
}
