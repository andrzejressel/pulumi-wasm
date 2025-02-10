#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod get_subscription_iam_policy {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GetSubscriptionIamPolicyArgs {
        /// The project in which the resource belongs. If it
        /// is not provided, the provider project is used.
        #[builder(into, default)]
        pub project: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The subscription name or id to bind to attach IAM policy to.
        #[builder(into)]
        pub subscription: pulumi_gestalt_rust::InputOrOutput<String>,
    }
    #[allow(dead_code)]
    pub struct GetSubscriptionIamPolicyResult {
        /// (Computed) The etag of the IAM policy.
        pub etag: pulumi_gestalt_rust::Output<String>,
        /// The provider-assigned unique ID for this managed resource.
        pub id: pulumi_gestalt_rust::Output<String>,
        /// (Computed) The policy data
        pub policy_data: pulumi_gestalt_rust::Output<String>,
        pub project: pulumi_gestalt_rust::Output<String>,
        pub subscription: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn invoke(
        context: &pulumi_gestalt_rust::Context,
        args: GetSubscriptionIamPolicyArgs,
    ) -> GetSubscriptionIamPolicyResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let project_binding = args.project.get_output(context);
        let subscription_binding = args.subscription.get_output(context);
        let request = pulumi_gestalt_rust::InvokeResourceRequest {
            token: "gcp:pubsub/getSubscriptionIamPolicy:getSubscriptionIamPolicy".into(),
            version: super::super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "project".into(),
                    value: project_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "subscription".into(),
                    value: subscription_binding.get_id(),
                },
            ],
        };
        let o = context.invoke_resource(request);
        GetSubscriptionIamPolicyResult {
            etag: o.get_field("etag"),
            id: o.get_field("id"),
            policy_data: o.get_field("policyData"),
            project: o.get_field("project"),
            subscription: o.get_field("subscription"),
        }
    }
}
