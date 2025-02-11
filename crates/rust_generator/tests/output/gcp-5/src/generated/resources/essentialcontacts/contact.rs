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
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod contact {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct ContactArgs {
        /// The email address to send notifications to. This does not need to be a Google account.
        #[builder(into)]
        pub email: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The preferred language for notifications, as a ISO 639-1 language code. See Supported languages for a list of supported languages.
        #[builder(into)]
        pub language_tag: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The categories of notifications that the contact will receive communications for.
        #[builder(into)]
        pub notification_category_subscriptions: pulumi_gestalt_rust::InputOrOutput<
            Vec<String>,
        >,
        /// The resource to save this contact for. Format: organizations/{organization_id}, folders/{folder_id} or projects/{project_id}
        ///
        ///
        /// - - -
        #[builder(into)]
        pub parent: pulumi_gestalt_rust::InputOrOutput<String>,
    }
    #[allow(dead_code)]
    pub struct ContactResult {
        /// The email address to send notifications to. This does not need to be a Google account.
        pub email: pulumi_gestalt_rust::Output<String>,
        /// The preferred language for notifications, as a ISO 639-1 language code. See Supported languages for a list of supported languages.
        pub language_tag: pulumi_gestalt_rust::Output<String>,
        /// The identifier for the contact. Format: {resourceType}/{resource_id}/contacts/{contact_id}
        pub name: pulumi_gestalt_rust::Output<String>,
        /// The categories of notifications that the contact will receive communications for.
        pub notification_category_subscriptions: pulumi_gestalt_rust::Output<
            Vec<String>,
        >,
        /// The resource to save this contact for. Format: organizations/{organization_id}, folders/{folder_id} or projects/{project_id}
        ///
        ///
        /// - - -
        pub parent: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: ContactArgs,
    ) -> ContactResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let email_binding = args.email.get_output(context);
        let language_tag_binding = args.language_tag.get_output(context);
        let notification_category_subscriptions_binding = args
            .notification_category_subscriptions
            .get_output(context);
        let parent_binding = args.parent.get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "gcp:essentialcontacts/contact:Contact".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "email".into(),
                    value: &email_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "languageTag".into(),
                    value: &language_tag_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "notificationCategorySubscriptions".into(),
                    value: &notification_category_subscriptions_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "parent".into(),
                    value: &parent_binding.drop_type(),
                },
            ],
        };
        let o = context.register_resource(request);
        ContactResult {
            email: o.get_field("email"),
            language_tag: o.get_field("languageTag"),
            name: o.get_field("name"),
            notification_category_subscriptions: o
                .get_field("notificationCategorySubscriptions"),
            parent: o.get_field("parent"),
        }
    }
}
