#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod get_service {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GetServiceArgs {
        /// Specifies the name of the SignalR service.
        #[builder(into)]
        pub name: pulumi_gestalt_rust::InputOrOutput<String>,
        /// Specifies the name of the resource group the SignalR service is located in.
        #[builder(into)]
        pub resource_group_name: pulumi_gestalt_rust::InputOrOutput<String>,
    }
    #[allow(dead_code)]
    pub struct GetServiceResult {
        /// Is aad auth enabled for this SignalR service?
        pub aad_auth_enabled: pulumi_gestalt_rust::Output<bool>,
        /// The FQDN of the SignalR service.
        pub hostname: pulumi_gestalt_rust::Output<String>,
        /// The provider-assigned unique ID for this managed resource.
        pub id: pulumi_gestalt_rust::Output<String>,
        /// The publicly accessible IP of the SignalR service.
        pub ip_address: pulumi_gestalt_rust::Output<String>,
        /// Is local auth enable for this SignalR serviced?
        pub local_auth_enabled: pulumi_gestalt_rust::Output<bool>,
        /// Specifies the supported Azure location where the SignalR service exists.
        pub location: pulumi_gestalt_rust::Output<String>,
        pub name: pulumi_gestalt_rust::Output<String>,
        /// The primary access key of the SignalR service.
        pub primary_access_key: pulumi_gestalt_rust::Output<String>,
        /// The primary connection string of the SignalR service.
        pub primary_connection_string: pulumi_gestalt_rust::Output<String>,
        /// Is public network access enabled for this SignalR service?
        pub public_network_access_enabled: pulumi_gestalt_rust::Output<bool>,
        /// The publicly accessible port of the SignalR service which is designed for browser/client use.
        pub public_port: pulumi_gestalt_rust::Output<i32>,
        pub resource_group_name: pulumi_gestalt_rust::Output<String>,
        /// The secondary access key of the SignalR service.
        pub secondary_access_key: pulumi_gestalt_rust::Output<String>,
        /// The secondary connection string of the SignalR service.
        pub secondary_connection_string: pulumi_gestalt_rust::Output<String>,
        /// The publicly accessible port of the SignalR service which is designed for customer server side use.
        pub server_port: pulumi_gestalt_rust::Output<i32>,
        /// The serverless connection timeout of this SignalR service.
        pub serverless_connection_timeout_in_seconds: pulumi_gestalt_rust::Output<i32>,
        pub tags: pulumi_gestalt_rust::Output<std::collections::HashMap<String, String>>,
        /// Is tls client cert enabled for this SignalR service?
        pub tls_client_cert_enabled: pulumi_gestalt_rust::Output<bool>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn invoke(
        context: &pulumi_gestalt_rust::Context,
        args: GetServiceArgs,
    ) -> GetServiceResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let name_binding = args.name.get_output(context);
        let resource_group_name_binding = args.resource_group_name.get_output(context);
        let request = pulumi_gestalt_rust::InvokeResourceRequest {
            token: "azure:signalr/getService:getService".into(),
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
        GetServiceResult {
            aad_auth_enabled: o.get_field("aadAuthEnabled"),
            hostname: o.get_field("hostname"),
            id: o.get_field("id"),
            ip_address: o.get_field("ipAddress"),
            local_auth_enabled: o.get_field("localAuthEnabled"),
            location: o.get_field("location"),
            name: o.get_field("name"),
            primary_access_key: o.get_field("primaryAccessKey"),
            primary_connection_string: o.get_field("primaryConnectionString"),
            public_network_access_enabled: o.get_field("publicNetworkAccessEnabled"),
            public_port: o.get_field("publicPort"),
            resource_group_name: o.get_field("resourceGroupName"),
            secondary_access_key: o.get_field("secondaryAccessKey"),
            secondary_connection_string: o.get_field("secondaryConnectionString"),
            server_port: o.get_field("serverPort"),
            serverless_connection_timeout_in_seconds: o
                .get_field("serverlessConnectionTimeoutInSeconds"),
            tags: o.get_field("tags"),
            tls_client_cert_enabled: o.get_field("tlsClientCertEnabled"),
        }
    }
}
