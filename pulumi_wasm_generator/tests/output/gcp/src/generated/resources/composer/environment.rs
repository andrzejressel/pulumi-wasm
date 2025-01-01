///
///
/// ## Import
///
/// Environment can be imported using any of these accepted formats:
///
/// * `projects/{{project}}/locations/{{region}}/environments/{{name}}`
///
/// * `{{project}}/{{region}}/{{name}}`
///
/// * `{{name}}`
///
/// When using the `pulumi import` command, Environment can be imported using one of the formats above. For example:
///
/// ```sh
/// $ pulumi import gcp:composer/environment:Environment default projects/{{project}}/locations/{{region}}/environments/{{name}}
/// ```
///
/// ```sh
/// $ pulumi import gcp:composer/environment:Environment default {{project}}/{{region}}/{{name}}
/// ```
///
/// ```sh
/// $ pulumi import gcp:composer/environment:Environment default {{name}}
/// ```
///
pub mod environment {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct EnvironmentArgs {
        /// Configuration parameters for this environment.
        #[builder(into, default)]
        pub config: pulumi_wasm_rust::Output<
            Option<super::super::types::composer::EnvironmentConfig>,
        >,
        /// User-defined labels for this environment. The labels map can contain no more than 64 entries. Entries of the labels map
        /// are UTF8 strings that comply with the following restrictions: Label keys must be between 1 and 63 characters long and
        /// must conform to the following regular expression: a-z?. Label values must be between 0 and 63 characters long and must
        /// conform to the regular expression (a-z?)?. No more than 64 labels can be associated with a given environment. Both keys
        /// and values must be <= 128 bytes in size. **Note**: This field is non-authoritative, and will only manage the labels
        /// present in your configuration. Please refer to the field 'effective_labels' for all of the labels present on the
        /// resource.
        #[builder(into, default)]
        pub labels: pulumi_wasm_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// Name of the environment.
        #[builder(into, default)]
        pub name: pulumi_wasm_rust::Output<Option<String>>,
        /// The ID of the project in which the resource belongs. If it is not provided, the provider project is used.
        #[builder(into, default)]
        pub project: pulumi_wasm_rust::Output<Option<String>>,
        /// The location or Compute Engine region for the environment.
        #[builder(into, default)]
        pub region: pulumi_wasm_rust::Output<Option<String>>,
        /// Configuration options for storage used by Composer environment.
        #[builder(into, default)]
        pub storage_config: pulumi_wasm_rust::Output<
            Option<super::super::types::composer::EnvironmentStorageConfig>,
        >,
    }
    #[allow(dead_code)]
    pub struct EnvironmentResult {
        /// Configuration parameters for this environment.
        pub config: pulumi_wasm_rust::Output<
            super::super::types::composer::EnvironmentConfig,
        >,
        pub effective_labels: pulumi_wasm_rust::Output<
            std::collections::HashMap<String, String>,
        >,
        /// User-defined labels for this environment. The labels map can contain no more than 64 entries. Entries of the labels map
        /// are UTF8 strings that comply with the following restrictions: Label keys must be between 1 and 63 characters long and
        /// must conform to the following regular expression: a-z?. Label values must be between 0 and 63 characters long and must
        /// conform to the regular expression (a-z?)?. No more than 64 labels can be associated with a given environment. Both keys
        /// and values must be <= 128 bytes in size. **Note**: This field is non-authoritative, and will only manage the labels
        /// present in your configuration. Please refer to the field 'effective_labels' for all of the labels present on the
        /// resource.
        pub labels: pulumi_wasm_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// Name of the environment.
        pub name: pulumi_wasm_rust::Output<String>,
        /// The ID of the project in which the resource belongs. If it is not provided, the provider project is used.
        pub project: pulumi_wasm_rust::Output<String>,
        /// The combination of labels configured directly on the resource and default labels configured on the provider.
        pub pulumi_labels: pulumi_wasm_rust::Output<
            std::collections::HashMap<String, String>,
        >,
        /// The location or Compute Engine region for the environment.
        pub region: pulumi_wasm_rust::Output<String>,
        /// Configuration options for storage used by Composer environment.
        pub storage_config: pulumi_wasm_rust::Output<
            super::super::types::composer::EnvironmentStorageConfig,
        >,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(name: &str, args: EnvironmentArgs) -> EnvironmentResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let config_binding = args.config.get_inner();
        let labels_binding = args.labels.get_inner();
        let name_binding = args.name.get_inner();
        let project_binding = args.project.get_inner();
        let region_binding = args.region.get_inner();
        let storage_config_binding = args.storage_config.get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "gcp:composer/environment:Environment".into(),
            name: name.to_string(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "config".into(),
                    value: &config_binding,
                },
                register_interface::ObjectField {
                    name: "labels".into(),
                    value: &labels_binding,
                },
                register_interface::ObjectField {
                    name: "name".into(),
                    value: &name_binding,
                },
                register_interface::ObjectField {
                    name: "project".into(),
                    value: &project_binding,
                },
                register_interface::ObjectField {
                    name: "region".into(),
                    value: &region_binding,
                },
                register_interface::ObjectField {
                    name: "storageConfig".into(),
                    value: &storage_config_binding,
                },
            ]),
            results: Vec::from([
                register_interface::ResultField {
                    name: "config".into(),
                },
                register_interface::ResultField {
                    name: "effectiveLabels".into(),
                },
                register_interface::ResultField {
                    name: "labels".into(),
                },
                register_interface::ResultField {
                    name: "name".into(),
                },
                register_interface::ResultField {
                    name: "project".into(),
                },
                register_interface::ResultField {
                    name: "pulumiLabels".into(),
                },
                register_interface::ResultField {
                    name: "region".into(),
                },
                register_interface::ResultField {
                    name: "storageConfig".into(),
                },
            ]),
        };
        let o = register_interface::register(&request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        EnvironmentResult {
            config: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("config").unwrap(),
            ),
            effective_labels: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("effectiveLabels").unwrap(),
            ),
            labels: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("labels").unwrap(),
            ),
            name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("name").unwrap(),
            ),
            project: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("project").unwrap(),
            ),
            pulumi_labels: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("pulumiLabels").unwrap(),
            ),
            region: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("region").unwrap(),
            ),
            storage_config: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("storageConfig").unwrap(),
            ),
        }
    }
}
