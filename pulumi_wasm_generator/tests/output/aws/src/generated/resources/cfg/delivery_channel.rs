/// Provides an AWS Config Delivery Channel.
///
/// > **Note:** Delivery Channel requires a Configuration Recorder to be present. Use of `depends_on` (as shown below) is recommended to avoid race conditions.
///
/// ## Example Usage
///
/// ```ignore
/// use pulumi_wasm_rust::Output;
/// use pulumi_wasm_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let assumeRole = get_policy_document::invoke(
///         GetPolicyDocumentArgs::builder()
///             .statements(
///                 vec![
///                     GetPolicyDocumentStatement::builder()
///                     .actions(vec!["sts:AssumeRole",]).effect("Allow")
///                     .principals(vec![GetPolicyDocumentStatementPrincipal::builder()
///                     .identifiers(vec!["config.amazonaws.com",]). type ("Service")
///                     .build_struct(),]).build_struct(),
///                 ],
///             )
///             .build_struct(),
///     );
///     let p = get_policy_document::invoke(
///         GetPolicyDocumentArgs::builder()
///             .statements(
///                 vec![
///                     GetPolicyDocumentStatement::builder().actions(vec!["s3:*",])
///                     .effect("Allow").resources(vec!["${b.arn}", "${b.arn}/*",])
///                     .build_struct(),
///                 ],
///             )
///             .build_struct(),
///     );
///     let b = bucket_v_2::create(
///         "b",
///         BucketV2Args::builder()
///             .bucket("example-awsconfig")
///             .force_destroy(true)
///             .build_struct(),
///     );
///     let foo = delivery_channel::create(
///         "foo",
///         DeliveryChannelArgs::builder()
///             .name("example")
///             .s_3_bucket_name("${b.bucket}")
///             .build_struct(),
///     );
///     let fooRecorder = recorder::create(
///         "fooRecorder",
///         RecorderArgs::builder().name("example").role_arn("${r.arn}").build_struct(),
///     );
///     let pRolePolicy = role_policy::create(
///         "pRolePolicy",
///         RolePolicyArgs::builder()
///             .name("awsconfig-example")
///             .policy("${p.json}")
///             .role("${r.id}")
///             .build_struct(),
///     );
///     let r = role::create(
///         "r",
///         RoleArgs::builder()
///             .assume_role_policy("${assumeRole.json}")
///             .name("awsconfig-example")
///             .build_struct(),
///     );
/// }
/// ```
///
/// ## Import
///
/// Using `pulumi import`, import Delivery Channel using the name. For example:
///
/// ```sh
/// $ pulumi import aws:cfg/deliveryChannel:DeliveryChannel foo example
/// ```
pub mod delivery_channel {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct DeliveryChannelArgs {
        /// The name of the delivery channel. Defaults to `default`. Changing it recreates the resource.
        #[builder(into, default)]
        pub name: pulumi_wasm_rust::Output<Option<String>>,
        /// The name of the S3 bucket used to store the configuration history.
        #[builder(into)]
        pub s3_bucket_name: pulumi_wasm_rust::Output<String>,
        /// The prefix for the specified S3 bucket.
        #[builder(into, default)]
        pub s3_key_prefix: pulumi_wasm_rust::Output<Option<String>>,
        /// The ARN of the AWS KMS key used to encrypt objects delivered by AWS Config. Must belong to the same Region as the destination S3 bucket.
        #[builder(into, default)]
        pub s3_kms_key_arn: pulumi_wasm_rust::Output<Option<String>>,
        /// Options for how AWS Config delivers configuration snapshots. See below
        #[builder(into, default)]
        pub snapshot_delivery_properties: pulumi_wasm_rust::Output<
            Option<super::super::types::cfg::DeliveryChannelSnapshotDeliveryProperties>,
        >,
        /// The ARN of the SNS topic that AWS Config delivers notifications to.
        #[builder(into, default)]
        pub sns_topic_arn: pulumi_wasm_rust::Output<Option<String>>,
    }
    #[allow(dead_code)]
    pub struct DeliveryChannelResult {
        /// The name of the delivery channel. Defaults to `default`. Changing it recreates the resource.
        pub name: pulumi_wasm_rust::Output<String>,
        /// The name of the S3 bucket used to store the configuration history.
        pub s3_bucket_name: pulumi_wasm_rust::Output<String>,
        /// The prefix for the specified S3 bucket.
        pub s3_key_prefix: pulumi_wasm_rust::Output<Option<String>>,
        /// The ARN of the AWS KMS key used to encrypt objects delivered by AWS Config. Must belong to the same Region as the destination S3 bucket.
        pub s3_kms_key_arn: pulumi_wasm_rust::Output<Option<String>>,
        /// Options for how AWS Config delivers configuration snapshots. See below
        pub snapshot_delivery_properties: pulumi_wasm_rust::Output<
            Option<super::super::types::cfg::DeliveryChannelSnapshotDeliveryProperties>,
        >,
        /// The ARN of the SNS topic that AWS Config delivers notifications to.
        pub sns_topic_arn: pulumi_wasm_rust::Output<Option<String>>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(name: &str, args: DeliveryChannelArgs) -> DeliveryChannelResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let name_binding = args.name.get_inner();
        let s3_bucket_name_binding = args.s3_bucket_name.get_inner();
        let s3_key_prefix_binding = args.s3_key_prefix.get_inner();
        let s3_kms_key_arn_binding = args.s3_kms_key_arn.get_inner();
        let snapshot_delivery_properties_binding = args
            .snapshot_delivery_properties
            .get_inner();
        let sns_topic_arn_binding = args.sns_topic_arn.get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "aws:cfg/deliveryChannel:DeliveryChannel".into(),
            name: name.to_string(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "name".into(),
                    value: &name_binding,
                },
                register_interface::ObjectField {
                    name: "s3BucketName".into(),
                    value: &s3_bucket_name_binding,
                },
                register_interface::ObjectField {
                    name: "s3KeyPrefix".into(),
                    value: &s3_key_prefix_binding,
                },
                register_interface::ObjectField {
                    name: "s3KmsKeyArn".into(),
                    value: &s3_kms_key_arn_binding,
                },
                register_interface::ObjectField {
                    name: "snapshotDeliveryProperties".into(),
                    value: &snapshot_delivery_properties_binding,
                },
                register_interface::ObjectField {
                    name: "snsTopicArn".into(),
                    value: &sns_topic_arn_binding,
                },
            ]),
            results: Vec::from([
                register_interface::ResultField {
                    name: "name".into(),
                },
                register_interface::ResultField {
                    name: "s3BucketName".into(),
                },
                register_interface::ResultField {
                    name: "s3KeyPrefix".into(),
                },
                register_interface::ResultField {
                    name: "s3KmsKeyArn".into(),
                },
                register_interface::ResultField {
                    name: "snapshotDeliveryProperties".into(),
                },
                register_interface::ResultField {
                    name: "snsTopicArn".into(),
                },
            ]),
        };
        let o = register_interface::register(&request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        DeliveryChannelResult {
            name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("name").unwrap(),
            ),
            s3_bucket_name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("s3BucketName").unwrap(),
            ),
            s3_key_prefix: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("s3KeyPrefix").unwrap(),
            ),
            s3_kms_key_arn: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("s3KmsKeyArn").unwrap(),
            ),
            snapshot_delivery_properties: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("snapshotDeliveryProperties").unwrap(),
            ),
            sns_topic_arn: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("snsTopicArn").unwrap(),
            ),
        }
    }
}