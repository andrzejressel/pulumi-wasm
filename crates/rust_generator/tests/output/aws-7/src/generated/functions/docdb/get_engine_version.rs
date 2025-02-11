#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod get_engine_version {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GetEngineVersionArgs {
        /// DB engine. (Default: `docdb`)
        #[builder(into, default)]
        pub engine: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Name of a specific DB parameter group family. An example parameter group family is `docdb3.6`.
        #[builder(into, default)]
        pub parameter_group_family: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Ordered list of preferred engine versions. The first match in this list will be returned. If no preferred matches are found and the original search returned more than one result, an error is returned. If both the `version` and `preferred_versions` arguments are not configured, the data source will return the default version for the engine.
        #[builder(into, default)]
        pub preferred_versions: pulumi_gestalt_rust::InputOrOutput<Option<Vec<String>>>,
        /// Version of the DB engine. For example, `3.6.0`. If `version` and `preferred_versions` are not set, the data source will provide information for the AWS-defined default version. If both the `version` and `preferred_versions` arguments are not configured, the data source will return the default version for the engine.
        #[builder(into, default)]
        pub version: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
    }
    #[allow(dead_code)]
    pub struct GetEngineVersionResult {
        pub engine: pulumi_gestalt_rust::Output<Option<String>>,
        /// Description of the database engine.
        pub engine_description: pulumi_gestalt_rust::Output<String>,
        /// Set of log types that the database engine has available for export to CloudWatch Logs.
        pub exportable_log_types: pulumi_gestalt_rust::Output<Vec<String>>,
        /// The provider-assigned unique ID for this managed resource.
        pub id: pulumi_gestalt_rust::Output<String>,
        pub parameter_group_family: pulumi_gestalt_rust::Output<String>,
        pub preferred_versions: pulumi_gestalt_rust::Output<Option<Vec<String>>>,
        /// Indicates whether the engine version supports exporting the log types specified by `exportable_log_types` to CloudWatch Logs.
        pub supports_log_exports_to_cloudwatch: pulumi_gestalt_rust::Output<bool>,
        /// A set of engine versions that this database engine version can be upgraded to.
        pub valid_upgrade_targets: pulumi_gestalt_rust::Output<Vec<String>>,
        pub version: pulumi_gestalt_rust::Output<String>,
        /// Description of the database engine version.
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
        let engine_binding = args.engine.get_output(context);
        let parameter_group_family_binding = args
            .parameter_group_family
            .get_output(context);
        let preferred_versions_binding = args.preferred_versions.get_output(context);
        let version_binding = args.version.get_output(context);
        let request = pulumi_gestalt_rust::InvokeResourceRequest {
            token: "aws:docdb/getEngineVersion:getEngineVersion".into(),
            version: super::super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "engine".into(),
                    value: &engine_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "parameterGroupFamily".into(),
                    value: &parameter_group_family_binding.drop_type(),
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
            engine: o.get_field("engine"),
            engine_description: o.get_field("engineDescription"),
            exportable_log_types: o.get_field("exportableLogTypes"),
            id: o.get_field("id"),
            parameter_group_family: o.get_field("parameterGroupFamily"),
            preferred_versions: o.get_field("preferredVersions"),
            supports_log_exports_to_cloudwatch: o
                .get_field("supportsLogExportsToCloudwatch"),
            valid_upgrade_targets: o.get_field("validUpgradeTargets"),
            version: o.get_field("version"),
            version_description: o.get_field("versionDescription"),
        }
    }
}
