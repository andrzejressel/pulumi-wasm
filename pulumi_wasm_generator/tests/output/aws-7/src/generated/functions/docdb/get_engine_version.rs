pub mod get_engine_version {
    #[derive(pulumi_wasm_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GetEngineVersionArgs {
        /// DB engine. (Default: `docdb`)
        #[builder(into, default)]
        pub engine: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        /// Name of a specific DB parameter group family. An example parameter group family is `docdb3.6`.
        #[builder(into, default)]
        pub parameter_group_family: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        /// Ordered list of preferred engine versions. The first match in this list will be returned. If no preferred matches are found and the original search returned more than one result, an error is returned. If both the `version` and `preferred_versions` arguments are not configured, the data source will return the default version for the engine.
        #[builder(into, default)]
        pub preferred_versions: pulumi_wasm_rust::InputOrOutput<Option<Vec<String>>>,
        /// Version of the DB engine. For example, `3.6.0`. If `version` and `preferred_versions` are not set, the data source will provide information for the AWS-defined default version. If both the `version` and `preferred_versions` arguments are not configured, the data source will return the default version for the engine.
        #[builder(into, default)]
        pub version: pulumi_wasm_rust::InputOrOutput<Option<String>>,
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
        /// Indicates whether the engine version supports exporting the log types specified by `exportable_log_types` to CloudWatch Logs.
        pub supports_log_exports_to_cloudwatch: pulumi_wasm_rust::Output<bool>,
        /// A set of engine versions that this database engine version can be upgraded to.
        pub valid_upgrade_targets: pulumi_wasm_rust::Output<Vec<String>>,
        pub version: pulumi_wasm_rust::Output<String>,
        /// Description of the database engine version.
        pub version_description: pulumi_wasm_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn invoke(
        context: &pulumi_wasm_rust::PulumiContext,
        args: GetEngineVersionArgs,
    ) -> GetEngineVersionResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let engine_binding = args.engine.get_output(context).get_inner();
        let parameter_group_family_binding = args
            .parameter_group_family
            .get_output(context)
            .get_inner();
        let preferred_versions_binding = args
            .preferred_versions
            .get_output(context)
            .get_inner();
        let version_binding = args.version.get_output(context).get_inner();
        let request = register_interface::ResourceInvokeRequest {
            token: "aws:docdb/getEngineVersion:getEngineVersion".into(),
            version: super::super::super::get_version(),
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
                    name: "supportsLogExportsToCloudwatch".into(),
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
        let o = register_interface::invoke(context.get_inner(), &request);
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
            supports_log_exports_to_cloudwatch: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("supportsLogExportsToCloudwatch").unwrap(),
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
