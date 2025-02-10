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
    pub fn invoke(context: &pulumi_gestalt_rust::Context) -> GetUptimeCheckIPsResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let request = pulumi_gestalt_rust::InvokeResourceRequest {
            token: "gcp:monitoring/getUptimeCheckIPs:getUptimeCheckIPs".into(),
            version: super::super::super::get_version(),
            object: &[],
        };
        let o = context.invoke_resource(request);
        GetUptimeCheckIPsResult {
            id: o.get_field("id"),
            uptime_check_ips: o.get_field("uptimeCheckIps"),
        }
    }
}
