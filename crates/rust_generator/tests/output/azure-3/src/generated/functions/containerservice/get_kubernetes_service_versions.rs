#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod get_kubernetes_service_versions {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GetKubernetesServiceVersionsArgs {
        /// Should Preview versions of Kubernetes in AKS be included? Defaults to `true`
        #[builder(into, default)]
        pub include_preview: pulumi_gestalt_rust::InputOrOutput<Option<bool>>,
        /// Specifies the location in which to query for versions.
        #[builder(into)]
        pub location: pulumi_gestalt_rust::InputOrOutput<String>,
        /// A prefix filter for the versions of Kubernetes which should be returned; for example `1.` will return `1.9` to `1.14`, whereas `1.12` will return `1.12.2`.
        #[builder(into, default)]
        pub version_prefix: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
    }
    #[allow(dead_code)]
    pub struct GetKubernetesServiceVersionsResult {
        /// The N-1 minor non-preview version and latest patch.
        pub default_version: pulumi_gestalt_rust::Output<String>,
        /// The provider-assigned unique ID for this managed resource.
        pub id: pulumi_gestalt_rust::Output<String>,
        pub include_preview: pulumi_gestalt_rust::Output<Option<bool>>,
        /// The most recent version available. If `include_preview == false`, this is the most recent non-preview version available.
        pub latest_version: pulumi_gestalt_rust::Output<String>,
        pub location: pulumi_gestalt_rust::Output<String>,
        pub version_prefix: pulumi_gestalt_rust::Output<Option<String>>,
        /// The list of all supported versions.
        pub versions: pulumi_gestalt_rust::Output<Vec<String>>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn invoke(
        context: &pulumi_gestalt_rust::Context,
        args: GetKubernetesServiceVersionsArgs,
    ) -> GetKubernetesServiceVersionsResult {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::register_interface;
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let include_preview_binding = args.include_preview.get_output(context);
        let location_binding = args.location.get_output(context);
        let version_prefix_binding = args.version_prefix.get_output(context);
        let request = pulumi_gestalt_rust::InvokeResourceRequest {
            token: "azure:containerservice/getKubernetesServiceVersions:getKubernetesServiceVersions"
                .into(),
            version: super::super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "includePreview".into(),
                    value: include_preview_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "location".into(),
                    value: location_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "versionPrefix".into(),
                    value: version_prefix_binding.get_id(),
                },
            ],
        };
        let o = context.invoke_resource(request);
        GetKubernetesServiceVersionsResult {
            default_version: o.get_field("defaultVersion"),
            id: o.get_field("id"),
            include_preview: o.get_field("includePreview"),
            latest_version: o.get_field("latestVersion"),
            location: o.get_field("location"),
            version_prefix: o.get_field("versionPrefix"),
            versions: o.get_field("versions"),
        }
    }
}
