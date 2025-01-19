/// Manages a Certificate for an NGINX Deployment.
///
/// ## Example Usage
///
/// ```yaml
/// resources:
///   example:
///     type: azure:core:ResourceGroup
///     properties:
///       name: example-rg
///       location: West Europe
///   examplePublicIp:
///     type: azure:network:PublicIp
///     name: example
///     properties:
///       name: example
///       resourceGroupName: ${example.name}
///       location: ${example.location}
///       allocationMethod: Static
///       sku: Standard
///       tags:
///         environment: Production
///   exampleVirtualNetwork:
///     type: azure:network:VirtualNetwork
///     name: example
///     properties:
///       name: example-vnet
///       addressSpaces:
///         - 10.0.0.0/16
///       location: ${example.location}
///       resourceGroupName: ${example.name}
///   exampleSubnet:
///     type: azure:network:Subnet
///     name: example
///     properties:
///       name: example-subnet
///       resourceGroupName: ${example.name}
///       virtualNetworkName: ${exampleVirtualNetwork.name}
///       addressPrefixes:
///         - 10.0.2.0/24
///       delegations:
///         - name: delegation
///           serviceDelegation:
///             name: NGINX.NGINXPLUS/nginxDeployments
///             actions:
///               - Microsoft.Network/virtualNetworks/subnets/join/action
///   exampleDeployment:
///     type: azure:nginx:Deployment
///     name: example
///     properties:
///       name: example-nginx
///       resourceGroupName: ${example.name}
///       sku: publicpreview_Monthly_gmz7xq9ge3py
///       location: ${example.location}
///       managedResourceGroup: example
///       diagnoseSupportEnabled: true
///       frontendPublic:
///         ipAddresses:
///           - ${examplePublicIp.id}
///       networkInterfaces:
///         - subnetId: ${exampleSubnet.id}
///   exampleKeyVault:
///     type: azure:keyvault:KeyVault
///     name: example
///     properties:
///       name: examplekeyvault
///       location: ${example.location}
///       resourceGroupName: ${example.name}
///       tenantId: ${current.tenantId}
///       skuName: premium
///       accessPolicies:
///         - tenantId: ${current.tenantId}
///           objectId: ${current.objectId}
///           certificatePermissions:
///             - Create
///             - Delete
///             - DeleteIssuers
///             - Get
///             - GetIssuers
///             - Import
///             - List
///             - ListIssuers
///             - ManageContacts
///             - ManageIssuers
///             - SetIssuers
///             - Update
///   exampleCertificate:
///     type: azure:keyvault:Certificate
///     name: example
///     properties:
///       name: imported-cert
///       keyVaultId: ${exampleKeyVault.id}
///       certificate:
///         contents:
///           fn::invoke:
///             function: std:filebase64
///             arguments:
///               input: certificate-to-import.pfx
///             return: result
///         password: ""
///   exampleCertificate2:
///     type: azure:nginx:Certificate
///     name: example
///     properties:
///       name: examplecert
///       nginxDeploymentId: ${exampleDeployment.id}
///       keyVirtualPath: /src/cert/soservermekey.key
///       certificateVirtualPath: /src/cert/server.cert
///       keyVaultSecretId: ${exampleCertificate.secretId}
/// variables:
///   current:
///     fn::invoke:
///       function: azure:core:getClientConfig
///       arguments: {}
/// ```
///
/// ## Import
///
/// An NGINX Certificate can be imported using the `resource id`, e.g.
///
/// ```sh
/// $ pulumi import azure:nginx/certificate:Certificate example /subscriptions/00000000-0000-0000-0000-000000000000/resourceGroups/group1/providers/Nginx.NginxPlus/nginxDeployments/deploy1/certificates/cer1
/// ```
///
pub mod certificate {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct CertificateArgs {
        /// Specify the path to the certificate file of this certificate.
        #[builder(into)]
        pub certificate_virtual_path: pulumi_wasm_rust::Output<String>,
        /// Specify the ID of the Key Vault Secret for this certificate.
        #[builder(into)]
        pub key_vault_secret_id: pulumi_wasm_rust::Output<String>,
        /// Specify the path to the key file of this certificate.
        #[builder(into)]
        pub key_virtual_path: pulumi_wasm_rust::Output<String>,
        /// The name which should be used for this NGINX Certificate. Changing this forces a new NGINX Certificate to be created.
        #[builder(into, default)]
        pub name: pulumi_wasm_rust::Output<Option<String>>,
        /// The ID of the NGINX Deployment that this Certificate should be associated with. Changing this forces a new NGINX Certificate to be created.
        #[builder(into)]
        pub nginx_deployment_id: pulumi_wasm_rust::Output<String>,
    }
    #[allow(dead_code)]
    pub struct CertificateResult {
        /// Specify the path to the certificate file of this certificate.
        pub certificate_virtual_path: pulumi_wasm_rust::Output<String>,
        /// Specify the ID of the Key Vault Secret for this certificate.
        pub key_vault_secret_id: pulumi_wasm_rust::Output<String>,
        /// Specify the path to the key file of this certificate.
        pub key_virtual_path: pulumi_wasm_rust::Output<String>,
        /// The name which should be used for this NGINX Certificate. Changing this forces a new NGINX Certificate to be created.
        pub name: pulumi_wasm_rust::Output<String>,
        /// The ID of the NGINX Deployment that this Certificate should be associated with. Changing this forces a new NGINX Certificate to be created.
        pub nginx_deployment_id: pulumi_wasm_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(name: &str, args: CertificateArgs) -> CertificateResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let certificate_virtual_path_binding = args.certificate_virtual_path.get_inner();
        let key_vault_secret_id_binding = args.key_vault_secret_id.get_inner();
        let key_virtual_path_binding = args.key_virtual_path.get_inner();
        let name_binding = args.name.get_inner();
        let nginx_deployment_id_binding = args.nginx_deployment_id.get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "azure:nginx/certificate:Certificate".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "certificateVirtualPath".into(),
                    value: &certificate_virtual_path_binding,
                },
                register_interface::ObjectField {
                    name: "keyVaultSecretId".into(),
                    value: &key_vault_secret_id_binding,
                },
                register_interface::ObjectField {
                    name: "keyVirtualPath".into(),
                    value: &key_virtual_path_binding,
                },
                register_interface::ObjectField {
                    name: "name".into(),
                    value: &name_binding,
                },
                register_interface::ObjectField {
                    name: "nginxDeploymentId".into(),
                    value: &nginx_deployment_id_binding,
                },
            ]),
            results: Vec::from([
                register_interface::ResultField {
                    name: "certificateVirtualPath".into(),
                },
                register_interface::ResultField {
                    name: "keyVaultSecretId".into(),
                },
                register_interface::ResultField {
                    name: "keyVirtualPath".into(),
                },
                register_interface::ResultField {
                    name: "name".into(),
                },
                register_interface::ResultField {
                    name: "nginxDeploymentId".into(),
                },
            ]),
        };
        let o = register_interface::register(&request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        CertificateResult {
            certificate_virtual_path: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("certificateVirtualPath").unwrap(),
            ),
            key_vault_secret_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("keyVaultSecretId").unwrap(),
            ),
            key_virtual_path: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("keyVirtualPath").unwrap(),
            ),
            name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("name").unwrap(),
            ),
            nginx_deployment_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("nginxDeploymentId").unwrap(),
            ),
        }
    }
}
