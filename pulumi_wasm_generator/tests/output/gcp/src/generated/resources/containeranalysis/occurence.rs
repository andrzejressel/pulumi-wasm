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
pub mod occurence {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
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
        pub attestation: pulumi_wasm_rust::Output<
            super::super::types::containeranalysis::OccurenceAttestation,
        >,
        /// The analysis note associated with this occurrence, in the form of
        /// projects/[PROJECT]/notes/[NOTE_ID]. This field can be used as a
        /// filter in list requests.
        #[builder(into)]
        pub note_name: pulumi_wasm_rust::Output<String>,
        #[builder(into, default)]
        pub project: pulumi_wasm_rust::Output<Option<String>>,
        /// A description of actions that can be taken to remedy the note.
        #[builder(into, default)]
        pub remediation: pulumi_wasm_rust::Output<Option<String>>,
        /// Required. Immutable. A URI that represents the resource for which
        /// the occurrence applies. For example,
        /// https://gcr.io/project/image@sha256:123abc for a Docker image.
        #[builder(into)]
        pub resource_uri: pulumi_wasm_rust::Output<String>,
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
        pub attestation: pulumi_wasm_rust::Output<
            super::super::types::containeranalysis::OccurenceAttestation,
        >,
        /// The time when the repository was created.
        pub create_time: pulumi_wasm_rust::Output<String>,
        /// The note kind which explicitly denotes which of the occurrence
        /// details are specified. This field can be used as a filter in list
        /// requests.
        pub kind: pulumi_wasm_rust::Output<String>,
        /// The name of the occurrence.
        pub name: pulumi_wasm_rust::Output<String>,
        /// The analysis note associated with this occurrence, in the form of
        /// projects/[PROJECT]/notes/[NOTE_ID]. This field can be used as a
        /// filter in list requests.
        pub note_name: pulumi_wasm_rust::Output<String>,
        pub project: pulumi_wasm_rust::Output<String>,
        /// A description of actions that can be taken to remedy the note.
        pub remediation: pulumi_wasm_rust::Output<Option<String>>,
        /// Required. Immutable. A URI that represents the resource for which
        /// the occurrence applies. For example,
        /// https://gcr.io/project/image@sha256:123abc for a Docker image.
        pub resource_uri: pulumi_wasm_rust::Output<String>,
        /// The time when the repository was last updated.
        pub update_time: pulumi_wasm_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(name: &str, args: OccurenceArgs) -> OccurenceResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let attestation_binding = args.attestation.get_inner();
        let note_name_binding = args.note_name.get_inner();
        let project_binding = args.project.get_inner();
        let remediation_binding = args.remediation.get_inner();
        let resource_uri_binding = args.resource_uri.get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "gcp:containeranalysis/occurence:Occurence".into(),
            name: name.to_string(),
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
            results: Vec::from([
                register_interface::ResultField {
                    name: "attestation".into(),
                },
                register_interface::ResultField {
                    name: "createTime".into(),
                },
                register_interface::ResultField {
                    name: "kind".into(),
                },
                register_interface::ResultField {
                    name: "name".into(),
                },
                register_interface::ResultField {
                    name: "noteName".into(),
                },
                register_interface::ResultField {
                    name: "project".into(),
                },
                register_interface::ResultField {
                    name: "remediation".into(),
                },
                register_interface::ResultField {
                    name: "resourceUri".into(),
                },
                register_interface::ResultField {
                    name: "updateTime".into(),
                },
            ]),
        };
        let o = register_interface::register(&request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        OccurenceResult {
            attestation: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("attestation").unwrap(),
            ),
            create_time: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("createTime").unwrap(),
            ),
            kind: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("kind").unwrap(),
            ),
            name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("name").unwrap(),
            ),
            note_name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("noteName").unwrap(),
            ),
            project: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("project").unwrap(),
            ),
            remediation: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("remediation").unwrap(),
            ),
            resource_uri: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("resourceUri").unwrap(),
            ),
            update_time: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("updateTime").unwrap(),
            ),
        }
    }
}
