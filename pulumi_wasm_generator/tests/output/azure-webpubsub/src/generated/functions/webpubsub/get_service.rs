pub mod get_service {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GetServiceArgs {
        /// Specifies the name of the Web Pubsub service.
        #[builder(into)]
        pub name: pulumi_wasm_rust::Output<String>,
        /// Specifies the name of the resource group the Web Pubsub service is located in.
        #[builder(into)]
        pub resource_group_name: pulumi_wasm_rust::Output<String>,
    }
    #[allow(dead_code)]
    pub struct GetServiceResult {
        pub aad_auth_enabled: pulumi_wasm_rust::Output<bool>,
        pub capacity: pulumi_wasm_rust::Output<i32>,
        pub external_ip: pulumi_wasm_rust::Output<String>,
        /// The FQDN of the Web Pubsub service.
        pub hostname: pulumi_wasm_rust::Output<String>,
        /// The provider-assigned unique ID for this managed resource.
        pub id: pulumi_wasm_rust::Output<String>,
        pub local_auth_enabled: pulumi_wasm_rust::Output<bool>,
        /// The Azure location where the Web Pubsub service exists.
        pub location: pulumi_wasm_rust::Output<String>,
        pub name: pulumi_wasm_rust::Output<String>,
        /// The primary access key of the Web Pubsub service.
        pub primary_access_key: pulumi_wasm_rust::Output<String>,
        /// The primary connection string of the Web Pubsub service.
        pub primary_connection_string: pulumi_wasm_rust::Output<String>,
        pub public_network_access_enabled: pulumi_wasm_rust::Output<bool>,
        /// The publicly accessible port of the Web Pubsub service which is designed for browser/client use.
        pub public_port: pulumi_wasm_rust::Output<i32>,
        pub resource_group_name: pulumi_wasm_rust::Output<String>,
        /// The secondary access key of the Web Pubsub service.
        pub secondary_access_key: pulumi_wasm_rust::Output<String>,
        /// The secondary connection string of the Web Pubsub service.
        pub secondary_connection_string: pulumi_wasm_rust::Output<String>,
        /// The publicly accessible port of the Web Pubsub service which is designed for customer server side use.
        pub server_port: pulumi_wasm_rust::Output<i32>,
        pub sku: pulumi_wasm_rust::Output<String>,
        pub tags: pulumi_wasm_rust::Output<std::collections::HashMap<String, String>>,
        pub tls_client_cert_enabled: pulumi_wasm_rust::Output<bool>,
        pub version: pulumi_wasm_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn invoke(args: GetServiceArgs) -> GetServiceResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let name_binding = args.name.get_inner();
        let resource_group_name_binding = args.resource_group_name.get_inner();
        let request = register_interface::ResourceInvokeRequest {
            token: "azure:webpubsub/getService:getService".into(),
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
            results: Vec::from([
                register_interface::ResultField {
                    name: "aadAuthEnabled".into(),
                },
                register_interface::ResultField {
                    name: "capacity".into(),
                },
                register_interface::ResultField {
                    name: "externalIp".into(),
                },
                register_interface::ResultField {
                    name: "hostname".into(),
                },
                register_interface::ResultField {
                    name: "id".into(),
                },
                register_interface::ResultField {
                    name: "localAuthEnabled".into(),
                },
                register_interface::ResultField {
                    name: "location".into(),
                },
                register_interface::ResultField {
                    name: "name".into(),
                },
                register_interface::ResultField {
                    name: "primaryAccessKey".into(),
                },
                register_interface::ResultField {
                    name: "primaryConnectionString".into(),
                },
                register_interface::ResultField {
                    name: "publicNetworkAccessEnabled".into(),
                },
                register_interface::ResultField {
                    name: "publicPort".into(),
                },
                register_interface::ResultField {
                    name: "resourceGroupName".into(),
                },
                register_interface::ResultField {
                    name: "secondaryAccessKey".into(),
                },
                register_interface::ResultField {
                    name: "secondaryConnectionString".into(),
                },
                register_interface::ResultField {
                    name: "serverPort".into(),
                },
                register_interface::ResultField {
                    name: "sku".into(),
                },
                register_interface::ResultField {
                    name: "tags".into(),
                },
                register_interface::ResultField {
                    name: "tlsClientCertEnabled".into(),
                },
                register_interface::ResultField {
                    name: "version".into(),
                },
            ]),
        };
        let o = register_interface::invoke(&request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        GetServiceResult {
            aad_auth_enabled: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("aadAuthEnabled").unwrap(),
            ),
            capacity: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("capacity").unwrap(),
            ),
            external_ip: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("externalIp").unwrap(),
            ),
            hostname: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("hostname").unwrap(),
            ),
            id: pulumi_wasm_rust::__private::into_domain(hashmap.remove("id").unwrap()),
            local_auth_enabled: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("localAuthEnabled").unwrap(),
            ),
            location: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("location").unwrap(),
            ),
            name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("name").unwrap(),
            ),
            primary_access_key: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("primaryAccessKey").unwrap(),
            ),
            primary_connection_string: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("primaryConnectionString").unwrap(),
            ),
            public_network_access_enabled: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("publicNetworkAccessEnabled").unwrap(),
            ),
            public_port: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("publicPort").unwrap(),
            ),
            resource_group_name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("resourceGroupName").unwrap(),
            ),
            secondary_access_key: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("secondaryAccessKey").unwrap(),
            ),
            secondary_connection_string: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("secondaryConnectionString").unwrap(),
            ),
            server_port: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("serverPort").unwrap(),
            ),
            sku: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("sku").unwrap(),
            ),
            tags: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("tags").unwrap(),
            ),
            tls_client_cert_enabled: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("tlsClientCertEnabled").unwrap(),
            ),
            version: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("version").unwrap(),
            ),
        }
    }
}
