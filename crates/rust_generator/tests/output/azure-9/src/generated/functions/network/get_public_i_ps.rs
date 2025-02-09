#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod get_public_i_ps {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GetPublicIPsArgs {
        /// The Allocation Type for the Public IP Address. Possible values include `Static` or `Dynamic`.
        #[builder(into, default)]
        pub allocation_type: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Filter to include IP Addresses which are attached to a device, such as a VM/LB (`Attached`) or unattached (`Unattached`).
        #[builder(into, default)]
        pub attachment_status: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// A prefix match used for the IP Addresses `name` field, case sensitive.
        #[builder(into, default)]
        pub name_prefix: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Specifies the name of the resource group.
        #[builder(into)]
        pub resource_group_name: pulumi_gestalt_rust::InputOrOutput<String>,
    }
    #[allow(dead_code)]
    pub struct GetPublicIPsResult {
        pub allocation_type: pulumi_gestalt_rust::Output<Option<String>>,
        pub attachment_status: pulumi_gestalt_rust::Output<Option<String>>,
        /// The provider-assigned unique ID for this managed resource.
        pub id: pulumi_gestalt_rust::Output<String>,
        pub name_prefix: pulumi_gestalt_rust::Output<Option<String>>,
        /// A List of `public_ips` blocks as defined below filtered by the criteria above.
        pub public_ips: pulumi_gestalt_rust::Output<
            Vec<super::super::super::types::network::GetPublicIPsPublicIp>,
        >,
        pub resource_group_name: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn invoke(
        context: &pulumi_gestalt_rust::Context,
        args: GetPublicIPsArgs,
    ) -> GetPublicIPsResult {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::register_interface;
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let allocation_type_binding = args.allocation_type.get_output(context);
        let attachment_status_binding = args.attachment_status.get_output(context);
        let name_prefix_binding = args.name_prefix.get_output(context);
        let resource_group_name_binding = args.resource_group_name.get_output(context);
        let request = pulumi_gestalt_rust::InvokeResourceRequest {
            token: "azure:network/getPublicIPs:getPublicIPs".into(),
            version: super::super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "allocationType".into(),
                    value: allocation_type_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "attachmentStatus".into(),
                    value: attachment_status_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "namePrefix".into(),
                    value: name_prefix_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "resourceGroupName".into(),
                    value: resource_group_name_binding.get_id(),
                },
            ],
        };
        let o = context.invoke_resource(request);
        GetPublicIPsResult {
            allocation_type: o.get_field("allocationType"),
            attachment_status: o.get_field("attachmentStatus"),
            id: o.get_field("id"),
            name_prefix: o.get_field("namePrefix"),
            public_ips: o.get_field("publicIps"),
            resource_group_name: o.get_field("resourceGroupName"),
        }
    }
}
