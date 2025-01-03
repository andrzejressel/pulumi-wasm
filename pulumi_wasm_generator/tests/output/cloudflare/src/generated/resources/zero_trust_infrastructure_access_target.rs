/// The [Infrastructure Access Target](https://developers.cloudflare.com/cloudflare-one/connections/connect-networks/use-cases/ssh/ssh-infrastructure-access/#4-add-a-target) resource allows you to configure Infrastructure Access Targets for an account.
///
/// ## Example Usage
///
/// ```ignore
/// use pulumi_wasm_rust::Output;
/// use pulumi_wasm_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let example = zero_trust_infrastructure_access_target::create(
///         "example",
///         ZeroTrustInfrastructureAccessTargetArgs::builder()
///             .account_id("f037e56e89293a057740de681ac9abbe")
///             .hostname("example-target")
///             .ip(
///                 ZeroTrustInfrastructureAccessTargetIp::builder()
///                     .ipv4(
///                         ZeroTrustInfrastructureAccessTargetIpIpv4::builder()
///                             .ipAddr("198.51.100.1")
///                             .virtualNetworkId("238dccd1-149b-463d-8228-560ab83a54fd")
///                             .build_struct(),
///                     )
///                     .ipv6(
///                         ZeroTrustInfrastructureAccessTargetIpIpv6::builder()
///                             .ipAddr("2001:db8::")
///                             .virtualNetworkId("238dccd1-149b-463d-8228-560ab83a54fd")
///                             .build_struct(),
///                     )
///                     .build_struct(),
///             )
///             .build_struct(),
///     );
///     let ipv4OnlyExample = zero_trust_infrastructure_access_target::create(
///         "ipv4OnlyExample",
///         ZeroTrustInfrastructureAccessTargetArgs::builder()
///             .account_id("f037e56e89293a057740de681ac9abbe")
///             .hostname("example-ipv4-only")
///             .ip(
///                 ZeroTrustInfrastructureAccessTargetIp::builder()
///                     .ipv4(
///                         ZeroTrustInfrastructureAccessTargetIpIpv4::builder()
///                             .ipAddr("198.51.100.1")
///                             .virtualNetworkId("238dccd1-149b-463d-8228-560ab83a54fd")
///                             .build_struct(),
///                     )
///                     .build_struct(),
///             )
///             .build_struct(),
///     );
/// }
/// ```
///
/// ## Import
///
/// ```sh
/// $ pulumi import cloudflare:index/zeroTrustInfrastructureAccessTarget:ZeroTrustInfrastructureAccessTarget example <account_id>
/// ```
///
pub mod zero_trust_infrastructure_access_target {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct ZeroTrustInfrastructureAccessTargetArgs {
        /// The account identifier to target for the resource.
        #[builder(into)]
        pub account_id: pulumi_wasm_rust::Output<String>,
        /// A non-unique field that refers to a target.
        #[builder(into)]
        pub hostname: pulumi_wasm_rust::Output<String>,
        /// The IPv4/IPv6 address that identifies where to reach a target.
        #[builder(into)]
        pub ip: pulumi_wasm_rust::Output<
            super::types::ZeroTrustInfrastructureAccessTargetIp,
        >,
    }
    #[allow(dead_code)]
    pub struct ZeroTrustInfrastructureAccessTargetResult {
        /// The account identifier to target for the resource.
        pub account_id: pulumi_wasm_rust::Output<String>,
        /// The date and time at which the target was created.
        pub created_at: pulumi_wasm_rust::Output<String>,
        /// A non-unique field that refers to a target.
        pub hostname: pulumi_wasm_rust::Output<String>,
        /// The IPv4/IPv6 address that identifies where to reach a target.
        pub ip: pulumi_wasm_rust::Output<
            super::types::ZeroTrustInfrastructureAccessTargetIp,
        >,
        /// The date and time at which the target was last modified.
        pub modified_at: pulumi_wasm_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        name: &str,
        args: ZeroTrustInfrastructureAccessTargetArgs,
    ) -> ZeroTrustInfrastructureAccessTargetResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let account_id_binding = args.account_id.get_inner();
        let hostname_binding = args.hostname.get_inner();
        let ip_binding = args.ip.get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "cloudflare:index/zeroTrustInfrastructureAccessTarget:ZeroTrustInfrastructureAccessTarget"
                .into(),
            name: name.to_string(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "accountId".into(),
                    value: &account_id_binding,
                },
                register_interface::ObjectField {
                    name: "hostname".into(),
                    value: &hostname_binding,
                },
                register_interface::ObjectField {
                    name: "ip".into(),
                    value: &ip_binding,
                },
            ]),
            results: Vec::from([
                register_interface::ResultField {
                    name: "accountId".into(),
                },
                register_interface::ResultField {
                    name: "createdAt".into(),
                },
                register_interface::ResultField {
                    name: "hostname".into(),
                },
                register_interface::ResultField {
                    name: "ip".into(),
                },
                register_interface::ResultField {
                    name: "modifiedAt".into(),
                },
            ]),
        };
        let o = register_interface::register(&request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        ZeroTrustInfrastructureAccessTargetResult {
            account_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("accountId").unwrap(),
            ),
            created_at: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("createdAt").unwrap(),
            ),
            hostname: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("hostname").unwrap(),
            ),
            ip: pulumi_wasm_rust::__private::into_domain(hashmap.remove("ip").unwrap()),
            modified_at: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("modifiedAt").unwrap(),
            ),
        }
    }
}
