pub mod get_configuration {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GetConfigurationArgs {
        /// The ID of the Nginx Deployment.
        #[builder(into)]
        pub nginx_deployment_id: pulumi_wasm_rust::Output<String>,
    }
    #[allow(dead_code)]
    pub struct GetConfigurationResult {
        /// A `config_file` block as defined below.
        pub config_files: pulumi_wasm_rust::Output<
            Vec<super::super::super::types::nginx::GetConfigurationConfigFile>,
        >,
        /// The provider-assigned unique ID for this managed resource.
        pub id: pulumi_wasm_rust::Output<String>,
        pub nginx_deployment_id: pulumi_wasm_rust::Output<String>,
        /// The package data for this configuration.
        pub package_data: pulumi_wasm_rust::Output<String>,
        /// A `protected_file` block as defined below.
        pub protected_files: pulumi_wasm_rust::Output<
            Vec<super::super::super::types::nginx::GetConfigurationProtectedFile>,
        >,
        /// The root file path of this Nginx Configuration.
        pub root_file: pulumi_wasm_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn invoke(args: GetConfigurationArgs) -> GetConfigurationResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let nginx_deployment_id_binding = args.nginx_deployment_id.get_inner();
        let request = register_interface::ResourceInvokeRequest {
            token: "azure:nginx/getConfiguration:getConfiguration".into(),
            version: super::super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "nginxDeploymentId".into(),
                    value: &nginx_deployment_id_binding,
                },
            ]),
            results: Vec::from([
                register_interface::ResultField {
                    name: "configFiles".into(),
                },
                register_interface::ResultField {
                    name: "id".into(),
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
        let o = register_interface::invoke(&request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        GetConfigurationResult {
            config_files: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("configFiles").unwrap(),
            ),
            id: pulumi_wasm_rust::__private::into_domain(hashmap.remove("id").unwrap()),
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
