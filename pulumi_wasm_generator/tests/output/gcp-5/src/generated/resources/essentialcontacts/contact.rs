/// A contact that will receive notifications from Google Cloud.
///
///
/// To get more information about Contact, see:
///
/// * [API documentation](https://cloud.google.com/resource-manager/docs/reference/essentialcontacts/rest/v1/projects.contacts)
/// * How-to Guides
///     * [Official Documentation](https://cloud.google.com/resource-manager/docs/managing-notification-contacts)
///
/// > **Warning:** If you are using User ADCs (Application Default Credentials) with this resource,
/// you must specify a `billing_project` and set `user_project_override` to true
/// in the provider configuration. Otherwise the Essential Contacts API will return a 403 error.
/// Your account must have the `serviceusage.services.use` permission on the
/// `billing_project` you defined.
///
/// ## Example Usage
///
/// ### Essential Contact
///
///
/// ```yaml
/// resources:
///   contact:
///     type: gcp:essentialcontacts:Contact
///     properties:
///       parent: ${project.id}
///       email: foo@bar.com
///       languageTag: en-GB
///       notificationCategorySubscriptions:
///         - ALL
/// variables:
///   project:
///     fn::invoke:
///       function: gcp:organizations:getProject
///       arguments: {}
/// ```
///
/// ## Import
///
/// Contact can be imported using any of these accepted formats:
///
/// * `{{name}}`
///
/// When using the `pulumi import` command, Contact can be imported using one of the formats above. For example:
///
/// ```sh
/// $ pulumi import gcp:essentialcontacts/contact:Contact default {{name}}
/// ```
///
pub mod contact {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct ContactArgs {
        /// The email address to send notifications to. This does not need to be a Google account.
        #[builder(into)]
        pub email: pulumi_wasm_rust::Output<String>,
        /// The preferred language for notifications, as a ISO 639-1 language code. See Supported languages for a list of supported languages.
        #[builder(into)]
        pub language_tag: pulumi_wasm_rust::Output<String>,
        /// The categories of notifications that the contact will receive communications for.
        #[builder(into)]
        pub notification_category_subscriptions: pulumi_wasm_rust::Output<Vec<String>>,
        /// The resource to save this contact for. Format: organizations/{organization_id}, folders/{folder_id} or projects/{project_id}
        ///
        ///
        /// - - -
        #[builder(into)]
        pub parent: pulumi_wasm_rust::Output<String>,
    }
    #[allow(dead_code)]
    pub struct ContactResult {
        /// The email address to send notifications to. This does not need to be a Google account.
        pub email: pulumi_wasm_rust::Output<String>,
        /// The preferred language for notifications, as a ISO 639-1 language code. See Supported languages for a list of supported languages.
        pub language_tag: pulumi_wasm_rust::Output<String>,
        /// The identifier for the contact. Format: {resourceType}/{resource_id}/contacts/{contact_id}
        pub name: pulumi_wasm_rust::Output<String>,
        /// The categories of notifications that the contact will receive communications for.
        pub notification_category_subscriptions: pulumi_wasm_rust::Output<Vec<String>>,
        /// The resource to save this contact for. Format: organizations/{organization_id}, folders/{folder_id} or projects/{project_id}
        ///
        ///
        /// - - -
        pub parent: pulumi_wasm_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(name: &str, args: ContactArgs) -> ContactResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let email_binding = args.email.get_inner();
        let language_tag_binding = args.language_tag.get_inner();
        let notification_category_subscriptions_binding = args
            .notification_category_subscriptions
            .get_inner();
        let parent_binding = args.parent.get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "gcp:essentialcontacts/contact:Contact".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "email".into(),
                    value: &email_binding,
                },
                register_interface::ObjectField {
                    name: "languageTag".into(),
                    value: &language_tag_binding,
                },
                register_interface::ObjectField {
                    name: "notificationCategorySubscriptions".into(),
                    value: &notification_category_subscriptions_binding,
                },
                register_interface::ObjectField {
                    name: "parent".into(),
                    value: &parent_binding,
                },
            ]),
            results: Vec::from([
                register_interface::ResultField {
                    name: "email".into(),
                },
                register_interface::ResultField {
                    name: "languageTag".into(),
                },
                register_interface::ResultField {
                    name: "name".into(),
                },
                register_interface::ResultField {
                    name: "notificationCategorySubscriptions".into(),
                },
                register_interface::ResultField {
                    name: "parent".into(),
                },
            ]),
        };
        let o = register_interface::register(&request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        ContactResult {
            email: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("email").unwrap(),
            ),
            language_tag: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("languageTag").unwrap(),
            ),
            name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("name").unwrap(),
            ),
            notification_category_subscriptions: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("notificationCategorySubscriptions").unwrap(),
            ),
            parent: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("parent").unwrap(),
            ),
        }
    }
}
