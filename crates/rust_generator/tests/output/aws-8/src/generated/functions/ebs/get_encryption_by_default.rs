#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod get_encryption_by_default {
    #[allow(dead_code)]
    pub struct GetEncryptionByDefaultResult {
        /// Whether or not default EBS encryption is enabled. Returns as `true` or `false`.
        pub enabled: pulumi_gestalt_rust::Output<bool>,
        /// The provider-assigned unique ID for this managed resource.
        pub id: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn invoke(
        context: &pulumi_gestalt_rust::Context,
    ) -> GetEncryptionByDefaultResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let request = pulumi_gestalt_rust::InvokeResourceRequest {
            token: "aws:ebs/getEncryptionByDefault:getEncryptionByDefault".into(),
            version: super::super::super::get_version(),
            object: &[],
        };
        let o = context.invoke_resource(request);
        GetEncryptionByDefaultResult {
            enabled: o.get_field("enabled"),
            id: o.get_field("id"),
        }
    }
}
