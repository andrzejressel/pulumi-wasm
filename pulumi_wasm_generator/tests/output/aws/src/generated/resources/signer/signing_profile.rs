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
pub mod signing_profile {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct SigningProfileArgs {
        #[builder(into, default)]
        pub name: pulumi_wasm_rust::Output<Option<String>>,
        #[builder(into, default)]
        pub name_prefix: pulumi_wasm_rust::Output<Option<String>>,
        /// The ID of the platform that is used by the target signing profile.
        #[builder(into)]
        pub platform_id: pulumi_wasm_rust::Output<String>,
        /// The validity period for a signing job. See `signature_validity_period` Block below for details.
        #[builder(into, default)]
        pub signature_validity_period: pulumi_wasm_rust::Output<
            Option<super::super::types::signer::SigningProfileSignatureValidityPeriod>,
        >,
        /// The AWS Certificate Manager certificate that will be used to sign code with the new signing profile. See `signing_material` Block below for details.
        #[builder(into, default)]
        pub signing_material: pulumi_wasm_rust::Output<
            Option<super::super::types::signer::SigningProfileSigningMaterial>,
        >,
        /// A list of tags associated with the signing profile. If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
        #[builder(into, default)]
        pub tags: pulumi_wasm_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
    }
    #[allow(dead_code)]
    pub struct SigningProfileResult {
        /// The Amazon Resource Name (ARN) for the signing profile.
        pub arn: pulumi_wasm_rust::Output<String>,
        pub name: pulumi_wasm_rust::Output<String>,
        pub name_prefix: pulumi_wasm_rust::Output<String>,
        /// A human-readable name for the signing platform associated with the signing profile.
        pub platform_display_name: pulumi_wasm_rust::Output<String>,
        /// The ID of the platform that is used by the target signing profile.
        pub platform_id: pulumi_wasm_rust::Output<String>,
        /// Revocation information for a signing profile. See `revocation_record` Block below for details.
        pub revocation_records: pulumi_wasm_rust::Output<
            Vec<super::super::types::signer::SigningProfileRevocationRecord>,
        >,
        /// The validity period for a signing job. See `signature_validity_period` Block below for details.
        pub signature_validity_period: pulumi_wasm_rust::Output<
            super::super::types::signer::SigningProfileSignatureValidityPeriod,
        >,
        /// The AWS Certificate Manager certificate that will be used to sign code with the new signing profile. See `signing_material` Block below for details.
        pub signing_material: pulumi_wasm_rust::Output<
            super::super::types::signer::SigningProfileSigningMaterial,
        >,
        /// The status of the target signing profile.
        pub status: pulumi_wasm_rust::Output<String>,
        /// A list of tags associated with the signing profile. If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
        pub tags: pulumi_wasm_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// A map of tags assigned to the resource, including those inherited from the provider `default_tags` configuration block.
        pub tags_all: pulumi_wasm_rust::Output<
            std::collections::HashMap<String, String>,
        >,
        /// The current version of the signing profile.
        pub version: pulumi_wasm_rust::Output<String>,
        /// The signing profile ARN, including the profile version.
        pub version_arn: pulumi_wasm_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(name: &str, args: SigningProfileArgs) -> SigningProfileResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let name_binding = args.name.get_inner();
        let name_prefix_binding = args.name_prefix.get_inner();
        let platform_id_binding = args.platform_id.get_inner();
        let signature_validity_period_binding = args
            .signature_validity_period
            .get_inner();
        let signing_material_binding = args.signing_material.get_inner();
        let tags_binding = args.tags.get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "aws:signer/signingProfile:SigningProfile".into(),
            name: name.to_string(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "name".into(),
                    value: &name_binding,
                },
                register_interface::ObjectField {
                    name: "namePrefix".into(),
                    value: &name_prefix_binding,
                },
                register_interface::ObjectField {
                    name: "platformId".into(),
                    value: &platform_id_binding,
                },
                register_interface::ObjectField {
                    name: "signatureValidityPeriod".into(),
                    value: &signature_validity_period_binding,
                },
                register_interface::ObjectField {
                    name: "signingMaterial".into(),
                    value: &signing_material_binding,
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
                    name: "name".into(),
                },
                register_interface::ResultField {
                    name: "namePrefix".into(),
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
                    name: "signatureValidityPeriod".into(),
                },
                register_interface::ResultField {
                    name: "signingMaterial".into(),
                },
                register_interface::ResultField {
                    name: "status".into(),
                },
                register_interface::ResultField {
                    name: "tags".into(),
                },
                register_interface::ResultField {
                    name: "tagsAll".into(),
                },
                register_interface::ResultField {
                    name: "version".into(),
                },
                register_interface::ResultField {
                    name: "versionArn".into(),
                },
            ]),
        };
        let o = register_interface::register(&request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        SigningProfileResult {
            arn: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("arn").unwrap(),
            ),
            name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("name").unwrap(),
            ),
            name_prefix: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("namePrefix").unwrap(),
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
            signature_validity_period: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("signatureValidityPeriod").unwrap(),
            ),
            signing_material: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("signingMaterial").unwrap(),
            ),
            status: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("status").unwrap(),
            ),
            tags: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("tags").unwrap(),
            ),
            tags_all: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("tagsAll").unwrap(),
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