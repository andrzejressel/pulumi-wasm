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
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: EmailTemplateArgs,
    ) -> EmailTemplateResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let email_templates_binding = args.email_templates.get_output(context);
        let tags_binding = args.tags.get_output(context);
        let template_name_binding = args.template_name.get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "aws:pinpoint/emailTemplate:EmailTemplate".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "emailTemplates".into(),
                    value: &email_templates_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "tags".into(),
                    value: &tags_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "templateName".into(),
                    value: &template_name_binding.drop_type(),
                },
            ],
        };
        let o = context.register_resource(request);
        EmailTemplateResult {
            arn: o.get_field("arn"),
            email_templates: o.get_field("emailTemplates"),
            tags: o.get_field("tags"),
            tags_all: o.get_field("tagsAll"),
            template_name: o.get_field("templateName"),
        }
    }
}
