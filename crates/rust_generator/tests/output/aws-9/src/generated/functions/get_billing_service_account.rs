#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod get_billing_service_account {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GetBillingServiceAccountArgs {
        /// ID of the AWS billing service account.
        #[builder(into, default)]
        pub id: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
    }
    #[allow(dead_code)]
    pub struct GetBillingServiceAccountResult {
        /// ARN of the AWS billing service account.
        pub arn: pulumi_gestalt_rust::Output<String>,
        /// ID of the AWS billing service account.
        pub id: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn invoke(
        context: &pulumi_gestalt_rust::Context,
        args: GetBillingServiceAccountArgs,
    ) -> GetBillingServiceAccountResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let id_binding = args.id.get_output(context);
        let request = pulumi_gestalt_rust::InvokeResourceRequest {
            token: "aws:index/getBillingServiceAccount:getBillingServiceAccount".into(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "id".into(),
                    value: id_binding.get_id(),
                },
            ],
        };
        let o = context.invoke_resource(request);
        GetBillingServiceAccountResult {
            arn: o.get_field("arn"),
            id: o.get_field("id"),
        }
    }
}
