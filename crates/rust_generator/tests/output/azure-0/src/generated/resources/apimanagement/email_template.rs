/// Manages a API Management Email Template.
///
/// ## Example Usage
///
/// ```ignore
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let example = resource_group::create(
///         "example",
///         ResourceGroupArgs::builder()
///             .location("West Europe")
///             .name("example-resources")
///             .build_struct(),
///     );
///     let exampleEmailTemplate = email_template::create(
///         "exampleEmailTemplate",
///         EmailTemplateArgs::builder()
///             .api_management_name("${exampleService.name}")
///             .body(
///                 "<!DOCTYPE html >\n<html>\n<head>\n  <meta charset=\"UTF-8\" />\n  <title>Customized Letter Title</title>\n</head>\n<body>\n  <p style=\"font-size:12pt;font-family:'Segoe UI'\">Dear $DevFirstName $DevLastName,</p>\n</body>\n</html>",
///             )
///             .resource_group_name("${example.name}")
///             .subject(
///                 "Customized confirmation email for your new $OrganizationName API account",
///             )
///             .template_name("ConfirmSignUpIdentityDefault")
///             .build_struct(),
///     );
///     let exampleService = service::create(
///         "exampleService",
///         ServiceArgs::builder()
///             .location("${example.location}")
///             .name("example-apim")
///             .publisher_email("company@terraform.io")
///             .publisher_name("My Company")
///             .resource_group_name("${example.name}")
///             .sku_name("Developer_1")
///             .build_struct(),
///     );
/// }
/// ```
///
/// ## Import
///
/// API Management Email Templates can be imported using the `resource id`, e.g.
///
/// ```sh
/// $ pulumi import azure:apimanagement/emailTemplate:EmailTemplate example /subscriptions/00000000-0000-0000-0000-000000000000/resourceGroups/group1/providers/Microsoft.ApiManagement/service/instance1/templates/template1
/// ```
///
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod email_template {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct EmailTemplateArgs {
        /// The name of the API Management Service in which the Email Template should exist. Changing this forces a new API Management Email Template to be created.
        #[builder(into)]
        pub api_management_name: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The body of the Email. Its format has to be a well-formed HTML document.
        ///
        /// > **NOTE:** In `subject` and `body` predefined parameters can be used. The available parameters depend on the template. Schema to use a parameter: `$` followed by the `parameter.name` - `$<parameter.name>`. The available parameters can be seen in the Notification templates section of the API-Management Service instance within the Azure Portal.
        #[builder(into)]
        pub body: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The name of the Resource Group where the API Management Email Template should exist. Changing this forces a new API Management Email Template to be created.
        #[builder(into)]
        pub resource_group_name: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The subject of the Email.
        #[builder(into)]
        pub subject: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The name of the Email Template. Possible values are `AccountClosedDeveloper`, `ApplicationApprovedNotificationMessage`, `ConfirmSignUpIdentityDefault`, `EmailChangeIdentityDefault`, `InviteUserNotificationMessage`, `NewCommentNotificationMessage`, `NewDeveloperNotificationMessage`, `NewIssueNotificationMessage`, `PasswordResetByAdminNotificationMessage`, `PasswordResetIdentityDefault`, `PurchaseDeveloperNotificationMessage`, `QuotaLimitApproachingDeveloperNotificationMessage`, `RejectDeveloperNotificationMessage`, `RequestDeveloperNotificationMessage`. Changing this forces a new API Management Email Template to be created.
        #[builder(into)]
        pub template_name: pulumi_gestalt_rust::InputOrOutput<String>,
    }
    #[allow(dead_code)]
    pub struct EmailTemplateResult {
        /// The name of the API Management Service in which the Email Template should exist. Changing this forces a new API Management Email Template to be created.
        pub api_management_name: pulumi_gestalt_rust::Output<String>,
        /// The body of the Email. Its format has to be a well-formed HTML document.
        ///
        /// > **NOTE:** In `subject` and `body` predefined parameters can be used. The available parameters depend on the template. Schema to use a parameter: `$` followed by the `parameter.name` - `$<parameter.name>`. The available parameters can be seen in the Notification templates section of the API-Management Service instance within the Azure Portal.
        pub body: pulumi_gestalt_rust::Output<String>,
        /// The description of the Email Template.
        pub description: pulumi_gestalt_rust::Output<String>,
        /// The name of the Resource Group where the API Management Email Template should exist. Changing this forces a new API Management Email Template to be created.
        pub resource_group_name: pulumi_gestalt_rust::Output<String>,
        /// The subject of the Email.
        pub subject: pulumi_gestalt_rust::Output<String>,
        /// The name of the Email Template. Possible values are `AccountClosedDeveloper`, `ApplicationApprovedNotificationMessage`, `ConfirmSignUpIdentityDefault`, `EmailChangeIdentityDefault`, `InviteUserNotificationMessage`, `NewCommentNotificationMessage`, `NewDeveloperNotificationMessage`, `NewIssueNotificationMessage`, `PasswordResetByAdminNotificationMessage`, `PasswordResetIdentityDefault`, `PurchaseDeveloperNotificationMessage`, `QuotaLimitApproachingDeveloperNotificationMessage`, `RejectDeveloperNotificationMessage`, `RequestDeveloperNotificationMessage`. Changing this forces a new API Management Email Template to be created.
        pub template_name: pulumi_gestalt_rust::Output<String>,
        /// The title of the Email Template.
        pub title: pulumi_gestalt_rust::Output<String>,
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
        let api_management_name_binding = args
            .api_management_name
            .get_output(context)
            .get_inner();
        let body_binding = args.body.get_output(context).get_inner();
        let resource_group_name_binding = args
            .resource_group_name
            .get_output(context)
            .get_inner();
        let subject_binding = args.subject.get_output(context).get_inner();
        let template_name_binding = args.template_name.get_output(context).get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "azure:apimanagement/emailTemplate:EmailTemplate".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "apiManagementName".into(),
                    value: &api_management_name_binding,
                },
                register_interface::ObjectField {
                    name: "body".into(),
                    value: &body_binding,
                },
                register_interface::ObjectField {
                    name: "resourceGroupName".into(),
                    value: &resource_group_name_binding,
                },
                register_interface::ObjectField {
                    name: "subject".into(),
                    value: &subject_binding,
                },
                register_interface::ObjectField {
                    name: "templateName".into(),
                    value: &template_name_binding,
                },
            ]),
        };
        let o = register_interface::register(context.get_inner(), &request);
        EmailTemplateResult {
            api_management_name: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("apiManagementName"),
            ),
            body: pulumi_gestalt_rust::__private::into_domain(o.extract_field("body")),
            description: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("description"),
            ),
            resource_group_name: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("resourceGroupName"),
            ),
            subject: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("subject"),
            ),
            template_name: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("templateName"),
            ),
            title: pulumi_gestalt_rust::__private::into_domain(o.extract_field("title")),
        }
    }
}
