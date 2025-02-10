/// Creates a Signer Signing Profile. A signing profile contains information about the code signing configuration parameters that can be used by a given code signing user.
///
/// ## Example Usage
///
/// ```yaml
/// resources:
///   testSp:
///     type: aws:signer:SigningProfile
///     name: test_sp
///     properties:
///       platformId: AWSLambda-SHA384-ECDSA
///   prodSp:
///     type: aws:signer:SigningProfile
///     name: prod_sp
///     properties:
///       platformId: AWSLambda-SHA384-ECDSA
///       namePrefix: prod_sp_
///       signatureValidityPeriod:
///         value: 5
///         type: YEARS
///       tags:
///         tag1: value1
///         tag2: value2
/// ```
///
/// ## Import
///
/// Using `pulumi import`, import Signer signing profiles using the `name`. For example:
///
/// ```sh
/// $ pulumi import aws:signer/signingProfile:SigningProfile test_signer_signing_profile test_sp_DdW3Mk1foYL88fajut4mTVFGpuwfd4ACO6ANL0D1uIj7lrn8adK
/// ```
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod signing_profile {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct SigningProfileArgs {
        #[builder(into, default)]
        pub name: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        #[builder(into, default)]
        pub name_prefix: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The ID of the platform that is used by the target signing profile.
        #[builder(into)]
        pub platform_id: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The validity period for a signing job. See `signature_validity_period` Block below for details.
        #[builder(into, default)]
        pub signature_validity_period: pulumi_gestalt_rust::InputOrOutput<
            Option<super::super::types::signer::SigningProfileSignatureValidityPeriod>,
        >,
        /// The AWS Certificate Manager certificate that will be used to sign code with the new signing profile. See `signing_material` Block below for details.
        #[builder(into, default)]
        pub signing_material: pulumi_gestalt_rust::InputOrOutput<
            Option<super::super::types::signer::SigningProfileSigningMaterial>,
        >,
        /// A list of tags associated with the signing profile. If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
        #[builder(into, default)]
        pub tags: pulumi_gestalt_rust::InputOrOutput<
            Option<std::collections::HashMap<String, String>>,
        >,
    }
    #[allow(dead_code)]
    pub struct SigningProfileResult {
        /// The Amazon Resource Name (ARN) for the signing profile.
        pub arn: pulumi_gestalt_rust::Output<String>,
        pub name: pulumi_gestalt_rust::Output<String>,
        pub name_prefix: pulumi_gestalt_rust::Output<String>,
        /// A human-readable name for the signing platform associated with the signing profile.
        pub platform_display_name: pulumi_gestalt_rust::Output<String>,
        /// The ID of the platform that is used by the target signing profile.
        pub platform_id: pulumi_gestalt_rust::Output<String>,
        /// Revocation information for a signing profile. See `revocation_record` Block below for details.
        pub revocation_records: pulumi_gestalt_rust::Output<
            Vec<super::super::types::signer::SigningProfileRevocationRecord>,
        >,
        /// The validity period for a signing job. See `signature_validity_period` Block below for details.
        pub signature_validity_period: pulumi_gestalt_rust::Output<
            super::super::types::signer::SigningProfileSignatureValidityPeriod,
        >,
        /// The AWS Certificate Manager certificate that will be used to sign code with the new signing profile. See `signing_material` Block below for details.
        pub signing_material: pulumi_gestalt_rust::Output<
            super::super::types::signer::SigningProfileSigningMaterial,
        >,
        /// The status of the target signing profile.
        pub status: pulumi_gestalt_rust::Output<String>,
        /// A list of tags associated with the signing profile. If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
        pub tags: pulumi_gestalt_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// A map of tags assigned to the resource, including those inherited from the provider `default_tags` configuration block.
        pub tags_all: pulumi_gestalt_rust::Output<
            std::collections::HashMap<String, String>,
        >,
        /// The current version of the signing profile.
        pub version: pulumi_gestalt_rust::Output<String>,
        /// The signing profile ARN, including the profile version.
        pub version_arn: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: SigningProfileArgs,
    ) -> SigningProfileResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let name_binding = args.name.get_output(context);
        let name_prefix_binding = args.name_prefix.get_output(context);
        let platform_id_binding = args.platform_id.get_output(context);
        let signature_validity_period_binding = args
            .signature_validity_period
            .get_output(context);
        let signing_material_binding = args.signing_material.get_output(context);
        let tags_binding = args.tags.get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "aws:signer/signingProfile:SigningProfile".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "name".into(),
                    value: name_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "namePrefix".into(),
                    value: name_prefix_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "platformId".into(),
                    value: platform_id_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "signatureValidityPeriod".into(),
                    value: signature_validity_period_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "signingMaterial".into(),
                    value: signing_material_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "tags".into(),
                    value: tags_binding.get_id(),
                },
            ],
        };
        let o = context.register_resource(request);
        SigningProfileResult {
            arn: o.get_field("arn"),
            name: o.get_field("name"),
            name_prefix: o.get_field("namePrefix"),
            platform_display_name: o.get_field("platformDisplayName"),
            platform_id: o.get_field("platformId"),
            revocation_records: o.get_field("revocationRecords"),
            signature_validity_period: o.get_field("signatureValidityPeriod"),
            signing_material: o.get_field("signingMaterial"),
            status: o.get_field("status"),
            tags: o.get_field("tags"),
            tags_all: o.get_field("tagsAll"),
            version: o.get_field("version"),
            version_arn: o.get_field("versionArn"),
        }
    }
}
