#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod get_engine_version {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GetEngineVersionArgs {
        /// Whether the engine version must be an AWS-defined default version. Some engines have multiple default versions, such as for each major version. Using `default_only` may help avoid `multiple RDS engine versions` errors. See also `latest`.
        #[builder(into, default)]
        pub default_only: pulumi_gestalt_rust::InputOrOutput<Option<bool>>,
        /// Database engine. Engine values include `aurora`, `aurora-mysql`, `aurora-postgresql`, `docdb`, `mariadb`, `mysql`, `neptune`, `oracle-ee`, `oracle-se`, `oracle-se1`, `oracle-se2`, `postgres`, `sqlserver-ee`, `sqlserver-ex`, `sqlserver-se`, and `sqlserver-web`.
        ///
        /// The following arguments are optional:
        #[builder(into)]
        pub engine: pulumi_gestalt_rust::InputOrOutput<String>,
        /// One or more name/value pairs to use in filtering versions. There are several valid keys; for a full reference, check out [describe-db-engine-versions in the AWS CLI reference](https://awscli.amazonaws.com/v2/documentation/api/latest/reference/rds/describe-db-engine-versions.html).
        #[builder(into, default)]
        pub filters: pulumi_gestalt_rust::InputOrOutput<
            Option<Vec<super::super::super::types::rds::GetEngineVersionFilter>>,
        >,
        /// Whether the engine version must have one or more major upgrade targets. Not including `has_major_target` or setting it to `false` doesn't imply that there's no corresponding major upgrade target for the engine version.
        #[builder(into, default)]
        pub has_major_target: pulumi_gestalt_rust::InputOrOutput<Option<bool>>,
        /// Whether the engine version must have one or more minor upgrade targets. Not including `has_minor_target` or setting it to `false` doesn't imply that there's no corresponding minor upgrade target for the engine version.
        #[builder(into, default)]
        pub has_minor_target: pulumi_gestalt_rust::InputOrOutput<Option<bool>>,
        /// Whether the engine version `status` can either be `deprecated` or `available`. When not set or set to `false`, the engine version `status` will always be `available`.
        #[builder(into, default)]
        pub include_all: pulumi_gestalt_rust::InputOrOutput<Option<bool>>,
        /// Whether the engine version is the most recent version matching the other criteria. This is different from `default_only` in important ways: "default" relies on AWS-defined defaults, the latest version isn't always the default, and AWS might have multiple default versions for an engine. As a result, `default_only` might not prevent errors from `multiple RDS engine versions`, while `latest` will. (`latest` can be used with `default_only`.) **Note:** The data source uses a best-effort approach at selecting the latest version. Due to the complexity of version identifiers across engines and incomplete version date information provided by AWS, using `latest` may not always result in the engine version being the actual latest version.
        #[builder(into, default)]
        pub latest: pulumi_gestalt_rust::InputOrOutput<Option<bool>>,
        /// Name of a specific database parameter group family. Examples of parameter group families are `mysql8.0`, `mariadb10.4`, and `postgres12`.
        #[builder(into, default)]
        pub parameter_group_family: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Ordered list of preferred major version upgrade targets. The engine version will be the first match in the list unless the `latest` parameter is set to `true`. The engine version will be the default version if you don't include any criteria, such as `preferred_major_targets`.
        #[builder(into, default)]
        pub preferred_major_targets: pulumi_gestalt_rust::InputOrOutput<
            Option<Vec<String>>,
        >,
        /// Ordered list of preferred version upgrade targets. The engine version will be the first match in this list unless the `latest` parameter is set to `true`. The engine version will be the default version if you don't include any criteria, such as `preferred_upgrade_targets`.
        #[builder(into, default)]
        pub preferred_upgrade_targets: pulumi_gestalt_rust::InputOrOutput<
            Option<Vec<String>>,
        >,
        /// Ordered list of preferred versions. The engine version will be the first match in this list unless the `latest` parameter is set to `true`. The engine version will be the default version if you don't include any criteria, such as `preferred_versions`.
        #[builder(into, default)]
        pub preferred_versions: pulumi_gestalt_rust::InputOrOutput<Option<Vec<String>>>,
        #[builder(into, default)]
        pub version: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
    }
    #[allow(dead_code)]
    pub struct GetEngineVersionResult {
        /// Default character set for new instances of the engine version.
        pub default_character_set: pulumi_gestalt_rust::Output<String>,
        pub default_only: pulumi_gestalt_rust::Output<Option<bool>>,
        pub engine: pulumi_gestalt_rust::Output<String>,
        /// Description of the engine.
        pub engine_description: pulumi_gestalt_rust::Output<String>,
        /// Set of log types that the engine version has available for export to CloudWatch Logs.
        pub exportable_log_types: pulumi_gestalt_rust::Output<Vec<String>>,
        pub filters: pulumi_gestalt_rust::Output<
            Option<Vec<super::super::super::types::rds::GetEngineVersionFilter>>,
        >,
        pub has_major_target: pulumi_gestalt_rust::Output<Option<bool>>,
        pub has_minor_target: pulumi_gestalt_rust::Output<Option<bool>>,
        /// The provider-assigned unique ID for this managed resource.
        pub id: pulumi_gestalt_rust::Output<String>,
        pub include_all: pulumi_gestalt_rust::Output<Option<bool>>,
        pub latest: pulumi_gestalt_rust::Output<Option<bool>>,
        pub parameter_group_family: pulumi_gestalt_rust::Output<String>,
        pub preferred_major_targets: pulumi_gestalt_rust::Output<Option<Vec<String>>>,
        pub preferred_upgrade_targets: pulumi_gestalt_rust::Output<Option<Vec<String>>>,
        pub preferred_versions: pulumi_gestalt_rust::Output<Option<Vec<String>>>,
        /// Status of the engine version, either `available` or `deprecated`.
        pub status: pulumi_gestalt_rust::Output<String>,
        /// Set of character sets supported by th engine version.
        pub supported_character_sets: pulumi_gestalt_rust::Output<Vec<String>>,
        /// Set of features supported by the engine version.
        pub supported_feature_names: pulumi_gestalt_rust::Output<Vec<String>>,
        /// Set of supported engine version modes.
        pub supported_modes: pulumi_gestalt_rust::Output<Vec<String>>,
        /// Set of the time zones supported by the engine version.
        pub supported_timezones: pulumi_gestalt_rust::Output<Vec<String>>,
        /// Whether you can use Aurora global databases with the engine version.
        pub supports_global_databases: pulumi_gestalt_rust::Output<bool>,
        /// Whether the engine version supports Aurora Limitless Database.
        pub supports_limitless_database: pulumi_gestalt_rust::Output<bool>,
        /// Whether the engine version supports exporting the log types specified by `exportable_log_types` to CloudWatch Logs.
        pub supports_log_exports_to_cloudwatch: pulumi_gestalt_rust::Output<bool>,
        /// Whether you can use Aurora parallel query with the engine version.
        pub supports_parallel_query: pulumi_gestalt_rust::Output<bool>,
        /// Whether the engine version supports read replicas.
        pub supports_read_replica: pulumi_gestalt_rust::Output<bool>,
        /// Set of versions that are valid major version upgrades for the engine version.
        pub valid_major_targets: pulumi_gestalt_rust::Output<Vec<String>>,
        /// Set of versions that are valid minor version upgrades for the engine version.
        pub valid_minor_targets: pulumi_gestalt_rust::Output<Vec<String>>,
        /// Set of versions that are valid major or minor upgrades for the engine version.
        pub valid_upgrade_targets: pulumi_gestalt_rust::Output<Vec<String>>,
        pub version: pulumi_gestalt_rust::Output<String>,
        /// Complete engine version.
        pub version_actual: pulumi_gestalt_rust::Output<String>,
        /// Description of the engine version.
        pub version_description: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn invoke(
        context: &pulumi_gestalt_rust::Context,
        args: GetEngineVersionArgs,
    ) -> GetEngineVersionResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let default_only_binding = args.default_only.get_output(context);
        let engine_binding = args.engine.get_output(context);
        let filters_binding = args.filters.get_output(context);
        let has_major_target_binding = args.has_major_target.get_output(context);
        let has_minor_target_binding = args.has_minor_target.get_output(context);
        let include_all_binding = args.include_all.get_output(context);
        let latest_binding = args.latest.get_output(context);
        let parameter_group_family_binding = args
            .parameter_group_family
            .get_output(context);
        let preferred_major_targets_binding = args
            .preferred_major_targets
            .get_output(context);
        let preferred_upgrade_targets_binding = args
            .preferred_upgrade_targets
            .get_output(context);
        let preferred_versions_binding = args.preferred_versions.get_output(context);
        let version_binding = args.version.get_output(context);
        let request = pulumi_gestalt_rust::InvokeResourceRequest {
            token: "aws:rds/getEngineVersion:getEngineVersion".into(),
            version: super::super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "defaultOnly".into(),
                    value: &default_only_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "engine".into(),
                    value: &engine_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "filters".into(),
                    value: &filters_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "hasMajorTarget".into(),
                    value: &has_major_target_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "hasMinorTarget".into(),
                    value: &has_minor_target_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "includeAll".into(),
                    value: &include_all_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "latest".into(),
                    value: &latest_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "parameterGroupFamily".into(),
                    value: &parameter_group_family_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "preferredMajorTargets".into(),
                    value: &preferred_major_targets_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "preferredUpgradeTargets".into(),
                    value: &preferred_upgrade_targets_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "preferredVersions".into(),
                    value: &preferred_versions_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "version".into(),
                    value: &version_binding.drop_type(),
                },
            ],
        };
        let o = context.invoke_resource(request);
        GetEngineVersionResult {
            default_character_set: o.get_field("defaultCharacterSet"),
            default_only: o.get_field("defaultOnly"),
            engine: o.get_field("engine"),
            engine_description: o.get_field("engineDescription"),
            exportable_log_types: o.get_field("exportableLogTypes"),
            filters: o.get_field("filters"),
            has_major_target: o.get_field("hasMajorTarget"),
            has_minor_target: o.get_field("hasMinorTarget"),
            id: o.get_field("id"),
            include_all: o.get_field("includeAll"),
            latest: o.get_field("latest"),
            parameter_group_family: o.get_field("parameterGroupFamily"),
            preferred_major_targets: o.get_field("preferredMajorTargets"),
            preferred_upgrade_targets: o.get_field("preferredUpgradeTargets"),
            preferred_versions: o.get_field("preferredVersions"),
            status: o.get_field("status"),
            supported_character_sets: o.get_field("supportedCharacterSets"),
            supported_feature_names: o.get_field("supportedFeatureNames"),
            supported_modes: o.get_field("supportedModes"),
            supported_timezones: o.get_field("supportedTimezones"),
            supports_global_databases: o.get_field("supportsGlobalDatabases"),
            supports_limitless_database: o.get_field("supportsLimitlessDatabase"),
            supports_log_exports_to_cloudwatch: o
                .get_field("supportsLogExportsToCloudwatch"),
            supports_parallel_query: o.get_field("supportsParallelQuery"),
            supports_read_replica: o.get_field("supportsReadReplica"),
            valid_major_targets: o.get_field("validMajorTargets"),
            valid_minor_targets: o.get_field("validMinorTargets"),
            valid_upgrade_targets: o.get_field("validUpgradeTargets"),
            version: o.get_field("version"),
            version_actual: o.get_field("versionActual"),
            version_description: o.get_field("versionDescription"),
        }
    }
}
