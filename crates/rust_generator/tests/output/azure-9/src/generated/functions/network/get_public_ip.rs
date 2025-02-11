#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod get_public_ip {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GetPublicIpArgs {
        /// Specifies the name of the public IP address.
        #[builder(into)]
        pub name: pulumi_gestalt_rust::InputOrOutput<String>,
        /// Specifies the name of the resource group.
        #[builder(into)]
        pub resource_group_name: pulumi_gestalt_rust::InputOrOutput<String>,
    }
    #[allow(dead_code)]
    pub struct GetPublicIpResult {
        /// The allocation method for this IP address. Possible values are `Static` or `Dynamic`.
        pub allocation_method: pulumi_gestalt_rust::Output<String>,
        /// The DDoS protection mode of the public IP.
        pub ddos_protection_mode: pulumi_gestalt_rust::Output<String>,
        /// The ID of DDoS protection plan associated with the public IP.
        pub ddos_protection_plan_id: pulumi_gestalt_rust::Output<String>,
        /// The label for the Domain Name.
        pub domain_name_label: pulumi_gestalt_rust::Output<String>,
        /// Fully qualified domain name of the A DNS record associated with the public IP. This is the concatenation of the domainNameLabel and the regionalized DNS zone.
        pub fqdn: pulumi_gestalt_rust::Output<String>,
        /// The provider-assigned unique ID for this managed resource.
        pub id: pulumi_gestalt_rust::Output<String>,
        /// Specifies the timeout for the TCP idle connection.
        pub idle_timeout_in_minutes: pulumi_gestalt_rust::Output<i32>,
        /// The IP address value that was allocated.
        pub ip_address: pulumi_gestalt_rust::Output<String>,
        /// A mapping of tags to assigned to the resource.
        pub ip_tags: pulumi_gestalt_rust::Output<
            std::collections::HashMap<String, String>,
        >,
        /// The IP version being used, for example `IPv4` or `IPv6`.
        pub ip_version: pulumi_gestalt_rust::Output<String>,
        /// The region that this public ip exists.
        pub location: pulumi_gestalt_rust::Output<String>,
        pub name: pulumi_gestalt_rust::Output<String>,
        pub resource_group_name: pulumi_gestalt_rust::Output<String>,
        /// The fully qualified domain name that resolves to this public IP address.
        pub reverse_fqdn: pulumi_gestalt_rust::Output<String>,
        /// The SKU of the Public IP.
        pub sku: pulumi_gestalt_rust::Output<String>,
        /// A mapping of tags to assigned to the resource.
        pub tags: pulumi_gestalt_rust::Output<std::collections::HashMap<String, String>>,
        /// A list of Availability Zones in which this Public IP is located.
        pub zones: pulumi_gestalt_rust::Output<Vec<String>>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn invoke(
        context: &pulumi_gestalt_rust::Context,
        args: GetPublicIpArgs,
    ) -> GetPublicIpResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let name_binding = args.name.get_output(context);
        let resource_group_name_binding = args.resource_group_name.get_output(context);
        let request = pulumi_gestalt_rust::InvokeResourceRequest {
            token: "azure:network/getPublicIP:getPublicIP".into(),
            version: super::super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "name".into(),
                    value: &name_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "resourceGroupName".into(),
                    value: &resource_group_name_binding.drop_type(),
                },
            ],
        };
        let o = context.invoke_resource(request);
        GetPublicIpResult {
            allocation_method: o.get_field("allocationMethod"),
            ddos_protection_mode: o.get_field("ddosProtectionMode"),
            ddos_protection_plan_id: o.get_field("ddosProtectionPlanId"),
            domain_name_label: o.get_field("domainNameLabel"),
            fqdn: o.get_field("fqdn"),
            id: o.get_field("id"),
            idle_timeout_in_minutes: o.get_field("idleTimeoutInMinutes"),
            ip_address: o.get_field("ipAddress"),
            ip_tags: o.get_field("ipTags"),
            ip_version: o.get_field("ipVersion"),
            location: o.get_field("location"),
            name: o.get_field("name"),
            resource_group_name: o.get_field("resourceGroupName"),
            reverse_fqdn: o.get_field("reverseFqdn"),
            sku: o.get_field("sku"),
            tags: o.get_field("tags"),
            zones: o.get_field("zones"),
        }
    }
}
