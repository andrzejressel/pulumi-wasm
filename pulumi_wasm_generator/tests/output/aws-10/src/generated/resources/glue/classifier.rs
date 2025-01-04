/// Provides a Glue Classifier resource.
///
/// > **NOTE:** It is only valid to create one type of classifier (CSV, grok, JSON, or XML). Changing classifier types will recreate the classifier.
///
/// ## Example Usage
///
/// ### CSV Classifier
///
/// ```ignore
/// use pulumi_wasm_rust::Output;
/// use pulumi_wasm_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let example = classifier::create(
///         "example",
///         ClassifierArgs::builder()
///             .csv_classifier(
///                 ClassifierCsvClassifier::builder()
///                     .allowSingleColumn(false)
///                     .containsHeader("PRESENT")
///                     .delimiter(",")
///                     .disableValueTrimming(false)
///                     .headers(vec!["example1", "example2",])
///                     .quoteSymbol("'")
///                     .build_struct(),
///             )
///             .name("example")
///             .build_struct(),
///     );
/// }
/// ```
///
/// ### Grok Classifier
///
/// ```ignore
/// use pulumi_wasm_rust::Output;
/// use pulumi_wasm_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let example = classifier::create(
///         "example",
///         ClassifierArgs::builder()
///             .grok_classifier(
///                 ClassifierGrokClassifier::builder()
///                     .classification("example")
///                     .grokPattern("example")
///                     .build_struct(),
///             )
///             .name("example")
///             .build_struct(),
///     );
/// }
/// ```
///
/// ### JSON Classifier
///
/// ```ignore
/// use pulumi_wasm_rust::Output;
/// use pulumi_wasm_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let example = classifier::create(
///         "example",
///         ClassifierArgs::builder()
///             .json_classifier(
///                 ClassifierJsonClassifier::builder().jsonPath("example").build_struct(),
///             )
///             .name("example")
///             .build_struct(),
///     );
/// }
/// ```
///
/// ### XML Classifier
///
/// ```ignore
/// use pulumi_wasm_rust::Output;
/// use pulumi_wasm_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let example = classifier::create(
///         "example",
///         ClassifierArgs::builder()
///             .name("example")
///             .xml_classifier(
///                 ClassifierXmlClassifier::builder()
///                     .classification("example")
///                     .rowTag("example")
///                     .build_struct(),
///             )
///             .build_struct(),
///     );
/// }
/// ```
///
/// ## Import
///
/// Using `pulumi import`, import Glue Classifiers using their name. For example:
///
/// ```sh
/// $ pulumi import aws:glue/classifier:Classifier MyClassifier MyClassifier
/// ```
pub mod classifier {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct ClassifierArgs {
        /// A classifier for CSV content. Defined below.
        #[builder(into, default)]
        pub csv_classifier: pulumi_wasm_rust::Output<
            Option<super::super::types::glue::ClassifierCsvClassifier>,
        >,
        /// A classifier that uses grok patterns. Defined below.
        #[builder(into, default)]
        pub grok_classifier: pulumi_wasm_rust::Output<
            Option<super::super::types::glue::ClassifierGrokClassifier>,
        >,
        /// A classifier for JSON content. Defined below.
        #[builder(into, default)]
        pub json_classifier: pulumi_wasm_rust::Output<
            Option<super::super::types::glue::ClassifierJsonClassifier>,
        >,
        /// The name of the classifier.
        #[builder(into, default)]
        pub name: pulumi_wasm_rust::Output<Option<String>>,
        /// A classifier for XML content. Defined below.
        #[builder(into, default)]
        pub xml_classifier: pulumi_wasm_rust::Output<
            Option<super::super::types::glue::ClassifierXmlClassifier>,
        >,
    }
    #[allow(dead_code)]
    pub struct ClassifierResult {
        /// A classifier for CSV content. Defined below.
        pub csv_classifier: pulumi_wasm_rust::Output<
            Option<super::super::types::glue::ClassifierCsvClassifier>,
        >,
        /// A classifier that uses grok patterns. Defined below.
        pub grok_classifier: pulumi_wasm_rust::Output<
            Option<super::super::types::glue::ClassifierGrokClassifier>,
        >,
        /// A classifier for JSON content. Defined below.
        pub json_classifier: pulumi_wasm_rust::Output<
            Option<super::super::types::glue::ClassifierJsonClassifier>,
        >,
        /// The name of the classifier.
        pub name: pulumi_wasm_rust::Output<String>,
        /// A classifier for XML content. Defined below.
        pub xml_classifier: pulumi_wasm_rust::Output<
            Option<super::super::types::glue::ClassifierXmlClassifier>,
        >,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(name: &str, args: ClassifierArgs) -> ClassifierResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let csv_classifier_binding = args.csv_classifier.get_inner();
        let grok_classifier_binding = args.grok_classifier.get_inner();
        let json_classifier_binding = args.json_classifier.get_inner();
        let name_binding = args.name.get_inner();
        let xml_classifier_binding = args.xml_classifier.get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "aws:glue/classifier:Classifier".into(),
            name: name.to_string(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "csvClassifier".into(),
                    value: &csv_classifier_binding,
                },
                register_interface::ObjectField {
                    name: "grokClassifier".into(),
                    value: &grok_classifier_binding,
                },
                register_interface::ObjectField {
                    name: "jsonClassifier".into(),
                    value: &json_classifier_binding,
                },
                register_interface::ObjectField {
                    name: "name".into(),
                    value: &name_binding,
                },
                register_interface::ObjectField {
                    name: "xmlClassifier".into(),
                    value: &xml_classifier_binding,
                },
            ]),
            results: Vec::from([
                register_interface::ResultField {
                    name: "csvClassifier".into(),
                },
                register_interface::ResultField {
                    name: "grokClassifier".into(),
                },
                register_interface::ResultField {
                    name: "jsonClassifier".into(),
                },
                register_interface::ResultField {
                    name: "name".into(),
                },
                register_interface::ResultField {
                    name: "xmlClassifier".into(),
                },
            ]),
        };
        let o = register_interface::register(&request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        ClassifierResult {
            csv_classifier: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("csvClassifier").unwrap(),
            ),
            grok_classifier: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("grokClassifier").unwrap(),
            ),
            json_classifier: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("jsonClassifier").unwrap(),
            ),
            name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("name").unwrap(),
            ),
            xml_classifier: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("xmlClassifier").unwrap(),
            ),
        }
    }
}
