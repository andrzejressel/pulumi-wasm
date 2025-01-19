/// Manages a Service Catalog self-service action.
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
///     let example = service_action::create(
///         "example",
///         ServiceActionArgs::builder()
///             .definition(
///                 ServiceActionDefinition::builder()
///                     .name("AWS-RestartEC2Instance")
///                     .build_struct(),
///             )
///             .description("Motor generator unit")
///             .name("MGU")
///             .build_struct(),
///     );
/// }
/// ```
///
/// ## Import
///
/// Using `pulumi import`, import `aws_servicecatalog_service_action` using the service action ID. For example:
///
/// ```sh
/// $ pulumi import aws:servicecatalog/serviceAction:ServiceAction example act-f1w12eperfslh
/// ```
pub mod service_action {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct ServiceActionArgs {
        /// Language code. Valid values are `en` (English), `jp` (Japanese), and `zh` (Chinese). Default is `en`.
        #[builder(into, default)]
        pub accept_language: pulumi_wasm_rust::Output<Option<String>>,
        /// Self-service action definition configuration block. Detailed below.
        #[builder(into)]
        pub definition: pulumi_wasm_rust::Output<
            super::super::types::servicecatalog::ServiceActionDefinition,
        >,
        /// Self-service action description.
        #[builder(into, default)]
        pub description: pulumi_wasm_rust::Output<Option<String>>,
        /// Self-service action name.
        ///
        /// The following arguments are optional:
        #[builder(into, default)]
        pub name: pulumi_wasm_rust::Output<Option<String>>,
    }
    #[allow(dead_code)]
    pub struct ServiceActionResult {
        /// Language code. Valid values are `en` (English), `jp` (Japanese), and `zh` (Chinese). Default is `en`.
        pub accept_language: pulumi_wasm_rust::Output<Option<String>>,
        /// Self-service action definition configuration block. Detailed below.
        pub definition: pulumi_wasm_rust::Output<
            super::super::types::servicecatalog::ServiceActionDefinition,
        >,
        /// Self-service action description.
        pub description: pulumi_wasm_rust::Output<String>,
        /// Self-service action name.
        ///
        /// The following arguments are optional:
        pub name: pulumi_wasm_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(name: &str, args: ServiceActionArgs) -> ServiceActionResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let accept_language_binding = args.accept_language.get_inner();
        let definition_binding = args.definition.get_inner();
        let description_binding = args.description.get_inner();
        let name_binding = args.name.get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "aws:servicecatalog/serviceAction:ServiceAction".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "acceptLanguage".into(),
                    value: &accept_language_binding,
                },
                register_interface::ObjectField {
                    name: "definition".into(),
                    value: &definition_binding,
                },
                register_interface::ObjectField {
                    name: "description".into(),
                    value: &description_binding,
                },
                register_interface::ObjectField {
                    name: "name".into(),
                    value: &name_binding,
                },
            ]),
            results: Vec::from([
                register_interface::ResultField {
                    name: "acceptLanguage".into(),
                },
                register_interface::ResultField {
                    name: "definition".into(),
                },
                register_interface::ResultField {
                    name: "description".into(),
                },
                register_interface::ResultField {
                    name: "name".into(),
                },
            ]),
        };
        let o = register_interface::register(&request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        ServiceActionResult {
            accept_language: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("acceptLanguage").unwrap(),
            ),
            definition: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("definition").unwrap(),
            ),
            description: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("description").unwrap(),
            ),
            name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("name").unwrap(),
            ),
        }
    }
}
