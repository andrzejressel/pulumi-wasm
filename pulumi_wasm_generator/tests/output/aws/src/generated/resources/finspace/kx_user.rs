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
pub mod kx_user {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct KxUserArgs {
        /// Unique identifier for the KX environment.
        #[builder(into)]
        pub environment_id: pulumi_wasm_rust::Output<String>,
        /// IAM role ARN to be associated with the user.
        ///
        /// The following arguments are optional:
        #[builder(into)]
        pub iam_role: pulumi_wasm_rust::Output<String>,
        /// A unique identifier for the user.
        #[builder(into, default)]
        pub name: pulumi_wasm_rust::Output<Option<String>>,
        /// Key-value mapping of resource tags. If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
        #[builder(into, default)]
        pub tags: pulumi_wasm_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
    }
    #[allow(dead_code)]
    pub struct KxUserResult {
        /// Amazon Resource Name (ARN) identifier of the KX user.
        pub arn: pulumi_wasm_rust::Output<String>,
        /// Unique identifier for the KX environment.
        pub environment_id: pulumi_wasm_rust::Output<String>,
        /// IAM role ARN to be associated with the user.
        ///
        /// The following arguments are optional:
        pub iam_role: pulumi_wasm_rust::Output<String>,
        /// A unique identifier for the user.
        pub name: pulumi_wasm_rust::Output<String>,
        /// Key-value mapping of resource tags. If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
        pub tags: pulumi_wasm_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// Map of tags assigned to the resource, including those inherited from the provider `default_tags` configuration block.
        pub tags_all: pulumi_wasm_rust::Output<
            std::collections::HashMap<String, String>,
        >,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(name: &str, args: KxUserArgs) -> KxUserResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let environment_id_binding = args.environment_id.get_inner();
        let iam_role_binding = args.iam_role.get_inner();
        let name_binding = args.name.get_inner();
        let tags_binding = args.tags.get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "aws:finspace/kxUser:KxUser".into(),
            name: name.to_string(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "environmentId".into(),
                    value: &environment_id_binding,
                },
                register_interface::ObjectField {
                    name: "iamRole".into(),
                    value: &iam_role_binding,
                },
                register_interface::ObjectField {
                    name: "name".into(),
                    value: &name_binding,
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
                    name: "environmentId".into(),
                },
                register_interface::ResultField {
                    name: "iamRole".into(),
                },
                register_interface::ResultField {
                    name: "name".into(),
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
        KxUserResult {
            arn: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("arn").unwrap(),
            ),
            environment_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("environmentId").unwrap(),
            ),
            iam_role: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("iamRole").unwrap(),
            ),
            name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("name").unwrap(),
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
