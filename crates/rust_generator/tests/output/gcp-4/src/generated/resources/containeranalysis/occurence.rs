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
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: OccurenceArgs,
    ) -> OccurenceResult {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::register_interface;
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let attestation_binding = args.attestation.get_output(context);
        let note_name_binding = args.note_name.get_output(context);
        let project_binding = args.project.get_output(context);
        let remediation_binding = args.remediation.get_output(context);
        let resource_uri_binding = args.resource_uri.get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "gcp:containeranalysis/occurence:Occurence".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "attestation".into(),
                    value: attestation_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "noteName".into(),
                    value: note_name_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "project".into(),
                    value: project_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "remediation".into(),
                    value: remediation_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "resourceUri".into(),
                    value: resource_uri_binding.get_id(),
                },
            ],
        };
        let o = context.register_resource(request);
        OccurenceResult {
            attestation: o.get_field("attestation"),
            create_time: o.get_field("createTime"),
            kind: o.get_field("kind"),
            name: o.get_field("name"),
            note_name: o.get_field("noteName"),
            project: o.get_field("project"),
            remediation: o.get_field("remediation"),
            resource_uri: o.get_field("resourceUri"),
            update_time: o.get_field("updateTime"),
        }
    }
}
