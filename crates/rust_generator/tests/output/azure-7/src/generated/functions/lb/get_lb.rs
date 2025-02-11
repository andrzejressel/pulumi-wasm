#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod get_lb {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GetLbArgs {
        /// Specifies the name of the Load Balancer.
        #[builder(into)]
        pub name: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The name of the Resource Group in which the Load Balancer exists.
        #[builder(into)]
        pub resource_group_name: pulumi_gestalt_rust::InputOrOutput<String>,
    }
    #[allow(dead_code)]
    pub struct GetLbResult {
        /// A `frontend_ip_configuration` block as documented below.
        pub frontend_ip_configurations: pulumi_gestalt_rust::Output<
            Vec<super::super::super::types::lb::GetLbFrontendIpConfiguration>,
        >,
        /// The provider-assigned unique ID for this managed resource.
        pub id: pulumi_gestalt_rust::Output<String>,
        /// The Azure location where the Load Balancer exists.
        pub location: pulumi_gestalt_rust::Output<String>,
        /// The name of the Frontend IP Configuration.
        pub name: pulumi_gestalt_rust::Output<String>,
        /// Private IP Address to assign to the Load Balancer.
        pub private_ip_address: pulumi_gestalt_rust::Output<String>,
        /// The list of private IP address assigned to the load balancer in `frontend_ip_configuration` blocks, if any.
        pub private_ip_addresses: pulumi_gestalt_rust::Output<Vec<String>>,
        pub resource_group_name: pulumi_gestalt_rust::Output<String>,
        /// The SKU of the Load Balancer.
        pub sku: pulumi_gestalt_rust::Output<String>,
        /// A mapping of tags assigned to the resource.
        pub tags: pulumi_gestalt_rust::Output<std::collections::HashMap<String, String>>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn invoke(
        context: &pulumi_gestalt_rust::Context,
        args: GetLbArgs,
    ) -> GetLbResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let name_binding = args.name.get_output(context);
        let resource_group_name_binding = args.resource_group_name.get_output(context);
        let request = pulumi_gestalt_rust::InvokeResourceRequest {
            token: "azure:lb/getLB:getLB".into(),
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
        GetLbResult {
            frontend_ip_configurations: o.get_field("frontendIpConfigurations"),
            id: o.get_field("id"),
            location: o.get_field("location"),
            name: o.get_field("name"),
            private_ip_address: o.get_field("privateIpAddress"),
            private_ip_addresses: o.get_field("privateIpAddresses"),
            resource_group_name: o.get_field("resourceGroupName"),
            sku: o.get_field("sku"),
            tags: o.get_field("tags"),
        }
    }
}
