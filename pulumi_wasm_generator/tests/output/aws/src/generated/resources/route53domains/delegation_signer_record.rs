/// Provides a resource to manage a [delegation signer record](https://docs.aws.amazon.com/Route53/latest/DeveloperGuide/dns-configuring-dnssec-enable-signing.html#dns-configuring-dnssec-enable-signing-step-1) in the parent DNS zone for domains registered with Route53.
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
///   exampleDelegationSignerRecord:
///     type: aws:route53domains:DelegationSignerRecord
///     name: example
///     properties:
///       domainName: example.com
///       signingAttributes:
///         algorithm: ${exampleKeySigningKey.signingAlgorithmType}
///         flags: ${exampleKeySigningKey.flag}
///         publicKey: ${exampleKeySigningKey.publicKey}
/// variables:
///   current:
///     fn::invoke:
///       Function: aws:getCallerIdentity
///       Arguments: {}
/// ```
///
/// ## Import
///
/// Using `pulumi import`, import delegation signer records using the domain name and DNSSEC key ID, separated by a comma (`,`). For example:
///
/// ```sh
/// $ pulumi import aws:route53domains/delegationSignerRecord:DelegationSignerRecord example example.com,40DE3534F5324DBDAC598ACEDB5B1E26A5368732D9C791D1347E4FBDDF6FC343
/// ```
pub mod delegation_signer_record {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct DelegationSignerRecordArgs {
        /// The name of the domain that will have its parent DNS zone updated with the Delegation Signer record.
        #[builder(into)]
        pub domain_name: pulumi_wasm_rust::Output<String>,
        /// The information about a key, including the algorithm, public key-value, and flags.
        #[builder(into, default)]
        pub signing_attributes: pulumi_wasm_rust::Output<
            Option<
                super::super::types::route53domains::DelegationSignerRecordSigningAttributes,
            >,
        >,
        #[builder(into, default)]
        pub timeouts: pulumi_wasm_rust::Output<
            Option<super::super::types::route53domains::DelegationSignerRecordTimeouts>,
        >,
    }
    #[allow(dead_code)]
    pub struct DelegationSignerRecordResult {
        /// An ID assigned to the created DS record.
        pub dnssec_key_id: pulumi_wasm_rust::Output<String>,
        /// The name of the domain that will have its parent DNS zone updated with the Delegation Signer record.
        pub domain_name: pulumi_wasm_rust::Output<String>,
        /// The information about a key, including the algorithm, public key-value, and flags.
        pub signing_attributes: pulumi_wasm_rust::Output<
            Option<
                super::super::types::route53domains::DelegationSignerRecordSigningAttributes,
            >,
        >,
        pub timeouts: pulumi_wasm_rust::Output<
            Option<super::super::types::route53domains::DelegationSignerRecordTimeouts>,
        >,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        name: &str,
        args: DelegationSignerRecordArgs,
    ) -> DelegationSignerRecordResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let domain_name_binding = args.domain_name.get_inner();
        let signing_attributes_binding = args.signing_attributes.get_inner();
        let timeouts_binding = args.timeouts.get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "aws:route53domains/delegationSignerRecord:DelegationSignerRecord"
                .into(),
            name: name.to_string(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "domainName".into(),
                    value: &domain_name_binding,
                },
                register_interface::ObjectField {
                    name: "signingAttributes".into(),
                    value: &signing_attributes_binding,
                },
                register_interface::ObjectField {
                    name: "timeouts".into(),
                    value: &timeouts_binding,
                },
            ]),
            results: Vec::from([
                register_interface::ResultField {
                    name: "dnssecKeyId".into(),
                },
                register_interface::ResultField {
                    name: "domainName".into(),
                },
                register_interface::ResultField {
                    name: "signingAttributes".into(),
                },
                register_interface::ResultField {
                    name: "timeouts".into(),
                },
            ]),
        };
        let o = register_interface::register(&request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        DelegationSignerRecordResult {
            dnssec_key_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("dnssecKeyId").unwrap(),
            ),
            domain_name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("domainName").unwrap(),
            ),
            signing_attributes: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("signingAttributes").unwrap(),
            ),
            timeouts: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("timeouts").unwrap(),
            ),
        }
    }
}