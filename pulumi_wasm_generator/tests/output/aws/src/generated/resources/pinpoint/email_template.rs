/// Provides a Pinpoint Email Template resource
///
/// ## Example Usage
///
/// ```ignore
/// use pulumi_wasm_rust::Output;
/// use pulumi_wasm_rust::{add_export, pulumi_main};
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
pub mod email_template {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct EmailTemplateArgs {
        /// Specifies the content and settings for a message template that can be used in messages that are sent through the email channel. See Email Template
        #[builder(into, default)]
        pub email_templates: pulumi_wasm_rust::Output<
            Option<Vec<super::super::types::pinpoint::EmailTemplateEmailTemplate>>,
        >,
        #[builder(into, default)]
        pub tags: pulumi_wasm_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// name of the message template. A template name must start with an alphanumeric character and can contain a maximum of 128 characters. The characters can be alphanumeric characters, underscores (_), or hyphens (-). Template names are case sensitive.
        #[builder(into)]
        pub template_name: pulumi_wasm_rust::Output<String>,
    }
    #[allow(dead_code)]
    pub struct EmailTemplateResult {
        /// Amazon Resource Name (ARN) of the message template.
        pub arn: pulumi_wasm_rust::Output<String>,
        /// Specifies the content and settings for a message template that can be used in messages that are sent through the email channel. See Email Template
        pub email_templates: pulumi_wasm_rust::Output<
            Option<Vec<super::super::types::pinpoint::EmailTemplateEmailTemplate>>,
        >,
        pub tags: pulumi_wasm_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        pub tags_all: pulumi_wasm_rust::Output<
            std::collections::HashMap<String, String>,
        >,
        /// name of the message template. A template name must start with an alphanumeric character and can contain a maximum of 128 characters. The characters can be alphanumeric characters, underscores (_), or hyphens (-). Template names are case sensitive.
        pub template_name: pulumi_wasm_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(name: &str, args: EmailTemplateArgs) -> EmailTemplateResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let email_templates_binding = args.email_templates.get_inner();
        let tags_binding = args.tags.get_inner();
        let template_name_binding = args.template_name.get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "aws:pinpoint/emailTemplate:EmailTemplate".into(),
            name: name.to_string(),
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
            results: Vec::from([
                register_interface::ResultField {
                    name: "arn".into(),
                },
                register_interface::ResultField {
                    name: "emailTemplates".into(),
                },
                register_interface::ResultField {
                    name: "tags".into(),
                },
                register_interface::ResultField {
                    name: "tagsAll".into(),
                },
                register_interface::ResultField {
                    name: "templateName".into(),
                },
            ]),
        };
        let o = register_interface::register(&request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        EmailTemplateResult {
            arn: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("arn").unwrap(),
            ),
            email_templates: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("emailTemplates").unwrap(),
            ),
            tags: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("tags").unwrap(),
            ),
            tags_all: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("tagsAll").unwrap(),
            ),
            template_name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("templateName").unwrap(),
            ),
        }
    }
}