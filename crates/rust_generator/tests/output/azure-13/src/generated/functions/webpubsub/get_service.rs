pub mod get_service {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GetServiceArgs {
        /// Specifies the name of the Web Pubsub service.
        #[builder(into)]
        pub name: pulumi_gestalt_rust::InputOrOutput<String>,
        /// Specifies the name of the resource group the Web Pubsub service is located in.
        #[builder(into)]
        pub resource_group_name: pulumi_gestalt_rust::InputOrOutput<String>,
    }
    #[allow(dead_code)]
    pub struct GetServiceResult {
        pub aad_auth_enabled: pulumi_gestalt_rust::Output<bool>,
        pub capacity: pulumi_gestalt_rust::Output<i32>,
        pub external_ip: pulumi_gestalt_rust::Output<String>,
        /// The FQDN of the Web Pubsub service.
        pub hostname: pulumi_gestalt_rust::Output<String>,
        /// The provider-assigned unique ID for this managed resource.
        pub id: pulumi_gestalt_rust::Output<String>,
        pub local_auth_enabled: pulumi_gestalt_rust::Output<bool>,
        /// The Azure location where the Web Pubsub service exists.
        pub location: pulumi_gestalt_rust::Output<String>,
        pub name: pulumi_gestalt_rust::Output<String>,
        /// The primary access key of the Web Pubsub service.
        pub primary_access_key: pulumi_gestalt_rust::Output<String>,
        /// The primary connection string of the Web Pubsub service.
        pub primary_connection_string: pulumi_gestalt_rust::Output<String>,
        pub public_network_access_enabled: pulumi_gestalt_rust::Output<bool>,
        /// The publicly accessible port of the Web Pubsub service which is designed for browser/client use.
        pub public_port: pulumi_gestalt_rust::Output<i32>,
        pub resource_group_name: pulumi_gestalt_rust::Output<String>,
        /// The secondary access key of the Web Pubsub service.
        pub secondary_access_key: pulumi_gestalt_rust::Output<String>,
        /// The secondary connection string of the Web Pubsub service.
        pub secondary_connection_string: pulumi_gestalt_rust::Output<String>,
        /// The publicly accessible port of the Web Pubsub service which is designed for customer server side use.
        pub server_port: pulumi_gestalt_rust::Output<i32>,
        pub sku: pulumi_gestalt_rust::Output<String>,
        pub tags: pulumi_gestalt_rust::Output<std::collections::HashMap<String, String>>,
        pub tls_client_cert_enabled: pulumi_gestalt_rust::Output<bool>,
        pub version: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn invoke(
        context: &pulumi_gestalt_rust::PulumiContext,
        args: GetServiceArgs,
    ) -> GetServiceResult {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::register_interface;
        use std::collections::HashMap;
        let name_binding = args.name.get_output(context).get_inner();
        let resource_group_name_binding = args
            .resource_group_name
            .get_output(context)
            .get_inner();
        let request = register_interface::ResourceInvokeRequest {
            token: "azure:webpubsub/getService:getService".into(),
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
            aad_auth_enabled: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("aadAuthEnabled"),
            ),
            capacity: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("capacity"),
            ),
            external_ip: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("externalIp"),
            ),
            hostname: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("hostname"),
            ),
            id: pulumi_gestalt_rust::__private::into_domain(o.extract_field("id")),
            local_auth_enabled: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("localAuthEnabled"),
            ),
            location: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("location"),
            ),
            name: pulumi_gestalt_rust::__private::into_domain(o.extract_field("name")),
            primary_access_key: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("primaryAccessKey"),
            ),
            primary_connection_string: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("primaryConnectionString"),
            ),
            public_network_access_enabled: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("publicNetworkAccessEnabled"),
            ),
            public_port: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("publicPort"),
            ),
            resource_group_name: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("resourceGroupName"),
            ),
            secondary_access_key: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("secondaryAccessKey"),
            ),
            secondary_connection_string: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("secondaryConnectionString"),
            ),
            server_port: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("serverPort"),
            ),
            sku: pulumi_gestalt_rust::__private::into_domain(o.extract_field("sku")),
            tags: pulumi_gestalt_rust::__private::into_domain(o.extract_field("tags")),
            tls_client_cert_enabled: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("tlsClientCertEnabled"),
            ),
            version: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("version"),
            ),
        }
    }
}
