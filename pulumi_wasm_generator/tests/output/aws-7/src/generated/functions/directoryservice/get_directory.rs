pub mod get_directory {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GetDirectoryArgs {
        /// ID of the directory.
        #[builder(into)]
        pub directory_id: pulumi_wasm_rust::Output<String>,
        /// A map of tags assigned to the directory/connector.
        #[builder(into, default)]
        pub tags: pulumi_wasm_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
    }
    #[allow(dead_code)]
    pub struct GetDirectoryResult {
        /// Access URL for the directory/connector, such as http://alias.awsapps.com.
        pub access_url: pulumi_wasm_rust::Output<String>,
        /// Alias for the directory/connector, such as `d-991708b282.awsapps.com`.
        pub alias: pulumi_wasm_rust::Output<String>,
        pub connect_settings: pulumi_wasm_rust::Output<
            Vec<super::super::super::types::directoryservice::GetDirectoryConnectSetting>,
        >,
        /// Textual description for the directory/connector.
        pub description: pulumi_wasm_rust::Output<String>,
        pub directory_id: pulumi_wasm_rust::Output<String>,
        /// List of IP addresses of the DNS servers for the directory/connector.
        pub dns_ip_addresses: pulumi_wasm_rust::Output<Vec<String>>,
        /// (for `MicrosoftAD`) Microsoft AD edition (`Standard` or `Enterprise`).
        pub edition: pulumi_wasm_rust::Output<String>,
        /// Directory/connector single-sign on status.
        pub enable_sso: pulumi_wasm_rust::Output<bool>,
        /// The provider-assigned unique ID for this managed resource.
        pub id: pulumi_wasm_rust::Output<String>,
        /// Fully qualified name for the directory/connector.
        pub name: pulumi_wasm_rust::Output<String>,
        pub radius_settings: pulumi_wasm_rust::Output<
            Vec<super::super::super::types::directoryservice::GetDirectoryRadiusSetting>,
        >,
        /// ID of the security group created by the directory/connector.
        pub security_group_id: pulumi_wasm_rust::Output<String>,
        /// Short name of the directory/connector, such as `CORP`.
        pub short_name: pulumi_wasm_rust::Output<String>,
        /// (for `SimpleAD` and `ADConnector`) Size of the directory/connector (`Small` or `Large`).
        pub size: pulumi_wasm_rust::Output<String>,
        /// A map of tags assigned to the directory/connector.
        pub tags: pulumi_wasm_rust::Output<std::collections::HashMap<String, String>>,
        /// Directory type (`SimpleAD`, `ADConnector` or `MicrosoftAD`).
        pub type_: pulumi_wasm_rust::Output<String>,
        pub vpc_settings: pulumi_wasm_rust::Output<
            Vec<super::super::super::types::directoryservice::GetDirectoryVpcSetting>,
        >,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn invoke(args: GetDirectoryArgs) -> GetDirectoryResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let directory_id_binding = args.directory_id.get_inner();
        let tags_binding = args.tags.get_inner();
        let request = register_interface::ResourceInvokeRequest {
            token: "aws:directoryservice/getDirectory:getDirectory".into(),
            version: super::super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "directoryId".into(),
                    value: &directory_id_binding,
                },
                register_interface::ObjectField {
                    name: "tags".into(),
                    value: &tags_binding,
                },
            ]),
            results: Vec::from([
                register_interface::ResultField {
                    name: "accessUrl".into(),
                },
                register_interface::ResultField {
                    name: "alias".into(),
                },
                register_interface::ResultField {
                    name: "connectSettings".into(),
                },
                register_interface::ResultField {
                    name: "description".into(),
                },
                register_interface::ResultField {
                    name: "directoryId".into(),
                },
                register_interface::ResultField {
                    name: "dnsIpAddresses".into(),
                },
                register_interface::ResultField {
                    name: "edition".into(),
                },
                register_interface::ResultField {
                    name: "enableSso".into(),
                },
                register_interface::ResultField {
                    name: "id".into(),
                },
                register_interface::ResultField {
                    name: "name".into(),
                },
                register_interface::ResultField {
                    name: "radiusSettings".into(),
                },
                register_interface::ResultField {
                    name: "securityGroupId".into(),
                },
                register_interface::ResultField {
                    name: "shortName".into(),
                },
                register_interface::ResultField {
                    name: "size".into(),
                },
                register_interface::ResultField {
                    name: "tags".into(),
                },
                register_interface::ResultField {
                    name: "type".into(),
                },
                register_interface::ResultField {
                    name: "vpcSettings".into(),
                },
            ]),
        };
        let o = register_interface::invoke(&request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        GetDirectoryResult {
            access_url: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("accessUrl").unwrap(),
            ),
            alias: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("alias").unwrap(),
            ),
            connect_settings: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("connectSettings").unwrap(),
            ),
            description: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("description").unwrap(),
            ),
            directory_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("directoryId").unwrap(),
            ),
            dns_ip_addresses: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("dnsIpAddresses").unwrap(),
            ),
            edition: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("edition").unwrap(),
            ),
            enable_sso: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("enableSso").unwrap(),
            ),
            id: pulumi_wasm_rust::__private::into_domain(hashmap.remove("id").unwrap()),
            name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("name").unwrap(),
            ),
            radius_settings: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("radiusSettings").unwrap(),
            ),
            security_group_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("securityGroupId").unwrap(),
            ),
            short_name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("shortName").unwrap(),
            ),
            size: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("size").unwrap(),
            ),
            tags: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("tags").unwrap(),
            ),
            type_: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("type").unwrap(),
            ),
            vpc_settings: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("vpcSettings").unwrap(),
            ),
        }
    }
}
