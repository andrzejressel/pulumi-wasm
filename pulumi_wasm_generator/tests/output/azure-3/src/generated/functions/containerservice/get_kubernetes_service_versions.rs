pub mod get_kubernetes_service_versions {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GetKubernetesServiceVersionsArgs {
        /// Should Preview versions of Kubernetes in AKS be included? Defaults to `true`
        #[builder(into, default)]
        pub include_preview: pulumi_wasm_rust::Output<Option<bool>>,
        /// Specifies the location in which to query for versions.
        #[builder(into)]
        pub location: pulumi_wasm_rust::Output<String>,
        /// A prefix filter for the versions of Kubernetes which should be returned; for example `1.` will return `1.9` to `1.14`, whereas `1.12` will return `1.12.2`.
        #[builder(into, default)]
        pub version_prefix: pulumi_wasm_rust::Output<Option<String>>,
    }
    #[allow(dead_code)]
    pub struct GetKubernetesServiceVersionsResult {
        /// The N-1 minor non-preview version and latest patch.
        pub default_version: pulumi_wasm_rust::Output<String>,
        /// The provider-assigned unique ID for this managed resource.
        pub id: pulumi_wasm_rust::Output<String>,
        pub include_preview: pulumi_wasm_rust::Output<Option<bool>>,
        /// The most recent version available. If `include_preview == false`, this is the most recent non-preview version available.
        pub latest_version: pulumi_wasm_rust::Output<String>,
        pub location: pulumi_wasm_rust::Output<String>,
        pub version_prefix: pulumi_wasm_rust::Output<Option<String>>,
        /// The list of all supported versions.
        pub versions: pulumi_wasm_rust::Output<Vec<String>>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn invoke(
        args: GetKubernetesServiceVersionsArgs,
    ) -> GetKubernetesServiceVersionsResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let include_preview_binding = args.include_preview.get_inner();
        let location_binding = args.location.get_inner();
        let version_prefix_binding = args.version_prefix.get_inner();
        let request = register_interface::ResourceInvokeRequest {
            token: "azure:containerservice/getKubernetesServiceVersions:getKubernetesServiceVersions"
                .into(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "includePreview".into(),
                    value: &include_preview_binding,
                },
                register_interface::ObjectField {
                    name: "location".into(),
                    value: &location_binding,
                },
                register_interface::ObjectField {
                    name: "versionPrefix".into(),
                    value: &version_prefix_binding,
                },
            ]),
            results: Vec::from([
                register_interface::ResultField {
                    name: "defaultVersion".into(),
                },
                register_interface::ResultField {
                    name: "id".into(),
                },
                register_interface::ResultField {
                    name: "includePreview".into(),
                },
                register_interface::ResultField {
                    name: "latestVersion".into(),
                },
                register_interface::ResultField {
                    name: "location".into(),
                },
                register_interface::ResultField {
                    name: "versionPrefix".into(),
                },
                register_interface::ResultField {
                    name: "versions".into(),
                },
            ]),
        };
        let o = register_interface::invoke(&request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        GetKubernetesServiceVersionsResult {
            default_version: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("defaultVersion").unwrap(),
            ),
            id: pulumi_wasm_rust::__private::into_domain(hashmap.remove("id").unwrap()),
            include_preview: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("includePreview").unwrap(),
            ),
            latest_version: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("latestVersion").unwrap(),
            ),
            location: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("location").unwrap(),
            ),
            version_prefix: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("versionPrefix").unwrap(),
            ),
            versions: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("versions").unwrap(),
            ),
        }
    }
}
