/// Provides an Amplify Webhook resource.
///
/// ## Example Usage
///
/// ```ignore
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let example = app::create("example", AppArgs::builder().name("app").build_struct());
///     let master = branch::create(
///         "master",
///         BranchArgs::builder()
///             .app_id("${example.id}")
///             .branch_name("master")
///             .build_struct(),
///     );
///     let masterWebhook = webhook::create(
///         "masterWebhook",
///         WebhookArgs::builder()
///             .app_id("${example.id}")
///             .branch_name("${master.branchName}")
///             .description("triggermaster")
///             .build_struct(),
///     );
/// }
/// ```
///
/// ## Import
///
/// Using `pulumi import`, import Amplify webhook using a webhook ID. For example:
///
/// ```sh
/// $ pulumi import aws:amplify/webhook:Webhook master a26b22a0-748b-4b57-b9a0-ae7e601fe4b1
/// ```
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod webhook {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct WebhookArgs {
        /// Unique ID for an Amplify app.
        #[builder(into)]
        pub app_id: pulumi_gestalt_rust::InputOrOutput<String>,
        /// Name for a branch that is part of the Amplify app.
        #[builder(into)]
        pub branch_name: pulumi_gestalt_rust::InputOrOutput<String>,
        /// Description for a webhook.
        #[builder(into, default)]
        pub description: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
    }
    #[allow(dead_code)]
    pub struct WebhookResult {
        /// Unique ID for an Amplify app.
        pub app_id: pulumi_gestalt_rust::Output<String>,
        /// ARN for the webhook.
        pub arn: pulumi_gestalt_rust::Output<String>,
        /// Name for a branch that is part of the Amplify app.
        pub branch_name: pulumi_gestalt_rust::Output<String>,
        /// Description for a webhook.
        pub description: pulumi_gestalt_rust::Output<Option<String>>,
        /// URL of the webhook.
        pub url: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: WebhookArgs,
    ) -> WebhookResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let app_id_binding = args.app_id.get_output(context);
        let branch_name_binding = args.branch_name.get_output(context);
        let description_binding = args.description.get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "aws:amplify/webhook:Webhook".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "appId".into(),
                    value: &app_id_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "branchName".into(),
                    value: &branch_name_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "description".into(),
                    value: &description_binding.drop_type(),
                },
            ],
        };
        let o = context.register_resource(request);
        WebhookResult {
            app_id: o.get_field("appId"),
            arn: o.get_field("arn"),
            branch_name: o.get_field("branchName"),
            description: o.get_field("description"),
            url: o.get_field("url"),
        }
    }
}
