#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod get_subnet {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GetSubnetArgs {
        /// Name of the resource.
        /// UserDefined subnets are named in the format of "service-n", where n ranges from 1 to 5.
        /// Management subnets have arbitary names including "vmotion", "vsan", "system-management" etc. More details about subnet names can be found on the cloud console.
        #[builder(into)]
        pub name: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The resource name of the private cloud that this subnet belongs.
        #[builder(into)]
        pub parent: pulumi_gestalt_rust::InputOrOutput<String>,
    }
    #[allow(dead_code)]
    pub struct GetSubnetResult {
        pub create_time: pulumi_gestalt_rust::Output<String>,
        pub dhcp_address_ranges: pulumi_gestalt_rust::Output<
            Vec<super::super::super::types::vmwareengine::GetSubnetDhcpAddressRange>,
        >,
        pub gateway_id: pulumi_gestalt_rust::Output<String>,
        pub gateway_ip: pulumi_gestalt_rust::Output<String>,
        /// The provider-assigned unique ID for this managed resource.
        pub id: pulumi_gestalt_rust::Output<String>,
        pub ip_cidr_range: pulumi_gestalt_rust::Output<String>,
        pub name: pulumi_gestalt_rust::Output<String>,
        pub parent: pulumi_gestalt_rust::Output<String>,
        pub standard_config: pulumi_gestalt_rust::Output<bool>,
        pub state: pulumi_gestalt_rust::Output<String>,
        pub type_: pulumi_gestalt_rust::Output<String>,
        pub uid: pulumi_gestalt_rust::Output<String>,
        pub update_time: pulumi_gestalt_rust::Output<String>,
        pub vlan_id: pulumi_gestalt_rust::Output<i32>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn invoke(
        context: &pulumi_gestalt_rust::Context,
        args: GetSubnetArgs,
    ) -> GetSubnetResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let name_binding = args.name.get_output(context);
        let parent_binding = args.parent.get_output(context);
        let request = pulumi_gestalt_rust::InvokeResourceRequest {
            token: "gcp:vmwareengine/getSubnet:getSubnet".into(),
            version: super::super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "name".into(),
                    value: &name_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "parent".into(),
                    value: &parent_binding.drop_type(),
                },
            ],
        };
        let o = context.invoke_resource(request);
        GetSubnetResult {
            create_time: o.get_field("createTime"),
            dhcp_address_ranges: o.get_field("dhcpAddressRanges"),
            gateway_id: o.get_field("gatewayId"),
            gateway_ip: o.get_field("gatewayIp"),
            id: o.get_field("id"),
            ip_cidr_range: o.get_field("ipCidrRange"),
            name: o.get_field("name"),
            parent: o.get_field("parent"),
            standard_config: o.get_field("standardConfig"),
            state: o.get_field("state"),
            type_: o.get_field("type"),
            uid: o.get_field("uid"),
            update_time: o.get_field("updateTime"),
            vlan_id: o.get_field("vlanId"),
        }
    }
}
