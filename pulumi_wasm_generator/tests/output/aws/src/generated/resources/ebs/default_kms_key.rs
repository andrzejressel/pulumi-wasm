/// Provides a resource to manage the default customer master key (CMK) that your AWS account uses to encrypt EBS volumes.
///
/// Your AWS account has an AWS-managed default CMK that is used for encrypting an EBS volume when no CMK is specified in the API call that creates the volume.
/// By using the `aws.ebs.DefaultKmsKey` resource, you can specify a customer-managed CMK to use in place of the AWS-managed default CMK.
///
/// > **NOTE:** Creating an `aws.ebs.DefaultKmsKey` resource does not enable default EBS encryption. Use the `aws.ebs.EncryptionByDefault` to enable default EBS encryption.
///
/// > **NOTE:** Destroying this resource will reset the default CMK to the account's AWS-managed default CMK for EBS.
///
/// ## Example Usage
///
/// ```ignore
/// use pulumi_wasm_rust::Output;
/// use pulumi_wasm_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let example = default_kms_key::create(
///         "example",
///         DefaultKmsKeyArgs::builder().key_arn("${exampleAwsKmsKey.arn}").build_struct(),
///     );
/// }
/// ```
///
/// ## Import
///
/// Using `pulumi import`, import the EBS default KMS CMK using the KMS key ARN. For example:
///
/// ```sh
/// $ pulumi import aws:ebs/defaultKmsKey:DefaultKmsKey example arn:aws:kms:us-east-1:123456789012:key/abcd-1234
/// ```
pub mod default_kms_key {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct DefaultKmsKeyArgs {
        /// The ARN of the AWS Key Management Service (AWS KMS) customer master key (CMK) to use to encrypt the EBS volume.
        #[builder(into)]
        pub key_arn: pulumi_wasm_rust::Output<String>,
    }
    #[allow(dead_code)]
    pub struct DefaultKmsKeyResult {
        /// The ARN of the AWS Key Management Service (AWS KMS) customer master key (CMK) to use to encrypt the EBS volume.
        pub key_arn: pulumi_wasm_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(name: &str, args: DefaultKmsKeyArgs) -> DefaultKmsKeyResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let key_arn_binding = args.key_arn.get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "aws:ebs/defaultKmsKey:DefaultKmsKey".into(),
            name: name.to_string(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "keyArn".into(),
                    value: &key_arn_binding,
                },
            ]),
            results: Vec::from([
                register_interface::ResultField {
                    name: "keyArn".into(),
                },
            ]),
        };
        let o = register_interface::register(&request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        DefaultKmsKeyResult {
            key_arn: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("keyArn").unwrap(),
            ),
        }
    }
}