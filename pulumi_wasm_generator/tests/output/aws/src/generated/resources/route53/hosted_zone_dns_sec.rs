/// Manages Route 53 Hosted Zone Domain Name System Security Extensions (DNSSEC). For more information about managing DNSSEC in Route 53, see the [Route 53 Developer Guide](https://docs.aws.amazon.com/Route53/latest/DeveloperGuide/dns-configuring-dnssec.html).
///
/// !> **WARNING:** If you disable DNSSEC signing for your hosted zone before the DNS changes have propagated, your domain could become unavailable on the internet. When you remove the DS records, you must wait until the longest TTL for the DS records that you remove has expired before you complete the step to disable DNSSEC signing. Please refer to the [Route 53 Developer Guide - Disable DNSSEC](https://docs.aws.amazon.com/Route53/latest/DeveloperGuide/dns-configuring-dnssec-disable.html) for a detailed breakdown on the steps required to disable DNSSEC safely for a hosted zone.
///
/// > **Note:** Route53 hosted zones are global resources, and as such any `aws.kms.Key` that you use as part of a signing key needs to be located in the `us-east-1` region. In the example below, the main AWS provider declaration is for `us-east-1`, however if you are provisioning your AWS resources in a different region, you will need to specify a provider alias and use that attached to the `aws.kms.Key` resource as described in the provider alias documentation.
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
///                 - kms:Verify
///               Effect: Allow
///               Principal:
///                 Service: dnssec-route53.amazonaws.com
///               Resource: '*'
///               Sid: Allow Route 53 DNSSEC Service
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
///       hostedZoneId: ${exampleZone.id}
///       keyManagementServiceArn: ${example.arn}
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
/// Using `pulumi import`, import `aws_route53_hosted_zone_dnssec` resources using the Route 53 Hosted Zone identifier. For example:
///
/// ```sh
/// $ pulumi import aws:route53/hostedZoneDnsSec:HostedZoneDnsSec example Z1D633PJN98FT9
/// ```
pub mod hosted_zone_dns_sec {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct HostedZoneDnsSecArgs {
        /// Identifier of the Route 53 Hosted Zone.
        ///
        /// The following arguments are optional:
        #[builder(into)]
        pub hosted_zone_id: pulumi_wasm_rust::Output<String>,
        /// Hosted Zone signing status. Valid values: `SIGNING`, `NOT_SIGNING`. Defaults to `SIGNING`.
        #[builder(into, default)]
        pub signing_status: pulumi_wasm_rust::Output<Option<String>>,
    }
    #[allow(dead_code)]
    pub struct HostedZoneDnsSecResult {
        /// Identifier of the Route 53 Hosted Zone.
        ///
        /// The following arguments are optional:
        pub hosted_zone_id: pulumi_wasm_rust::Output<String>,
        /// Hosted Zone signing status. Valid values: `SIGNING`, `NOT_SIGNING`. Defaults to `SIGNING`.
        pub signing_status: pulumi_wasm_rust::Output<Option<String>>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(name: &str, args: HostedZoneDnsSecArgs) -> HostedZoneDnsSecResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let hosted_zone_id_binding = args.hosted_zone_id.get_inner();
        let signing_status_binding = args.signing_status.get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "aws:route53/hostedZoneDnsSec:HostedZoneDnsSec".into(),
            name: name.to_string(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "hostedZoneId".into(),
                    value: &hosted_zone_id_binding,
                },
                register_interface::ObjectField {
                    name: "signingStatus".into(),
                    value: &signing_status_binding,
                },
            ]),
            results: Vec::from([
                register_interface::ResultField {
                    name: "hostedZoneId".into(),
                },
                register_interface::ResultField {
                    name: "signingStatus".into(),
                },
            ]),
        };
        let o = register_interface::register(&request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        HostedZoneDnsSecResult {
            hosted_zone_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("hostedZoneId").unwrap(),
            ),
            signing_status: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("signingStatus").unwrap(),
            ),
        }
    }
}