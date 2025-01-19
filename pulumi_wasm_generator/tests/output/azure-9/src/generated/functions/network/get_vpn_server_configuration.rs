pub mod get_vpn_server_configuration {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GetVpnServerConfigurationArgs {
        /// The Name of the VPN Server Configuration.
        #[builder(into)]
        pub name: pulumi_wasm_rust::Output<String>,
        /// The name of the Resource Group where the VPN Server Configuration exists.
        #[builder(into)]
        pub resource_group_name: pulumi_wasm_rust::Output<String>,
    }
    #[allow(dead_code)]
    pub struct GetVpnServerConfigurationResult {
        /// A `azure_active_directory_authentication` block as defined below.
        pub azure_active_directory_authentications: pulumi_wasm_rust::Output<
            Vec<
                super::super::super::types::network::GetVpnServerConfigurationAzureActiveDirectoryAuthentication,
            >,
        >,
        /// One or more `client_revoked_certificate` blocks as defined below.
        pub client_revoked_certificates: pulumi_wasm_rust::Output<
            Vec<
                super::super::super::types::network::GetVpnServerConfigurationClientRevokedCertificate,
            >,
        >,
        /// One or more `client_root_certificate` blocks as defined below.
        pub client_root_certificates: pulumi_wasm_rust::Output<
            Vec<
                super::super::super::types::network::GetVpnServerConfigurationClientRootCertificate,
            >,
        >,
        /// The provider-assigned unique ID for this managed resource.
        pub id: pulumi_wasm_rust::Output<String>,
        /// The `bgp_settings` block as defined below.
        pub ipsec_policies: pulumi_wasm_rust::Output<
            Vec<
                super::super::super::types::network::GetVpnServerConfigurationIpsecPolicy,
            >,
        >,
        /// The Azure Region where the VPN Server Configuration exists.
        pub location: pulumi_wasm_rust::Output<String>,
        /// The name used to uniquely identify this certificate.
        pub name: pulumi_wasm_rust::Output<String>,
        /// A `radius` block as defined below.
        pub radii: pulumi_wasm_rust::Output<
            Vec<super::super::super::types::network::GetVpnServerConfigurationRadius>,
        >,
        pub resource_group_name: pulumi_wasm_rust::Output<String>,
        /// A mapping of tags to assign to the VPN Server Configuration.
        pub tags: pulumi_wasm_rust::Output<std::collections::HashMap<String, String>>,
        /// The list of Authentication Types applicable for the VPN Server Configuration.
        pub vpn_authentication_types: pulumi_wasm_rust::Output<Vec<String>>,
        /// The list of VPN Protocols to use for the VPN Server Configuration.
        pub vpn_protocols: pulumi_wasm_rust::Output<Vec<String>>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn invoke(
        args: GetVpnServerConfigurationArgs,
    ) -> GetVpnServerConfigurationResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let name_binding = args.name.get_inner();
        let resource_group_name_binding = args.resource_group_name.get_inner();
        let request = register_interface::ResourceInvokeRequest {
            token: "azure:network/getVpnServerConfiguration:getVpnServerConfiguration"
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
            results: Vec::from([
                register_interface::ResultField {
                    name: "azureActiveDirectoryAuthentications".into(),
                },
                register_interface::ResultField {
                    name: "clientRevokedCertificates".into(),
                },
                register_interface::ResultField {
                    name: "clientRootCertificates".into(),
                },
                register_interface::ResultField {
                    name: "id".into(),
                },
                register_interface::ResultField {
                    name: "ipsecPolicies".into(),
                },
                register_interface::ResultField {
                    name: "location".into(),
                },
                register_interface::ResultField {
                    name: "name".into(),
                },
                register_interface::ResultField {
                    name: "radii".into(),
                },
                register_interface::ResultField {
                    name: "resourceGroupName".into(),
                },
                register_interface::ResultField {
                    name: "tags".into(),
                },
                register_interface::ResultField {
                    name: "vpnAuthenticationTypes".into(),
                },
                register_interface::ResultField {
                    name: "vpnProtocols".into(),
                },
            ]),
        };
        let o = register_interface::invoke(&request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        GetVpnServerConfigurationResult {
            azure_active_directory_authentications: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("azureActiveDirectoryAuthentications").unwrap(),
            ),
            client_revoked_certificates: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("clientRevokedCertificates").unwrap(),
            ),
            client_root_certificates: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("clientRootCertificates").unwrap(),
            ),
            id: pulumi_wasm_rust::__private::into_domain(hashmap.remove("id").unwrap()),
            ipsec_policies: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("ipsecPolicies").unwrap(),
            ),
            location: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("location").unwrap(),
            ),
            name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("name").unwrap(),
            ),
            radii: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("radii").unwrap(),
            ),
            resource_group_name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("resourceGroupName").unwrap(),
            ),
            tags: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("tags").unwrap(),
            ),
            vpn_authentication_types: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("vpnAuthenticationTypes").unwrap(),
            ),
            vpn_protocols: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("vpnProtocols").unwrap(),
            ),
        }
    }
}
