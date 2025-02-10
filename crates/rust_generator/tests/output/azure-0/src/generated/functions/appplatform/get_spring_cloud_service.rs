#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod get_spring_cloud_service {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GetSpringCloudServiceArgs {
        /// Specifies The name of the Spring Cloud Service resource.
        #[builder(into)]
        pub name: pulumi_gestalt_rust::InputOrOutput<String>,
        /// Specifies the name of the Resource Group where the Spring Cloud Service exists.
        #[builder(into)]
        pub resource_group_name: pulumi_gestalt_rust::InputOrOutput<String>,
    }
    #[allow(dead_code)]
    pub struct GetSpringCloudServiceResult {
        /// A `config_server_git_setting` block as defined below.
        pub config_server_git_settings: pulumi_gestalt_rust::Output<
            Vec<
                super::super::super::types::appplatform::GetSpringCloudServiceConfigServerGitSetting,
            >,
        >,
        /// The provider-assigned unique ID for this managed resource.
        pub id: pulumi_gestalt_rust::Output<String>,
        /// The location of Spring Cloud Service.
        pub location: pulumi_gestalt_rust::Output<String>,
        /// The name to identify on the Git repository.
        pub name: pulumi_gestalt_rust::Output<String>,
        /// A list of the outbound Public IP Addresses used by this Spring Cloud Service.
        pub outbound_public_ip_addresses: pulumi_gestalt_rust::Output<Vec<String>>,
        /// A list of `required_network_traffic_rules` blocks as defined below.
        pub required_network_traffic_rules: pulumi_gestalt_rust::Output<
            Vec<
                super::super::super::types::appplatform::GetSpringCloudServiceRequiredNetworkTrafficRule,
            >,
        >,
        pub resource_group_name: pulumi_gestalt_rust::Output<String>,
        /// A mapping of tags assigned to Spring Cloud Service.
        pub tags: pulumi_gestalt_rust::Output<std::collections::HashMap<String, String>>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn invoke(
        context: &pulumi_gestalt_rust::Context,
        args: GetSpringCloudServiceArgs,
    ) -> GetSpringCloudServiceResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let name_binding = args.name.get_output(context);
        let resource_group_name_binding = args.resource_group_name.get_output(context);
        let request = pulumi_gestalt_rust::InvokeResourceRequest {
            token: "azure:appplatform/getSpringCloudService:getSpringCloudService"
                .into(),
            version: super::super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "name".into(),
                    value: name_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "resourceGroupName".into(),
                    value: resource_group_name_binding.get_id(),
                },
            ],
        };
        let o = context.invoke_resource(request);
        GetSpringCloudServiceResult {
            config_server_git_settings: o.get_field("configServerGitSettings"),
            id: o.get_field("id"),
            location: o.get_field("location"),
            name: o.get_field("name"),
            outbound_public_ip_addresses: o.get_field("outboundPublicIpAddresses"),
            required_network_traffic_rules: o.get_field("requiredNetworkTrafficRules"),
            resource_group_name: o.get_field("resourceGroupName"),
            tags: o.get_field("tags"),
        }
    }
}
