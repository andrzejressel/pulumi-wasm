/// Provides a resource to create a SES template.
///
/// ## Example Usage
///
/// ```ignore
/// use pulumi_wasm_rust::Output;
/// use pulumi_wasm_rust::{add_export, pulumi_main};
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
pub mod template {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct TemplateArgs {
        /// The HTML body of the email. Must be less than 500KB in size, including both the text and HTML parts.
        #[builder(into, default)]
        pub html: pulumi_wasm_rust::Output<Option<String>>,
        /// The name of the template. Cannot exceed 64 characters. You will refer to this name when you send email.
        #[builder(into, default)]
        pub name: pulumi_wasm_rust::Output<Option<String>>,
        /// The subject line of the email.
        #[builder(into, default)]
        pub subject: pulumi_wasm_rust::Output<Option<String>>,
        /// The email body that will be visible to recipients whose email clients do not display HTML. Must be less than 500KB in size, including both the text and HTML parts.
        #[builder(into, default)]
        pub text: pulumi_wasm_rust::Output<Option<String>>,
    }
    #[allow(dead_code)]
    pub struct TemplateResult {
        /// The ARN of the SES template
        pub arn: pulumi_wasm_rust::Output<String>,
        /// The HTML body of the email. Must be less than 500KB in size, including both the text and HTML parts.
        pub html: pulumi_wasm_rust::Output<Option<String>>,
        /// The name of the template. Cannot exceed 64 characters. You will refer to this name when you send email.
        pub name: pulumi_wasm_rust::Output<String>,
        /// The subject line of the email.
        pub subject: pulumi_wasm_rust::Output<Option<String>>,
        /// The email body that will be visible to recipients whose email clients do not display HTML. Must be less than 500KB in size, including both the text and HTML parts.
        pub text: pulumi_wasm_rust::Output<Option<String>>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(name: &str, args: TemplateArgs) -> TemplateResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let html_binding = args.html.get_inner();
        let name_binding = args.name.get_inner();
        let subject_binding = args.subject.get_inner();
        let text_binding = args.text.get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "aws:ses/template:Template".into(),
            name: name.to_string(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "html".into(),
                    value: &html_binding,
                },
                register_interface::ObjectField {
                    name: "name".into(),
                    value: &name_binding,
                },
                register_interface::ObjectField {
                    name: "subject".into(),
                    value: &subject_binding,
                },
                register_interface::ObjectField {
                    name: "text".into(),
                    value: &text_binding,
                },
            ]),
            results: Vec::from([
                register_interface::ResultField {
                    name: "arn".into(),
                },
                register_interface::ResultField {
                    name: "html".into(),
                },
                register_interface::ResultField {
                    name: "name".into(),
                },
                register_interface::ResultField {
                    name: "subject".into(),
                },
                register_interface::ResultField {
                    name: "text".into(),
                },
            ]),
        };
        let o = register_interface::register(&request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        TemplateResult {
            arn: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("arn").unwrap(),
            ),
            html: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("html").unwrap(),
            ),
            name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("name").unwrap(),
            ),
            subject: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("subject").unwrap(),
            ),
            text: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("text").unwrap(),
            ),
        }
    }
}