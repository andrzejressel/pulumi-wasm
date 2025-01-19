/// Provides a resource to manage an S3 Object Lambda Access Point resource policy.
///
/// ## Example Usage
///
/// ```yaml
/// resources:
///   example:
///     type: aws:s3:BucketV2
///     properties:
///       bucket: example
///   exampleAccessPoint:
///     type: aws:s3:AccessPoint
///     name: example
///     properties:
///       bucket: ${example.id}
///       name: example
///   exampleObjectLambdaAccessPoint:
///     type: aws:s3control:ObjectLambdaAccessPoint
///     name: example
///     properties:
///       name: example
///       configuration:
///         supportingAccessPoint: ${exampleAccessPoint.arn}
///         transformationConfigurations:
///           - actions:
///               - GetObject
///             contentTransformation:
///               awsLambda:
///                 functionArn: ${exampleAwsLambdaFunction.arn}
///   exampleObjectLambdaAccessPointPolicy:
///     type: aws:s3control:ObjectLambdaAccessPointPolicy
///     name: example
///     properties:
///       name: ${exampleObjectLambdaAccessPoint.name}
///       policy:
///         fn::toJSON:
///           Version: 2008-10-17
///           Statement:
///             - Effect: Allow
///               Action: s3-object-lambda:GetObject
///               Principal:
///                 AWS: ${current.accountId}
///               Resource: ${exampleObjectLambdaAccessPoint.arn}
/// ```
///
/// ## Import
///
/// Using `pulumi import`, import Object Lambda Access Point policies using the `account_id` and `name`, separated by a colon (`:`). For example:
///
/// ```sh
/// $ pulumi import aws:s3control/objectLambdaAccessPointPolicy:ObjectLambdaAccessPointPolicy example 123456789012:example
/// ```
pub mod object_lambda_access_point_policy {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct ObjectLambdaAccessPointPolicyArgs {
        /// The AWS account ID for the account that owns the Object Lambda Access Point. Defaults to automatically determined account ID of the AWS provider.
        #[builder(into, default)]
        pub account_id: pulumi_wasm_rust::Output<Option<String>>,
        /// The name of the Object Lambda Access Point.
        #[builder(into, default)]
        pub name: pulumi_wasm_rust::Output<Option<String>>,
        /// The Object Lambda Access Point resource policy document.
        #[builder(into)]
        pub policy: pulumi_wasm_rust::Output<String>,
    }
    #[allow(dead_code)]
    pub struct ObjectLambdaAccessPointPolicyResult {
        /// The AWS account ID for the account that owns the Object Lambda Access Point. Defaults to automatically determined account ID of the AWS provider.
        pub account_id: pulumi_wasm_rust::Output<String>,
        /// Indicates whether this access point currently has a policy that allows public access.
        pub has_public_access_policy: pulumi_wasm_rust::Output<bool>,
        /// The name of the Object Lambda Access Point.
        pub name: pulumi_wasm_rust::Output<String>,
        /// The Object Lambda Access Point resource policy document.
        pub policy: pulumi_wasm_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        name: &str,
        args: ObjectLambdaAccessPointPolicyArgs,
    ) -> ObjectLambdaAccessPointPolicyResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let account_id_binding = args.account_id.get_inner();
        let name_binding = args.name.get_inner();
        let policy_binding = args.policy.get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "aws:s3control/objectLambdaAccessPointPolicy:ObjectLambdaAccessPointPolicy"
                .into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "accountId".into(),
                    value: &account_id_binding,
                },
                register_interface::ObjectField {
                    name: "name".into(),
                    value: &name_binding,
                },
                register_interface::ObjectField {
                    name: "policy".into(),
                    value: &policy_binding,
                },
            ]),
            results: Vec::from([
                register_interface::ResultField {
                    name: "accountId".into(),
                },
                register_interface::ResultField {
                    name: "hasPublicAccessPolicy".into(),
                },
                register_interface::ResultField {
                    name: "name".into(),
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
        ObjectLambdaAccessPointPolicyResult {
            account_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("accountId").unwrap(),
            ),
            has_public_access_policy: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("hasPublicAccessPolicy").unwrap(),
            ),
            name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("name").unwrap(),
            ),
            policy: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("policy").unwrap(),
            ),
        }
    }
}
