/// An occurrence is an instance of a Note, or type of analysis that
/// can be done for a resource.
///
///
/// To get more information about Occurrence, see:
///
/// * [API documentation](https://cloud.google.com/container-analysis/api/reference/rest/)
/// * How-to Guides
///     * [Official Documentation](https://cloud.google.com/container-analysis/)
///
/// ## Example Usage
///
/// ### Container Analysis Occurrence Kms
///
///
/// ```yaml
/// resources:
///   attestor:
///     type: gcp:binaryauthorization:Attestor
///     properties:
///       name: attestor
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
///       name: attestation-note
///       attestationAuthority:
///         hint:
///           humanReadableName: Attestor Note
///   occurrence:
///     type: gcp:containeranalysis:Occurence
///     properties:
///       resourceUri: gcr.io/my-project/my-image
///       noteName: ${note.id}
///       attestation:
///         serializedPayload:
///           fn::invoke:
///             function: std:filebase64
///             arguments:
///               input: path/to/my/payload.json
///             return: result
///         signatures:
///           - publicKeyId: ${version.id}
///             serializedPayload:
///               fn::invoke:
///                 function: std:filebase64
///                 arguments:
///                   input: path/to/my/payload.json.sig
///                 return: result
/// variables:
///   keyring:
///     fn::invoke:
///       function: gcp:kms:getKMSKeyRing
///       arguments:
///         name: my-key-ring
///         location: global
///   crypto-key:
///     fn::invoke:
///       function: gcp:kms:getKMSCryptoKey
///       arguments:
///         name: my-key
///         keyRing: ${keyring.id}
///   version:
///     fn::invoke:
///       function: gcp:kms:getKMSCryptoKeyVersion
///       arguments:
///         cryptoKey: ${["crypto-key"].id}
/// ```
///
/// ## Import
///
/// Occurrence can be imported using any of these accepted formats:
///
/// * `projects/{{project}}/occurrences/{{name}}`
///
/// * `{{project}}/{{name}}`
///
/// * `{{name}}`
///
/// When using the `pulumi import` command, Occurrence can be imported using one of the formats above. For example:
///
/// ```sh
/// $ pulumi import gcp:containeranalysis/occurence:Occurence default projects/{{project}}/occurrences/{{name}}
/// ```
///
/// ```sh
/// $ pulumi import gcp:containeranalysis/occurence:Occurence default {{project}}/{{name}}
/// ```
///
/// ```sh
/// $ pulumi import gcp:containeranalysis/occurence:Occurence default {{name}}
/// ```
///
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod occurence {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct OccurenceArgs {
        /// Occurrence that represents a single "attestation". The authenticity
        /// of an attestation can be verified using the attached signature.
        /// If the verifier trusts the public key of the signer, then verifying
        /// the signature is sufficient to establish trust. In this circumstance,
        /// the authority to which this attestation is attached is primarily
        /// useful for lookup (how to find this attestation if you already
        /// know the authority and artifact to be verified) and intent (for
        /// which authority this attestation was intended to sign.
        /// Structure is documented below.
        #[builder(into)]
        pub attestation: pulumi_gestalt_rust::InputOrOutput<
            super::super::types::containeranalysis::OccurenceAttestation,
        >,
        /// The analysis note associated with this occurrence, in the form of
        /// projects/[PROJECT]/notes/[NOTE_ID]. This field can be used as a
        /// filter in list requests.
        #[builder(into)]
        pub note_name: pulumi_gestalt_rust::InputOrOutput<String>,
        #[builder(into, default)]
        pub project: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// A description of actions that can be taken to remedy the note.
        #[builder(into, default)]
        pub remediation: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Required. Immutable. A URI that represents the resource for which
        /// the occurrence applies. For example,
        /// https://gcr.io/project/image@sha256:123abc for a Docker image.
        #[builder(into)]
        pub resource_uri: pulumi_gestalt_rust::InputOrOutput<String>,
    }
    #[allow(dead_code)]
    pub struct OccurenceResult {
        /// Occurrence that represents a single "attestation". The authenticity
        /// of an attestation can be verified using the attached signature.
        /// If the verifier trusts the public key of the signer, then verifying
        /// the signature is sufficient to establish trust. In this circumstance,
        /// the authority to which this attestation is attached is primarily
        /// useful for lookup (how to find this attestation if you already
        /// know the authority and artifact to be verified) and intent (for
        /// which authority this attestation was intended to sign.
        /// Structure is documented below.
        pub attestation: pulumi_gestalt_rust::Output<
            super::super::types::containeranalysis::OccurenceAttestation,
        >,
        /// The time when the repository was created.
        pub create_time: pulumi_gestalt_rust::Output<String>,
        /// The note kind which explicitly denotes which of the occurrence
        /// details are specified. This field can be used as a filter in list
        /// requests.
        pub kind: pulumi_gestalt_rust::Output<String>,
        /// The name of the occurrence.
        pub name: pulumi_gestalt_rust::Output<String>,
        /// The analysis note associated with this occurrence, in the form of
        /// projects/[PROJECT]/notes/[NOTE_ID]. This field can be used as a
        /// filter in list requests.
        pub note_name: pulumi_gestalt_rust::Output<String>,
        pub project: pulumi_gestalt_rust::Output<String>,
        /// A description of actions that can be taken to remedy the note.
        pub remediation: pulumi_gestalt_rust::Output<Option<String>>,
        /// Required. Immutable. A URI that represents the resource for which
        /// the occurrence applies. For example,
        /// https://gcr.io/project/image@sha256:123abc for a Docker image.
        pub resource_uri: pulumi_gestalt_rust::Output<String>,
        /// The time when the repository was last updated.
        pub update_time: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::PulumiContext,
        name: &str,
        args: OccurenceArgs,
    ) -> OccurenceResult {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::register_interface;
        use std::collections::HashMap;
        let attestation_binding_1 = args.attestation.get_output(context);
        let attestation_binding = attestation_binding_1.get_inner();
        let note_name_binding_1 = args.note_name.get_output(context);
        let note_name_binding = note_name_binding_1.get_inner();
        let project_binding_1 = args.project.get_output(context);
        let project_binding = project_binding_1.get_inner();
        let remediation_binding_1 = args.remediation.get_output(context);
        let remediation_binding = remediation_binding_1.get_inner();
        let resource_uri_binding_1 = args.resource_uri.get_output(context);
        let resource_uri_binding = resource_uri_binding_1.get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "gcp:containeranalysis/occurence:Occurence".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "attestation".into(),
                    value: &attestation_binding,
                },
                register_interface::ObjectField {
                    name: "noteName".into(),
                    value: &note_name_binding,
                },
                register_interface::ObjectField {
                    name: "project".into(),
                    value: &project_binding,
                },
                register_interface::ObjectField {
                    name: "remediation".into(),
                    value: &remediation_binding,
                },
                register_interface::ObjectField {
                    name: "resourceUri".into(),
                    value: &resource_uri_binding,
                },
            ]),
        };
        let o = register_interface::register(context.get_inner(), &request);
        OccurenceResult {
            attestation: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("attestation"),
            ),
            create_time: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("createTime"),
            ),
            kind: pulumi_gestalt_rust::__private::into_domain(o.extract_field("kind")),
            name: pulumi_gestalt_rust::__private::into_domain(o.extract_field("name")),
            note_name: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("noteName"),
            ),
            project: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("project"),
            ),
            remediation: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("remediation"),
            ),
            resource_uri: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("resourceUri"),
            ),
            update_time: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("updateTime"),
            ),
        }
    }
}
