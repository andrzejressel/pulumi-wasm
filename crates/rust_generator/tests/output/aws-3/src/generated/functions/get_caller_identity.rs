#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod get_caller_identity {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GetCallerIdentityArgs {
        /// Account ID number of the account that owns or contains the calling entity.
        #[builder(into, default)]
        pub id: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
    }
    #[allow(dead_code)]
    pub struct GetCallerIdentityResult {
        /// AWS Account ID number of the account that owns or contains the calling entity.
        pub account_id: pulumi_gestalt_rust::Output<String>,
        /// ARN associated with the calling entity.
        pub arn: pulumi_gestalt_rust::Output<String>,
        /// Account ID number of the account that owns or contains the calling entity.
        pub id: pulumi_gestalt_rust::Output<String>,
        /// Unique identifier of the calling entity.
        pub user_id: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn invoke(
        context: &pulumi_gestalt_rust::Context,
        args: GetCallerIdentityArgs,
    ) -> GetCallerIdentityResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let id_binding = args.id.get_output(context);
        let request = pulumi_gestalt_rust::InvokeResourceRequest {
            token: "aws:index/getCallerIdentity:getCallerIdentity".into(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "id".into(),
                    value: id_binding.get_id(),
                },
            ],
        };
        let o = context.invoke_resource(request);
        GetCallerIdentityResult {
            account_id: o.get_field("accountId"),
            arn: o.get_field("arn"),
            id: o.get_field("id"),
            user_id: o.get_field("userId"),
        }
    }
}
