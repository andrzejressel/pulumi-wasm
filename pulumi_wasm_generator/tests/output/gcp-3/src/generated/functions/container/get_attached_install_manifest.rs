pub mod get_attached_install_manifest {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GetAttachedInstallManifestArgs {
        /// The name that will be used when creating the attached cluster resource.
        #[builder(into)]
        pub cluster_id: pulumi_wasm_rust::Output<String>,
        /// The location to list versions for.
        #[builder(into)]
        pub location: pulumi_wasm_rust::Output<String>,
        /// The platform version for the cluster. A list of valid values can be retrieved using the `gcp.container.getAttachedVersions` data source.
        #[builder(into)]
        pub platform_version: pulumi_wasm_rust::Output<String>,
        /// ID of the project to list available platform versions for. Should match the project the cluster will be deployed to.
        /// Defaults to the project that the provider is authenticated with.
        #[builder(into)]
        pub project: pulumi_wasm_rust::Output<String>,
    }
    #[allow(dead_code)]
    pub struct GetAttachedInstallManifestResult {
        pub cluster_id: pulumi_wasm_rust::Output<String>,
        /// The provider-assigned unique ID for this managed resource.
        pub id: pulumi_wasm_rust::Output<String>,
        pub location: pulumi_wasm_rust::Output<String>,
        /// A string with the YAML manifest that needs to be applied to the cluster.
        pub manifest: pulumi_wasm_rust::Output<String>,
        pub platform_version: pulumi_wasm_rust::Output<String>,
        pub project: pulumi_wasm_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn invoke(
        args: GetAttachedInstallManifestArgs,
    ) -> GetAttachedInstallManifestResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let cluster_id_binding = args.cluster_id.get_inner();
        let location_binding = args.location.get_inner();
        let platform_version_binding = args.platform_version.get_inner();
        let project_binding = args.project.get_inner();
        let request = register_interface::ResourceInvokeRequest {
            token: "gcp:container/getAttachedInstallManifest:getAttachedInstallManifest"
                .into(),
            version: super::super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "clusterId".into(),
                    value: &cluster_id_binding,
                },
                register_interface::ObjectField {
                    name: "location".into(),
                    value: &location_binding,
                },
                register_interface::ObjectField {
                    name: "platformVersion".into(),
                    value: &platform_version_binding,
                },
                register_interface::ObjectField {
                    name: "project".into(),
                    value: &project_binding,
                },
            ]),
            results: Vec::from([
                register_interface::ResultField {
                    name: "clusterId".into(),
                },
                register_interface::ResultField {
                    name: "id".into(),
                },
                register_interface::ResultField {
                    name: "location".into(),
                },
                register_interface::ResultField {
                    name: "manifest".into(),
                },
                register_interface::ResultField {
                    name: "platformVersion".into(),
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
        GetAttachedInstallManifestResult {
            cluster_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("clusterId").unwrap(),
            ),
            id: pulumi_wasm_rust::__private::into_domain(hashmap.remove("id").unwrap()),
            location: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("location").unwrap(),
            ),
            manifest: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("manifest").unwrap(),
            ),
            platform_version: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("platformVersion").unwrap(),
            ),
            project: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("project").unwrap(),
            ),
        }
    }
}
