/// A policy for container image binary authorization.
///
///
/// To get more information about Policy, see:
///
/// * [API documentation](https://cloud.google.com/binary-authorization/docs/reference/rest/)
/// * How-to Guides
///     * [Official Documentation](https://cloud.google.com/binary-authorization/)
///
/// ## Example Usage
///
/// ### Binary Authorization Policy Basic
///
///
/// ```ignore
/// use pulumi_wasm_rust::Output;
/// use pulumi_wasm_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let attestor = attestor::create(
///         "attestor",
///         AttestorArgs::builder()
///             .attestation_authority_note(
///                 AttestorAttestationAuthorityNote::builder()
///                     .noteReference("${note.name}")
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
///                             .humanReadableName("My attestor")
///                             .build_struct(),
///                     )
///                     .build_struct(),
///             )
///             .name("test-attestor-note")
///             .build_struct(),
///     );
///     let policy = policy::create(
///         "policy",
///         PolicyArgs::builder()
///             .admission_whitelist_patterns(
///                 vec![
///                     PolicyAdmissionWhitelistPattern::builder()
///                     .namePattern("gcr.io/google_containers/*").build_struct(),
///                 ],
///             )
///             .cluster_admission_rules(
///                 vec![
///                     PolicyClusterAdmissionRule::builder()
///                     .cluster("us-central1-a.prod-cluster")
///                     .enforcementMode("ENFORCED_BLOCK_AND_AUDIT_LOG")
///                     .evaluationMode("REQUIRE_ATTESTATION")
///                     .requireAttestationsBies(vec!["${attestor.name}",]).build_struct(),
///                 ],
///             )
///             .default_admission_rule(
///                 PolicyDefaultAdmissionRule::builder()
///                     .enforcementMode("ENFORCED_BLOCK_AND_AUDIT_LOG")
///                     .evaluationMode("ALWAYS_ALLOW")
///                     .build_struct(),
///             )
///             .build_struct(),
///     );
/// }
/// ```
/// ### Binary Authorization Policy Global Evaluation
///
///
/// ```ignore
/// use pulumi_wasm_rust::Output;
/// use pulumi_wasm_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let attestor = attestor::create(
///         "attestor",
///         AttestorArgs::builder()
///             .attestation_authority_note(
///                 AttestorAttestationAuthorityNote::builder()
///                     .noteReference("${note.name}")
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
///                             .humanReadableName("My attestor")
///                             .build_struct(),
///                     )
///                     .build_struct(),
///             )
///             .name("test-attestor-note")
///             .build_struct(),
///     );
///     let policy = policy::create(
///         "policy",
///         PolicyArgs::builder()
///             .default_admission_rule(
///                 PolicyDefaultAdmissionRule::builder()
///                     .enforcementMode("ENFORCED_BLOCK_AND_AUDIT_LOG")
///                     .evaluationMode("REQUIRE_ATTESTATION")
///                     .requireAttestationsBies(vec!["${attestor.name}",])
///                     .build_struct(),
///             )
///             .global_policy_evaluation_mode("ENABLE")
///             .build_struct(),
///     );
/// }
/// ```
///
/// ## Import
///
/// Policy can be imported using any of these accepted formats:
///
/// * `projects/{{project}}`
///
/// * `{{project}}`
///
/// When using the `pulumi import` command, Policy can be imported using one of the formats above. For example:
///
/// ```sh
/// $ pulumi import gcp:binaryauthorization/policy:Policy default projects/{{project}}
/// ```
///
/// ```sh
/// $ pulumi import gcp:binaryauthorization/policy:Policy default {{project}}
/// ```
///
pub mod policy {
    #[derive(pulumi_wasm_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct PolicyArgs {
        /// A whitelist of image patterns to exclude from admission rules. If an image's name matches a whitelist pattern, the
        /// image's admission requests will always be permitted regardless of your admission rules.
        #[builder(into, default)]
        pub admission_whitelist_patterns: pulumi_wasm_rust::InputOrOutput<
            Option<
                Vec<
                    super::super::types::binaryauthorization::PolicyAdmissionWhitelistPattern,
                >,
            >,
        >,
        /// Per-cluster admission rules. An admission rule specifies either that all container images used in a pod creation request
        /// must be attested to by one or more attestors, that all pod creations will be allowed, or that all pod creations will be
        /// denied. There can be at most one admission rule per cluster spec. Identifier format: '{{location}}.{{clusterId}}'. A
        /// location is either a compute zone (e.g. 'us-central1-a') or a region (e.g. 'us-central1').
        #[builder(into, default)]
        pub cluster_admission_rules: pulumi_wasm_rust::InputOrOutput<
            Option<
                Vec<super::super::types::binaryauthorization::PolicyClusterAdmissionRule>,
            >,
        >,
        /// Default admission rule for a cluster without a per-cluster admission
        /// rule.
        /// Structure is documented below.
        #[builder(into)]
        pub default_admission_rule: pulumi_wasm_rust::InputOrOutput<
            super::super::types::binaryauthorization::PolicyDefaultAdmissionRule,
        >,
        /// A descriptive comment.
        #[builder(into, default)]
        pub description: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        /// Controls the evaluation of a Google-maintained global admission policy for common system-level images. Images not
        /// covered by the global policy will be subject to the project admission policy. Possible values: ["ENABLE", "DISABLE"]
        #[builder(into, default)]
        pub global_policy_evaluation_mode: pulumi_wasm_rust::InputOrOutput<
            Option<String>,
        >,
        #[builder(into, default)]
        pub project: pulumi_wasm_rust::InputOrOutput<Option<String>>,
    }
    #[allow(dead_code)]
    pub struct PolicyResult {
        /// A whitelist of image patterns to exclude from admission rules. If an image's name matches a whitelist pattern, the
        /// image's admission requests will always be permitted regardless of your admission rules.
        pub admission_whitelist_patterns: pulumi_wasm_rust::Output<
            Option<
                Vec<
                    super::super::types::binaryauthorization::PolicyAdmissionWhitelistPattern,
                >,
            >,
        >,
        /// Per-cluster admission rules. An admission rule specifies either that all container images used in a pod creation request
        /// must be attested to by one or more attestors, that all pod creations will be allowed, or that all pod creations will be
        /// denied. There can be at most one admission rule per cluster spec. Identifier format: '{{location}}.{{clusterId}}'. A
        /// location is either a compute zone (e.g. 'us-central1-a') or a region (e.g. 'us-central1').
        pub cluster_admission_rules: pulumi_wasm_rust::Output<
            Option<
                Vec<super::super::types::binaryauthorization::PolicyClusterAdmissionRule>,
            >,
        >,
        /// Default admission rule for a cluster without a per-cluster admission
        /// rule.
        /// Structure is documented below.
        pub default_admission_rule: pulumi_wasm_rust::Output<
            super::super::types::binaryauthorization::PolicyDefaultAdmissionRule,
        >,
        /// A descriptive comment.
        pub description: pulumi_wasm_rust::Output<Option<String>>,
        /// Controls the evaluation of a Google-maintained global admission policy for common system-level images. Images not
        /// covered by the global policy will be subject to the project admission policy. Possible values: ["ENABLE", "DISABLE"]
        pub global_policy_evaluation_mode: pulumi_wasm_rust::Output<String>,
        pub project: pulumi_wasm_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_wasm_rust::PulumiContext,
        name: &str,
        args: PolicyArgs,
    ) -> PolicyResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let admission_whitelist_patterns_binding = args
            .admission_whitelist_patterns
            .get_output(context)
            .get_inner();
        let cluster_admission_rules_binding = args
            .cluster_admission_rules
            .get_output(context)
            .get_inner();
        let default_admission_rule_binding = args
            .default_admission_rule
            .get_output(context)
            .get_inner();
        let description_binding = args.description.get_output(context).get_inner();
        let global_policy_evaluation_mode_binding = args
            .global_policy_evaluation_mode
            .get_output(context)
            .get_inner();
        let project_binding = args.project.get_output(context).get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "gcp:binaryauthorization/policy:Policy".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "admissionWhitelistPatterns".into(),
                    value: &admission_whitelist_patterns_binding,
                },
                register_interface::ObjectField {
                    name: "clusterAdmissionRules".into(),
                    value: &cluster_admission_rules_binding,
                },
                register_interface::ObjectField {
                    name: "defaultAdmissionRule".into(),
                    value: &default_admission_rule_binding,
                },
                register_interface::ObjectField {
                    name: "description".into(),
                    value: &description_binding,
                },
                register_interface::ObjectField {
                    name: "globalPolicyEvaluationMode".into(),
                    value: &global_policy_evaluation_mode_binding,
                },
                register_interface::ObjectField {
                    name: "project".into(),
                    value: &project_binding,
                },
            ]),
        };
        let o = register_interface::register(context.get_inner(), &request);
        PolicyResult {
            admission_whitelist_patterns: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("admissionWhitelistPatterns"),
            ),
            cluster_admission_rules: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("clusterAdmissionRules"),
            ),
            default_admission_rule: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("defaultAdmissionRule"),
            ),
            description: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("description"),
            ),
            global_policy_evaluation_mode: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("globalPolicyEvaluationMode"),
            ),
            project: pulumi_wasm_rust::__private::into_domain(o.extract_field("project")),
        }
    }
}
