/// Access Keys Configuration defines the rotation policy for the keys
/// that access will use to sign data.
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod zero_trust_key_access_key_configuration {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct ZeroTrustKeyAccessKeyConfigurationArgs {
        /// The account identifier to target for the resource.
        #[builder(into)]
        pub account_id: pulumi_gestalt_rust::InputOrOutput<String>,
        /// Number of days to trigger a rotation of the keys.
        #[builder(into, default)]
        pub key_rotation_interval_days: pulumi_gestalt_rust::InputOrOutput<Option<i32>>,
    }
    #[allow(dead_code)]
    pub struct ZeroTrustKeyAccessKeyConfigurationResult {
        /// The account identifier to target for the resource.
        pub account_id: pulumi_gestalt_rust::Output<String>,
        /// Number of days to trigger a rotation of the keys.
        pub key_rotation_interval_days: pulumi_gestalt_rust::Output<i32>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: ZeroTrustKeyAccessKeyConfigurationArgs,
    ) -> ZeroTrustKeyAccessKeyConfigurationResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let account_id_binding = args.account_id.get_output(context);
        let key_rotation_interval_days_binding = args
            .key_rotation_interval_days
            .get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "cloudflare:index/zeroTrustKeyAccessKeyConfiguration:ZeroTrustKeyAccessKeyConfiguration"
                .into(),
            name: name.to_string(),
            version: super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "accountId".into(),
                    value: account_id_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "keyRotationIntervalDays".into(),
                    value: key_rotation_interval_days_binding.get_id(),
                },
            ],
        };
        let o = context.register_resource(request);
        ZeroTrustKeyAccessKeyConfigurationResult {
            account_id: o.get_field("accountId"),
            key_rotation_interval_days: o.get_field("keyRotationIntervalDays"),
        }
    }
}
