/// A Container Analysis note is a high-level piece of metadata that
/// describes a type of analysis that can be done for a resource.
///
///
/// To get more information about Note, see:
///
/// * [API documentation](https://cloud.google.com/container-analysis/api/reference/rest/)
/// * How-to Guides
///     * [Creating Attestations (Occurrences)](https://cloud.google.com/binary-authorization/docs/making-attestations)
///     * [Official Documentation](https://cloud.google.com/container-analysis/)
///
/// ## Example Usage
///
/// ### Container Analysis Note Basic
///
///
/// ```ignore
/// use pulumi_wasm_rust::Output;
/// use pulumi_wasm_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
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
///             .name("attestor-note")
///             .build_struct(),
///     );
/// }
/// ```
/// ### Container Analysis Note Attestation Full
///
///
/// ```ignore
/// use pulumi_wasm_rust::Output;
/// use pulumi_wasm_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
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
///             .expiration_time("2120-10-02T15:01:23.045123456Z")
///             .long_description("a longer description of test note")
///             .name("attestor-note")
///             .related_urls(
///                 vec![
///                     NoteRelatedUrl::builder().label("foo").url("some.url")
///                     .build_struct(), NoteRelatedUrl::builder().url("google.com")
///                     .build_struct(),
///                 ],
///             )
///             .short_description("test note")
///             .build_struct(),
///     );
/// }
/// ```
///
/// ## Import
///
/// Note can be imported using any of these accepted formats:
///
/// * `projects/{{project}}/notes/{{name}}`
///
/// * `{{project}}/{{name}}`
///
/// * `{{name}}`
///
/// When using the `pulumi import` command, Note can be imported using one of the formats above. For example:
///
/// ```sh
/// $ pulumi import gcp:containeranalysis/note:Note default projects/{{project}}/notes/{{name}}
/// ```
///
/// ```sh
/// $ pulumi import gcp:containeranalysis/note:Note default {{project}}/{{name}}
/// ```
///
/// ```sh
/// $ pulumi import gcp:containeranalysis/note:Note default {{name}}
/// ```
///
pub mod note {
    #[derive(pulumi_wasm_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct NoteArgs {
        /// Note kind that represents a logical attestation "role" or "authority".
        /// For example, an organization might have one AttestationAuthority for
        /// "QA" and one for "build". This Note is intended to act strictly as a
        /// grouping mechanism for the attached Occurrences (Attestations). This
        /// grouping mechanism also provides a security boundary, since IAM ACLs
        /// gate the ability for a principle to attach an Occurrence to a given
        /// Note. It also provides a single point of lookup to find all attached
        /// Attestation Occurrences, even if they don't all live in the same
        /// project.
        /// Structure is documented below.
        #[builder(into)]
        pub attestation_authority: pulumi_wasm_rust::InputOrOutput<
            super::super::types::containeranalysis::NoteAttestationAuthority,
        >,
        /// Time of expiration for this note. Leave empty if note does not expire.
        #[builder(into, default)]
        pub expiration_time: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        /// A detailed description of the note
        #[builder(into, default)]
        pub long_description: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        /// The name of the note.
        #[builder(into, default)]
        pub name: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        #[builder(into, default)]
        pub project: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        /// Names of other notes related to this note.
        #[builder(into, default)]
        pub related_note_names: pulumi_wasm_rust::InputOrOutput<Option<Vec<String>>>,
        /// URLs associated with this note and related metadata.
        #[builder(into, default)]
        pub related_urls: pulumi_wasm_rust::InputOrOutput<
            Option<Vec<super::super::types::containeranalysis::NoteRelatedUrl>>,
        >,
        /// A one sentence description of the note.
        #[builder(into, default)]
        pub short_description: pulumi_wasm_rust::InputOrOutput<Option<String>>,
    }
    #[allow(dead_code)]
    pub struct NoteResult {
        /// Note kind that represents a logical attestation "role" or "authority".
        /// For example, an organization might have one AttestationAuthority for
        /// "QA" and one for "build". This Note is intended to act strictly as a
        /// grouping mechanism for the attached Occurrences (Attestations). This
        /// grouping mechanism also provides a security boundary, since IAM ACLs
        /// gate the ability for a principle to attach an Occurrence to a given
        /// Note. It also provides a single point of lookup to find all attached
        /// Attestation Occurrences, even if they don't all live in the same
        /// project.
        /// Structure is documented below.
        pub attestation_authority: pulumi_wasm_rust::Output<
            super::super::types::containeranalysis::NoteAttestationAuthority,
        >,
        /// The time this note was created.
        pub create_time: pulumi_wasm_rust::Output<String>,
        /// Time of expiration for this note. Leave empty if note does not expire.
        pub expiration_time: pulumi_wasm_rust::Output<Option<String>>,
        /// The type of analysis this note describes
        pub kind: pulumi_wasm_rust::Output<String>,
        /// A detailed description of the note
        pub long_description: pulumi_wasm_rust::Output<Option<String>>,
        /// The name of the note.
        pub name: pulumi_wasm_rust::Output<String>,
        pub project: pulumi_wasm_rust::Output<String>,
        /// Names of other notes related to this note.
        pub related_note_names: pulumi_wasm_rust::Output<Option<Vec<String>>>,
        /// URLs associated with this note and related metadata.
        pub related_urls: pulumi_wasm_rust::Output<
            Option<Vec<super::super::types::containeranalysis::NoteRelatedUrl>>,
        >,
        /// A one sentence description of the note.
        pub short_description: pulumi_wasm_rust::Output<Option<String>>,
        /// The time this note was last updated.
        pub update_time: pulumi_wasm_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_wasm_rust::PulumiContext,
        name: &str,
        args: NoteArgs,
    ) -> NoteResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let attestation_authority_binding = args
            .attestation_authority
            .get_output(context)
            .get_inner();
        let expiration_time_binding = args
            .expiration_time
            .get_output(context)
            .get_inner();
        let long_description_binding = args
            .long_description
            .get_output(context)
            .get_inner();
        let name_binding = args.name.get_output(context).get_inner();
        let project_binding = args.project.get_output(context).get_inner();
        let related_note_names_binding = args
            .related_note_names
            .get_output(context)
            .get_inner();
        let related_urls_binding = args.related_urls.get_output(context).get_inner();
        let short_description_binding = args
            .short_description
            .get_output(context)
            .get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "gcp:containeranalysis/note:Note".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "attestationAuthority".into(),
                    value: &attestation_authority_binding,
                },
                register_interface::ObjectField {
                    name: "expirationTime".into(),
                    value: &expiration_time_binding,
                },
                register_interface::ObjectField {
                    name: "longDescription".into(),
                    value: &long_description_binding,
                },
                register_interface::ObjectField {
                    name: "name".into(),
                    value: &name_binding,
                },
                register_interface::ObjectField {
                    name: "project".into(),
                    value: &project_binding,
                },
                register_interface::ObjectField {
                    name: "relatedNoteNames".into(),
                    value: &related_note_names_binding,
                },
                register_interface::ObjectField {
                    name: "relatedUrls".into(),
                    value: &related_urls_binding,
                },
                register_interface::ObjectField {
                    name: "shortDescription".into(),
                    value: &short_description_binding,
                },
            ]),
        };
        let o = register_interface::register(context.get_inner(), &request);
        NoteResult {
            attestation_authority: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("attestationAuthority"),
            ),
            create_time: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("createTime"),
            ),
            expiration_time: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("expirationTime"),
            ),
            kind: pulumi_wasm_rust::__private::into_domain(o.extract_field("kind")),
            long_description: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("longDescription"),
            ),
            name: pulumi_wasm_rust::__private::into_domain(o.extract_field("name")),
            project: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("project"),
            ),
            related_note_names: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("relatedNoteNames"),
            ),
            related_urls: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("relatedUrls"),
            ),
            short_description: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("shortDescription"),
            ),
            update_time: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("updateTime"),
            ),
        }
    }
}
