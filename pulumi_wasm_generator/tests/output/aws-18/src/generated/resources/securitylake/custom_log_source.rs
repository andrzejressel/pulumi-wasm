/// Resource for managing an AWS Security Lake Custom Log Source.
///
/// > **NOTE:** The underlying `aws.securitylake.DataLake` must be configured before creating the `aws.securitylake.CustomLogSource`. Use a `depends_on` statement.
///
/// ## Example Usage
///
/// ### Basic Usage
///
/// ```ignore
/// use pulumi_wasm_rust::Output;
/// use pulumi_wasm_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let example = custom_log_source::create(
///         "example",
///         CustomLogSourceArgs::builder()
///             .configuration(
///                 CustomLogSourceConfiguration::builder()
///                     .crawlerConfiguration(
///                         CustomLogSourceConfigurationCrawlerConfiguration::builder()
///                             .roleArn("${customLog.arn}")
///                             .build_struct(),
///                     )
///                     .providerIdentity(
///                         CustomLogSourceConfigurationProviderIdentity::builder()
///                             .externalId("example-id")
///                             .principal("123456789012")
///                             .build_struct(),
///                     )
///                     .build_struct(),
///             )
///             .event_classes(vec!["FILE_ACTIVITY",])
///             .source_name("example-name")
///             .source_version("1.0")
///             .build_struct(),
///     );
/// }
/// ```
///
/// ## Import
///
/// Using `pulumi import`, import Custom log sources using the source name. For example:
///
/// ```sh
/// $ pulumi import aws:securitylake/customLogSource:CustomLogSource example example-name
/// ```
pub mod custom_log_source {
    #[derive(pulumi_wasm_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct CustomLogSourceArgs {
        /// The configuration for the third-party custom source.
        #[builder(into, default)]
        pub configuration: pulumi_wasm_rust::InputOrOutput<
            Option<super::super::types::securitylake::CustomLogSourceConfiguration>,
        >,
        /// The Open Cybersecurity Schema Framework (OCSF) event classes which describes the type of data that the custom source will send to Security Lake.
        #[builder(into, default)]
        pub event_classes: pulumi_wasm_rust::InputOrOutput<Option<Vec<String>>>,
        /// Specify the name for a third-party custom source.
        /// This must be a Regionally unique value.
        /// Has a maximum length of 20.
        #[builder(into)]
        pub source_name: pulumi_wasm_rust::InputOrOutput<String>,
        /// Specify the source version for the third-party custom source, to limit log collection to a specific version of custom data source.
        #[builder(into, default)]
        pub source_version: pulumi_wasm_rust::InputOrOutput<Option<String>>,
    }
    #[allow(dead_code)]
    pub struct CustomLogSourceResult {
        /// The attributes of a third-party custom source.
        pub attributes: pulumi_wasm_rust::Output<
            Vec<super::super::types::securitylake::CustomLogSourceAttribute>,
        >,
        /// The configuration for the third-party custom source.
        pub configuration: pulumi_wasm_rust::Output<
            Option<super::super::types::securitylake::CustomLogSourceConfiguration>,
        >,
        /// The Open Cybersecurity Schema Framework (OCSF) event classes which describes the type of data that the custom source will send to Security Lake.
        pub event_classes: pulumi_wasm_rust::Output<Option<Vec<String>>>,
        /// The details of the log provider for a third-party custom source.
        pub provider_details: pulumi_wasm_rust::Output<
            Vec<super::super::types::securitylake::CustomLogSourceProviderDetail>,
        >,
        /// Specify the name for a third-party custom source.
        /// This must be a Regionally unique value.
        /// Has a maximum length of 20.
        pub source_name: pulumi_wasm_rust::Output<String>,
        /// Specify the source version for the third-party custom source, to limit log collection to a specific version of custom data source.
        pub source_version: pulumi_wasm_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_wasm_rust::PulumiContext,
        name: &str,
        args: CustomLogSourceArgs,
    ) -> CustomLogSourceResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let configuration_binding = args.configuration.get_output(context).get_inner();
        let event_classes_binding = args.event_classes.get_output(context).get_inner();
        let source_name_binding = args.source_name.get_output(context).get_inner();
        let source_version_binding = args.source_version.get_output(context).get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "aws:securitylake/customLogSource:CustomLogSource".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "configuration".into(),
                    value: &configuration_binding,
                },
                register_interface::ObjectField {
                    name: "eventClasses".into(),
                    value: &event_classes_binding,
                },
                register_interface::ObjectField {
                    name: "sourceName".into(),
                    value: &source_name_binding,
                },
                register_interface::ObjectField {
                    name: "sourceVersion".into(),
                    value: &source_version_binding,
                },
            ]),
            results: Vec::from([
                register_interface::ResultField {
                    name: "attributes".into(),
                },
                register_interface::ResultField {
                    name: "configuration".into(),
                },
                register_interface::ResultField {
                    name: "eventClasses".into(),
                },
                register_interface::ResultField {
                    name: "providerDetails".into(),
                },
                register_interface::ResultField {
                    name: "sourceName".into(),
                },
                register_interface::ResultField {
                    name: "sourceVersion".into(),
                },
            ]),
        };
        let o = register_interface::register(context.get_inner(), &request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        CustomLogSourceResult {
            attributes: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("attributes").unwrap(),
            ),
            configuration: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("configuration").unwrap(),
            ),
            event_classes: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("eventClasses").unwrap(),
            ),
            provider_details: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("providerDetails").unwrap(),
            ),
            source_name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("sourceName").unwrap(),
            ),
            source_version: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("sourceVersion").unwrap(),
            ),
        }
    }
}
