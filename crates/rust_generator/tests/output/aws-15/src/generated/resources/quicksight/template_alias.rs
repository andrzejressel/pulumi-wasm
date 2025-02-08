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
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod template_alias {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct TemplateAliasArgs {
        /// Display name of the template alias.
        #[builder(into)]
        pub alias_name: pulumi_gestalt_rust::InputOrOutput<String>,
        /// AWS account ID.
        #[builder(into, default)]
        pub aws_account_id: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// ID of the template.
        #[builder(into)]
        pub template_id: pulumi_gestalt_rust::InputOrOutput<String>,
        /// Version number of the template.
        ///
        /// The following arguments are optional:
        #[builder(into)]
        pub template_version_number: pulumi_gestalt_rust::InputOrOutput<i32>,
    }
    #[allow(dead_code)]
    pub struct TemplateAliasResult {
        /// Display name of the template alias.
        pub alias_name: pulumi_gestalt_rust::Output<String>,
        /// Amazon Resource Name (ARN) of the template alias.
        pub arn: pulumi_gestalt_rust::Output<String>,
        /// AWS account ID.
        pub aws_account_id: pulumi_gestalt_rust::Output<String>,
        /// ID of the template.
        pub template_id: pulumi_gestalt_rust::Output<String>,
        /// Version number of the template.
        ///
        /// The following arguments are optional:
        pub template_version_number: pulumi_gestalt_rust::Output<i32>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::PulumiContext,
        name: &str,
        args: TemplateAliasArgs,
    ) -> TemplateAliasResult {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::register_interface;
        use std::collections::HashMap;
        let alias_name_binding = args.alias_name.get_output(context).get_inner();
        let aws_account_id_binding = args.aws_account_id.get_output(context).get_inner();
        let template_id_binding = args.template_id.get_output(context).get_inner();
        let template_version_number_binding = args
            .template_version_number
            .get_output(context)
            .get_inner();
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
        };
        let o = register_interface::register(context.get_inner(), &request);
        TemplateAliasResult {
            alias_name: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("aliasName"),
            ),
            arn: pulumi_gestalt_rust::__private::into_domain(o.extract_field("arn")),
            aws_account_id: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("awsAccountId"),
            ),
            template_id: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("templateId"),
            ),
            template_version_number: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("templateVersionNumber"),
            ),
        }
    }
}
