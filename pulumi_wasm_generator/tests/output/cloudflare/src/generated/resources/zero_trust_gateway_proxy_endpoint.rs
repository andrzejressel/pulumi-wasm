/// Provides a Cloudflare Teams Proxy Endpoint resource. Teams Proxy
/// Endpoints are used for pointing proxy clients at Cloudflare Secure
/// Gateway.
///
/// ## Example Usage
///
/// ```ignore
/// use pulumi_wasm_rust::Output;
/// use pulumi_wasm_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let example = zero_trust_gateway_proxy_endpoint::create(
///         "example",
///         ZeroTrustGatewayProxyEndpointArgs::builder()
///             .account_id("f037e56e89293a057740de681ac9abbe")
///             .ips(vec!["192.0.2.0/24",])
///             .name("office")
///             .build_struct(),
///     );
/// }
/// ```
///
/// ## Import
///
/// ```sh
/// $ pulumi import cloudflare:index/zeroTrustGatewayProxyEndpoint:ZeroTrustGatewayProxyEndpoint example <account_id>/<proxy_endpoint_id>
/// ```
///
pub mod zero_trust_gateway_proxy_endpoint {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct ZeroTrustGatewayProxyEndpointArgs {
        /// The account identifier to target for the resource.
        #[builder(into)]
        pub account_id: pulumi_wasm_rust::Output<String>,
        /// The networks CIDRs that will be allowed to initiate proxy connections.
        #[builder(into)]
        pub ips: pulumi_wasm_rust::Output<Vec<String>>,
        /// Name of the teams proxy endpoint.
        #[builder(into)]
        pub name: pulumi_wasm_rust::Output<String>,
    }
    #[allow(dead_code)]
    pub struct ZeroTrustGatewayProxyEndpointResult {
        /// The account identifier to target for the resource.
        pub account_id: pulumi_wasm_rust::Output<String>,
        /// The networks CIDRs that will be allowed to initiate proxy connections.
        pub ips: pulumi_wasm_rust::Output<Vec<String>>,
        /// Name of the teams proxy endpoint.
        pub name: pulumi_wasm_rust::Output<String>,
        /// The FQDN that proxy clients should be pointed at.
        pub subdomain: pulumi_wasm_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        name: &str,
        args: ZeroTrustGatewayProxyEndpointArgs,
    ) -> ZeroTrustGatewayProxyEndpointResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let account_id_binding = args.account_id.get_inner();
        let ips_binding = args.ips.get_inner();
        let name_binding = args.name.get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "cloudflare:index/zeroTrustGatewayProxyEndpoint:ZeroTrustGatewayProxyEndpoint"
                .into(),
            name: name.to_string(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "accountId".into(),
                    value: &account_id_binding,
                },
                register_interface::ObjectField {
                    name: "ips".into(),
                    value: &ips_binding,
                },
                register_interface::ObjectField {
                    name: "name".into(),
                    value: &name_binding,
                },
            ]),
            results: Vec::from([
                register_interface::ResultField {
                    name: "accountId".into(),
                },
                register_interface::ResultField {
                    name: "ips".into(),
                },
                register_interface::ResultField {
                    name: "name".into(),
                },
                register_interface::ResultField {
                    name: "subdomain".into(),
                },
            ]),
        };
        let o = register_interface::register(&request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        ZeroTrustGatewayProxyEndpointResult {
            account_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("accountId").unwrap(),
            ),
            ips: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("ips").unwrap(),
            ),
            name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("name").unwrap(),
            ),
            subdomain: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("subdomain").unwrap(),
            ),
        }
    }
}
