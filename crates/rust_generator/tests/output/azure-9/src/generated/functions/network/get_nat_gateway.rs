#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod get_nat_gateway {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GetNatGatewayArgs {
        /// Specifies the Name of the NAT Gateway.
        #[builder(into)]
        pub name: pulumi_gestalt_rust::InputOrOutput<String>,
        /// A list of existing Public IP Address resource IDs which the NAT Gateway is using.
        #[builder(into, default)]
        pub public_ip_address_ids: pulumi_gestalt_rust::InputOrOutput<
            Option<Vec<String>>,
        >,
        /// A list of existing Public IP Prefix resource IDs which the NAT Gateway is using.
        #[builder(into, default)]
        pub public_ip_prefix_ids: pulumi_gestalt_rust::InputOrOutput<
            Option<Vec<String>>,
        >,
        /// Specifies the name of the Resource Group where the NAT Gateway exists.
        #[builder(into)]
        pub resource_group_name: pulumi_gestalt_rust::InputOrOutput<String>,
    }
    #[allow(dead_code)]
    pub struct GetNatGatewayResult {
        /// The provider-assigned unique ID for this managed resource.
        pub id: pulumi_gestalt_rust::Output<String>,
        /// The idle timeout in minutes which is used for the NAT Gateway.
        pub idle_timeout_in_minutes: pulumi_gestalt_rust::Output<i32>,
        /// The location where the NAT Gateway exists.
        pub location: pulumi_gestalt_rust::Output<String>,
        pub name: pulumi_gestalt_rust::Output<String>,
        /// A list of existing Public IP Address resource IDs which the NAT Gateway is using.
        pub public_ip_address_ids: pulumi_gestalt_rust::Output<Vec<String>>,
        /// A list of existing Public IP Prefix resource IDs which the NAT Gateway is using.
        pub public_ip_prefix_ids: pulumi_gestalt_rust::Output<Vec<String>>,
        pub resource_group_name: pulumi_gestalt_rust::Output<String>,
        /// The Resource GUID of the NAT Gateway.
        pub resource_guid: pulumi_gestalt_rust::Output<String>,
        /// The SKU used by the NAT Gateway.
        pub sku_name: pulumi_gestalt_rust::Output<String>,
        /// A mapping of tags assigned to the resource.
        pub tags: pulumi_gestalt_rust::Output<std::collections::HashMap<String, String>>,
        /// A list of Availability Zones which the NAT Gateway exists in.
        pub zones: pulumi_gestalt_rust::Output<Vec<String>>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn invoke(
        context: &pulumi_gestalt_rust::Context,
        args: GetNatGatewayArgs,
    ) -> GetNatGatewayResult {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::register_interface;
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let name_binding = args.name.get_output(context);
        let public_ip_address_ids_binding = args
            .public_ip_address_ids
            .get_output(context);
        let public_ip_prefix_ids_binding = args.public_ip_prefix_ids.get_output(context);
        let resource_group_name_binding = args.resource_group_name.get_output(context);
        let request = pulumi_gestalt_rust::InvokeResourceRequest {
            token: "azure:network/getNatGateway:getNatGateway".into(),
            version: super::super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "name".into(),
                    value: name_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "publicIpAddressIds".into(),
                    value: public_ip_address_ids_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "publicIpPrefixIds".into(),
                    value: public_ip_prefix_ids_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "resourceGroupName".into(),
                    value: resource_group_name_binding.get_id(),
                },
            ],
        };
        let o = context.invoke_resource(request);
        GetNatGatewayResult {
            id: o.get_field("id"),
            idle_timeout_in_minutes: o.get_field("idleTimeoutInMinutes"),
            location: o.get_field("location"),
            name: o.get_field("name"),
            public_ip_address_ids: o.get_field("publicIpAddressIds"),
            public_ip_prefix_ids: o.get_field("publicIpPrefixIds"),
            resource_group_name: o.get_field("resourceGroupName"),
            resource_guid: o.get_field("resourceGuid"),
            sku_name: o.get_field("skuName"),
            tags: o.get_field("tags"),
            zones: o.get_field("zones"),
        }
    }
}
