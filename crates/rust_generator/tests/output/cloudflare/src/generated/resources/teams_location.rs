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
///     let example = teams_location::create(
///         "example",
///         TeamsLocationArgs::builder()
///             .account_id("f037e56e89293a057740de681ac9abbe")
///             .client_default(true)
///             .ecs_support(false)
///             .name("office")
///             .networks(
///                 vec![
///                     TeamsLocationNetwork::builder().network("203.0.113.1/32")
///                     .build_struct(), TeamsLocationNetwork::builder()
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
/// $ pulumi import cloudflare:index/teamsLocation:TeamsLocation example <account_id>/<teams_location_id>
/// ```
///
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod teams_location {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct TeamsLocationArgs {
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
            Option<Vec<super::types::TeamsLocationNetwork>>,
        >,
    }
    #[allow(dead_code)]
    pub struct TeamsLocationResult {
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
            Option<Vec<super::types::TeamsLocationNetwork>>,
        >,
        pub policy_ids: pulumi_gestalt_rust::Output<Vec<String>>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: TeamsLocationArgs,
    ) -> TeamsLocationResult {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::register_interface;
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let account_id_binding = args.account_id.get_output(context);
        let client_default_binding = args.client_default.get_output(context);
        let ecs_support_binding = args.ecs_support.get_output(context);
        let name_binding = args.name.get_output(context);
        let networks_binding = args.networks.get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "cloudflare:index/teamsLocation:TeamsLocation".into(),
            name: name.to_string(),
            version: super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "accountId".into(),
                    value: account_id_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "clientDefault".into(),
                    value: client_default_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "ecsSupport".into(),
                    value: ecs_support_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "name".into(),
                    value: name_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "networks".into(),
                    value: networks_binding.get_id(),
                },
            ],
        };
        let o = context.register_resource(request);
        TeamsLocationResult {
            account_id: o.get_field("accountId"),
            anonymized_logs_enabled: o.get_field("anonymizedLogsEnabled"),
            client_default: o.get_field("clientDefault"),
            doh_subdomain: o.get_field("dohSubdomain"),
            ecs_support: o.get_field("ecsSupport"),
            ip: o.get_field("ip"),
            ipv4_destination: o.get_field("ipv4Destination"),
            name: o.get_field("name"),
            networks: o.get_field("networks"),
            policy_ids: o.get_field("policyIds"),
        }
    }
}
