pub mod get_engine_version {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GetEngineVersionArgs {
        /// Whether the engine version must be an AWS-defined default version. Some engines have multiple default versions, such as for each major version. Using `default_only` may help avoid `multiple RDS engine versions` errors. See also `latest`.
        #[builder(into, default)]
        pub default_only: pulumi_wasm_rust::Output<Option<bool>>,
        /// Database engine. Engine values include `aurora`, `aurora-mysql`, `aurora-postgresql`, `docdb`, `mariadb`, `mysql`, `neptune`, `oracle-ee`, `oracle-se`, `oracle-se1`, `oracle-se2`, `postgres`, `sqlserver-ee`, `sqlserver-ex`, `sqlserver-se`, and `sqlserver-web`.
        ///
        /// The following arguments are optional:
        #[builder(into)]
        pub engine: pulumi_wasm_rust::Output<String>,
        /// One or more name/value pairs to use in filtering versions. There are several valid keys; for a full reference, check out [describe-db-engine-versions in the AWS CLI reference](https://awscli.amazonaws.com/v2/documentation/api/latest/reference/rds/describe-db-engine-versions.html).
        #[builder(into, default)]
        pub filters: pulumi_wasm_rust::Output<
            Option<Vec<super::super::super::types::rds::GetEngineVersionFilter>>,
        >,
        /// Whether the engine version must have one or more major upgrade targets. Not including `has_major_target` or setting it to `false` doesn't imply that there's no corresponding major upgrade target for the engine version.
        #[builder(into, default)]
        pub has_major_target: pulumi_wasm_rust::Output<Option<bool>>,
        /// Whether the engine version must have one or more minor upgrade targets. Not including `has_minor_target` or setting it to `false` doesn't imply that there's no corresponding minor upgrade target for the engine version.
        #[builder(into, default)]
        pub has_minor_target: pulumi_wasm_rust::Output<Option<bool>>,
        /// Whether the engine version `status` can either be `deprecated` or `available`. When not set or set to `false`, the engine version `status` will always be `available`.
        #[builder(into, default)]
        pub include_all: pulumi_wasm_rust::Output<Option<bool>>,
        /// Whether the engine version is the most recent version matching the other criteria. This is different from `default_only` in important ways: "default" relies on AWS-defined defaults, the latest version isn't always the default, and AWS might have multiple default versions for an engine. As a result, `default_only` might not prevent errors from `multiple RDS engine versions`, while `latest` will. (`latest` can be used with `default_only`.) **Note:** The data source uses a best-effort approach at selecting the latest version. Due to the complexity of version identifiers across engines and incomplete version date information provided by AWS, using `latest` may not always result in the engine version being the actual latest version.
        #[builder(into, default)]
        pub latest: pulumi_wasm_rust::Output<Option<bool>>,
        /// Name of a specific database parameter group family. Examples of parameter group families are `mysql8.0`, `mariadb10.4`, and `postgres12`.
        #[builder(into, default)]
        pub parameter_group_family: pulumi_wasm_rust::Output<Option<String>>,
        /// Ordered list of preferred major version upgrade targets. The engine version will be the first match in the list unless the `latest` parameter is set to `true`. The engine version will be the default version if you don't include any criteria, such as `preferred_major_targets`.
        #[builder(into, default)]
        pub preferred_major_targets: pulumi_wasm_rust::Output<Option<Vec<String>>>,
        /// Ordered list of preferred version upgrade targets. The engine version will be the first match in this list unless the `latest` parameter is set to `true`. The engine version will be the default version if you don't include any criteria, such as `preferred_upgrade_targets`.
        #[builder(into, default)]
        pub preferred_upgrade_targets: pulumi_wasm_rust::Output<Option<Vec<String>>>,
        /// Ordered list of preferred versions. The engine version will be the first match in this list unless the `latest` parameter is set to `true`. The engine version will be the default version if you don't include any criteria, such as `preferred_versions`.
        #[builder(into, default)]
        pub preferred_versions: pulumi_wasm_rust::Output<Option<Vec<String>>>,
        #[builder(into, default)]
        pub version: pulumi_wasm_rust::Output<Option<String>>,
    }
    #[allow(dead_code)]
    pub struct GetEngineVersionResult {
        /// Default character set for new instances of the engine version.
        pub default_character_set: pulumi_wasm_rust::Output<String>,
        pub default_only: pulumi_wasm_rust::Output<Option<bool>>,
        pub engine: pulumi_wasm_rust::Output<String>,
        /// Description of the engine.
        pub engine_description: pulumi_wasm_rust::Output<String>,
        /// Set of log types that the engine version has available for export to CloudWatch Logs.
        pub exportable_log_types: pulumi_wasm_rust::Output<Vec<String>>,
        pub filters: pulumi_wasm_rust::Output<
            Option<Vec<super::super::super::types::rds::GetEngineVersionFilter>>,
        >,
        pub has_major_target: pulumi_wasm_rust::Output<Option<bool>>,
        pub has_minor_target: pulumi_wasm_rust::Output<Option<bool>>,
        /// The provider-assigned unique ID for this managed resource.
        pub id: pulumi_wasm_rust::Output<String>,
        pub include_all: pulumi_wasm_rust::Output<Option<bool>>,
        pub latest: pulumi_wasm_rust::Output<Option<bool>>,
        pub parameter_group_family: pulumi_wasm_rust::Output<String>,
        pub preferred_major_targets: pulumi_wasm_rust::Output<Option<Vec<String>>>,
        pub preferred_upgrade_targets: pulumi_wasm_rust::Output<Option<Vec<String>>>,
        pub preferred_versions: pulumi_wasm_rust::Output<Option<Vec<String>>>,
        /// Status of the engine version, either `available` or `deprecated`.
        pub status: pulumi_wasm_rust::Output<String>,
        /// Set of character sets supported by th engine version.
        pub supported_character_sets: pulumi_wasm_rust::Output<Vec<String>>,
        /// Set of features supported by the engine version.
        pub supported_feature_names: pulumi_wasm_rust::Output<Vec<String>>,
        /// Set of supported engine version modes.
        pub supported_modes: pulumi_wasm_rust::Output<Vec<String>>,
        /// Set of the time zones supported by the engine version.
        pub supported_timezones: pulumi_wasm_rust::Output<Vec<String>>,
        /// Whether you can use Aurora global databases with the engine version.
        pub supports_global_databases: pulumi_wasm_rust::Output<bool>,
        /// Whether the engine version supports Aurora Limitless Database.
        pub supports_limitless_database: pulumi_wasm_rust::Output<bool>,
        /// Whether the engine version supports exporting the log types specified by `exportable_log_types` to CloudWatch Logs.
        pub supports_log_exports_to_cloudwatch: pulumi_wasm_rust::Output<bool>,
        /// Whether you can use Aurora parallel query with the engine version.
        pub supports_parallel_query: pulumi_wasm_rust::Output<bool>,
        /// Whether the engine version supports read replicas.
        pub supports_read_replica: pulumi_wasm_rust::Output<bool>,
        /// Set of versions that are valid major version upgrades for the engine version.
        pub valid_major_targets: pulumi_wasm_rust::Output<Vec<String>>,
        /// Set of versions that are valid minor version upgrades for the engine version.
        pub valid_minor_targets: pulumi_wasm_rust::Output<Vec<String>>,
        /// Set of versions that are valid major or minor upgrades for the engine version.
        pub valid_upgrade_targets: pulumi_wasm_rust::Output<Vec<String>>,
        pub version: pulumi_wasm_rust::Output<String>,
        /// Complete engine version.
        pub version_actual: pulumi_wasm_rust::Output<String>,
        /// Description of the engine version.
        pub version_description: pulumi_wasm_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn invoke(args: GetEngineVersionArgs) -> GetEngineVersionResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let default_only_binding = args.default_only.get_inner();
        let engine_binding = args.engine.get_inner();
        let filters_binding = args.filters.get_inner();
        let has_major_target_binding = args.has_major_target.get_inner();
        let has_minor_target_binding = args.has_minor_target.get_inner();
        let include_all_binding = args.include_all.get_inner();
        let latest_binding = args.latest.get_inner();
        let parameter_group_family_binding = args.parameter_group_family.get_inner();
        let preferred_major_targets_binding = args.preferred_major_targets.get_inner();
        let preferred_upgrade_targets_binding = args
            .preferred_upgrade_targets
            .get_inner();
        let preferred_versions_binding = args.preferred_versions.get_inner();
        let version_binding = args.version.get_inner();
        let request = register_interface::ResourceInvokeRequest {
            token: "aws:rds/getEngineVersion:getEngineVersion".into(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "defaultOnly".into(),
                    value: &default_only_binding,
                },
                register_interface::ObjectField {
                    name: "engine".into(),
                    value: &engine_binding,
                },
                register_interface::ObjectField {
                    name: "filters".into(),
                    value: &filters_binding,
                },
                register_interface::ObjectField {
                    name: "hasMajorTarget".into(),
                    value: &has_major_target_binding,
                },
                register_interface::ObjectField {
                    name: "hasMinorTarget".into(),
                    value: &has_minor_target_binding,
                },
                register_interface::ObjectField {
                    name: "includeAll".into(),
                    value: &include_all_binding,
                },
                register_interface::ObjectField {
                    name: "latest".into(),
                    value: &latest_binding,
                },
                register_interface::ObjectField {
                    name: "parameterGroupFamily".into(),
                    value: &parameter_group_family_binding,
                },
                register_interface::ObjectField {
                    name: "preferredMajorTargets".into(),
                    value: &preferred_major_targets_binding,
                },
                register_interface::ObjectField {
                    name: "preferredUpgradeTargets".into(),
                    value: &preferred_upgrade_targets_binding,
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
                    name: "defaultCharacterSet".into(),
                },
                register_interface::ResultField {
                    name: "defaultOnly".into(),
                },
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
                    name: "filters".into(),
                },
                register_interface::ResultField {
                    name: "hasMajorTarget".into(),
                },
                register_interface::ResultField {
                    name: "hasMinorTarget".into(),
                },
                register_interface::ResultField {
                    name: "id".into(),
                },
                register_interface::ResultField {
                    name: "includeAll".into(),
                },
                register_interface::ResultField {
                    name: "latest".into(),
                },
                register_interface::ResultField {
                    name: "parameterGroupFamily".into(),
                },
                register_interface::ResultField {
                    name: "preferredMajorTargets".into(),
                },
                register_interface::ResultField {
                    name: "preferredUpgradeTargets".into(),
                },
                register_interface::ResultField {
                    name: "preferredVersions".into(),
                },
                register_interface::ResultField {
                    name: "status".into(),
                },
                register_interface::ResultField {
                    name: "supportedCharacterSets".into(),
                },
                register_interface::ResultField {
                    name: "supportedFeatureNames".into(),
                },
                register_interface::ResultField {
                    name: "supportedModes".into(),
                },
                register_interface::ResultField {
                    name: "supportedTimezones".into(),
                },
                register_interface::ResultField {
                    name: "supportsGlobalDatabases".into(),
                },
                register_interface::ResultField {
                    name: "supportsLimitlessDatabase".into(),
                },
                register_interface::ResultField {
                    name: "supportsLogExportsToCloudwatch".into(),
                },
                register_interface::ResultField {
                    name: "supportsParallelQuery".into(),
                },
                register_interface::ResultField {
                    name: "supportsReadReplica".into(),
                },
                register_interface::ResultField {
                    name: "validMajorTargets".into(),
                },
                register_interface::ResultField {
                    name: "validMinorTargets".into(),
                },
                register_interface::ResultField {
                    name: "validUpgradeTargets".into(),
                },
                register_interface::ResultField {
                    name: "version".into(),
                },
                register_interface::ResultField {
                    name: "versionActual".into(),
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
            default_character_set: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("defaultCharacterSet").unwrap(),
            ),
            default_only: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("defaultOnly").unwrap(),
            ),
            engine: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("engine").unwrap(),
            ),
            engine_description: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("engineDescription").unwrap(),
            ),
            exportable_log_types: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("exportableLogTypes").unwrap(),
            ),
            filters: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("filters").unwrap(),
            ),
            has_major_target: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("hasMajorTarget").unwrap(),
            ),
            has_minor_target: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("hasMinorTarget").unwrap(),
            ),
            id: pulumi_wasm_rust::__private::into_domain(hashmap.remove("id").unwrap()),
            include_all: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("includeAll").unwrap(),
            ),
            latest: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("latest").unwrap(),
            ),
            parameter_group_family: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("parameterGroupFamily").unwrap(),
            ),
            preferred_major_targets: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("preferredMajorTargets").unwrap(),
            ),
            preferred_upgrade_targets: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("preferredUpgradeTargets").unwrap(),
            ),
            preferred_versions: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("preferredVersions").unwrap(),
            ),
            status: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("status").unwrap(),
            ),
            supported_character_sets: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("supportedCharacterSets").unwrap(),
            ),
            supported_feature_names: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("supportedFeatureNames").unwrap(),
            ),
            supported_modes: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("supportedModes").unwrap(),
            ),
            supported_timezones: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("supportedTimezones").unwrap(),
            ),
            supports_global_databases: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("supportsGlobalDatabases").unwrap(),
            ),
            supports_limitless_database: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("supportsLimitlessDatabase").unwrap(),
            ),
            supports_log_exports_to_cloudwatch: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("supportsLogExportsToCloudwatch").unwrap(),
            ),
            supports_parallel_query: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("supportsParallelQuery").unwrap(),
            ),
            supports_read_replica: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("supportsReadReplica").unwrap(),
            ),
            valid_major_targets: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("validMajorTargets").unwrap(),
            ),
            valid_minor_targets: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("validMinorTargets").unwrap(),
            ),
            valid_upgrade_targets: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("validUpgradeTargets").unwrap(),
            ),
            version: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("version").unwrap(),
            ),
            version_actual: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("versionActual").unwrap(),
            ),
            version_description: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("versionDescription").unwrap(),
            ),
        }
    }
}
