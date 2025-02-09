/// Provides a Pinpoint Email Template resource
///
/// ## Example Usage
///
/// ```ignore
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let test = email_template::create(
///         "test",
///         EmailTemplateArgs::builder()
///             .email_templates(
///                 vec![
///                     EmailTemplateEmailTemplate::builder()
///                     .headers(vec![EmailTemplateEmailTemplateHeader::builder()
///                     .name("testingname").value("testingvalue").build_struct(),])
///                     .subject("testing").textPart("we are testing template text part")
///                     .build_struct(),
///                 ],
///             )
///             .template_name("testing")
///             .build_struct(),
///     );
/// }
/// ```
///
/// ## Import
///
/// Using `pulumi import`, import Pinpoint Email Template using the `template_name`. For example:
///
/// ```sh
/// $ pulumi import aws:pinpoint/emailTemplate:EmailTemplate reset template_name
/// ```
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod email_template {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct EmailTemplateArgs {
        /// Specifies the content and settings for a message template that can be used in messages that are sent through the email channel. See Email Template
        #[builder(into, default)]
        pub email_templates: pulumi_gestalt_rust::InputOrOutput<
            Option<Vec<super::super::types::pinpoint::EmailTemplateEmailTemplate>>,
        >,
        #[builder(into, default)]
        pub tags: pulumi_gestalt_rust::InputOrOutput<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// name of the message template. A template name must start with an alphanumeric character and can contain a maximum of 128 characters. The characters can be alphanumeric characters, underscores (_), or hyphens (-). Template names are case sensitive.
        #[builder(into)]
        pub template_name: pulumi_gestalt_rust::InputOrOutput<String>,
    }
    #[allow(dead_code)]
    pub struct EmailTemplateResult {
        /// Amazon Resource Name (ARN) of the message template.
        pub arn: pulumi_gestalt_rust::Output<String>,
        /// Specifies the content and settings for a message template that can be used in messages that are sent through the email channel. See Email Template
        pub email_templates: pulumi_gestalt_rust::Output<
            Option<Vec<super::super::types::pinpoint::EmailTemplateEmailTemplate>>,
        >,
        pub tags: pulumi_gestalt_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        pub tags_all: pulumi_gestalt_rust::Output<
            std::collections::HashMap<String, String>,
        >,
        /// name of the message template. A template name must start with an alphanumeric character and can contain a maximum of 128 characters. The characters can be alphanumeric characters, underscores (_), or hyphens (-). Template names are case sensitive.
        pub template_name: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::PulumiContext,
        name: &str,
        args: EmailTemplateArgs,
    ) -> EmailTemplateResult {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::register_interface;
        use std::collections::HashMap;
        let email_templates_binding_1 = args.email_templates.get_output(context);
        let email_templates_binding = email_templates_binding_1.get_inner();
        let tags_binding_1 = args.tags.get_output(context);
        let tags_binding = tags_binding_1.get_inner();
        let template_name_binding_1 = args.template_name.get_output(context);
        let template_name_binding = template_name_binding_1.get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "aws:pinpoint/emailTemplate:EmailTemplate".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "emailTemplates".into(),
                    value: &email_templates_binding,
                },
                register_interface::ObjectField {
                    name: "tags".into(),
                    value: &tags_binding,
                },
                register_interface::ObjectField {
                    name: "templateName".into(),
                    value: &template_name_binding,
                },
            ]),
        };
        let o = register_interface::register(context.get_inner(), &request);
        EmailTemplateResult {
            arn: pulumi_gestalt_rust::__private::into_domain(o.extract_field("arn")),
            email_templates: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("emailTemplates"),
            ),
            tags: pulumi_gestalt_rust::__private::into_domain(o.extract_field("tags")),
            tags_all: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("tagsAll"),
            ),
            template_name: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("templateName"),
            ),
        }
    }
}
