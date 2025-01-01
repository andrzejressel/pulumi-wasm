pub mod get_engine_versions {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GetEngineVersionsArgs {
        /// The location (region or zone) to list versions for.
        /// Must exactly match the location the cluster will be deployed in, or listed
        /// versions may not be available. If `location`, `region`, and `zone` are not
        /// specified, the provider-level zone must be set and is used instead.
        #[builder(into, default)]
        pub location: pulumi_wasm_rust::Output<Option<String>>,
        /// ID of the project to list available cluster versions for. Should match the project the cluster will be deployed to.
        /// Defaults to the project that the provider is authenticated with.
        #[builder(into, default)]
        pub project: pulumi_wasm_rust::Output<Option<String>>,
        /// If provided, the provider will only return versions
        /// that match the string prefix. For example, `1.11.` will match all `1.11` series
        /// releases. Since this is just a string match, it's recommended that you append a
        /// `.` after minor versions to ensure that prefixes such as `1.1` don't match
        /// versions like `1.12.5-gke.10` accidentally. See [the docs on versioning schema](https://cloud.google.com/kubernetes-engine/versioning-and-upgrades#versioning_scheme)
        /// for full details on how version strings are formatted.
        #[builder(into, default)]
        pub version_prefix: pulumi_wasm_rust::Output<Option<String>>,
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
    pub fn invoke(args: GetEngineVersionsArgs) -> GetEngineVersionsResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let location_binding = args.location.get_inner();
        let project_binding = args.project.get_inner();
        let version_prefix_binding = args.version_prefix.get_inner();
        let request = register_interface::ResourceInvokeRequest {
            token: "gcp:container/getEngineVersions:getEngineVersions".into(),
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
            results: Vec::from([
                register_interface::ResultField {
                    name: "defaultClusterVersion".into(),
                },
                register_interface::ResultField {
                    name: "id".into(),
                },
                register_interface::ResultField {
                    name: "latestMasterVersion".into(),
                },
                register_interface::ResultField {
                    name: "latestNodeVersion".into(),
                },
                register_interface::ResultField {
                    name: "location".into(),
                },
                register_interface::ResultField {
                    name: "project".into(),
                },
                register_interface::ResultField {
                    name: "releaseChannelDefaultVersion".into(),
                },
                register_interface::ResultField {
                    name: "releaseChannelLatestVersion".into(),
                },
                register_interface::ResultField {
                    name: "validMasterVersions".into(),
                },
                register_interface::ResultField {
                    name: "validNodeVersions".into(),
                },
                register_interface::ResultField {
                    name: "versionPrefix".into(),
                },
            ]),
        };
        let o = register_interface::invoke(&request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        GetEngineVersionsResult {
            default_cluster_version: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("defaultClusterVersion").unwrap(),
            ),
            id: pulumi_wasm_rust::__private::into_domain(hashmap.remove("id").unwrap()),
            latest_master_version: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("latestMasterVersion").unwrap(),
            ),
            latest_node_version: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("latestNodeVersion").unwrap(),
            ),
            location: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("location").unwrap(),
            ),
            project: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("project").unwrap(),
            ),
            release_channel_default_version: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("releaseChannelDefaultVersion").unwrap(),
            ),
            release_channel_latest_version: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("releaseChannelLatestVersion").unwrap(),
            ),
            valid_master_versions: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("validMasterVersions").unwrap(),
            ),
            valid_node_versions: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("validNodeVersions").unwrap(),
            ),
            version_prefix: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("versionPrefix").unwrap(),
            ),
        }
    }
}
