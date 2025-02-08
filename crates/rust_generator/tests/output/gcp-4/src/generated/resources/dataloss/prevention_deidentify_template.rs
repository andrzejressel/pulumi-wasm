/// Allows creation of templates to de-identify content.
///
///
/// To get more information about DeidentifyTemplate, see:
///
/// * [API documentation](https://cloud.google.com/dlp/docs/reference/rest/v2/projects.deidentifyTemplates)
/// * How-to Guides
///     * [Official Documentation](https://cloud.google.com/dlp/docs/concepts-templates)
///
///
///
/// ## Example Usage
///
/// ### Dlp Deidentify Template Basic
///
///
/// ```ignore
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let basic = prevention_deidentify_template::create(
///         "basic",
///         PreventionDeidentifyTemplateArgs::builder()
///             .deidentify_config(
///                 PreventionDeidentifyTemplateDeidentifyConfig::builder()
///                     .infoTypeTransformations(
///                         PreventionDeidentifyTemplateDeidentifyConfigInfoTypeTransformations::builder()
///                             .transformations(
///                                 vec![
///                                     PreventionDeidentifyTemplateDeidentifyConfigInfoTypeTransformationsTransformation::builder()
///                                     .infoTypes(vec![PreventionDeidentifyTemplateDeidentifyConfigInfoTypeTransformationsTransformationInfoType::builder()
///                                     .name("FIRST_NAME").build_struct(),])
///                                     .primitiveTransformation(PreventionDeidentifyTemplateDeidentifyConfigInfoTypeTransformationsTransformationPrimitiveTransformation::builder()
///                                     .replaceWithInfoTypeConfig(true).build_struct())
///                                     .build_struct(),
///                                     PreventionDeidentifyTemplateDeidentifyConfigInfoTypeTransformationsTransformation::builder()
///                                     .infoTypes(vec![PreventionDeidentifyTemplateDeidentifyConfigInfoTypeTransformationsTransformationInfoType::builder()
///                                     .name("PHONE_NUMBER").build_struct(),
///                                     PreventionDeidentifyTemplateDeidentifyConfigInfoTypeTransformationsTransformationInfoType::builder()
///                                     .name("AGE").build_struct(),])
///                                     .primitiveTransformation(PreventionDeidentifyTemplateDeidentifyConfigInfoTypeTransformationsTransformationPrimitiveTransformation::builder()
///                                     .replaceConfig(PreventionDeidentifyTemplateDeidentifyConfigInfoTypeTransformationsTransformationPrimitiveTransformationReplaceConfig::builder()
///                                     .newValue(PreventionDeidentifyTemplateDeidentifyConfigInfoTypeTransformationsTransformationPrimitiveTransformationReplaceConfigNewValue::builder()
///                                     .integerValue(9).build_struct()).build_struct())
///                                     .build_struct()).build_struct(),
///                                     PreventionDeidentifyTemplateDeidentifyConfigInfoTypeTransformationsTransformation::builder()
///                                     .infoTypes(vec![PreventionDeidentifyTemplateDeidentifyConfigInfoTypeTransformationsTransformationInfoType::builder()
///                                     .name("EMAIL_ADDRESS").build_struct(),
///                                     PreventionDeidentifyTemplateDeidentifyConfigInfoTypeTransformationsTransformationInfoType::builder()
///                                     .name("LAST_NAME").build_struct(),])
///                                     .primitiveTransformation(PreventionDeidentifyTemplateDeidentifyConfigInfoTypeTransformationsTransformationPrimitiveTransformation::builder()
///                                     .characterMaskConfig(PreventionDeidentifyTemplateDeidentifyConfigInfoTypeTransformationsTransformationPrimitiveTransformationCharacterMaskConfig::builder()
///                                     .charactersToIgnores(vec![PreventionDeidentifyTemplateDeidentifyConfigInfoTypeTransformationsTransformationPrimitiveTransformationCharacterMaskConfigCharactersToIgnore::builder()
///                                     .commonCharactersToIgnore("PUNCTUATION").build_struct(),])
///                                     .maskingCharacter("X").numberToMask(4).reverseOrder(true)
///                                     .build_struct()).build_struct()).build_struct(),
///                                     PreventionDeidentifyTemplateDeidentifyConfigInfoTypeTransformationsTransformation::builder()
///                                     .infoTypes(vec![PreventionDeidentifyTemplateDeidentifyConfigInfoTypeTransformationsTransformationInfoType::builder()
///                                     .name("DATE_OF_BIRTH").build_struct(),])
///                                     .primitiveTransformation(PreventionDeidentifyTemplateDeidentifyConfigInfoTypeTransformationsTransformationPrimitiveTransformation::builder()
///                                     .replaceConfig(PreventionDeidentifyTemplateDeidentifyConfigInfoTypeTransformationsTransformationPrimitiveTransformationReplaceConfig::builder()
///                                     .newValue(PreventionDeidentifyTemplateDeidentifyConfigInfoTypeTransformationsTransformationPrimitiveTransformationReplaceConfigNewValue::builder()
///                                     .dateValue(PreventionDeidentifyTemplateDeidentifyConfigInfoTypeTransformationsTransformationPrimitiveTransformationReplaceConfigNewValueDateValue::builder()
///                                     .day(1).month(1).year(2020).build_struct()).build_struct())
///                                     .build_struct()).build_struct()).build_struct(),
///                                     PreventionDeidentifyTemplateDeidentifyConfigInfoTypeTransformationsTransformation::builder()
///                                     .infoTypes(vec![PreventionDeidentifyTemplateDeidentifyConfigInfoTypeTransformationsTransformationInfoType::builder()
///                                     .name("CREDIT_CARD_NUMBER").build_struct(),])
///                                     .primitiveTransformation(PreventionDeidentifyTemplateDeidentifyConfigInfoTypeTransformationsTransformationPrimitiveTransformation::builder()
///                                     .cryptoDeterministicConfig(PreventionDeidentifyTemplateDeidentifyConfigInfoTypeTransformationsTransformationPrimitiveTransformationCryptoDeterministicConfig::builder()
///                                     .context(PreventionDeidentifyTemplateDeidentifyConfigInfoTypeTransformationsTransformationPrimitiveTransformationCryptoDeterministicConfigContext::builder()
///                                     .name("sometweak").build_struct())
///                                     .cryptoKey(PreventionDeidentifyTemplateDeidentifyConfigInfoTypeTransformationsTransformationPrimitiveTransformationCryptoDeterministicConfigCryptoKey::builder()
///                                     .transient(PreventionDeidentifyTemplateDeidentifyConfigInfoTypeTransformationsTransformationPrimitiveTransformationCryptoDeterministicConfigCryptoKeyTransient::builder()
///                                     .name("beep").build_struct()).build_struct())
///                                     .surrogateInfoType(PreventionDeidentifyTemplateDeidentifyConfigInfoTypeTransformationsTransformationPrimitiveTransformationCryptoDeterministicConfigSurrogateInfoType::builder()
///                                     .name("abc").build_struct()).build_struct()).build_struct())
///                                     .build_struct(),
///                                 ],
///                             )
///                             .build_struct(),
///                     )
///                     .build_struct(),
///             )
///             .description("Description")
///             .display_name("Displayname")
///             .parent("projects/my-project-name")
///             .build_struct(),
///     );
/// }
/// ```
/// ### Dlp Deidentify Template Image Transformations
///
///
/// ```ignore
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let basic = prevention_deidentify_template::create(
///         "basic",
///         PreventionDeidentifyTemplateArgs::builder()
///             .deidentify_config(
///                 PreventionDeidentifyTemplateDeidentifyConfig::builder()
///                     .imageTransformations(
///                         PreventionDeidentifyTemplateDeidentifyConfigImageTransformations::builder()
///                             .transforms(
///                                 vec![
///                                     PreventionDeidentifyTemplateDeidentifyConfigImageTransformationsTransform::builder()
///                                     .redactionColor(PreventionDeidentifyTemplateDeidentifyConfigImageTransformationsTransformRedactionColor::builder()
///                                     .blue(1).green(0.2).red(0.5).build_struct())
///                                     .selectedInfoTypes(PreventionDeidentifyTemplateDeidentifyConfigImageTransformationsTransformSelectedInfoTypes::builder()
///                                     .infoTypes(vec![PreventionDeidentifyTemplateDeidentifyConfigImageTransformationsTransformSelectedInfoTypesInfoType::builder()
///                                     .name("COLOR_INFO").version("latest").build_struct(),])
///                                     .build_struct()).build_struct(),
///                                     PreventionDeidentifyTemplateDeidentifyConfigImageTransformationsTransform::builder()
///                                     .allInfoTypes(PreventionDeidentifyTemplateDeidentifyConfigImageTransformationsTransformAllInfoTypes::builder()
///                                     .build_struct()).build_struct(),
///                                     PreventionDeidentifyTemplateDeidentifyConfigImageTransformationsTransform::builder()
///                                     .allText(PreventionDeidentifyTemplateDeidentifyConfigImageTransformationsTransformAllText::builder()
///                                     .build_struct()).build_struct(),
///                                 ],
///                             )
///                             .build_struct(),
///                     )
///                     .build_struct(),
///             )
///             .description("Description")
///             .display_name("Displayname")
///             .parent("projects/my-project-name")
///             .build_struct(),
///     );
/// }
/// ```
///
/// ## Import
///
/// DeidentifyTemplate can be imported using any of these accepted formats:
///
/// * `{{parent}}/deidentifyTemplates/{{name}}`
///
/// * `{{parent}}/{{name}}`
///
/// When using the `pulumi import` command, DeidentifyTemplate can be imported using one of the formats above. For example:
///
/// ```sh
/// $ pulumi import gcp:dataloss/preventionDeidentifyTemplate:PreventionDeidentifyTemplate default {{parent}}/deidentifyTemplates/{{name}}
/// ```
///
/// ```sh
/// $ pulumi import gcp:dataloss/preventionDeidentifyTemplate:PreventionDeidentifyTemplate default {{parent}}/{{name}}
/// ```
///
#[allow(clippy::doc_lazy_continuation)]
pub mod prevention_deidentify_template {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct PreventionDeidentifyTemplateArgs {
        /// Configuration of the deidentify template
        /// Structure is documented below.
        #[builder(into)]
        pub deidentify_config: pulumi_gestalt_rust::InputOrOutput<
            super::super::types::dataloss::PreventionDeidentifyTemplateDeidentifyConfig,
        >,
        /// A description of the template.
        #[builder(into, default)]
        pub description: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// User set display name of the template.
        #[builder(into, default)]
        pub display_name: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The parent of the template in any of the following formats:
        /// * `projects/{{project}}`
        /// * `projects/{{project}}/locations/{{location}}`
        /// * `organizations/{{organization_id}}`
        /// * `organizations/{{organization_id}}/locations/{{location}}`
        #[builder(into)]
        pub parent: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The template id can contain uppercase and lowercase letters, numbers, and hyphens; that is, it must match the regular
        /// expression: [a-zA-Z\d-_]+. The maximum length is 100 characters. Can be empty to allow the system to generate one.
        #[builder(into, default)]
        pub template_id: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
    }
    #[allow(dead_code)]
    pub struct PreventionDeidentifyTemplateResult {
        /// The creation timestamp of an deidentifyTemplate. Set by the server.
        pub create_time: pulumi_gestalt_rust::Output<String>,
        /// Configuration of the deidentify template
        /// Structure is documented below.
        pub deidentify_config: pulumi_gestalt_rust::Output<
            super::super::types::dataloss::PreventionDeidentifyTemplateDeidentifyConfig,
        >,
        /// A description of the template.
        pub description: pulumi_gestalt_rust::Output<Option<String>>,
        /// User set display name of the template.
        pub display_name: pulumi_gestalt_rust::Output<Option<String>>,
        /// The resource name of the template. Set by the server.
        pub name: pulumi_gestalt_rust::Output<String>,
        /// The parent of the template in any of the following formats:
        /// * `projects/{{project}}`
        /// * `projects/{{project}}/locations/{{location}}`
        /// * `organizations/{{organization_id}}`
        /// * `organizations/{{organization_id}}/locations/{{location}}`
        pub parent: pulumi_gestalt_rust::Output<String>,
        /// The template id can contain uppercase and lowercase letters, numbers, and hyphens; that is, it must match the regular
        /// expression: [a-zA-Z\d-_]+. The maximum length is 100 characters. Can be empty to allow the system to generate one.
        pub template_id: pulumi_gestalt_rust::Output<String>,
        /// The last update timestamp of an deidentifyTemplate. Set by the server.
        pub update_time: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::PulumiContext,
        name: &str,
        args: PreventionDeidentifyTemplateArgs,
    ) -> PreventionDeidentifyTemplateResult {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::register_interface;
        use std::collections::HashMap;
        let deidentify_config_binding = args
            .deidentify_config
            .get_output(context)
            .get_inner();
        let description_binding = args.description.get_output(context).get_inner();
        let display_name_binding = args.display_name.get_output(context).get_inner();
        let parent_binding = args.parent.get_output(context).get_inner();
        let template_id_binding = args.template_id.get_output(context).get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "gcp:dataloss/preventionDeidentifyTemplate:PreventionDeidentifyTemplate"
                .into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "deidentifyConfig".into(),
                    value: &deidentify_config_binding,
                },
                register_interface::ObjectField {
                    name: "description".into(),
                    value: &description_binding,
                },
                register_interface::ObjectField {
                    name: "displayName".into(),
                    value: &display_name_binding,
                },
                register_interface::ObjectField {
                    name: "parent".into(),
                    value: &parent_binding,
                },
                register_interface::ObjectField {
                    name: "templateId".into(),
                    value: &template_id_binding,
                },
            ]),
        };
        let o = register_interface::register(context.get_inner(), &request);
        PreventionDeidentifyTemplateResult {
            create_time: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("createTime"),
            ),
            deidentify_config: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("deidentifyConfig"),
            ),
            description: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("description"),
            ),
            display_name: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("displayName"),
            ),
            name: pulumi_gestalt_rust::__private::into_domain(o.extract_field("name")),
            parent: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("parent"),
            ),
            template_id: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("templateId"),
            ),
            update_time: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("updateTime"),
            ),
        }
    }
}
