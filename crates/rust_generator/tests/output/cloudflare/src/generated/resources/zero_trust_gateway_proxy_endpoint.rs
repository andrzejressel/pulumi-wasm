/// Provides a Cloudflare Teams Proxy Endpoint resource. Teams Proxy
/// Endpoints are used for pointing proxy clients at Cloudflare Secure
/// Gateway.
///
/// ## Example Usage
///
/// ```ignore
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
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
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod zero_trust_gateway_proxy_endpoint {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct ZeroTrustGatewayProxyEndpointArgs {
        /// The account identifier to target for the resource.
        #[builder(into)]
        pub account_id: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The networks CIDRs that will be allowed to initiate proxy connections.
        #[builder(into)]
        pub ips: pulumi_gestalt_rust::InputOrOutput<Vec<String>>,
        /// Name of the teams proxy endpoint.
        #[builder(into)]
        pub name: pulumi_gestalt_rust::InputOrOutput<String>,
    }
    #[allow(dead_code)]
    pub struct ZeroTrustGatewayProxyEndpointResult {
        /// The account identifier to target for the resource.
        pub account_id: pulumi_gestalt_rust::Output<String>,
        /// The networks CIDRs that will be allowed to initiate proxy connections.
        pub ips: pulumi_gestalt_rust::Output<Vec<String>>,
        /// Name of the teams proxy endpoint.
        pub name: pulumi_gestalt_rust::Output<String>,
        /// The FQDN that proxy clients should be pointed at.
        pub subdomain: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: ZeroTrustGatewayProxyEndpointArgs,
    ) -> ZeroTrustGatewayProxyEndpointResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let account_id_binding = args.account_id.get_output(context);
        let ips_binding = args.ips.get_output(context);
        let name_binding = args.name.get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "cloudflare:index/zeroTrustGatewayProxyEndpoint:ZeroTrustGatewayProxyEndpoint"
                .into(),
            name: name.to_string(),
            version: super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "accountId".into(),
                    value: account_id_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "ips".into(),
                    value: ips_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "name".into(),
                    value: name_binding.get_id(),
                },
            ],
        };
        let o = context.register_resource(request);
        ZeroTrustGatewayProxyEndpointResult {
            account_id: o.get_field("accountId"),
            ips: o.get_field("ips"),
            name: o.get_field("name"),
            subdomain: o.get_field("subdomain"),
        }
    }
}
