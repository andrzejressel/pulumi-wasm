/// Manages a Route 53 Key Signing Key. To manage Domain Name System Security Extensions (DNSSEC) for a Hosted Zone, see the `aws.route53.HostedZoneDnsSec` resource. For more information about managing DNSSEC in Route 53, see the [Route 53 Developer Guide](https://docs.aws.amazon.com/Route53/latest/DeveloperGuide/dns-configuring-dnssec.html).
///
/// ## Example Usage
///
/// ```yaml
/// resources:
///   example:
///     type: aws:kms:Key
///     properties:
///       customerMasterKeySpec: ECC_NIST_P256
///       deletionWindowInDays: 7
///       keyUsage: SIGN_VERIFY
///       policy:
///         fn::toJSON:
///           Statement:
///             - Action:
///                 - kms:DescribeKey
///                 - kms:GetPublicKey
///                 - kms:Sign
///               Effect: Allow
///               Principal:
///                 Service: dnssec-route53.amazonaws.com
///               Sid: Allow Route 53 DNSSEC Service
///               Resource: '*'
///               Condition:
///                 StringEquals:
///                   aws:SourceAccount: ${current.accountId}
///                 ArnLike:
///                   aws:SourceArn: arn:aws:route53:::hostedzone/*
///             - Action: kms:CreateGrant
///               Effect: Allow
///               Principal:
///                 Service: dnssec-route53.amazonaws.com
///               Sid: Allow Route 53 DNSSEC Service to CreateGrant
///               Resource: '*'
///               Condition:
///                 Bool:
///                   kms:GrantIsForAWSResource: 'true'
///             - Action: kms:*
///               Effect: Allow
///               Principal:
///                 AWS: arn:aws:iam::${current.accountId}:root
///               Resource: '*'
///               Sid: Enable IAM User Permissions
///           Version: 2012-10-17
///   exampleZone:
///     type: aws:route53:Zone
///     name: example
///     properties:
///       name: example.com
///   exampleKeySigningKey:
///     type: aws:route53:KeySigningKey
///     name: example
///     properties:
///       hostedZoneId: ${test.id}
///       keyManagementServiceArn: ${testAwsKmsKey.arn}
///       name: example
///   exampleHostedZoneDnsSec:
///     type: aws:route53:HostedZoneDnsSec
///     name: example
///     properties:
///       hostedZoneId: ${exampleKeySigningKey.hostedZoneId}
///     options:
///       dependson:
///         - ${exampleKeySigningKey}
/// variables:
///   current:
///     fn::invoke:
///       Function: aws:getCallerIdentity
///       Arguments: {}
/// ```
///
/// ## Import
///
/// Using `pulumi import`, import `aws_route53_key_signing_key` resources using the Route 53 Hosted Zone identifier and KMS Key identifier, separated by a comma (`,`). For example:
///
/// ```sh
/// $ pulumi import aws:route53/keySigningKey:KeySigningKey example Z1D633PJN98FT9,example
/// ```
pub mod key_signing_key {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct KeySigningKeyArgs {
        /// Identifier of the Route 53 Hosted Zone.
        #[builder(into)]
        pub hosted_zone_id: pulumi_wasm_rust::Output<String>,
        /// Amazon Resource Name (ARN) of the Key Management Service (KMS) Key. This must be unique for each key-signing key (KSK) in a single hosted zone. This key must be in the `us-east-1` Region and meet certain requirements, which are described in the [Route 53 Developer Guide](https://docs.aws.amazon.com/Route53/latest/DeveloperGuide/dns-configuring-dnssec-cmk-requirements.html) and [Route 53 API Reference](https://docs.aws.amazon.com/Route53/latest/APIReference/API_CreateKeySigningKey.html).
        #[builder(into)]
        pub key_management_service_arn: pulumi_wasm_rust::Output<String>,
        /// Name of the key-signing key (KSK). Must be unique for each key-singing key in the same hosted zone.
        ///
        /// The following arguments are optional:
        #[builder(into, default)]
        pub name: pulumi_wasm_rust::Output<Option<String>>,
        /// Status of the key-signing key (KSK). Valid values: `ACTIVE`, `INACTIVE`. Defaults to `ACTIVE`.
        #[builder(into, default)]
        pub status: pulumi_wasm_rust::Output<Option<String>>,
    }
    #[allow(dead_code)]
    pub struct KeySigningKeyResult {
        /// A string used to represent the delegation signer digest algorithm. This value must follow the guidelines provided by [RFC-8624 Section 3.3](https://tools.ietf.org/html/rfc8624#section-3.3).
        pub digest_algorithm_mnemonic: pulumi_wasm_rust::Output<String>,
        /// An integer used to represent the delegation signer digest algorithm. This value must follow the guidelines provided by [RFC-8624 Section 3.3](https://tools.ietf.org/html/rfc8624#section-3.3).
        pub digest_algorithm_type: pulumi_wasm_rust::Output<i32>,
        /// A cryptographic digest of a DNSKEY resource record (RR). DNSKEY records are used to publish the public key that resolvers can use to verify DNSSEC signatures that are used to secure certain kinds of information provided by the DNS system.
        pub digest_value: pulumi_wasm_rust::Output<String>,
        /// A string that represents a DNSKEY record.
        pub dnskey_record: pulumi_wasm_rust::Output<String>,
        /// A string that represents a delegation signer (DS) record.
        pub ds_record: pulumi_wasm_rust::Output<String>,
        /// An integer that specifies how the key is used. For key-signing key (KSK), this value is always 257.
        pub flag: pulumi_wasm_rust::Output<i32>,
        /// Identifier of the Route 53 Hosted Zone.
        pub hosted_zone_id: pulumi_wasm_rust::Output<String>,
        /// Amazon Resource Name (ARN) of the Key Management Service (KMS) Key. This must be unique for each key-signing key (KSK) in a single hosted zone. This key must be in the `us-east-1` Region and meet certain requirements, which are described in the [Route 53 Developer Guide](https://docs.aws.amazon.com/Route53/latest/DeveloperGuide/dns-configuring-dnssec-cmk-requirements.html) and [Route 53 API Reference](https://docs.aws.amazon.com/Route53/latest/APIReference/API_CreateKeySigningKey.html).
        pub key_management_service_arn: pulumi_wasm_rust::Output<String>,
        /// An integer used to identify the DNSSEC record for the domain name. The process used to calculate the value is described in [RFC-4034 Appendix B](https://tools.ietf.org/rfc/rfc4034.txt).
        pub key_tag: pulumi_wasm_rust::Output<i32>,
        /// Name of the key-signing key (KSK). Must be unique for each key-singing key in the same hosted zone.
        ///
        /// The following arguments are optional:
        pub name: pulumi_wasm_rust::Output<String>,
        /// The public key, represented as a Base64 encoding, as required by [RFC-4034 Page 5](https://tools.ietf.org/rfc/rfc4034.txt).
        pub public_key: pulumi_wasm_rust::Output<String>,
        /// A string used to represent the signing algorithm. This value must follow the guidelines provided by [RFC-8624 Section 3.1](https://tools.ietf.org/html/rfc8624#section-3.1).
        pub signing_algorithm_mnemonic: pulumi_wasm_rust::Output<String>,
        /// An integer used to represent the signing algorithm. This value must follow the guidelines provided by [RFC-8624 Section 3.1](https://tools.ietf.org/html/rfc8624#section-3.1).
        pub signing_algorithm_type: pulumi_wasm_rust::Output<i32>,
        /// Status of the key-signing key (KSK). Valid values: `ACTIVE`, `INACTIVE`. Defaults to `ACTIVE`.
        pub status: pulumi_wasm_rust::Output<Option<String>>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(name: &str, args: KeySigningKeyArgs) -> KeySigningKeyResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let hosted_zone_id_binding = args.hosted_zone_id.get_inner();
        let key_management_service_arn_binding = args
            .key_management_service_arn
            .get_inner();
        let name_binding = args.name.get_inner();
        let status_binding = args.status.get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "aws:route53/keySigningKey:KeySigningKey".into(),
            name: name.to_string(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "hostedZoneId".into(),
                    value: &hosted_zone_id_binding,
                },
                register_interface::ObjectField {
                    name: "keyManagementServiceArn".into(),
                    value: &key_management_service_arn_binding,
                },
                register_interface::ObjectField {
                    name: "name".into(),
                    value: &name_binding,
                },
                register_interface::ObjectField {
                    name: "status".into(),
                    value: &status_binding,
                },
            ]),
            results: Vec::from([
                register_interface::ResultField {
                    name: "digestAlgorithmMnemonic".into(),
                },
                register_interface::ResultField {
                    name: "digestAlgorithmType".into(),
                },
                register_interface::ResultField {
                    name: "digestValue".into(),
                },
                register_interface::ResultField {
                    name: "dnskeyRecord".into(),
                },
                register_interface::ResultField {
                    name: "dsRecord".into(),
                },
                register_interface::ResultField {
                    name: "flag".into(),
                },
                register_interface::ResultField {
                    name: "hostedZoneId".into(),
                },
                register_interface::ResultField {
                    name: "keyManagementServiceArn".into(),
                },
                register_interface::ResultField {
                    name: "keyTag".into(),
                },
                register_interface::ResultField {
                    name: "name".into(),
                },
                register_interface::ResultField {
                    name: "publicKey".into(),
                },
                register_interface::ResultField {
                    name: "signingAlgorithmMnemonic".into(),
                },
                register_interface::ResultField {
                    name: "signingAlgorithmType".into(),
                },
                register_interface::ResultField {
                    name: "status".into(),
                },
            ]),
        };
        let o = register_interface::register(&request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        KeySigningKeyResult {
            digest_algorithm_mnemonic: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("digestAlgorithmMnemonic").unwrap(),
            ),
            digest_algorithm_type: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("digestAlgorithmType").unwrap(),
            ),
            digest_value: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("digestValue").unwrap(),
            ),
            dnskey_record: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("dnskeyRecord").unwrap(),
            ),
            ds_record: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("dsRecord").unwrap(),
            ),
            flag: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("flag").unwrap(),
            ),
            hosted_zone_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("hostedZoneId").unwrap(),
            ),
            key_management_service_arn: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("keyManagementServiceArn").unwrap(),
            ),
            key_tag: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("keyTag").unwrap(),
            ),
            name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("name").unwrap(),
            ),
            public_key: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("publicKey").unwrap(),
            ),
            signing_algorithm_mnemonic: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("signingAlgorithmMnemonic").unwrap(),
            ),
            signing_algorithm_type: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("signingAlgorithmType").unwrap(),
            ),
            status: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("status").unwrap(),
            ),
        }
    }
}
