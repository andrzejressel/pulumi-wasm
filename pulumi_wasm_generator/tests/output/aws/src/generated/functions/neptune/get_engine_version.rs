pub mod get_engine_version {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GetEngineVersionArgs {
        /// DB engine. (Default: `neptune`)
        #[builder(into, default)]
        pub engine: pulumi_wasm_rust::Output<Option<String>>,
        /// Name of a specific DB parameter group family. An example parameter group family is `neptune1`.
        #[builder(into, default)]
        pub parameter_group_family: pulumi_wasm_rust::Output<Option<String>>,
        /// Ordered list of preferred engine versions. The first match in this list will be returned. If no preferred matches are found and the original search returned more than one result, an error is returned. If both the `version` and `preferred_versions` arguments are not configured, the data source will return the default version for the engine.
        #[builder(into, default)]
        pub preferred_versions: pulumi_wasm_rust::Output<Option<Vec<String>>>,
        /// Version of the DB engine. For example, `1.0.1.0`, `1.0.2.2`, and `1.0.3.0`. If both the `version` and `preferred_versions` arguments are not configured, the data source will return the default version for the engine.
        #[builder(into, default)]
        pub version: pulumi_wasm_rust::Output<Option<String>>,
    }
    #[allow(dead_code)]
    pub struct GetEngineVersionResult {
        pub engine: pulumi_wasm_rust::Output<Option<String>>,
        /// Description of the database engine.
        pub engine_description: pulumi_wasm_rust::Output<String>,
        /// Set of log types that the database engine has available for export to CloudWatch Logs.
        pub exportable_log_types: pulumi_wasm_rust::Output<Vec<String>>,
        /// The provider-assigned unique ID for this managed resource.
        pub id: pulumi_wasm_rust::Output<String>,
        pub parameter_group_family: pulumi_wasm_rust::Output<String>,
        pub preferred_versions: pulumi_wasm_rust::Output<Option<Vec<String>>>,
        /// Set of the time zones supported by this engine.
        pub supported_timezones: pulumi_wasm_rust::Output<Vec<String>>,
        /// Indicates whether the engine version supports exporting the log types specified by `exportable_log_types` to CloudWatch Logs.
        pub supports_log_exports_to_cloudwatch: pulumi_wasm_rust::Output<bool>,
        /// Indicates whether the database engine version supports read replicas.
        pub supports_read_replica: pulumi_wasm_rust::Output<bool>,
        /// Set of engine versions that this database engine version can be upgraded to.
        pub valid_upgrade_targets: pulumi_wasm_rust::Output<Vec<String>>,
        pub version: pulumi_wasm_rust::Output<String>,
        /// Description of the database engine version.
        pub version_description: pulumi_wasm_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn invoke(args: GetEngineVersionArgs) -> GetEngineVersionResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let engine_binding = args.engine.get_inner();
        let parameter_group_family_binding = args.parameter_group_family.get_inner();
        let preferred_versions_binding = args.preferred_versions.get_inner();
        let version_binding = args.version.get_inner();
        let request = register_interface::ResourceInvokeRequest {
            token: "aws:neptune/getEngineVersion:getEngineVersion".into(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "engine".into(),
                    value: &engine_binding,
                },
                register_interface::ObjectField {
                    name: "parameterGroupFamily".into(),
                    value: &parameter_group_family_binding,
                },
                register_interface::ObjectField {
                    name: "preferredVersions".into(),
                    value: &preferred_versions_binding,
                },
                register_interface::ObjectField {
                    name: "version".into(),
                    value: &version_binding,
                },
            ]),
            results: Vec::from([
                register_interface::ResultField {
                    name: "engine".into(),
                },
                register_interface::ResultField {
                    name: "engineDescription".into(),
                },
                register_interface::ResultField {
                    name: "exportableLogTypes".into(),
                },
                register_interface::ResultField {
                    name: "id".into(),
                },
                register_interface::ResultField {
                    name: "parameterGroupFamily".into(),
                },
                register_interface::ResultField {
                    name: "preferredVersions".into(),
                },
                register_interface::ResultField {
                    name: "supportedTimezones".into(),
                },
                register_interface::ResultField {
                    name: "supportsLogExportsToCloudwatch".into(),
                },
                register_interface::ResultField {
                    name: "supportsReadReplica".into(),
                },
                register_interface::ResultField {
                    name: "validUpgradeTargets".into(),
                },
                register_interface::ResultField {
                    name: "version".into(),
                },
                register_interface::ResultField {
                    name: "versionDescription".into(),
                },
            ]),
        };
        let o = register_interface::invoke(&request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        GetEngineVersionResult {
            engine: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("engine").unwrap(),
            ),
            engine_description: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("engineDescription").unwrap(),
            ),
            exportable_log_types: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("exportableLogTypes").unwrap(),
            ),
            id: pulumi_wasm_rust::__private::into_domain(hashmap.remove("id").unwrap()),
            parameter_group_family: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("parameterGroupFamily").unwrap(),
            ),
            preferred_versions: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("preferredVersions").unwrap(),
            ),
            supported_timezones: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("supportedTimezones").unwrap(),
            ),
            supports_log_exports_to_cloudwatch: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("supportsLogExportsToCloudwatch").unwrap(),
            ),
            supports_read_replica: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("supportsReadReplica").unwrap(),
            ),
            valid_upgrade_targets: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("validUpgradeTargets").unwrap(),
            ),
            version: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("version").unwrap(),
            ),
            version_description: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("versionDescription").unwrap(),
            ),
        }
    }
}
