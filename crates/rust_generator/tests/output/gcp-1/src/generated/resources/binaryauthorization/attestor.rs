/// An attestor that attests to container image artifacts.
///
///
/// To get more information about Attestor, see:
///
/// * [API documentation](https://cloud.google.com/binary-authorization/docs/reference/rest/)
/// * How-to Guides
///     * [Official Documentation](https://cloud.google.com/binary-authorization/)
///
/// ## Example Usage
///
/// ### Binary Authorization Attestor Basic
///
///
/// ```ignore
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let attestor = attestor::create(
///         "attestor",
///         AttestorArgs::builder()
///             .attestation_authority_note(
///                 AttestorAttestationAuthorityNote::builder()
///                     .noteReference("${note.name}")
///                     .publicKeys(
///                         vec![
///                             AttestorAttestationAuthorityNotePublicKey::builder()
///                             .asciiArmoredPgpPublicKey("mQENBFtP0doBCADF+joTiXWKVuP8kJt3fgpBSjT9h8ezMfKA4aXZctYLx5wslWQl\nbB7Iu2ezkECNzoEeU7WxUe8a61pMCh9cisS9H5mB2K2uM4Jnf8tgFeXn3akJDVo0\noR1IC+Dp9mXbRSK3MAvKkOwWlG99sx3uEdvmeBRHBOO+grchLx24EThXFOyP9Fk6\nV39j6xMjw4aggLD15B4V0v9JqBDdJiIYFzszZDL6pJwZrzcP0z8JO4rTZd+f64bD\nMpj52j/pQfA8lZHOaAgb1OrthLdMrBAjoDjArV4Ek7vSbrcgYWcI6BhsQrFoxKdX\n83TZKai55ZCfCLIskwUIzA1NLVwyzCS+fSN/ABEBAAG0KCJUZXN0IEF0dGVzdG9y\nIiA8ZGFuYWhvZmZtYW5AZ29vZ2xlLmNvbT6JAU4EEwEIADgWIQRfWkqHt6hpTA1L\nuY060eeM4dc66AUCW0/R2gIbLwULCQgHAgYVCgkICwIEFgIDAQIeAQIXgAAKCRA6\n0eeM4dc66HdpCAC4ot3b0OyxPb0Ip+WT2U0PbpTBPJklesuwpIrM4Lh0N+1nVRLC\n51WSmVbM8BiAFhLbN9LpdHhds1kUrHF7+wWAjdR8sqAj9otc6HGRM/3qfa2qgh+U\nWTEk/3us/rYSi7T7TkMuutRMIa1IkR13uKiW56csEMnbOQpn9rDqwIr5R8nlZP5h\nMAU9vdm1DIv567meMqTaVZgR3w7bck2P49AO8lO5ERFpVkErtu/98y+rUy9d789l\n+OPuS1NGnxI1YKsNaWJF4uJVuvQuZ1twrhCbGNtVorO2U12+cEq+YtUxj7kmdOC1\nqoIRW6y0+UlAc+MbqfL0ziHDOAmcqz1GnROg\n=6Bvm\n")
///                             .build_struct(),
///                         ],
///                     )
///                     .build_struct(),
///             )
///             .name("test-attestor")
///             .build_struct(),
///     );
///     let note = note::create(
///         "note",
///         NoteArgs::builder()
///             .attestation_authority(
///                 NoteAttestationAuthority::builder()
///                     .hint(
///                         NoteAttestationAuthorityHint::builder()
///                             .humanReadableName("Attestor Note")
///                             .build_struct(),
///                     )
///                     .build_struct(),
///             )
///             .name("test-attestor-note")
///             .build_struct(),
///     );
/// }
/// ```
/// ### Binary Authorization Attestor Kms
///
///
/// ```yaml
/// resources:
///   attestor:
///     type: gcp:binaryauthorization:Attestor
///     properties:
///       name: test-attestor
///       attestationAuthorityNote:
///         noteReference: ${note.name}
///         publicKeys:
///           - id: ${version.id}
///             pkixPublicKey:
///               publicKeyPem: ${version.publicKeys[0].pem}
///               signatureAlgorithm: ${version.publicKeys[0].algorithm}
///   note:
///     type: gcp:containeranalysis:Note
///     properties:
///       name: test-attestor-note
///       attestationAuthority:
///         hint:
///           humanReadableName: Attestor Note
///   crypto-key:
///     type: gcp:kms:CryptoKey
///     properties:
///       name: test-attestor-key
///       keyRing: ${keyring.id}
///       purpose: ASYMMETRIC_SIGN
///       versionTemplate:
///         algorithm: RSA_SIGN_PKCS1_4096_SHA512
///   keyring:
///     type: gcp:kms:KeyRing
///     properties:
///       name: test-attestor-key-ring
///       location: global
/// variables:
///   version:
///     fn::invoke:
///       function: gcp:kms:getKMSCryptoKeyVersion
///       arguments:
///         cryptoKey: ${["crypto-key"].id}
/// ```
///
/// ## Import
///
/// Attestor can be imported using any of these accepted formats:
///
/// * `projects/{{project}}/attestors/{{name}}`
///
/// * `{{project}}/{{name}}`
///
/// * `{{name}}`
///
/// When using the `pulumi import` command, Attestor can be imported using one of the formats above. For example:
///
/// ```sh
/// $ pulumi import gcp:binaryauthorization/attestor:Attestor default projects/{{project}}/attestors/{{name}}
/// ```
///
/// ```sh
/// $ pulumi import gcp:binaryauthorization/attestor:Attestor default {{project}}/{{name}}
/// ```
///
/// ```sh
/// $ pulumi import gcp:binaryauthorization/attestor:Attestor default {{name}}
/// ```
///
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod attestor {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct AttestorArgs {
        /// A Container Analysis ATTESTATION_AUTHORITY Note, created by the user.
        /// Structure is documented below.
        #[builder(into)]
        pub attestation_authority_note: pulumi_gestalt_rust::InputOrOutput<
            super::super::types::binaryauthorization::AttestorAttestationAuthorityNote,
        >,
        /// A descriptive comment. This field may be updated. The field may be displayed in chooser dialogs.
        #[builder(into, default)]
        pub description: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The resource name.
        #[builder(into, default)]
        pub name: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        #[builder(into, default)]
        pub project: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
    }
    #[allow(dead_code)]
    pub struct AttestorResult {
        /// A Container Analysis ATTESTATION_AUTHORITY Note, created by the user.
        /// Structure is documented below.
        pub attestation_authority_note: pulumi_gestalt_rust::Output<
            super::super::types::binaryauthorization::AttestorAttestationAuthorityNote,
        >,
        /// A descriptive comment. This field may be updated. The field may be displayed in chooser dialogs.
        pub description: pulumi_gestalt_rust::Output<Option<String>>,
        /// The resource name.
        pub name: pulumi_gestalt_rust::Output<String>,
        pub project: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: AttestorArgs,
    ) -> AttestorResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let attestation_authority_note_binding = args
            .attestation_authority_note
            .get_output(context);
        let description_binding = args.description.get_output(context);
        let name_binding = args.name.get_output(context);
        let project_binding = args.project.get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "gcp:binaryauthorization/attestor:Attestor".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "attestationAuthorityNote".into(),
                    value: attestation_authority_note_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "description".into(),
                    value: description_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "name".into(),
                    value: name_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "project".into(),
                    value: project_binding.get_id(),
                },
            ],
        };
        let o = context.register_resource(request);
        AttestorResult {
            attestation_authority_note: o.get_field("attestationAuthorityNote"),
            description: o.get_field("description"),
            name: o.get_field("name"),
            project: o.get_field("project"),
        }
    }
}
