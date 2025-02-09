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
///     let example = teams_proxy_endpoint::create(
///         "example",
///         TeamsProxyEndpointArgs::builder()
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
/// $ pulumi import cloudflare:index/teamsProxyEndpoint:TeamsProxyEndpoint example <account_id>/<proxy_endpoint_id>
/// ```
///
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod teams_proxy_endpoint {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct TeamsProxyEndpointArgs {
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
    pub struct TeamsProxyEndpointResult {
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
        context: &pulumi_gestalt_rust::PulumiContext,
        name: &str,
        args: TeamsProxyEndpointArgs,
    ) -> TeamsProxyEndpointResult {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::register_interface;
        use std::collections::HashMap;
        let account_id_binding_1 = args.account_id.get_output(context);
        let account_id_binding = account_id_binding_1.get_inner();
        let ips_binding_1 = args.ips.get_output(context);
        let ips_binding = ips_binding_1.get_inner();
        let name_binding_1 = args.name.get_output(context);
        let name_binding = name_binding_1.get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "cloudflare:index/teamsProxyEndpoint:TeamsProxyEndpoint".into(),
            name: name.to_string(),
            version: super::get_version(),
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
        };
        let o = register_interface::register(context.get_inner(), &request);
        TeamsProxyEndpointResult {
            account_id: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("accountId"),
            ),
            ips: pulumi_gestalt_rust::__private::into_domain(o.extract_field("ips")),
            name: pulumi_gestalt_rust::__private::into_domain(o.extract_field("name")),
            subdomain: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("subdomain"),
            ),
        }
    }
}
