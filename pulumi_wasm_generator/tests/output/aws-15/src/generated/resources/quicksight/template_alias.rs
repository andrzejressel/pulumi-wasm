/// Resource for managing an AWS QuickSight Template Alias.
///
/// ## Example Usage
///
/// ### Basic Usage
///
/// ```yaml
/// resources:
///   example:
///     type: aws:quicksight:TemplateAlias
///     properties:
///       aliasName: example-alias
///       templateId: ${test.templateId}
///       templateVersionNumber: ${test.versionNumber}
/// ```
///
/// ## Import
///
/// Using `pulumi import`, import QuickSight Template Alias using the AWS account ID, template ID, and alias name separated by a comma (`,`). For example:
///
/// ```sh
/// $ pulumi import aws:quicksight/templateAlias:TemplateAlias example 123456789012,example-id,example-alias
/// ```
pub mod template_alias {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct TemplateAliasArgs {
        /// Display name of the template alias.
        #[builder(into)]
        pub alias_name: pulumi_wasm_rust::Output<String>,
        /// AWS account ID.
        #[builder(into, default)]
        pub aws_account_id: pulumi_wasm_rust::Output<Option<String>>,
        /// ID of the template.
        #[builder(into)]
        pub template_id: pulumi_wasm_rust::Output<String>,
        /// Version number of the template.
        ///
        /// The following arguments are optional:
        #[builder(into)]
        pub template_version_number: pulumi_wasm_rust::Output<i32>,
    }
    #[allow(dead_code)]
    pub struct TemplateAliasResult {
        /// Display name of the template alias.
        pub alias_name: pulumi_wasm_rust::Output<String>,
        /// Amazon Resource Name (ARN) of the template alias.
        pub arn: pulumi_wasm_rust::Output<String>,
        /// AWS account ID.
        pub aws_account_id: pulumi_wasm_rust::Output<String>,
        /// ID of the template.
        pub template_id: pulumi_wasm_rust::Output<String>,
        /// Version number of the template.
        ///
        /// The following arguments are optional:
        pub template_version_number: pulumi_wasm_rust::Output<i32>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(name: &str, args: TemplateAliasArgs) -> TemplateAliasResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let alias_name_binding = args.alias_name.get_inner();
        let aws_account_id_binding = args.aws_account_id.get_inner();
        let template_id_binding = args.template_id.get_inner();
        let template_version_number_binding = args.template_version_number.get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "aws:quicksight/templateAlias:TemplateAlias".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "aliasName".into(),
                    value: &alias_name_binding,
                },
                register_interface::ObjectField {
                    name: "awsAccountId".into(),
                    value: &aws_account_id_binding,
                },
                register_interface::ObjectField {
                    name: "templateId".into(),
                    value: &template_id_binding,
                },
                register_interface::ObjectField {
                    name: "templateVersionNumber".into(),
                    value: &template_version_number_binding,
                },
            ]),
            results: Vec::from([
                register_interface::ResultField {
                    name: "aliasName".into(),
                },
                register_interface::ResultField {
                    name: "arn".into(),
                },
                register_interface::ResultField {
                    name: "awsAccountId".into(),
                },
                register_interface::ResultField {
                    name: "templateId".into(),
                },
                register_interface::ResultField {
                    name: "templateVersionNumber".into(),
                },
            ]),
        };
        let o = register_interface::register(&request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        TemplateAliasResult {
            alias_name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("aliasName").unwrap(),
            ),
            arn: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("arn").unwrap(),
            ),
            aws_account_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("awsAccountId").unwrap(),
            ),
            template_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("templateId").unwrap(),
            ),
            template_version_number: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("templateVersionNumber").unwrap(),
            ),
        }
    }
}
