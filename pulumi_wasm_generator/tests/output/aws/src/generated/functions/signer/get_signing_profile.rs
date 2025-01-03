pub mod get_signing_profile {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GetSigningProfileArgs {
        /// Name of the target signing profile.
        #[builder(into)]
        pub name: pulumi_wasm_rust::Output<String>,
        /// List of tags associated with the signing profile.
        #[builder(into, default)]
        pub tags: pulumi_wasm_rust::Output<
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
    pub fn invoke(args: GetSigningProfileArgs) -> GetSigningProfileResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let name_binding = args.name.get_inner();
        let tags_binding = args.tags.get_inner();
        let request = register_interface::ResourceInvokeRequest {
            token: "aws:signer/getSigningProfile:getSigningProfile".into(),
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
            results: Vec::from([
                register_interface::ResultField {
                    name: "arn".into(),
                },
                register_interface::ResultField {
                    name: "id".into(),
                },
                register_interface::ResultField {
                    name: "name".into(),
                },
                register_interface::ResultField {
                    name: "platformDisplayName".into(),
                },
                register_interface::ResultField {
                    name: "platformId".into(),
                },
                register_interface::ResultField {
                    name: "revocationRecords".into(),
                },
                register_interface::ResultField {
                    name: "signatureValidityPeriods".into(),
                },
                register_interface::ResultField {
                    name: "status".into(),
                },
                register_interface::ResultField {
                    name: "tags".into(),
                },
                register_interface::ResultField {
                    name: "version".into(),
                },
                register_interface::ResultField {
                    name: "versionArn".into(),
                },
            ]),
        };
        let o = register_interface::invoke(&request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        GetSigningProfileResult {
            arn: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("arn").unwrap(),
            ),
            id: pulumi_wasm_rust::__private::into_domain(hashmap.remove("id").unwrap()),
            name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("name").unwrap(),
            ),
            platform_display_name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("platformDisplayName").unwrap(),
            ),
            platform_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("platformId").unwrap(),
            ),
            revocation_records: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("revocationRecords").unwrap(),
            ),
            signature_validity_periods: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("signatureValidityPeriods").unwrap(),
            ),
            status: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("status").unwrap(),
            ),
            tags: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("tags").unwrap(),
            ),
            version: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("version").unwrap(),
            ),
            version_arn: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("versionArn").unwrap(),
            ),
        }
    }
}
