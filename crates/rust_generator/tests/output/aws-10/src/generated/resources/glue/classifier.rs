/// Provides a Glue Classifier resource.
///
/// > **NOTE:** It is only valid to create one type of classifier (CSV, grok, JSON, or XML). Changing classifier types will recreate the classifier.
///
/// ## Example Usage
///
/// ### CSV Classifier
///
/// ```ignore
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
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
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
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
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
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
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
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
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod classifier {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct ClassifierArgs {
        /// A classifier for CSV content. Defined below.
        #[builder(into, default)]
        pub csv_classifier: pulumi_gestalt_rust::InputOrOutput<
            Option<super::super::types::glue::ClassifierCsvClassifier>,
        >,
        /// A classifier that uses grok patterns. Defined below.
        #[builder(into, default)]
        pub grok_classifier: pulumi_gestalt_rust::InputOrOutput<
            Option<super::super::types::glue::ClassifierGrokClassifier>,
        >,
        /// A classifier for JSON content. Defined below.
        #[builder(into, default)]
        pub json_classifier: pulumi_gestalt_rust::InputOrOutput<
            Option<super::super::types::glue::ClassifierJsonClassifier>,
        >,
        /// The name of the classifier.
        #[builder(into, default)]
        pub name: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// A classifier for XML content. Defined below.
        #[builder(into, default)]
        pub xml_classifier: pulumi_gestalt_rust::InputOrOutput<
            Option<super::super::types::glue::ClassifierXmlClassifier>,
        >,
    }
    #[allow(dead_code)]
    pub struct ClassifierResult {
        /// A classifier for CSV content. Defined below.
        pub csv_classifier: pulumi_gestalt_rust::Output<
            Option<super::super::types::glue::ClassifierCsvClassifier>,
        >,
        /// A classifier that uses grok patterns. Defined below.
        pub grok_classifier: pulumi_gestalt_rust::Output<
            Option<super::super::types::glue::ClassifierGrokClassifier>,
        >,
        /// A classifier for JSON content. Defined below.
        pub json_classifier: pulumi_gestalt_rust::Output<
            Option<super::super::types::glue::ClassifierJsonClassifier>,
        >,
        /// The name of the classifier.
        pub name: pulumi_gestalt_rust::Output<String>,
        /// A classifier for XML content. Defined below.
        pub xml_classifier: pulumi_gestalt_rust::Output<
            Option<super::super::types::glue::ClassifierXmlClassifier>,
        >,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: ClassifierArgs,
    ) -> ClassifierResult {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::register_interface;
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let csv_classifier_binding = args.csv_classifier.get_output(context);
        let grok_classifier_binding = args.grok_classifier.get_output(context);
        let json_classifier_binding = args.json_classifier.get_output(context);
        let name_binding = args.name.get_output(context);
        let xml_classifier_binding = args.xml_classifier.get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "aws:glue/classifier:Classifier".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "csvClassifier".into(),
                    value: csv_classifier_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "grokClassifier".into(),
                    value: grok_classifier_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "jsonClassifier".into(),
                    value: json_classifier_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "name".into(),
                    value: name_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "xmlClassifier".into(),
                    value: xml_classifier_binding.get_id(),
                },
            ],
        };
        let o = context.register_resource(request);
        ClassifierResult {
            csv_classifier: o.get_field("csvClassifier"),
            grok_classifier: o.get_field("grokClassifier"),
            json_classifier: o.get_field("jsonClassifier"),
            name: o.get_field("name"),
            xml_classifier: o.get_field("xmlClassifier"),
        }
    }
}
