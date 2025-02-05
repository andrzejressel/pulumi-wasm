pub mod get_kubernetes_service_versions {
    #[derive(pulumi_wasm_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GetKubernetesServiceVersionsArgs {
        /// Should Preview versions of Kubernetes in AKS be included? Defaults to `true`
        #[builder(into, default)]
        pub include_preview: pulumi_wasm_rust::InputOrOutput<Option<bool>>,
        /// Specifies the location in which to query for versions.
        #[builder(into)]
        pub location: pulumi_wasm_rust::InputOrOutput<String>,
        /// A prefix filter for the versions of Kubernetes which should be returned; for example `1.` will return `1.9` to `1.14`, whereas `1.12` will return `1.12.2`.
        #[builder(into, default)]
        pub version_prefix: pulumi_wasm_rust::InputOrOutput<Option<String>>,
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
        context: &pulumi_wasm_rust::PulumiContext,
        args: GetKubernetesServiceVersionsArgs,
    ) -> GetKubernetesServiceVersionsResult {
        use pulumi_wasm_rust::__private::pulumi_gestalt_adapter_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let include_preview_binding = args
            .include_preview
            .get_output(context)
            .get_inner();
        let location_binding = args.location.get_output(context).get_inner();
        let version_prefix_binding = args.version_prefix.get_output(context).get_inner();
        let request = register_interface::ResourceInvokeRequest {
            token: "azure:containerservice/getKubernetesServiceVersions:getKubernetesServiceVersions"
                .into(),
            version: super::super::super::get_version(),
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
        };
        let o = register_interface::invoke(context.get_inner(), &request);
        GetKubernetesServiceVersionsResult {
            default_version: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("defaultVersion"),
            ),
            id: pulumi_wasm_rust::__private::into_domain(o.extract_field("id")),
            include_preview: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("includePreview"),
            ),
            latest_version: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("latestVersion"),
            ),
            location: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("location"),
            ),
            version_prefix: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("versionPrefix"),
            ),
            versions: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("versions"),
            ),
        }
    }
}
