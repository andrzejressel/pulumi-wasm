/// Manages the configuration for a Nginx Deployment.
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
///       diagnoseSupportEnabled: true
///       frontendPublic:
///         ipAddresses:
///           - ${examplePublicIp.id}
///       networkInterfaces:
///         - subnetId: ${exampleSubnet.id}
///   exampleConfiguration:
///     type: azure:nginx:Configuration
///     name: example
///     properties:
///       nginxDeploymentId: ${exampleDeployment.id}
///       rootFile: /etc/nginx/nginx.conf
///       configFiles:
///         - content:
///             fn::invoke:
///               function: std:base64encode
///               arguments:
///                 input: |
///                   http {
///                       server {
///                           listen 80;
///                           location / {
///                               default_type text/html;
///                               return 200 '<!doctype html><html lang="en"><head></head><body>
///                                   <div>this one will be updated</div>
///                                   <div>at 10:38 am</div>
///                               </body></html>';
///                           }
///                           include site/*.conf;
///                       }
///                   }
///               return: result
///           virtualPath: /etc/nginx/nginx.conf
///         - content:
///             fn::invoke:
///               function: std:base64encode
///               arguments:
///                 input: |
///                   location /bbb {
///                    default_type text/html;
///                    return 200 '<!doctype html><html lang="en"><head></head><body>
///                     <div>this one will be updated</div>
///                     <div>at 10:38 am</div>
///                    </body></html>';
///                   }
///               return: result
///           virtualPath: /etc/nginx/site/b.conf
/// ```
///
/// ## Import
///
/// An Nginx Configuration can be imported using the `resource id`, e.g.
///
/// ```sh
/// $ pulumi import azure:nginx/configuration:Configuration example /subscriptions/12345678-1234-9876-4563-123456789012/resourceGroups/group1/providers/Nginx.NginxPlus/nginxDeployments/dep1/configurations/default
/// ```
///
pub mod configuration {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct ConfigurationArgs {
        /// One or more `config_file` blocks as defined below.
        #[builder(into, default)]
        pub config_files: pulumi_wasm_rust::Output<
            Option<Vec<super::super::types::nginx::ConfigurationConfigFile>>,
        >,
        /// The ID of the Nginx Deployment. Changing this forces a new Nginx Configuration to be created.
        #[builder(into)]
        pub nginx_deployment_id: pulumi_wasm_rust::Output<String>,
        /// Specifies the package data for this configuration.
        #[builder(into, default)]
        pub package_data: pulumi_wasm_rust::Output<Option<String>>,
        /// One or more `protected_file` blocks with sensitive information as defined below. If specified `config_file` must also be specified.
        #[builder(into, default)]
        pub protected_files: pulumi_wasm_rust::Output<
            Option<Vec<super::super::types::nginx::ConfigurationProtectedFile>>,
        >,
        /// Specifies the root file path of this Nginx Configuration.
        #[builder(into)]
        pub root_file: pulumi_wasm_rust::Output<String>,
    }
    #[allow(dead_code)]
    pub struct ConfigurationResult {
        /// One or more `config_file` blocks as defined below.
        pub config_files: pulumi_wasm_rust::Output<
            Option<Vec<super::super::types::nginx::ConfigurationConfigFile>>,
        >,
        /// The ID of the Nginx Deployment. Changing this forces a new Nginx Configuration to be created.
        pub nginx_deployment_id: pulumi_wasm_rust::Output<String>,
        /// Specifies the package data for this configuration.
        pub package_data: pulumi_wasm_rust::Output<Option<String>>,
        /// One or more `protected_file` blocks with sensitive information as defined below. If specified `config_file` must also be specified.
        pub protected_files: pulumi_wasm_rust::Output<
            Option<Vec<super::super::types::nginx::ConfigurationProtectedFile>>,
        >,
        /// Specifies the root file path of this Nginx Configuration.
        pub root_file: pulumi_wasm_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(name: &str, args: ConfigurationArgs) -> ConfigurationResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let config_files_binding = args.config_files.get_inner();
        let nginx_deployment_id_binding = args.nginx_deployment_id.get_inner();
        let package_data_binding = args.package_data.get_inner();
        let protected_files_binding = args.protected_files.get_inner();
        let root_file_binding = args.root_file.get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "azure:nginx/configuration:Configuration".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "configFiles".into(),
                    value: &config_files_binding,
                },
                register_interface::ObjectField {
                    name: "nginxDeploymentId".into(),
                    value: &nginx_deployment_id_binding,
                },
                register_interface::ObjectField {
                    name: "packageData".into(),
                    value: &package_data_binding,
                },
                register_interface::ObjectField {
                    name: "protectedFiles".into(),
                    value: &protected_files_binding,
                },
                register_interface::ObjectField {
                    name: "rootFile".into(),
                    value: &root_file_binding,
                },
            ]),
            results: Vec::from([
                register_interface::ResultField {
                    name: "configFiles".into(),
                },
                register_interface::ResultField {
                    name: "nginxDeploymentId".into(),
                },
                register_interface::ResultField {
                    name: "packageData".into(),
                },
                register_interface::ResultField {
                    name: "protectedFiles".into(),
                },
                register_interface::ResultField {
                    name: "rootFile".into(),
                },
            ]),
        };
        let o = register_interface::register(&request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        ConfigurationResult {
            config_files: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("configFiles").unwrap(),
            ),
            nginx_deployment_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("nginxDeploymentId").unwrap(),
            ),
            package_data: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("packageData").unwrap(),
            ),
            protected_files: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("protectedFiles").unwrap(),
            ),
            root_file: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("rootFile").unwrap(),
            ),
        }
    }
}
