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
        context: &pulumi_gestalt_rust::PulumiContext,
        args: GetSpringCloudServiceArgs,
    ) -> GetSpringCloudServiceResult {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::register_interface;
        use std::collections::HashMap;
        let name_binding = args.name.get_output(context).get_inner();
        let resource_group_name_binding = args
            .resource_group_name
            .get_output(context)
            .get_inner();
        let request = register_interface::ResourceInvokeRequest {
            token: "azure:appplatform/getSpringCloudService:getSpringCloudService"
                .into(),
            version: super::super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "name".into(),
                    value: &name_binding,
                },
                register_interface::ObjectField {
                    name: "resourceGroupName".into(),
                    value: &resource_group_name_binding,
                },
            ]),
        };
        let o = register_interface::invoke(context.get_inner(), &request);
        GetSpringCloudServiceResult {
            config_server_git_settings: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("configServerGitSettings"),
            ),
            id: pulumi_gestalt_rust::__private::into_domain(o.extract_field("id")),
            location: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("location"),
            ),
            name: pulumi_gestalt_rust::__private::into_domain(o.extract_field("name")),
            outbound_public_ip_addresses: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("outboundPublicIpAddresses"),
            ),
            required_network_traffic_rules: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("requiredNetworkTrafficRules"),
            ),
            resource_group_name: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("resourceGroupName"),
            ),
            tags: pulumi_gestalt_rust::__private::into_domain(o.extract_field("tags")),
        }
    }
}
