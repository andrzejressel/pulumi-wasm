/// Provides a resource to manage an S3 Access Point resource policy.
///
/// > **NOTE on Access Points and Access Point Policies:** The provider provides both a standalone Access Point Policy resource and an Access Point resource with a resource policy defined in-line. You cannot use an Access Point with in-line resource policy in conjunction with an Access Point Policy resource. Doing so will cause a conflict of policies and will overwrite the access point's resource policy.
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
///       publicAccessBlockConfiguration:
///         blockPublicAcls: true
///         blockPublicPolicy: false
///         ignorePublicAcls: true
///         restrictPublicBuckets: false
///   exampleAccessPointPolicy:
///     type: aws:s3control:AccessPointPolicy
///     name: example
///     properties:
///       accessPointArn: ${exampleAccessPoint.arn}
///       policy:
///         fn::toJSON:
///           Version: 2008-10-17
///           Statement:
///             - Effect: Allow
///               Action: s3:GetObjectTagging
///               Principal:
///                 AWS: '*'
///               Resource: ${exampleAccessPoint.arn}/object/*
/// ```
///
/// ## Import
///
/// Using `pulumi import`, import Access Point policies using the `access_point_arn`. For example:
///
/// ```sh
/// $ pulumi import aws:s3control/accessPointPolicy:AccessPointPolicy example arn:aws:s3:us-west-2:123456789012:accesspoint/example
/// ```
pub mod access_point_policy {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct AccessPointPolicyArgs {
        /// The ARN of the access point that you want to associate with the specified policy.
        #[builder(into)]
        pub access_point_arn: pulumi_wasm_rust::Output<String>,
        /// The policy that you want to apply to the specified access point.
        #[builder(into)]
        pub policy: pulumi_wasm_rust::Output<String>,
    }
    #[allow(dead_code)]
    pub struct AccessPointPolicyResult {
        /// The ARN of the access point that you want to associate with the specified policy.
        pub access_point_arn: pulumi_wasm_rust::Output<String>,
        /// Indicates whether this access point currently has a policy that allows public access.
        pub has_public_access_policy: pulumi_wasm_rust::Output<bool>,
        /// The policy that you want to apply to the specified access point.
        pub policy: pulumi_wasm_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(name: &str, args: AccessPointPolicyArgs) -> AccessPointPolicyResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let access_point_arn_binding = args.access_point_arn.get_inner();
        let policy_binding = args.policy.get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "aws:s3control/accessPointPolicy:AccessPointPolicy".into(),
            name: name.to_string(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "accessPointArn".into(),
                    value: &access_point_arn_binding,
                },
                register_interface::ObjectField {
                    name: "policy".into(),
                    value: &policy_binding,
                },
            ]),
            results: Vec::from([
                register_interface::ResultField {
                    name: "accessPointArn".into(),
                },
                register_interface::ResultField {
                    name: "hasPublicAccessPolicy".into(),
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
        AccessPointPolicyResult {
            access_point_arn: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("accessPointArn").unwrap(),
            ),
            has_public_access_policy: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("hasPublicAccessPolicy").unwrap(),
            ),
            policy: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("policy").unwrap(),
            ),
        }
    }
}