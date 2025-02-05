pub mod get_service {
    #[derive(pulumi_wasm_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GetServiceArgs {
        /// Specifies the name of the SignalR service.
        #[builder(into)]
        pub name: pulumi_wasm_rust::InputOrOutput<String>,
        /// Specifies the name of the resource group the SignalR service is located in.
        #[builder(into)]
        pub resource_group_name: pulumi_wasm_rust::InputOrOutput<String>,
    }
    #[allow(dead_code)]
    pub struct GetServiceResult {
        /// Is aad auth enabled for this SignalR service?
        pub aad_auth_enabled: pulumi_wasm_rust::Output<bool>,
        /// The FQDN of the SignalR service.
        pub hostname: pulumi_wasm_rust::Output<String>,
        /// The provider-assigned unique ID for this managed resource.
        pub id: pulumi_wasm_rust::Output<String>,
        /// The publicly accessible IP of the SignalR service.
        pub ip_address: pulumi_wasm_rust::Output<String>,
        /// Is local auth enable for this SignalR serviced?
        pub local_auth_enabled: pulumi_wasm_rust::Output<bool>,
        /// Specifies the supported Azure location where the SignalR service exists.
        pub location: pulumi_wasm_rust::Output<String>,
        pub name: pulumi_wasm_rust::Output<String>,
        /// The primary access key of the SignalR service.
        pub primary_access_key: pulumi_wasm_rust::Output<String>,
        /// The primary connection string of the SignalR service.
        pub primary_connection_string: pulumi_wasm_rust::Output<String>,
        /// Is public network access enabled for this SignalR service?
        pub public_network_access_enabled: pulumi_wasm_rust::Output<bool>,
        /// The publicly accessible port of the SignalR service which is designed for browser/client use.
        pub public_port: pulumi_wasm_rust::Output<i32>,
        pub resource_group_name: pulumi_wasm_rust::Output<String>,
        /// The secondary access key of the SignalR service.
        pub secondary_access_key: pulumi_wasm_rust::Output<String>,
        /// The secondary connection string of the SignalR service.
        pub secondary_connection_string: pulumi_wasm_rust::Output<String>,
        /// The publicly accessible port of the SignalR service which is designed for customer server side use.
        pub server_port: pulumi_wasm_rust::Output<i32>,
        /// The serverless connection timeout of this SignalR service.
        pub serverless_connection_timeout_in_seconds: pulumi_wasm_rust::Output<i32>,
        pub tags: pulumi_wasm_rust::Output<std::collections::HashMap<String, String>>,
        /// Is tls client cert enabled for this SignalR service?
        pub tls_client_cert_enabled: pulumi_wasm_rust::Output<bool>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn invoke(
        context: &pulumi_wasm_rust::PulumiContext,
        args: GetServiceArgs,
    ) -> GetServiceResult {
        use pulumi_wasm_rust::__private::pulumi_gestalt_adapter_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let name_binding = args.name.get_output(context).get_inner();
        let resource_group_name_binding = args
            .resource_group_name
            .get_output(context)
            .get_inner();
        let request = register_interface::ResourceInvokeRequest {
            token: "azure:signalr/getService:getService".into(),
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
        GetServiceResult {
            aad_auth_enabled: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("aadAuthEnabled"),
            ),
            hostname: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("hostname"),
            ),
            id: pulumi_wasm_rust::__private::into_domain(o.extract_field("id")),
            ip_address: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("ipAddress"),
            ),
            local_auth_enabled: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("localAuthEnabled"),
            ),
            location: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("location"),
            ),
            name: pulumi_wasm_rust::__private::into_domain(o.extract_field("name")),
            primary_access_key: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("primaryAccessKey"),
            ),
            primary_connection_string: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("primaryConnectionString"),
            ),
            public_network_access_enabled: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("publicNetworkAccessEnabled"),
            ),
            public_port: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("publicPort"),
            ),
            resource_group_name: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("resourceGroupName"),
            ),
            secondary_access_key: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("secondaryAccessKey"),
            ),
            secondary_connection_string: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("secondaryConnectionString"),
            ),
            server_port: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("serverPort"),
            ),
            serverless_connection_timeout_in_seconds: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("serverlessConnectionTimeoutInSeconds"),
            ),
            tags: pulumi_wasm_rust::__private::into_domain(o.extract_field("tags")),
            tls_client_cert_enabled: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("tlsClientCertEnabled"),
            ),
        }
    }
}
