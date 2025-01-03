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
pub mod cloud_formation_stack {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct CloudFormationStackArgs {
        /// The ARN of the application from the Serverless Application Repository.
        #[builder(into)]
        pub application_id: pulumi_wasm_rust::Output<String>,
        /// A list of capabilities. Valid values are `CAPABILITY_IAM`, `CAPABILITY_NAMED_IAM`, `CAPABILITY_RESOURCE_POLICY`, or `CAPABILITY_AUTO_EXPAND`
        #[builder(into)]
        pub capabilities: pulumi_wasm_rust::Output<Vec<String>>,
        /// The name of the stack to create. The resource deployed in AWS will be prefixed with `serverlessrepo-`
        #[builder(into, default)]
        pub name: pulumi_wasm_rust::Output<Option<String>>,
        /// A map of Parameter structures that specify input parameters for the stack.
        #[builder(into, default)]
        pub parameters: pulumi_wasm_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// The version of the application to deploy. If not supplied, deploys the latest version.
        #[builder(into, default)]
        pub semantic_version: pulumi_wasm_rust::Output<Option<String>>,
        /// A list of tags to associate with this stack. .If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
        #[builder(into, default)]
        pub tags: pulumi_wasm_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
    }
    #[allow(dead_code)]
    pub struct CloudFormationStackResult {
        /// The ARN of the application from the Serverless Application Repository.
        pub application_id: pulumi_wasm_rust::Output<String>,
        /// A list of capabilities. Valid values are `CAPABILITY_IAM`, `CAPABILITY_NAMED_IAM`, `CAPABILITY_RESOURCE_POLICY`, or `CAPABILITY_AUTO_EXPAND`
        pub capabilities: pulumi_wasm_rust::Output<Vec<String>>,
        /// The name of the stack to create. The resource deployed in AWS will be prefixed with `serverlessrepo-`
        pub name: pulumi_wasm_rust::Output<String>,
        /// A map of outputs from the stack.
        pub outputs: pulumi_wasm_rust::Output<std::collections::HashMap<String, String>>,
        /// A map of Parameter structures that specify input parameters for the stack.
        pub parameters: pulumi_wasm_rust::Output<
            std::collections::HashMap<String, String>,
        >,
        /// The version of the application to deploy. If not supplied, deploys the latest version.
        pub semantic_version: pulumi_wasm_rust::Output<String>,
        /// A list of tags to associate with this stack. .If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
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
    pub fn create(
        name: &str,
        args: CloudFormationStackArgs,
    ) -> CloudFormationStackResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let application_id_binding = args.application_id.get_inner();
        let capabilities_binding = args.capabilities.get_inner();
        let name_binding = args.name.get_inner();
        let parameters_binding = args.parameters.get_inner();
        let semantic_version_binding = args.semantic_version.get_inner();
        let tags_binding = args.tags.get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "aws:serverlessrepository/cloudFormationStack:CloudFormationStack"
                .into(),
            name: name.to_string(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "applicationId".into(),
                    value: &application_id_binding,
                },
                register_interface::ObjectField {
                    name: "capabilities".into(),
                    value: &capabilities_binding,
                },
                register_interface::ObjectField {
                    name: "name".into(),
                    value: &name_binding,
                },
                register_interface::ObjectField {
                    name: "parameters".into(),
                    value: &parameters_binding,
                },
                register_interface::ObjectField {
                    name: "semanticVersion".into(),
                    value: &semantic_version_binding,
                },
                register_interface::ObjectField {
                    name: "tags".into(),
                    value: &tags_binding,
                },
            ]),
            results: Vec::from([
                register_interface::ResultField {
                    name: "applicationId".into(),
                },
                register_interface::ResultField {
                    name: "capabilities".into(),
                },
                register_interface::ResultField {
                    name: "name".into(),
                },
                register_interface::ResultField {
                    name: "outputs".into(),
                },
                register_interface::ResultField {
                    name: "parameters".into(),
                },
                register_interface::ResultField {
                    name: "semanticVersion".into(),
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
        CloudFormationStackResult {
            application_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("applicationId").unwrap(),
            ),
            capabilities: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("capabilities").unwrap(),
            ),
            name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("name").unwrap(),
            ),
            outputs: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("outputs").unwrap(),
            ),
            parameters: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("parameters").unwrap(),
            ),
            semantic_version: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("semanticVersion").unwrap(),
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
