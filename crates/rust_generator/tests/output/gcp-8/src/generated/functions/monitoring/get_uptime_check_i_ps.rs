#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod get_uptime_check_i_ps {
    #[allow(dead_code)]
    pub struct GetUptimeCheckIPsResult {
        /// The provider-assigned unique ID for this managed resource.
        pub id: pulumi_gestalt_rust::Output<String>,
        /// A list of uptime check IPs used by Stackdriver Monitoring. Each `uptime_check_ip` contains:
        pub uptime_check_ips: pulumi_gestalt_rust::Output<
            Vec<super::super::super::types::monitoring::GetUptimeCheckIPsUptimeCheckIp>,
        >,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn invoke(
        context: &pulumi_gestalt_rust::PulumiContext,
    ) -> GetUptimeCheckIPsResult {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::register_interface;
        use std::collections::HashMap;
        let request = register_interface::ResourceInvokeRequest {
            token: "gcp:monitoring/getUptimeCheckIPs:getUptimeCheckIPs".into(),
            version: super::super::super::get_version(),
            object: Vec::from([]),
        };
        let o = register_interface::invoke(context.get_inner(), &request);
        GetUptimeCheckIPsResult {
            id: pulumi_gestalt_rust::__private::into_domain(o.extract_field("id")),
            uptime_check_ips: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("uptimeCheckIps"),
            ),
        }
    }
}
