/// Resource for managing an AWS Lex V2 Models Bot Version.
///
/// ## Example Usage
///
/// ### Basic Usage
///
/// ```yaml
/// resources:
///   test:
///     type: aws:lex:V2modelsBotVersion
///     properties:
///       botId: ${testAwsLexv2modelsBot.id}
///       localeSpecification:
///         en_US:
///           sourceBotVersion: DRAFT
/// ```
///
/// ## Import
///
/// Using `pulumi import`, import Lex V2 Models Bot Version using the `id`. For example:
///
/// ```sh
/// $ pulumi import aws:lex/v2modelsBotVersion:V2modelsBotVersion example id-12345678,1
/// ```
pub mod v_2_models_bot_version {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct V2modelsBotVersionArgs {
        /// Idientifier of the bot to create the version for.
        #[builder(into)]
        pub bot_id: pulumi_wasm_rust::Output<String>,
        /// Version number assigned to the version.
        #[builder(into, default)]
        pub bot_version: pulumi_wasm_rust::Output<Option<String>>,
        /// A description of the version. Use the description to help identify the version in lists.
        #[builder(into, default)]
        pub description: pulumi_wasm_rust::Output<Option<String>>,
        /// Specifies the locales that Amazon Lex adds to this version. You can choose the draft version or any other previously published version for each locale. When you specify a source version, the locale data is copied from the source version to the new version.
        ///
        /// The attribute value is a map with one or more entries, each of which has a locale name as the key and an object with the following attribute as the value:
        /// * `sourceBotVersion` - (Required) The version of a bot used for a bot locale. Valid values: `DRAFT`, a numeric version.
        #[builder(into)]
        pub locale_specification: pulumi_wasm_rust::Output<
            std::collections::HashMap<
                String,
                super::super::types::lex::V2ModelsBotVersionLocaleSpecification,
            >,
        >,
        #[builder(into, default)]
        pub timeouts: pulumi_wasm_rust::Output<
            Option<super::super::types::lex::V2ModelsBotVersionTimeouts>,
        >,
    }
    #[allow(dead_code)]
    pub struct V2modelsBotVersionResult {
        /// Idientifier of the bot to create the version for.
        pub bot_id: pulumi_wasm_rust::Output<String>,
        /// Version number assigned to the version.
        pub bot_version: pulumi_wasm_rust::Output<String>,
        /// A description of the version. Use the description to help identify the version in lists.
        pub description: pulumi_wasm_rust::Output<Option<String>>,
        /// Specifies the locales that Amazon Lex adds to this version. You can choose the draft version or any other previously published version for each locale. When you specify a source version, the locale data is copied from the source version to the new version.
        ///
        /// The attribute value is a map with one or more entries, each of which has a locale name as the key and an object with the following attribute as the value:
        /// * `sourceBotVersion` - (Required) The version of a bot used for a bot locale. Valid values: `DRAFT`, a numeric version.
        pub locale_specification: pulumi_wasm_rust::Output<
            std::collections::HashMap<
                String,
                super::super::types::lex::V2ModelsBotVersionLocaleSpecification,
            >,
        >,
        pub timeouts: pulumi_wasm_rust::Output<
            Option<super::super::types::lex::V2ModelsBotVersionTimeouts>,
        >,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(name: &str, args: V2modelsBotVersionArgs) -> V2modelsBotVersionResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let bot_id_binding = args.bot_id.get_inner();
        let bot_version_binding = args.bot_version.get_inner();
        let description_binding = args.description.get_inner();
        let locale_specification_binding = args.locale_specification.get_inner();
        let timeouts_binding = args.timeouts.get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "aws:lex/v2modelsBotVersion:V2modelsBotVersion".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "botId".into(),
                    value: &bot_id_binding,
                },
                register_interface::ObjectField {
                    name: "botVersion".into(),
                    value: &bot_version_binding,
                },
                register_interface::ObjectField {
                    name: "description".into(),
                    value: &description_binding,
                },
                register_interface::ObjectField {
                    name: "localeSpecification".into(),
                    value: &locale_specification_binding,
                },
                register_interface::ObjectField {
                    name: "timeouts".into(),
                    value: &timeouts_binding,
                },
            ]),
            results: Vec::from([
                register_interface::ResultField {
                    name: "botId".into(),
                },
                register_interface::ResultField {
                    name: "botVersion".into(),
                },
                register_interface::ResultField {
                    name: "description".into(),
                },
                register_interface::ResultField {
                    name: "localeSpecification".into(),
                },
                register_interface::ResultField {
                    name: "timeouts".into(),
                },
            ]),
        };
        let o = register_interface::register(&request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        V2modelsBotVersionResult {
            bot_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("botId").unwrap(),
            ),
            bot_version: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("botVersion").unwrap(),
            ),
            description: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("description").unwrap(),
            ),
            locale_specification: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("localeSpecification").unwrap(),
            ),
            timeouts: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("timeouts").unwrap(),
            ),
        }
    }
}
