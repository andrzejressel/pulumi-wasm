/// Manages a API Management Email Template.
///
/// ## Example Usage
///
/// ```ignore
/// use pulumi_wasm_rust::Output;
/// use pulumi_wasm_rust::{add_export, pulumi_main};
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
pub mod email_template {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct EmailTemplateArgs {
        /// The name of the API Management Service in which the Email Template should exist. Changing this forces a new API Management Email Template to be created.
        #[builder(into)]
        pub api_management_name: pulumi_wasm_rust::Output<String>,
        /// The body of the Email. Its format has to be a well-formed HTML document.
        ///
        /// > **NOTE:** In `subject` and `body` predefined parameters can be used. The available parameters depend on the template. Schema to use a parameter: `$` followed by the `parameter.name` - `$<parameter.name>`. The available parameters can be seen in the Notification templates section of the API-Management Service instance within the Azure Portal.
        #[builder(into)]
        pub body: pulumi_wasm_rust::Output<String>,
        /// The name of the Resource Group where the API Management Email Template should exist. Changing this forces a new API Management Email Template to be created.
        #[builder(into)]
        pub resource_group_name: pulumi_wasm_rust::Output<String>,
        /// The subject of the Email.
        #[builder(into)]
        pub subject: pulumi_wasm_rust::Output<String>,
        /// The name of the Email Template. Possible values are `AccountClosedDeveloper`, `ApplicationApprovedNotificationMessage`, `ConfirmSignUpIdentityDefault`, `EmailChangeIdentityDefault`, `InviteUserNotificationMessage`, `NewCommentNotificationMessage`, `NewDeveloperNotificationMessage`, `NewIssueNotificationMessage`, `PasswordResetByAdminNotificationMessage`, `PasswordResetIdentityDefault`, `PurchaseDeveloperNotificationMessage`, `QuotaLimitApproachingDeveloperNotificationMessage`, `RejectDeveloperNotificationMessage`, `RequestDeveloperNotificationMessage`. Changing this forces a new API Management Email Template to be created.
        #[builder(into)]
        pub template_name: pulumi_wasm_rust::Output<String>,
    }
    #[allow(dead_code)]
    pub struct EmailTemplateResult {
        /// The name of the API Management Service in which the Email Template should exist. Changing this forces a new API Management Email Template to be created.
        pub api_management_name: pulumi_wasm_rust::Output<String>,
        /// The body of the Email. Its format has to be a well-formed HTML document.
        ///
        /// > **NOTE:** In `subject` and `body` predefined parameters can be used. The available parameters depend on the template. Schema to use a parameter: `$` followed by the `parameter.name` - `$<parameter.name>`. The available parameters can be seen in the Notification templates section of the API-Management Service instance within the Azure Portal.
        pub body: pulumi_wasm_rust::Output<String>,
        /// The description of the Email Template.
        pub description: pulumi_wasm_rust::Output<String>,
        /// The name of the Resource Group where the API Management Email Template should exist. Changing this forces a new API Management Email Template to be created.
        pub resource_group_name: pulumi_wasm_rust::Output<String>,
        /// The subject of the Email.
        pub subject: pulumi_wasm_rust::Output<String>,
        /// The name of the Email Template. Possible values are `AccountClosedDeveloper`, `ApplicationApprovedNotificationMessage`, `ConfirmSignUpIdentityDefault`, `EmailChangeIdentityDefault`, `InviteUserNotificationMessage`, `NewCommentNotificationMessage`, `NewDeveloperNotificationMessage`, `NewIssueNotificationMessage`, `PasswordResetByAdminNotificationMessage`, `PasswordResetIdentityDefault`, `PurchaseDeveloperNotificationMessage`, `QuotaLimitApproachingDeveloperNotificationMessage`, `RejectDeveloperNotificationMessage`, `RequestDeveloperNotificationMessage`. Changing this forces a new API Management Email Template to be created.
        pub template_name: pulumi_wasm_rust::Output<String>,
        /// The title of the Email Template.
        pub title: pulumi_wasm_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(name: &str, args: EmailTemplateArgs) -> EmailTemplateResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let api_management_name_binding = args.api_management_name.get_inner();
        let body_binding = args.body.get_inner();
        let resource_group_name_binding = args.resource_group_name.get_inner();
        let subject_binding = args.subject.get_inner();
        let template_name_binding = args.template_name.get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "azure:apimanagement/emailTemplate:EmailTemplate".into(),
            name: name.to_string(),
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
            results: Vec::from([
                register_interface::ResultField {
                    name: "apiManagementName".into(),
                },
                register_interface::ResultField {
                    name: "body".into(),
                },
                register_interface::ResultField {
                    name: "description".into(),
                },
                register_interface::ResultField {
                    name: "resourceGroupName".into(),
                },
                register_interface::ResultField {
                    name: "subject".into(),
                },
                register_interface::ResultField {
                    name: "templateName".into(),
                },
                register_interface::ResultField {
                    name: "title".into(),
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
            api_management_name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("apiManagementName").unwrap(),
            ),
            body: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("body").unwrap(),
            ),
            description: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("description").unwrap(),
            ),
            resource_group_name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("resourceGroupName").unwrap(),
            ),
            subject: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("subject").unwrap(),
            ),
            template_name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("templateName").unwrap(),
            ),
            title: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("title").unwrap(),
            ),
        }
    }
}