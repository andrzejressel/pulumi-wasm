/// Manages an Access Analyzer Analyzer. More information can be found in the [Access Analyzer User Guide](https://docs.aws.amazon.com/IAM/latest/UserGuide/what-is-access-analyzer.html).
///
/// ## Example Usage
///
/// ### Account Analyzer
///
/// ```ignore
/// use pulumi_wasm_rust::Output;
/// use pulumi_wasm_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let example = analyzer::create(
///         "example",
///         AnalyzerArgs::builder().analyzer_name("example").build_struct(),
///     );
/// }
/// ```
///
/// ### Organization Analyzer
///
/// ```ignore
/// use pulumi_wasm_rust::Output;
/// use pulumi_wasm_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let example = organization::create(
///         "example",
///         OrganizationArgs::builder()
///             .aws_service_access_principals(vec!["access-analyzer.amazonaws.com",])
///             .build_struct(),
///     );
///     let exampleAnalyzer = analyzer::create(
///         "exampleAnalyzer",
///         AnalyzerArgs::builder()
///             .analyzer_name("example")
///             .type_("ORGANIZATION")
///             .build_struct(),
///     );
/// }
/// ```
///
/// ## Import
///
/// Using `pulumi import`, import Access Analyzer Analyzers using the `analyzer_name`. For example:
///
/// ```sh
/// $ pulumi import aws:accessanalyzer/analyzer:Analyzer example example
/// ```
pub mod analyzer {
    #[derive(pulumi_wasm_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct AnalyzerArgs {
        /// Name of the Analyzer.
        ///
        /// The following arguments are optional:
        #[builder(into)]
        pub analyzer_name: pulumi_wasm_rust::InputOrOutput<String>,
        /// A block that specifies the configuration of the analyzer. Documented below
        #[builder(into, default)]
        pub configuration: pulumi_wasm_rust::InputOrOutput<
            Option<super::super::types::accessanalyzer::AnalyzerConfiguration>,
        >,
        /// Key-value map of resource tags. If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
        #[builder(into, default)]
        pub tags: pulumi_wasm_rust::InputOrOutput<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// Type of Analyzer. Valid values are `ACCOUNT`, `ORGANIZATION`, `ACCOUNT_UNUSED_ACCESS `, `ORGANIZATION_UNUSED_ACCESS`. Defaults to `ACCOUNT`.
        #[builder(into, default)]
        pub type_: pulumi_wasm_rust::InputOrOutput<Option<String>>,
    }
    #[allow(dead_code)]
    pub struct AnalyzerResult {
        /// Name of the Analyzer.
        ///
        /// The following arguments are optional:
        pub analyzer_name: pulumi_wasm_rust::Output<String>,
        /// ARN of the Analyzer.
        pub arn: pulumi_wasm_rust::Output<String>,
        /// A block that specifies the configuration of the analyzer. Documented below
        pub configuration: pulumi_wasm_rust::Output<
            Option<super::super::types::accessanalyzer::AnalyzerConfiguration>,
        >,
        /// Key-value map of resource tags. If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
        pub tags: pulumi_wasm_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// Map of tags assigned to the resource, including those inherited from the provider `default_tags` configuration block.
        pub tags_all: pulumi_wasm_rust::Output<
            std::collections::HashMap<String, String>,
        >,
        /// Type of Analyzer. Valid values are `ACCOUNT`, `ORGANIZATION`, `ACCOUNT_UNUSED_ACCESS `, `ORGANIZATION_UNUSED_ACCESS`. Defaults to `ACCOUNT`.
        pub type_: pulumi_wasm_rust::Output<Option<String>>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_wasm_rust::PulumiContext,
        name: &str,
        args: AnalyzerArgs,
    ) -> AnalyzerResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let analyzer_name_binding = args.analyzer_name.get_output(context).get_inner();
        let configuration_binding = args.configuration.get_output(context).get_inner();
        let tags_binding = args.tags.get_output(context).get_inner();
        let type__binding = args.type_.get_output(context).get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "aws:accessanalyzer/analyzer:Analyzer".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "analyzerName".into(),
                    value: &analyzer_name_binding,
                },
                register_interface::ObjectField {
                    name: "configuration".into(),
                    value: &configuration_binding,
                },
                register_interface::ObjectField {
                    name: "tags".into(),
                    value: &tags_binding,
                },
                register_interface::ObjectField {
                    name: "type".into(),
                    value: &type__binding,
                },
            ]),
            results: Vec::from([
                register_interface::ResultField {
                    name: "analyzerName".into(),
                },
                register_interface::ResultField {
                    name: "arn".into(),
                },
                register_interface::ResultField {
                    name: "configuration".into(),
                },
                register_interface::ResultField {
                    name: "tags".into(),
                },
                register_interface::ResultField {
                    name: "tagsAll".into(),
                },
                register_interface::ResultField {
                    name: "type".into(),
                },
            ]),
        };
        let o = register_interface::register(context.get_inner(), &request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        AnalyzerResult {
            analyzer_name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("analyzerName").unwrap(),
            ),
            arn: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("arn").unwrap(),
            ),
            configuration: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("configuration").unwrap(),
            ),
            tags: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("tags").unwrap(),
            ),
            tags_all: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("tagsAll").unwrap(),
            ),
            type_: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("type").unwrap(),
            ),
        }
    }
}
