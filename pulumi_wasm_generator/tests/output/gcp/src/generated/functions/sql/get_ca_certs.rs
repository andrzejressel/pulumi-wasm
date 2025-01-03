pub mod get_ca_certs {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GetCaCertsArgs {
        /// The name or self link of the instance.
        #[builder(into)]
        pub instance: pulumi_wasm_rust::Output<String>,
        /// The ID of the project in which the resource belongs. If `project` is not provided, the provider project is used.
        #[builder(into, default)]
        pub project: pulumi_wasm_rust::Output<Option<String>>,
    }
    #[allow(dead_code)]
    pub struct GetCaCertsResult {
        /// SHA1 fingerprint of the currently active CA certificate.
        pub active_version: pulumi_wasm_rust::Output<String>,
        /// A list of server CA certificates for the instance. Each contains:
        pub certs: pulumi_wasm_rust::Output<
            Vec<super::super::super::types::sql::GetCaCertsCert>,
        >,
        /// The provider-assigned unique ID for this managed resource.
        pub id: pulumi_wasm_rust::Output<String>,
        pub instance: pulumi_wasm_rust::Output<String>,
        pub project: pulumi_wasm_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn invoke(args: GetCaCertsArgs) -> GetCaCertsResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let instance_binding = args.instance.get_inner();
        let project_binding = args.project.get_inner();
        let request = register_interface::ResourceInvokeRequest {
            token: "gcp:sql/getCaCerts:getCaCerts".into(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "instance".into(),
                    value: &instance_binding,
                },
                register_interface::ObjectField {
                    name: "project".into(),
                    value: &project_binding,
                },
            ]),
            results: Vec::from([
                register_interface::ResultField {
                    name: "activeVersion".into(),
                },
                register_interface::ResultField {
                    name: "certs".into(),
                },
                register_interface::ResultField {
                    name: "id".into(),
                },
                register_interface::ResultField {
                    name: "instance".into(),
                },
                register_interface::ResultField {
                    name: "project".into(),
                },
            ]),
        };
        let o = register_interface::invoke(&request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        GetCaCertsResult {
            active_version: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("activeVersion").unwrap(),
            ),
            certs: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("certs").unwrap(),
            ),
            id: pulumi_wasm_rust::__private::into_domain(hashmap.remove("id").unwrap()),
            instance: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("instance").unwrap(),
            ),
            project: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("project").unwrap(),
            ),
        }
    }
}
