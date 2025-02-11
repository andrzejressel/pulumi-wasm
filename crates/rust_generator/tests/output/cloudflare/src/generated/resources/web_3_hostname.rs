/// Manages Web3 hostnames for IPFS and Ethereum gateways.
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod web_3_hostname {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct Web3HostnameArgs {
        /// An optional description of the hostname.
        #[builder(into, default)]
        pub description: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// DNSLink value used if the target is ipfs.
        #[builder(into, default)]
        pub dnslink: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The hostname that will point to the target gateway via CNAME.
        #[builder(into)]
        pub name: pulumi_gestalt_rust::InputOrOutput<String>,
        /// Target gateway of the hostname.
        #[builder(into)]
        pub target: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The zone identifier to target for the resource.
        #[builder(into)]
        pub zone_id: pulumi_gestalt_rust::InputOrOutput<String>,
    }
    #[allow(dead_code)]
    pub struct Web3HostnameResult {
        /// Creation time.
        pub created_on: pulumi_gestalt_rust::Output<String>,
        /// An optional description of the hostname.
        pub description: pulumi_gestalt_rust::Output<Option<String>>,
        /// DNSLink value used if the target is ipfs.
        pub dnslink: pulumi_gestalt_rust::Output<Option<String>>,
        /// Last modification time.
        pub modified_on: pulumi_gestalt_rust::Output<String>,
        /// The hostname that will point to the target gateway via CNAME.
        pub name: pulumi_gestalt_rust::Output<String>,
        /// Status of the hostname's activation.
        pub status: pulumi_gestalt_rust::Output<String>,
        /// Target gateway of the hostname.
        pub target: pulumi_gestalt_rust::Output<String>,
        /// The zone identifier to target for the resource.
        pub zone_id: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: Web3HostnameArgs,
    ) -> Web3HostnameResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let description_binding = args.description.get_output(context);
        let dnslink_binding = args.dnslink.get_output(context);
        let name_binding = args.name.get_output(context);
        let target_binding = args.target.get_output(context);
        let zone_id_binding = args.zone_id.get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "cloudflare:index/web3Hostname:Web3Hostname".into(),
            name: name.to_string(),
            version: super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "description".into(),
                    value: &description_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "dnslink".into(),
                    value: &dnslink_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "name".into(),
                    value: &name_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "target".into(),
                    value: &target_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "zoneId".into(),
                    value: &zone_id_binding.drop_type(),
                },
            ],
        };
        let o = context.register_resource(request);
        Web3HostnameResult {
            created_on: o.get_field("createdOn"),
            description: o.get_field("description"),
            dnslink: o.get_field("dnslink"),
            modified_on: o.get_field("modifiedOn"),
            name: o.get_field("name"),
            status: o.get_field("status"),
            target: o.get_field("target"),
            zone_id: o.get_field("zoneId"),
        }
    }
}
