/// A configuration for an external identity provider.
///
///
/// To get more information about WorkloadIdentityPoolProvider, see:
///
/// * [API documentation](https://cloud.google.com/iam/docs/reference/rest/v1/projects.locations.workloadIdentityPools.providers)
/// * How-to Guides
///     * [Managing workload identity providers](https://cloud.google.com/iam/docs/manage-workload-identity-pools-providers#managing_workload_identity_providers)
///
/// ## Example Usage
///
/// ### Iam Workload Identity Pool Provider Aws Basic
///
///
/// ```ignore
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let example = workload_identity_pool_provider::create(
///         "example",
///         WorkloadIdentityPoolProviderArgs::builder()
///             .aws(
///                 WorkloadIdentityPoolProviderAws::builder()
///                     .accountId("999999999999")
///                     .build_struct(),
///             )
///             .workload_identity_pool_id("${pool.workloadIdentityPoolId}")
///             .workload_identity_pool_provider_id("example-prvdr")
///             .build_struct(),
///     );
///     let pool = workload_identity_pool::create(
///         "pool",
///         WorkloadIdentityPoolArgs::builder()
///             .workload_identity_pool_id("example-pool")
///             .build_struct(),
///     );
/// }
/// ```
/// ### Iam Workload Identity Pool Provider Aws Full
///
///
/// ```yaml
/// resources:
///   pool:
///     type: gcp:iam:WorkloadIdentityPool
///     properties:
///       workloadIdentityPoolId: example-pool
///   example:
///     type: gcp:iam:WorkloadIdentityPoolProvider
///     properties:
///       workloadIdentityPoolId: ${pool.workloadIdentityPoolId}
///       workloadIdentityPoolProviderId: example-prvdr
///       displayName: Name of provider
///       description: AWS identity pool provider for automated test
///       disabled: true
///       attributeCondition: attribute.aws_role=="arn:aws:sts::999999999999:assumed-role/stack-eu-central-1-lambdaRole"
///       attributeMapping:
///         google.subject: assertion.arn
///         attribute.aws_account: assertion.account
///         attribute.environment: 'assertion.arn.contains(":instance-profile/Production") ? "prod" : "test"'
///       aws:
///         accountId: '999999999999'
/// ```
/// ### Iam Workload Identity Pool Provider Github Actions
///
///
/// ```yaml
/// resources:
///   pool:
///     type: gcp:iam:WorkloadIdentityPool
///     properties:
///       workloadIdentityPoolId: example-pool
///   example:
///     type: gcp:iam:WorkloadIdentityPoolProvider
///     properties:
///       workloadIdentityPoolId: ${pool.workloadIdentityPoolId}
///       workloadIdentityPoolProviderId: example-prvdr
///       displayName: Name of provider
///       description: GitHub Actions identity pool provider for automated test
///       disabled: true
///       attributeCondition: |2
///             assertion.repository_owner_id == "123456789" &&
///             attribute.repository == "gh-org/gh-repo" &&
///             assertion.ref == "refs/heads/main" &&
///             assertion.ref_type == "branch"
///       attributeMapping:
///         google.subject: assertion.sub
///         attribute.actor: assertion.actor
///         attribute.aud: assertion.aud
///         attribute.repository: assertion.repository
///       oidc:
///         issuerUri: https://token.actions.githubusercontent.com
/// ```
/// ### Iam Workload Identity Pool Provider Oidc Basic
///
///
/// ```yaml
/// resources:
///   pool:
///     type: gcp:iam:WorkloadIdentityPool
///     properties:
///       workloadIdentityPoolId: example-pool
///   example:
///     type: gcp:iam:WorkloadIdentityPoolProvider
///     properties:
///       workloadIdentityPoolId: ${pool.workloadIdentityPoolId}
///       workloadIdentityPoolProviderId: example-prvdr
///       attributeMapping:
///         google.subject: assertion.sub
///       oidc:
///         issuerUri: https://sts.windows.net/azure-tenant-id
/// ```
/// ### Iam Workload Identity Pool Provider Oidc Full
///
///
/// ```yaml
/// resources:
///   pool:
///     type: gcp:iam:WorkloadIdentityPool
///     properties:
///       workloadIdentityPoolId: example-pool
///   example:
///     type: gcp:iam:WorkloadIdentityPoolProvider
///     properties:
///       workloadIdentityPoolId: ${pool.workloadIdentityPoolId}
///       workloadIdentityPoolProviderId: example-prvdr
///       displayName: Name of provider
///       description: OIDC identity pool provider for automated test
///       disabled: true
///       attributeCondition: '"e968c2ef-047c-498d-8d79-16ca1b61e77e" in assertion.groups'
///       attributeMapping:
///         google.subject: '"azure::" + assertion.tid + "::" + assertion.sub'
///         attribute.tid: assertion.tid
///         attribute.managed_identity_name: |2
///                 {
///                   "8bb39bdb-1cc5-4447-b7db-a19e920eb111":"workload1",
///                   "55d36609-9bcf-48e0-a366-a3cf19027d2a":"workload2"
///                 }[assertion.oid]
///       oidc:
///         allowedAudiences:
///           - https://example.com/gcp-oidc-federation
///           - example.com/gcp-oidc-federation
///         issuerUri: https://sts.windows.net/azure-tenant-id
/// ```
/// ### Iam Workload Identity Pool Provider Saml Basic
///
///
/// ```yaml
/// resources:
///   pool:
///     type: gcp:iam:WorkloadIdentityPool
///     properties:
///       workloadIdentityPoolId: example-pool
///   example:
///     type: gcp:iam:WorkloadIdentityPoolProvider
///     properties:
///       workloadIdentityPoolId: ${pool.workloadIdentityPoolId}
///       workloadIdentityPoolProviderId: example-prvdr
///       attributeMapping:
///         google.subject: assertion.arn
///         attribute.aws_account: assertion.account
///         attribute.environment: 'assertion.arn.contains(":instance-profile/Production") ? "prod" : "test"'
///       saml:
///         idpMetadataXml:
///           fn::invoke:
///             function: std:file
///             arguments:
///               input: test-fixtures/metadata.xml
///             return: result
/// ```
/// ### Iam Workload Identity Pool Provider Saml Full
///
///
/// ```yaml
/// resources:
///   pool:
///     type: gcp:iam:WorkloadIdentityPool
///     properties:
///       workloadIdentityPoolId: example-pool
///   example:
///     type: gcp:iam:WorkloadIdentityPoolProvider
///     properties:
///       workloadIdentityPoolId: ${pool.workloadIdentityPoolId}
///       workloadIdentityPoolProviderId: example-prvdr
///       displayName: Name of provider
///       description: SAML 2.0 identity pool provider for automated test
///       disabled: true
///       attributeMapping:
///         google.subject: assertion.arn
///         attribute.aws_account: assertion.account
///         attribute.environment: 'assertion.arn.contains(":instance-profile/Production") ? "prod" : "test"'
///       saml:
///         idpMetadataXml:
///           fn::invoke:
///             function: std:file
///             arguments:
///               input: test-fixtures/metadata.xml
///             return: result
/// ```
/// ### Iam Workload Identity Pool Provider Oidc Upload Key
///
///
/// ```yaml
/// resources:
///   pool:
///     type: gcp:iam:WorkloadIdentityPool
///     properties:
///       workloadIdentityPoolId: example-pool
///   example:
///     type: gcp:iam:WorkloadIdentityPoolProvider
///     properties:
///       workloadIdentityPoolId: ${pool.workloadIdentityPoolId}
///       workloadIdentityPoolProviderId: example-prvdr
///       displayName: Name of provider
///       description: OIDC identity pool provider for automated test
///       disabled: true
///       attributeCondition: '"e968c2ef-047c-498d-8d79-16ca1b61e77e" in assertion.groups'
///       attributeMapping:
///         google.subject: '"azure::" + assertion.tid + "::" + assertion.sub'
///         attribute.tid: assertion.tid
///         attribute.managed_identity_name: |2
///                 {
///                   "8bb39bdb-1cc5-4447-b7db-a19e920eb111":"workload1",
///                   "55d36609-9bcf-48e0-a366-a3cf19027d2a":"workload2"
///                 }[assertion.oid]
///       oidc:
///         allowedAudiences:
///           - https://example.com/gcp-oidc-federation
///           - example.com/gcp-oidc-federation
///         issuerUri: https://sts.windows.net/azure-tenant-id
///         jwksJson: '{"keys":[{"kty":"RSA","alg":"RS256","kid":"sif0AR-F6MuvksAyAOv-Pds08Bcf2eUMlxE30NofddA","use":"sig","e":"AQAB","n":"ylH1Chl1tpfti3lh51E1g5dPogzXDaQseqjsefGLknaNl5W6Wd4frBhHyE2t41Q5zgz_Ll0-NvWm0FlaG6brhrN9QZu6sJP1bM8WPfJVPgXOanxi7d7TXCkeNubGeiLTf5R3UXtS9Lm_guemU7MxDjDTelxnlgGCihOVTcL526suNJUdfXtpwUsvdU6_ZnAp9IpsuYjCtwPm9hPumlcZGMbxstdh07O4y4O90cVQClJOKSGQjAUCKJWXIQ0cqffGS_HuS_725CPzQ85SzYZzaNpgfhAER7kx_9P16ARM3BJz0PI5fe2hECE61J4GYU_BY43sxDfs7HyJpEXKLU9eWw"}]}'
/// ```
/// ### Iam Workload Identity Pool Provider X509 Basic
///
///
/// ```yaml
/// resources:
///   pool:
///     type: gcp:iam:WorkloadIdentityPool
///     properties:
///       workloadIdentityPoolId: example-pool
///   example:
///     type: gcp:iam:WorkloadIdentityPoolProvider
///     properties:
///       workloadIdentityPoolId: ${pool.workloadIdentityPoolId}
///       workloadIdentityPoolProviderId: example-prvdr
///       attributeMapping:
///         google.subject: assertion.subject.dn.cn
///       x509:
///         trustStore:
///           trustAnchors:
///             - pemCertificate:
///                 fn::invoke:
///                   function: std:file
///                   arguments:
///                     input: test-fixtures/trust_anchor.pem
///                   return: result
/// ```
/// ### Iam Workload Identity Pool Provider X509 Full
///
///
/// ```yaml
/// resources:
///   pool:
///     type: gcp:iam:WorkloadIdentityPool
///     properties:
///       workloadIdentityPoolId: example-pool
///   example:
///     type: gcp:iam:WorkloadIdentityPoolProvider
///     properties:
///       workloadIdentityPoolId: ${pool.workloadIdentityPoolId}
///       workloadIdentityPoolProviderId: example-prvdr
///       displayName: Name of provider
///       description: X.509 identity pool provider for automated test
///       disabled: true
///       attributeMapping:
///         google.subject: assertion.subject.dn.cn
///       x509:
///         trustStore:
///           trustAnchors:
///             - pemCertificate:
///                 fn::invoke:
///                   function: std:file
///                   arguments:
///                     input: test-fixtures/trust_anchor.pem
///                   return: result
///           intermediateCas:
///             - pemCertificate:
///                 fn::invoke:
///                   function: std:file
///                   arguments:
///                     input: test-fixtures/intermediate_ca.pem
///                   return: result
/// ```
///
/// ## Import
///
/// WorkloadIdentityPoolProvider can be imported using any of these accepted formats:
///
/// * `projects/{{project}}/locations/global/workloadIdentityPools/{{workload_identity_pool_id}}/providers/{{workload_identity_pool_provider_id}}`
///
/// * `{{project}}/{{workload_identity_pool_id}}/{{workload_identity_pool_provider_id}}`
///
/// * `{{workload_identity_pool_id}}/{{workload_identity_pool_provider_id}}`
///
/// When using the `pulumi import` command, WorkloadIdentityPoolProvider can be imported using one of the formats above. For example:
///
/// ```sh
/// $ pulumi import gcp:iam/workloadIdentityPoolProvider:WorkloadIdentityPoolProvider default projects/{{project}}/locations/global/workloadIdentityPools/{{workload_identity_pool_id}}/providers/{{workload_identity_pool_provider_id}}
/// ```
///
/// ```sh
/// $ pulumi import gcp:iam/workloadIdentityPoolProvider:WorkloadIdentityPoolProvider default {{project}}/{{workload_identity_pool_id}}/{{workload_identity_pool_provider_id}}
/// ```
///
/// ```sh
/// $ pulumi import gcp:iam/workloadIdentityPoolProvider:WorkloadIdentityPoolProvider default {{workload_identity_pool_id}}/{{workload_identity_pool_provider_id}}
/// ```
///
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod workload_identity_pool_provider {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct WorkloadIdentityPoolProviderArgs {
        /// [A Common Expression Language](https://opensource.google/projects/cel) expression, in
        /// plain text, to restrict what otherwise valid authentication credentials issued by the
        /// provider should not be accepted.
        /// The expression must output a boolean representing whether to allow the federation.
        /// The following keywords may be referenced in the expressions:
        #[builder(into, default)]
        pub attribute_condition: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Maps attributes from authentication credentials issued by an external identity provider
        /// to Google Cloud attributes, such as `subject` and `segment`.
        /// Each key must be a string specifying the Google Cloud IAM attribute to map to.
        /// The following keys are supported:
        /// * `google.subject`: The principal IAM is authenticating. You can reference this value
        /// in IAM bindings. This is also the subject that appears in Cloud Logging logs.
        /// Cannot exceed 127 characters.
        /// * `google.groups`: Groups the external identity belongs to. You can grant groups
        /// access to resources using an IAM `principalSet` binding; access applies to all
        /// members of the group.
        /// You can also provide custom attributes by specifying `attribute.{custom_attribute}`,
        /// where `{custom_attribute}` is the name of the custom attribute to be mapped. You can
        /// define a maximum of 50 custom attributes. The maximum length of a mapped attribute key
        /// is 100 characters, and the key may only contain the characters [a-z0-9_].
        /// You can reference these attributes in IAM policies to define fine-grained access for a
        /// workload to Google Cloud resources. For example:
        /// * `google.subject`:
        /// `principal://iam.googleapis.com/projects/{project}/locations/{location}/workloadIdentityPools/{pool}/subject/{value}`
        /// * `google.groups`:
        /// `principalSet://iam.googleapis.com/projects/{project}/locations/{location}/workloadIdentityPools/{pool}/group/{value}`
        /// * `attribute.{custom_attribute}`:
        /// `principalSet://iam.googleapis.com/projects/{project}/locations/{location}/workloadIdentityPools/{pool}/attribute.{custom_attribute}/{value}`
        /// Each value must be a [Common Expression Language](https://opensource.google/projects/cel)
        /// function that maps an identity provider credential to the normalized attribute specified
        /// by the corresponding map key.
        /// You can use the `assertion` keyword in the expression to access a JSON representation of
        /// the authentication credential issued by the provider.
        /// The maximum length of an attribute mapping expression is 2048 characters. When evaluated,
        /// the total size of all mapped attributes must not exceed 8KB.
        /// For AWS providers, the following rules apply:
        /// - If no attribute mapping is defined, the following default mapping applies:
        /// ```sh
        /// {
        /// "google.subject":"assertion.arn",
        /// "attribute.aws_role":
        /// "assertion.arn.contains('assumed-role')"
        /// " ? assertion.arn.extract('{account_arn}assumed-role/')"
        /// "   + 'assumed-role/'"
        /// "   + assertion.arn.extract('assumed-role/{role_name}/')"
        /// " : assertion.arn",
        /// }
        /// ```
        /// - If any custom attribute mappings are defined, they must include a mapping to the
        /// `google.subject` attribute.
        /// For OIDC providers, the following rules apply:
        /// - Custom attribute mappings must be defined, and must include a mapping to the
        /// `google.subject` attribute. For example, the following maps the `sub` claim of the
        /// incoming credential to the `subject` attribute on a Google token.
        /// ```sh
        /// {"google.subject": "assertion.sub"}
        /// ```
        #[builder(into, default)]
        pub attribute_mapping: pulumi_gestalt_rust::InputOrOutput<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// An Amazon Web Services identity provider. Not compatible with the property oidc or saml.
        /// Structure is documented below.
        #[builder(into, default)]
        pub aws: pulumi_gestalt_rust::InputOrOutput<
            Option<super::super::types::iam::WorkloadIdentityPoolProviderAws>,
        >,
        /// A description for the provider. Cannot exceed 256 characters.
        #[builder(into, default)]
        pub description: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Whether the provider is disabled. You cannot use a disabled provider to exchange tokens.
        /// However, existing tokens still grant access.
        #[builder(into, default)]
        pub disabled: pulumi_gestalt_rust::InputOrOutput<Option<bool>>,
        /// A display name for the provider. Cannot exceed 32 characters.
        #[builder(into, default)]
        pub display_name: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// An OpenId Connect 1.0 identity provider. Not compatible with the property aws or saml.
        /// Structure is documented below.
        #[builder(into, default)]
        pub oidc: pulumi_gestalt_rust::InputOrOutput<
            Option<super::super::types::iam::WorkloadIdentityPoolProviderOidc>,
        >,
        /// The ID of the project in which the resource belongs.
        /// If it is not provided, the provider project is used.
        #[builder(into, default)]
        pub project: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// An SAML 2.0 identity provider. Not compatible with the property oidc or aws.
        /// Structure is documented below.
        #[builder(into, default)]
        pub saml: pulumi_gestalt_rust::InputOrOutput<
            Option<super::super::types::iam::WorkloadIdentityPoolProviderSaml>,
        >,
        /// The ID used for the pool, which is the final component of the pool resource name. This
        /// value should be 4-32 characters, and may contain the characters [a-z0-9-]. The prefix
        /// `gcp-` is reserved for use by Google, and may not be specified.
        #[builder(into)]
        pub workload_identity_pool_id: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The ID for the provider, which becomes the final component of the resource name. This
        /// value must be 4-32 characters, and may contain the characters [a-z0-9-]. The prefix
        /// `gcp-` is reserved for use by Google, and may not be specified.
        ///
        ///
        /// - - -
        #[builder(into)]
        pub workload_identity_pool_provider_id: pulumi_gestalt_rust::InputOrOutput<
            String,
        >,
        /// An X.509-type identity provider represents a CA. It is trusted to assert a
        /// client identity if the client has a certificate that chains up to this CA.
        /// Structure is documented below.
        #[builder(into, default)]
        pub x509: pulumi_gestalt_rust::InputOrOutput<
            Option<super::super::types::iam::WorkloadIdentityPoolProviderX509>,
        >,
    }
    #[allow(dead_code)]
    pub struct WorkloadIdentityPoolProviderResult {
        /// [A Common Expression Language](https://opensource.google/projects/cel) expression, in
        /// plain text, to restrict what otherwise valid authentication credentials issued by the
        /// provider should not be accepted.
        /// The expression must output a boolean representing whether to allow the federation.
        /// The following keywords may be referenced in the expressions:
        pub attribute_condition: pulumi_gestalt_rust::Output<Option<String>>,
        /// Maps attributes from authentication credentials issued by an external identity provider
        /// to Google Cloud attributes, such as `subject` and `segment`.
        /// Each key must be a string specifying the Google Cloud IAM attribute to map to.
        /// The following keys are supported:
        /// * `google.subject`: The principal IAM is authenticating. You can reference this value
        /// in IAM bindings. This is also the subject that appears in Cloud Logging logs.
        /// Cannot exceed 127 characters.
        /// * `google.groups`: Groups the external identity belongs to. You can grant groups
        /// access to resources using an IAM `principalSet` binding; access applies to all
        /// members of the group.
        /// You can also provide custom attributes by specifying `attribute.{custom_attribute}`,
        /// where `{custom_attribute}` is the name of the custom attribute to be mapped. You can
        /// define a maximum of 50 custom attributes. The maximum length of a mapped attribute key
        /// is 100 characters, and the key may only contain the characters [a-z0-9_].
        /// You can reference these attributes in IAM policies to define fine-grained access for a
        /// workload to Google Cloud resources. For example:
        /// * `google.subject`:
        /// `principal://iam.googleapis.com/projects/{project}/locations/{location}/workloadIdentityPools/{pool}/subject/{value}`
        /// * `google.groups`:
        /// `principalSet://iam.googleapis.com/projects/{project}/locations/{location}/workloadIdentityPools/{pool}/group/{value}`
        /// * `attribute.{custom_attribute}`:
        /// `principalSet://iam.googleapis.com/projects/{project}/locations/{location}/workloadIdentityPools/{pool}/attribute.{custom_attribute}/{value}`
        /// Each value must be a [Common Expression Language](https://opensource.google/projects/cel)
        /// function that maps an identity provider credential to the normalized attribute specified
        /// by the corresponding map key.
        /// You can use the `assertion` keyword in the expression to access a JSON representation of
        /// the authentication credential issued by the provider.
        /// The maximum length of an attribute mapping expression is 2048 characters. When evaluated,
        /// the total size of all mapped attributes must not exceed 8KB.
        /// For AWS providers, the following rules apply:
        /// - If no attribute mapping is defined, the following default mapping applies:
        /// ```sh
        /// {
        /// "google.subject":"assertion.arn",
        /// "attribute.aws_role":
        /// "assertion.arn.contains('assumed-role')"
        /// " ? assertion.arn.extract('{account_arn}assumed-role/')"
        /// "   + 'assumed-role/'"
        /// "   + assertion.arn.extract('assumed-role/{role_name}/')"
        /// " : assertion.arn",
        /// }
        /// ```
        /// - If any custom attribute mappings are defined, they must include a mapping to the
        /// `google.subject` attribute.
        /// For OIDC providers, the following rules apply:
        /// - Custom attribute mappings must be defined, and must include a mapping to the
        /// `google.subject` attribute. For example, the following maps the `sub` claim of the
        /// incoming credential to the `subject` attribute on a Google token.
        /// ```sh
        /// {"google.subject": "assertion.sub"}
        /// ```
        pub attribute_mapping: pulumi_gestalt_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// An Amazon Web Services identity provider. Not compatible with the property oidc or saml.
        /// Structure is documented below.
        pub aws: pulumi_gestalt_rust::Output<
            Option<super::super::types::iam::WorkloadIdentityPoolProviderAws>,
        >,
        /// A description for the provider. Cannot exceed 256 characters.
        pub description: pulumi_gestalt_rust::Output<Option<String>>,
        /// Whether the provider is disabled. You cannot use a disabled provider to exchange tokens.
        /// However, existing tokens still grant access.
        pub disabled: pulumi_gestalt_rust::Output<Option<bool>>,
        /// A display name for the provider. Cannot exceed 32 characters.
        pub display_name: pulumi_gestalt_rust::Output<Option<String>>,
        /// The resource name of the provider as
        /// `projects/{project_number}/locations/global/workloadIdentityPools/{workload_identity_pool_id}/providers/{workload_identity_pool_provider_id}`.
        pub name: pulumi_gestalt_rust::Output<String>,
        /// An OpenId Connect 1.0 identity provider. Not compatible with the property aws or saml.
        /// Structure is documented below.
        pub oidc: pulumi_gestalt_rust::Output<
            Option<super::super::types::iam::WorkloadIdentityPoolProviderOidc>,
        >,
        /// The ID of the project in which the resource belongs.
        /// If it is not provided, the provider project is used.
        pub project: pulumi_gestalt_rust::Output<String>,
        /// An SAML 2.0 identity provider. Not compatible with the property oidc or aws.
        /// Structure is documented below.
        pub saml: pulumi_gestalt_rust::Output<
            Option<super::super::types::iam::WorkloadIdentityPoolProviderSaml>,
        >,
        /// The state of the provider.
        /// * STATE_UNSPECIFIED: State unspecified.
        /// * ACTIVE: The provider is active, and may be used to validate authentication credentials.
        /// * DELETED: The provider is soft-deleted. Soft-deleted providers are permanently deleted
        /// after approximately 30 days. You can restore a soft-deleted provider using
        /// UndeleteWorkloadIdentityPoolProvider. You cannot reuse the ID of a soft-deleted provider
        /// until it is permanently deleted.
        pub state: pulumi_gestalt_rust::Output<String>,
        /// The ID used for the pool, which is the final component of the pool resource name. This
        /// value should be 4-32 characters, and may contain the characters [a-z0-9-]. The prefix
        /// `gcp-` is reserved for use by Google, and may not be specified.
        pub workload_identity_pool_id: pulumi_gestalt_rust::Output<String>,
        /// The ID for the provider, which becomes the final component of the resource name. This
        /// value must be 4-32 characters, and may contain the characters [a-z0-9-]. The prefix
        /// `gcp-` is reserved for use by Google, and may not be specified.
        ///
        ///
        /// - - -
        pub workload_identity_pool_provider_id: pulumi_gestalt_rust::Output<String>,
        /// An X.509-type identity provider represents a CA. It is trusted to assert a
        /// client identity if the client has a certificate that chains up to this CA.
        /// Structure is documented below.
        pub x509: pulumi_gestalt_rust::Output<
            Option<super::super::types::iam::WorkloadIdentityPoolProviderX509>,
        >,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: WorkloadIdentityPoolProviderArgs,
    ) -> WorkloadIdentityPoolProviderResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let attribute_condition_binding = args.attribute_condition.get_output(context);
        let attribute_mapping_binding = args.attribute_mapping.get_output(context);
        let aws_binding = args.aws.get_output(context);
        let description_binding = args.description.get_output(context);
        let disabled_binding = args.disabled.get_output(context);
        let display_name_binding = args.display_name.get_output(context);
        let oidc_binding = args.oidc.get_output(context);
        let project_binding = args.project.get_output(context);
        let saml_binding = args.saml.get_output(context);
        let workload_identity_pool_id_binding = args
            .workload_identity_pool_id
            .get_output(context);
        let workload_identity_pool_provider_id_binding = args
            .workload_identity_pool_provider_id
            .get_output(context);
        let x509_binding = args.x509.get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "gcp:iam/workloadIdentityPoolProvider:WorkloadIdentityPoolProvider"
                .into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "attributeCondition".into(),
                    value: &attribute_condition_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "attributeMapping".into(),
                    value: &attribute_mapping_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "aws".into(),
                    value: &aws_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "description".into(),
                    value: &description_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "disabled".into(),
                    value: &disabled_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "displayName".into(),
                    value: &display_name_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "oidc".into(),
                    value: &oidc_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "project".into(),
                    value: &project_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "saml".into(),
                    value: &saml_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "workloadIdentityPoolId".into(),
                    value: &workload_identity_pool_id_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "workloadIdentityPoolProviderId".into(),
                    value: &workload_identity_pool_provider_id_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "x509".into(),
                    value: &x509_binding.drop_type(),
                },
            ],
        };
        let o = context.register_resource(request);
        WorkloadIdentityPoolProviderResult {
            attribute_condition: o.get_field("attributeCondition"),
            attribute_mapping: o.get_field("attributeMapping"),
            aws: o.get_field("aws"),
            description: o.get_field("description"),
            disabled: o.get_field("disabled"),
            display_name: o.get_field("displayName"),
            name: o.get_field("name"),
            oidc: o.get_field("oidc"),
            project: o.get_field("project"),
            saml: o.get_field("saml"),
            state: o.get_field("state"),
            workload_identity_pool_id: o.get_field("workloadIdentityPoolId"),
            workload_identity_pool_provider_id: o
                .get_field("workloadIdentityPoolProviderId"),
            x509: o.get_field("x509"),
        }
    }
}
