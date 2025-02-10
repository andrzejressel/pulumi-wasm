#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod get_signing_profile {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GetSigningProfileArgs {
        /// Name of the target signing profile.
        #[builder(into)]
        pub name: pulumi_gestalt_rust::InputOrOutput<String>,
        /// List of tags associated with the signing profile.
        #[builder(into, default)]
        pub tags: pulumi_gestalt_rust::InputOrOutput<
            Option<std::collections::HashMap<String, String>>,
        >,
    }
    #[allow(dead_code)]
    pub struct GetSigningProfileResult {
        /// ARN for the signing profile.
        pub arn: pulumi_gestalt_rust::Output<String>,
        /// The provider-assigned unique ID for this managed resource.
        pub id: pulumi_gestalt_rust::Output<String>,
        pub name: pulumi_gestalt_rust::Output<String>,
        /// A human-readable name for the signing platform associated with the signing profile.
        pub platform_display_name: pulumi_gestalt_rust::Output<String>,
        /// ID of the platform that is used by the target signing profile.
        pub platform_id: pulumi_gestalt_rust::Output<String>,
        /// Revocation information for a signing profile.
        pub revocation_records: pulumi_gestalt_rust::Output<
            Vec<super::super::super::types::signer::GetSigningProfileRevocationRecord>,
        >,
        /// The validity period for a signing job.
        pub signature_validity_periods: pulumi_gestalt_rust::Output<
            Vec<
                super::super::super::types::signer::GetSigningProfileSignatureValidityPeriod,
            >,
        >,
        /// Status of the target signing profile.
        pub status: pulumi_gestalt_rust::Output<String>,
        /// List of tags associated with the signing profile.
        pub tags: pulumi_gestalt_rust::Output<std::collections::HashMap<String, String>>,
        /// Current version of the signing profile.
        pub version: pulumi_gestalt_rust::Output<String>,
        /// Signing profile ARN, including the profile version.
        pub version_arn: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn invoke(
        context: &pulumi_gestalt_rust::Context,
        args: GetSigningProfileArgs,
    ) -> GetSigningProfileResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let name_binding = args.name.get_output(context);
        let tags_binding = args.tags.get_output(context);
        let request = pulumi_gestalt_rust::InvokeResourceRequest {
            token: "aws:signer/getSigningProfile:getSigningProfile".into(),
            version: super::super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "name".into(),
                    value: name_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "tags".into(),
                    value: tags_binding.get_id(),
                },
            ],
        };
        let o = context.invoke_resource(request);
        GetSigningProfileResult {
            arn: o.get_field("arn"),
            id: o.get_field("id"),
            name: o.get_field("name"),
            platform_display_name: o.get_field("platformDisplayName"),
            platform_id: o.get_field("platformId"),
            revocation_records: o.get_field("revocationRecords"),
            signature_validity_periods: o.get_field("signatureValidityPeriods"),
            status: o.get_field("status"),
            tags: o.get_field("tags"),
            version: o.get_field("version"),
            version_arn: o.get_field("versionArn"),
        }
    }
}
