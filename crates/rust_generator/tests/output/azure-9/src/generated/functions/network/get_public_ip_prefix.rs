#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod get_public_ip_prefix {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GetPublicIpPrefixArgs {
        /// Specifies the name of the public IP prefix.
        #[builder(into)]
        pub name: pulumi_gestalt_rust::InputOrOutput<String>,
        /// Specifies the name of the resource group.
        #[builder(into)]
        pub resource_group_name: pulumi_gestalt_rust::InputOrOutput<String>,
    }
    #[allow(dead_code)]
    pub struct GetPublicIpPrefixResult {
        /// The provider-assigned unique ID for this managed resource.
        pub id: pulumi_gestalt_rust::Output<String>,
        /// The Public IP address range, in CIDR notation.
        pub ip_prefix: pulumi_gestalt_rust::Output<String>,
        /// The supported Azure location where the resource exists.
        pub location: pulumi_gestalt_rust::Output<String>,
        pub name: pulumi_gestalt_rust::Output<String>,
        /// The number of bits of the prefix.
        pub prefix_length: pulumi_gestalt_rust::Output<i32>,
        pub resource_group_name: pulumi_gestalt_rust::Output<String>,
        /// The SKU of the Public IP Prefix.
        pub sku: pulumi_gestalt_rust::Output<String>,
        /// The SKU Tier of the Public IP.
        pub sku_tier: pulumi_gestalt_rust::Output<String>,
        /// A mapping of tags to assigned to the resource.
        pub tags: pulumi_gestalt_rust::Output<std::collections::HashMap<String, String>>,
        /// A list of Availability Zones in which this Public IP Prefix is located.
        pub zones: pulumi_gestalt_rust::Output<Vec<String>>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn invoke(
        context: &pulumi_gestalt_rust::Context,
        args: GetPublicIpPrefixArgs,
    ) -> GetPublicIpPrefixResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let name_binding = args.name.get_output(context);
        let resource_group_name_binding = args.resource_group_name.get_output(context);
        let request = pulumi_gestalt_rust::InvokeResourceRequest {
            token: "azure:network/getPublicIpPrefix:getPublicIpPrefix".into(),
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
        GetPublicIpPrefixResult {
            id: o.get_field("id"),
            ip_prefix: o.get_field("ipPrefix"),
            location: o.get_field("location"),
            name: o.get_field("name"),
            prefix_length: o.get_field("prefixLength"),
            resource_group_name: o.get_field("resourceGroupName"),
            sku: o.get_field("sku"),
            sku_tier: o.get_field("skuTier"),
            tags: o.get_field("tags"),
            zones: o.get_field("zones"),
        }
    }
}
