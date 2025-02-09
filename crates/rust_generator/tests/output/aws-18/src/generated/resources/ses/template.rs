/// Provides a resource to create a SES template.
///
/// ## Example Usage
///
/// ```ignore
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let myTemplate = template::create(
///         "myTemplate",
///         TemplateArgs::builder()
///             .html(
///                 "<h1>Hello {{name}},</h1><p>Your favorite animal is {{favoriteanimal}}.</p>",
///             )
///             .name("MyTemplate")
///             .subject("Greetings, {{name}}!")
///             .text("Hello {{name}},\r\nYour favorite animal is {{favoriteanimal}}.")
///             .build_struct(),
///     );
/// }
/// ```
///
/// ## Import
///
/// Using `pulumi import`, import SES templates using the template name. For example:
///
/// ```sh
/// $ pulumi import aws:ses/template:Template MyTemplate MyTemplate
/// ```
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod template {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct TemplateArgs {
        /// The HTML body of the email. Must be less than 500KB in size, including both the text and HTML parts.
        #[builder(into, default)]
        pub html: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The name of the template. Cannot exceed 64 characters. You will refer to this name when you send email.
        #[builder(into, default)]
        pub name: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The subject line of the email.
        #[builder(into, default)]
        pub subject: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The email body that will be visible to recipients whose email clients do not display HTML. Must be less than 500KB in size, including both the text and HTML parts.
        #[builder(into, default)]
        pub text: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
    }
    #[allow(dead_code)]
    pub struct TemplateResult {
        /// The ARN of the SES template
        pub arn: pulumi_gestalt_rust::Output<String>,
        /// The HTML body of the email. Must be less than 500KB in size, including both the text and HTML parts.
        pub html: pulumi_gestalt_rust::Output<Option<String>>,
        /// The name of the template. Cannot exceed 64 characters. You will refer to this name when you send email.
        pub name: pulumi_gestalt_rust::Output<String>,
        /// The subject line of the email.
        pub subject: pulumi_gestalt_rust::Output<Option<String>>,
        /// The email body that will be visible to recipients whose email clients do not display HTML. Must be less than 500KB in size, including both the text and HTML parts.
        pub text: pulumi_gestalt_rust::Output<Option<String>>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: TemplateArgs,
    ) -> TemplateResult {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::register_interface;
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let html_binding = args.html.get_output(context);
        let name_binding = args.name.get_output(context);
        let subject_binding = args.subject.get_output(context);
        let text_binding = args.text.get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "aws:ses/template:Template".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "html".into(),
                    value: html_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "name".into(),
                    value: name_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "subject".into(),
                    value: subject_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "text".into(),
                    value: text_binding.get_id(),
                },
            ],
        };
        let o = context.register_resource(request);
        TemplateResult {
            arn: o.get_field("arn"),
            html: o.get_field("html"),
            name: o.get_field("name"),
            subject: o.get_field("subject"),
            text: o.get_field("text"),
        }
    }
}
