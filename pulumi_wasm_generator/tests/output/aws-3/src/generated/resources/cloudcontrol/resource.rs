/// Manages a Cloud Control API Resource. The configuration and lifecycle handling of these resources is proxied through Cloud Control API handlers to the backend service.
///
/// ## Example Usage
///
/// ```yaml
/// resources:
///   example:
///     type: aws:cloudcontrol:Resource
///     properties:
///       typeName: AWS::ECS::Cluster
///       desiredState:
///         fn::toJSON:
///           ClusterName: example
///           Tags:
///             - Key: CostCenter
///               Value: IT
/// ```
pub mod resource {
    #[derive(pulumi_wasm_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct ResourceArgs {
        /// JSON string matching the CloudFormation resource type schema with desired configuration.
        #[builder(into)]
        pub desired_state: pulumi_wasm_rust::InputOrOutput<String>,
        /// Amazon Resource Name (ARN) of the IAM Role to assume for operations.
        #[builder(into, default)]
        pub role_arn: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        /// JSON string of the CloudFormation resource type schema which is used for plan time validation where possible. Automatically fetched if not provided. In large scale environments with multiple resources using the same `type_name`, it is recommended to fetch the schema once via the `aws.cloudformation.CloudFormationType` data source and use this argument to reduce `DescribeType` API operation throttling. This value is marked sensitive only to prevent large plan differences from showing.
        #[builder(into, default)]
        pub schema: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        /// CloudFormation resource type name. For example, `AWS::EC2::VPC`.
        ///
        /// The following arguments are optional:
        #[builder(into)]
        pub type_name: pulumi_wasm_rust::InputOrOutput<String>,
        /// Identifier of the CloudFormation resource type version.
        #[builder(into, default)]
        pub type_version_id: pulumi_wasm_rust::InputOrOutput<Option<String>>,
    }
    #[allow(dead_code)]
    pub struct ResourceResult {
        /// JSON string matching the CloudFormation resource type schema with desired configuration.
        pub desired_state: pulumi_wasm_rust::Output<String>,
        /// JSON string matching the CloudFormation resource type schema with current configuration. Underlying attributes can be referenced via the `jsondecode()` function, for example, `jsondecode(data.aws_cloudcontrolapi_resource.example.properties)["example"]`.
        pub properties: pulumi_wasm_rust::Output<String>,
        /// Amazon Resource Name (ARN) of the IAM Role to assume for operations.
        pub role_arn: pulumi_wasm_rust::Output<Option<String>>,
        /// JSON string of the CloudFormation resource type schema which is used for plan time validation where possible. Automatically fetched if not provided. In large scale environments with multiple resources using the same `type_name`, it is recommended to fetch the schema once via the `aws.cloudformation.CloudFormationType` data source and use this argument to reduce `DescribeType` API operation throttling. This value is marked sensitive only to prevent large plan differences from showing.
        pub schema: pulumi_wasm_rust::Output<String>,
        /// CloudFormation resource type name. For example, `AWS::EC2::VPC`.
        ///
        /// The following arguments are optional:
        pub type_name: pulumi_wasm_rust::Output<String>,
        /// Identifier of the CloudFormation resource type version.
        pub type_version_id: pulumi_wasm_rust::Output<Option<String>>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_wasm_rust::PulumiContext,
        name: &str,
        args: ResourceArgs,
    ) -> ResourceResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let desired_state_binding = args.desired_state.get_output(context).get_inner();
        let role_arn_binding = args.role_arn.get_output(context).get_inner();
        let schema_binding = args.schema.get_output(context).get_inner();
        let type_name_binding = args.type_name.get_output(context).get_inner();
        let type_version_id_binding = args
            .type_version_id
            .get_output(context)
            .get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "aws:cloudcontrol/resource:Resource".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "desiredState".into(),
                    value: &desired_state_binding,
                },
                register_interface::ObjectField {
                    name: "roleArn".into(),
                    value: &role_arn_binding,
                },
                register_interface::ObjectField {
                    name: "schema".into(),
                    value: &schema_binding,
                },
                register_interface::ObjectField {
                    name: "typeName".into(),
                    value: &type_name_binding,
                },
                register_interface::ObjectField {
                    name: "typeVersionId".into(),
                    value: &type_version_id_binding,
                },
            ]),
            results: Vec::from([
                register_interface::ResultField {
                    name: "desiredState".into(),
                },
                register_interface::ResultField {
                    name: "properties".into(),
                },
                register_interface::ResultField {
                    name: "roleArn".into(),
                },
                register_interface::ResultField {
                    name: "schema".into(),
                },
                register_interface::ResultField {
                    name: "typeName".into(),
                },
                register_interface::ResultField {
                    name: "typeVersionId".into(),
                },
            ]),
        };
        let o = register_interface::register(context.get_inner(), &request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        ResourceResult {
            desired_state: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("desiredState").unwrap(),
            ),
            properties: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("properties").unwrap(),
            ),
            role_arn: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("roleArn").unwrap(),
            ),
            schema: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("schema").unwrap(),
            ),
            type_name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("typeName").unwrap(),
            ),
            type_version_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("typeVersionId").unwrap(),
            ),
        }
    }
}
