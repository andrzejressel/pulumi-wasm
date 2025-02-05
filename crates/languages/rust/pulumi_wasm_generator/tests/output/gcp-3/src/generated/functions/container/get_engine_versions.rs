pub mod get_engine_versions {
    #[derive(pulumi_wasm_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GetEngineVersionsArgs {
        /// The location (region or zone) to list versions for.
        /// Must exactly match the location the cluster will be deployed in, or listed
        /// versions may not be available. If `location`, `region`, and `zone` are not
        /// specified, the provider-level zone must be set and is used instead.
        #[builder(into, default)]
        pub location: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        /// ID of the project to list available cluster versions for. Should match the project the cluster will be deployed to.
        /// Defaults to the project that the provider is authenticated with.
        #[builder(into, default)]
        pub project: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        /// If provided, the provider will only return versions
        /// that match the string prefix. For example, `1.11.` will match all `1.11` series
        /// releases. Since this is just a string match, it's recommended that you append a
        /// `.` after minor versions to ensure that prefixes such as `1.1` don't match
        /// versions like `1.12.5-gke.10` accidentally. See [the docs on versioning schema](https://cloud.google.com/kubernetes-engine/versioning-and-upgrades#versioning_scheme)
        /// for full details on how version strings are formatted.
        #[builder(into, default)]
        pub version_prefix: pulumi_wasm_rust::InputOrOutput<Option<String>>,
    }
    #[allow(dead_code)]
    pub struct GetEngineVersionsResult {
        /// Version of Kubernetes the service deploys by default.
        pub default_cluster_version: pulumi_wasm_rust::Output<String>,
        /// The provider-assigned unique ID for this managed resource.
        pub id: pulumi_wasm_rust::Output<String>,
        /// The latest version available in the given zone for use with master instances.
        pub latest_master_version: pulumi_wasm_rust::Output<String>,
        /// The latest version available in the given zone for use with node instances.
        pub latest_node_version: pulumi_wasm_rust::Output<String>,
        pub location: pulumi_wasm_rust::Output<Option<String>>,
        pub project: pulumi_wasm_rust::Output<Option<String>>,
        /// A map from a release channel name to the channel's default version. See the docs on [available release channel names](https://cloud.google.com/kubernetes-engine/docs/reference/rest/v1/projects.locations.clusters#Cluster.Channel_1) for more details.
        pub release_channel_default_version: pulumi_wasm_rust::Output<
            std::collections::HashMap<String, String>,
        >,
        /// A map from a release channel name to the channel's latest version. See the docs on [available release channel names](https://cloud.google.com/kubernetes-engine/docs/reference/rest/v1/projects.locations.clusters#Cluster.Channel_1) for more details.
        pub release_channel_latest_version: pulumi_wasm_rust::Output<
            std::collections::HashMap<String, String>,
        >,
        /// A list of versions available in the given zone for use with master instances.
        pub valid_master_versions: pulumi_wasm_rust::Output<Vec<String>>,
        /// A list of versions available in the given zone for use with node instances.
        pub valid_node_versions: pulumi_wasm_rust::Output<Vec<String>>,
        pub version_prefix: pulumi_wasm_rust::Output<Option<String>>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn invoke(
        context: &pulumi_wasm_rust::PulumiContext,
        args: GetEngineVersionsArgs,
    ) -> GetEngineVersionsResult {
        use pulumi_wasm_rust::__private::pulumi_gestalt_adapter_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let location_binding = args.location.get_output(context).get_inner();
        let project_binding = args.project.get_output(context).get_inner();
        let version_prefix_binding = args.version_prefix.get_output(context).get_inner();
        let request = register_interface::ResourceInvokeRequest {
            token: "gcp:container/getEngineVersions:getEngineVersions".into(),
            version: super::super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "location".into(),
                    value: &location_binding,
                },
                register_interface::ObjectField {
                    name: "project".into(),
                    value: &project_binding,
                },
                register_interface::ObjectField {
                    name: "versionPrefix".into(),
                    value: &version_prefix_binding,
                },
            ]),
        };
        let o = register_interface::invoke(context.get_inner(), &request);
        GetEngineVersionsResult {
            default_cluster_version: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("defaultClusterVersion"),
            ),
            id: pulumi_wasm_rust::__private::into_domain(o.extract_field("id")),
            latest_master_version: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("latestMasterVersion"),
            ),
            latest_node_version: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("latestNodeVersion"),
            ),
            location: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("location"),
            ),
            project: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("project"),
            ),
            release_channel_default_version: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("releaseChannelDefaultVersion"),
            ),
            release_channel_latest_version: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("releaseChannelLatestVersion"),
            ),
            valid_master_versions: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("validMasterVersions"),
            ),
            valid_node_versions: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("validNodeVersions"),
            ),
            version_prefix: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("versionPrefix"),
            ),
        }
    }
}
