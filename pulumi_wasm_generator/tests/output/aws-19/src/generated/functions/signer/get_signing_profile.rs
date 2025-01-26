pub mod get_signing_profile {
    #[derive(pulumi_wasm_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GetSigningProfileArgs {
        /// Name of the target signing profile.
        #[builder(into)]
        pub name: pulumi_wasm_rust::InputOrOutput<String>,
        /// List of tags associated with the signing profile.
        #[builder(into, default)]
        pub tags: pulumi_wasm_rust::InputOrOutput<
            Option<std::collections::HashMap<String, String>>,
        >,
    }
    #[allow(dead_code)]
    pub struct GetSigningProfileResult {
        /// ARN for the signing profile.
        pub arn: pulumi_wasm_rust::Output<String>,
        /// The provider-assigned unique ID for this managed resource.
        pub id: pulumi_wasm_rust::Output<String>,
        pub name: pulumi_wasm_rust::Output<String>,
        /// A human-readable name for the signing platform associated with the signing profile.
        pub platform_display_name: pulumi_wasm_rust::Output<String>,
        /// ID of the platform that is used by the target signing profile.
        pub platform_id: pulumi_wasm_rust::Output<String>,
        /// Revocation information for a signing profile.
        pub revocation_records: pulumi_wasm_rust::Output<
            Vec<super::super::super::types::signer::GetSigningProfileRevocationRecord>,
        >,
        /// The validity period for a signing job.
        pub signature_validity_periods: pulumi_wasm_rust::Output<
            Vec<
                super::super::super::types::signer::GetSigningProfileSignatureValidityPeriod,
            >,
        >,
        /// Status of the target signing profile.
        pub status: pulumi_wasm_rust::Output<String>,
        /// List of tags associated with the signing profile.
        pub tags: pulumi_wasm_rust::Output<std::collections::HashMap<String, String>>,
        /// Current version of the signing profile.
        pub version: pulumi_wasm_rust::Output<String>,
        /// Signing profile ARN, including the profile version.
        pub version_arn: pulumi_wasm_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn invoke(
        context: &pulumi_wasm_rust::PulumiContext,
        args: GetSigningProfileArgs,
    ) -> GetSigningProfileResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let name_binding = args.name.get_output(context).get_inner();
        let tags_binding = args.tags.get_output(context).get_inner();
        let request = register_interface::ResourceInvokeRequest {
            token: "aws:signer/getSigningProfile:getSigningProfile".into(),
            version: super::super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "name".into(),
                    value: &name_binding,
                },
                register_interface::ObjectField {
                    name: "tags".into(),
                    value: &tags_binding,
                },
            ]),
        };
        let o = register_interface::invoke(context.get_inner(), &request);
        GetSigningProfileResult {
            arn: pulumi_wasm_rust::__private::into_domain(o.extract_field("arn")),
            id: pulumi_wasm_rust::__private::into_domain(o.extract_field("id")),
            name: pulumi_wasm_rust::__private::into_domain(o.extract_field("name")),
            platform_display_name: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("platformDisplayName"),
            ),
            platform_id: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("platformId"),
            ),
            revocation_records: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("revocationRecords"),
            ),
            signature_validity_periods: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("signatureValidityPeriods"),
            ),
            status: pulumi_wasm_rust::__private::into_domain(o.extract_field("status")),
            tags: pulumi_wasm_rust::__private::into_domain(o.extract_field("tags")),
            version: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("version"),
            ),
            version_arn: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("versionArn"),
            ),
        }
    }
}
