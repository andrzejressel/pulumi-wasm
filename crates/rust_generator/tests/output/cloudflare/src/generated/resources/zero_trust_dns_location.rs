/// Provides a Cloudflare Teams Location resource. Teams Locations are
/// referenced when creating secure web gateway policies.
///
/// ## Example Usage
///
/// ```ignore
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let example = zero_trust_dns_location::create(
///         "example",
///         ZeroTrustDnsLocationArgs::builder()
///             .account_id("f037e56e89293a057740de681ac9abbe")
///             .client_default(true)
///             .ecs_support(false)
///             .name("office")
///             .networks(
///                 vec![
///                     ZeroTrustDnsLocationNetwork::builder().network("203.0.113.1/32")
///                     .build_struct(), ZeroTrustDnsLocationNetwork::builder()
///                     .network("203.0.113.2/32").build_struct(),
///                 ],
///             )
///             .build_struct(),
///     );
/// }
/// ```
///
/// ## Import
///
/// ```sh
/// $ pulumi import cloudflare:index/zeroTrustDnsLocation:ZeroTrustDnsLocation example <account_id>/<teams_location_id>
/// ```
///
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod zero_trust_dns_location {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct ZeroTrustDnsLocationArgs {
        /// The account identifier to target for the resource.
        #[builder(into)]
        pub account_id: pulumi_gestalt_rust::InputOrOutput<String>,
        /// Indicator that this is the default location.
        #[builder(into, default)]
        pub client_default: pulumi_gestalt_rust::InputOrOutput<Option<bool>>,
        /// Indicator that this location needs to resolve EDNS queries.
        #[builder(into, default)]
        pub ecs_support: pulumi_gestalt_rust::InputOrOutput<Option<bool>>,
        /// Name of the teams location.
        #[builder(into)]
        pub name: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The networks CIDRs that comprise the location.
        #[builder(into, default)]
        pub networks: pulumi_gestalt_rust::InputOrOutput<
            Option<Vec<super::types::ZeroTrustDnsLocationNetwork>>,
        >,
    }
    #[allow(dead_code)]
    pub struct ZeroTrustDnsLocationResult {
        /// The account identifier to target for the resource.
        pub account_id: pulumi_gestalt_rust::Output<String>,
        /// Indicator that anonymized logs are enabled.
        pub anonymized_logs_enabled: pulumi_gestalt_rust::Output<bool>,
        /// Indicator that this is the default location.
        pub client_default: pulumi_gestalt_rust::Output<Option<bool>>,
        /// The FQDN that DoH clients should be pointed at.
        pub doh_subdomain: pulumi_gestalt_rust::Output<String>,
        /// Indicator that this location needs to resolve EDNS queries.
        pub ecs_support: pulumi_gestalt_rust::Output<Option<bool>>,
        /// Client IP address.
        pub ip: pulumi_gestalt_rust::Output<String>,
        /// IP to direct all IPv4 DNS queries to.
        pub ipv4_destination: pulumi_gestalt_rust::Output<String>,
        /// Name of the teams location.
        pub name: pulumi_gestalt_rust::Output<String>,
        /// The networks CIDRs that comprise the location.
        pub networks: pulumi_gestalt_rust::Output<
            Option<Vec<super::types::ZeroTrustDnsLocationNetwork>>,
        >,
        pub policy_ids: pulumi_gestalt_rust::Output<Vec<String>>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::PulumiContext,
        name: &str,
        args: ZeroTrustDnsLocationArgs,
    ) -> ZeroTrustDnsLocationResult {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::register_interface;
        use std::collections::HashMap;
        let account_id_binding = args.account_id.get_output(context).get_inner();
        let client_default_binding = args.client_default.get_output(context).get_inner();
        let ecs_support_binding = args.ecs_support.get_output(context).get_inner();
        let name_binding = args.name.get_output(context).get_inner();
        let networks_binding = args.networks.get_output(context).get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "cloudflare:index/zeroTrustDnsLocation:ZeroTrustDnsLocation".into(),
            name: name.to_string(),
            version: super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "accountId".into(),
                    value: &account_id_binding,
                },
                register_interface::ObjectField {
                    name: "clientDefault".into(),
                    value: &client_default_binding,
                },
                register_interface::ObjectField {
                    name: "ecsSupport".into(),
                    value: &ecs_support_binding,
                },
                register_interface::ObjectField {
                    name: "name".into(),
                    value: &name_binding,
                },
                register_interface::ObjectField {
                    name: "networks".into(),
                    value: &networks_binding,
                },
            ]),
        };
        let o = register_interface::register(context.get_inner(), &request);
        ZeroTrustDnsLocationResult {
            account_id: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("accountId"),
            ),
            anonymized_logs_enabled: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("anonymizedLogsEnabled"),
            ),
            client_default: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("clientDefault"),
            ),
            doh_subdomain: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("dohSubdomain"),
            ),
            ecs_support: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("ecsSupport"),
            ),
            ip: pulumi_gestalt_rust::__private::into_domain(o.extract_field("ip")),
            ipv4_destination: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("ipv4Destination"),
            ),
            name: pulumi_gestalt_rust::__private::into_domain(o.extract_field("name")),
            networks: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("networks"),
            ),
            policy_ids: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("policyIds"),
            ),
        }
    }
}
