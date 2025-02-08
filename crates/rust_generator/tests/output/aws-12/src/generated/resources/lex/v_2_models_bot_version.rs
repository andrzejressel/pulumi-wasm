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
#[allow(clippy::doc_lazy_continuation)]
pub mod v_2_models_bot_version {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct V2modelsBotVersionArgs {
        /// Idientifier of the bot to create the version for.
        #[builder(into)]
        pub bot_id: pulumi_gestalt_rust::InputOrOutput<String>,
        /// Version number assigned to the version.
        #[builder(into, default)]
        pub bot_version: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// A description of the version. Use the description to help identify the version in lists.
        #[builder(into, default)]
        pub description: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Specifies the locales that Amazon Lex adds to this version. You can choose the draft version or any other previously published version for each locale. When you specify a source version, the locale data is copied from the source version to the new version.
        ///
        /// The attribute value is a map with one or more entries, each of which has a locale name as the key and an object with the following attribute as the value:
        /// * `sourceBotVersion` - (Required) The version of a bot used for a bot locale. Valid values: `DRAFT`, a numeric version.
        #[builder(into)]
        pub locale_specification: pulumi_gestalt_rust::InputOrOutput<
            std::collections::HashMap<
                String,
                super::super::types::lex::V2ModelsBotVersionLocaleSpecification,
            >,
        >,
        #[builder(into, default)]
        pub timeouts: pulumi_gestalt_rust::InputOrOutput<
            Option<super::super::types::lex::V2ModelsBotVersionTimeouts>,
        >,
    }
    #[allow(dead_code)]
    pub struct V2modelsBotVersionResult {
        /// Idientifier of the bot to create the version for.
        pub bot_id: pulumi_gestalt_rust::Output<String>,
        /// Version number assigned to the version.
        pub bot_version: pulumi_gestalt_rust::Output<String>,
        /// A description of the version. Use the description to help identify the version in lists.
        pub description: pulumi_gestalt_rust::Output<Option<String>>,
        /// Specifies the locales that Amazon Lex adds to this version. You can choose the draft version or any other previously published version for each locale. When you specify a source version, the locale data is copied from the source version to the new version.
        ///
        /// The attribute value is a map with one or more entries, each of which has a locale name as the key and an object with the following attribute as the value:
        /// * `sourceBotVersion` - (Required) The version of a bot used for a bot locale. Valid values: `DRAFT`, a numeric version.
        pub locale_specification: pulumi_gestalt_rust::Output<
            std::collections::HashMap<
                String,
                super::super::types::lex::V2ModelsBotVersionLocaleSpecification,
            >,
        >,
        pub timeouts: pulumi_gestalt_rust::Output<
            Option<super::super::types::lex::V2ModelsBotVersionTimeouts>,
        >,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::PulumiContext,
        name: &str,
        args: V2modelsBotVersionArgs,
    ) -> V2modelsBotVersionResult {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::register_interface;
        use std::collections::HashMap;
        let bot_id_binding = args.bot_id.get_output(context).get_inner();
        let bot_version_binding = args.bot_version.get_output(context).get_inner();
        let description_binding = args.description.get_output(context).get_inner();
        let locale_specification_binding = args
            .locale_specification
            .get_output(context)
            .get_inner();
        let timeouts_binding = args.timeouts.get_output(context).get_inner();
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
        };
        let o = register_interface::register(context.get_inner(), &request);
        V2modelsBotVersionResult {
            bot_id: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("botId"),
            ),
            bot_version: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("botVersion"),
            ),
            description: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("description"),
            ),
            locale_specification: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("localeSpecification"),
            ),
            timeouts: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("timeouts"),
            ),
        }
    }
}
