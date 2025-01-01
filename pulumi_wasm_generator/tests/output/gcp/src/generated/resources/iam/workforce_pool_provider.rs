/// A configuration for an external identity provider.
///
///
/// To get more information about WorkforcePoolProvider, see:
///
/// * [API documentation](https://cloud.google.com/iam/docs/reference/rest/v1/locations.workforcePools.providers)
/// * How-to Guides
///     * [Configure a provider within the workforce pool](https://cloud.google.com/iam/docs/manage-workforce-identity-pools-providers#configure_a_provider_within_the_workforce_pool)
///
/// > **Note:** Ask your Google Cloud account team to request access to workforce identity federation for your
/// billing/quota project. The account team notifies you when the project is granted access.
///
///
///
/// ## Example Usage
///
/// ### Iam Workforce Pool Provider Saml Basic
///
///
/// ```yaml
/// resources:
///   pool:
///     type: gcp:iam:WorkforcePool
///     properties:
///       workforcePoolId: example-pool
///       parent: organizations/123456789
///       location: global
///   example:
///     type: gcp:iam:WorkforcePoolProvider
///     properties:
///       workforcePoolId: ${pool.workforcePoolId}
///       location: ${pool.location}
///       providerId: example-prvdr
///       attributeMapping:
///         google.subject: assertion.sub
///       saml:
///         idpMetadataXml: <?xml version="1.0"?><md:EntityDescriptor xmlns:md="urn:oasis:names:tc:SAML:2.0:metadata" entityID="https://test.com"><md:IDPSSODescriptor protocolSupportEnumeration="urn:oasis:names:tc:SAML:2.0:protocol"> <md:KeyDescriptor use="signing"><ds:KeyInfo xmlns:ds="http://www.w3.org/2000/09/xmldsig#"><ds:X509Data><ds:X509Certificate>MIIDpDCCAoygAwIBAgIGAX7/5qPhMA0GCSqGSIb3DQEBCwUAMIGSMQswCQYDVQQGEwJVUzETMBEGA1UECAwKQ2FsaWZvcm5pYTEWMBQGA1UEBwwNU2FuIEZyYW5jaXNjbzENMAsGA1UECgwET2t0YTEUMBIGA1UECwwLU1NPUHJvdmlkZXIxEzARBgNVBAMMCmRldi00NTg0MjExHDAaBgkqhkiG9w0BCQEWDWluZm9Ab2t0YS5jb20wHhcNMjIwMjE2MDAxOTEyWhcNMzIwMjE2MDAyMDEyWjCBkjELMAkGA1UEBhMCVVMxEzARBgNVBAgMCkNhbGlmb3JuaWExFjAUBgNVBAcMDVNhbiBGcmFuY2lzY28xDTALBgNVBAoMBE9rdGExFDASBgNVBAsMC1NTT1Byb3ZpZGVyMRMwEQYDVQQDDApkZXYtNDU4NDIxMRwwGgYJKoZIhvcNAQkBFg1pbmZvQG9rdGEuY29tMIIBIjANBgkqhkiG9w0BAQEFAAOCAQ8AMIIBCgKCAQEAxrBl7GKz52cRpxF9xCsirnRuMxnhFBaUrsHqAQrLqWmdlpNYZTVg+T9iQ+aq/iE68L+BRZcZniKIvW58wqqS0ltXVvIkXuDSvnvnkkI5yMIVErR20K8jSOKQm1FmK+fgAJ4koshFiu9oLiqu0Ejc0DuL3/XRsb4RuxjktKTb1khgBBtb+7idEk0sFR0RPefAweXImJkDHDm7SxjDwGJUubbqpdTxasPr0W+AHI1VUzsUsTiHAoyb0XDkYqHfDzhj/ZdIEl4zHQ3bEZvlD984ztAnmX2SuFLLKfXeAAGHei8MMixJvwxYkkPeYZ/5h8WgBZPP4heS2CPjwYExt29L8QIDAQABMA0GCSqGSIb3DQEBCwUAA4IBAQARjJFz++a9Z5IQGFzsZMrX2EDR5ML4xxUiQkbhld1S1PljOLcYFARDmUC2YYHOueU4ee8Jid9nPGEUebV/4Jok+b+oQh+dWMgiWjSLI7h5q4OYZ3VJtdlVwgMFt2iz+/4yBKMUZ50g3Qgg36vE34us+eKitg759JgCNsibxn0qtJgSPm0sgP2L6yTaLnoEUbXBRxCwynTSkp9ZijZqEzbhN0e2dWv7Rx/nfpohpDP6vEiFImKFHpDSv3M/5de1ytQzPFrZBYt9WlzlYwE1aD9FHCxdd+rWgYMVVoRaRmndpV/Rq3QUuDuFJtaoX11bC7ExkOpg9KstZzA63i3VcfYv</ds:X509Certificate></ds:X509Data></ds:KeyInfo></md:KeyDescriptor><md:SingleSignOnService Binding="urn:oasis:names:tc:SAML:2.0:bindings:HTTP-Redirect" Location="https://test.com/sso"/></md:IDPSSODescriptor></md:EntityDescriptor>
/// ```
/// ### Iam Workforce Pool Provider Saml Full
///
///
/// ```yaml
/// resources:
///   pool:
///     type: gcp:iam:WorkforcePool
///     properties:
///       workforcePoolId: example-pool
///       parent: organizations/123456789
///       location: global
///   example:
///     type: gcp:iam:WorkforcePoolProvider
///     properties:
///       workforcePoolId: ${pool.workforcePoolId}
///       location: ${pool.location}
///       providerId: example-prvdr
///       attributeMapping:
///         google.subject: assertion.sub
///       saml:
///         idpMetadataXml: <?xml version="1.0"?><md:EntityDescriptor xmlns:md="urn:oasis:names:tc:SAML:2.0:metadata" entityID="https://test.com"><md:IDPSSODescriptor protocolSupportEnumeration="urn:oasis:names:tc:SAML:2.0:protocol"> <md:KeyDescriptor use="signing"><ds:KeyInfo xmlns:ds="http://www.w3.org/2000/09/xmldsig#"><ds:X509Data><ds:X509Certificate>MIIDpDCCAoygAwIBAgIGAX7/5qPhMA0GCSqGSIb3DQEBCwUAMIGSMQswCQYDVQQGEwJVUzETMBEGA1UECAwKQ2FsaWZvcm5pYTEWMBQGA1UEBwwNU2FuIEZyYW5jaXNjbzENMAsGA1UECgwET2t0YTEUMBIGA1UECwwLU1NPUHJvdmlkZXIxEzARBgNVBAMMCmRldi00NTg0MjExHDAaBgkqhkiG9w0BCQEWDWluZm9Ab2t0YS5jb20wHhcNMjIwMjE2MDAxOTEyWhcNMzIwMjE2MDAyMDEyWjCBkjELMAkGA1UEBhMCVVMxEzARBgNVBAgMCkNhbGlmb3JuaWExFjAUBgNVBAcMDVNhbiBGcmFuY2lzY28xDTALBgNVBAoMBE9rdGExFDASBgNVBAsMC1NTT1Byb3ZpZGVyMRMwEQYDVQQDDApkZXYtNDU4NDIxMRwwGgYJKoZIhvcNAQkBFg1pbmZvQG9rdGEuY29tMIIBIjANBgkqhkiG9w0BAQEFAAOCAQ8AMIIBCgKCAQEAxrBl7GKz52cRpxF9xCsirnRuMxnhFBaUrsHqAQrLqWmdlpNYZTVg+T9iQ+aq/iE68L+BRZcZniKIvW58wqqS0ltXVvIkXuDSvnvnkkI5yMIVErR20K8jSOKQm1FmK+fgAJ4koshFiu9oLiqu0Ejc0DuL3/XRsb4RuxjktKTb1khgBBtb+7idEk0sFR0RPefAweXImJkDHDm7SxjDwGJUubbqpdTxasPr0W+AHI1VUzsUsTiHAoyb0XDkYqHfDzhj/ZdIEl4zHQ3bEZvlD984ztAnmX2SuFLLKfXeAAGHei8MMixJvwxYkkPeYZ/5h8WgBZPP4heS2CPjwYExt29L8QIDAQABMA0GCSqGSIb3DQEBCwUAA4IBAQARjJFz++a9Z5IQGFzsZMrX2EDR5ML4xxUiQkbhld1S1PljOLcYFARDmUC2YYHOueU4ee8Jid9nPGEUebV/4Jok+b+oQh+dWMgiWjSLI7h5q4OYZ3VJtdlVwgMFt2iz+/4yBKMUZ50g3Qgg36vE34us+eKitg759JgCNsibxn0qtJgSPm0sgP2L6yTaLnoEUbXBRxCwynTSkp9ZijZqEzbhN0e2dWv7Rx/nfpohpDP6vEiFImKFHpDSv3M/5de1ytQzPFrZBYt9WlzlYwE1aD9FHCxdd+rWgYMVVoRaRmndpV/Rq3QUuDuFJtaoX11bC7ExkOpg9KstZzA63i3VcfYv</ds:X509Certificate></ds:X509Data></ds:KeyInfo></md:KeyDescriptor><md:SingleSignOnService Binding="urn:oasis:names:tc:SAML:2.0:bindings:HTTP-Redirect" Location="https://test.com/sso"/></md:IDPSSODescriptor></md:EntityDescriptor>
///       displayName: Display name
///       description: A sample SAML workforce pool provider.
///       disabled: false
///       attributeCondition: 'true'
/// ```
/// ### Iam Workforce Pool Provider Oidc Basic
///
///
/// ```yaml
/// resources:
///   pool:
///     type: gcp:iam:WorkforcePool
///     properties:
///       workforcePoolId: example-pool
///       parent: organizations/123456789
///       location: global
///   example:
///     type: gcp:iam:WorkforcePoolProvider
///     properties:
///       workforcePoolId: ${pool.workforcePoolId}
///       location: ${pool.location}
///       providerId: example-prvdr
///       attributeMapping:
///         google.subject: assertion.sub
///       oidc:
///         issuerUri: https://accounts.thirdparty.com
///         clientId: client-id
///         clientSecret:
///           value:
///             plainText: client-secret
///         webSsoConfig:
///           responseType: CODE
///           assertionClaimsBehavior: MERGE_USER_INFO_OVER_ID_TOKEN_CLAIMS
/// ```
/// ### Iam Workforce Pool Provider Oidc Full
///
///
/// ```yaml
/// resources:
///   pool:
///     type: gcp:iam:WorkforcePool
///     properties:
///       workforcePoolId: example-pool
///       parent: organizations/123456789
///       location: global
///   example:
///     type: gcp:iam:WorkforcePoolProvider
///     properties:
///       workforcePoolId: ${pool.workforcePoolId}
///       location: ${pool.location}
///       providerId: example-prvdr
///       attributeMapping:
///         google.subject: assertion.sub
///       oidc:
///         issuerUri: https://accounts.thirdparty.com
///         clientId: client-id
///         clientSecret:
///           value:
///             plainText: client-secret
///         webSsoConfig:
///           responseType: CODE
///           assertionClaimsBehavior: MERGE_USER_INFO_OVER_ID_TOKEN_CLAIMS
///           additionalScopes:
///             - groups
///             - roles
///       displayName: Display name
///       description: A sample OIDC workforce pool provider.
///       disabled: false
///       attributeCondition: 'true'
/// ```
/// ### Iam Workforce Pool Provider Extra Attributes Oauth2 Config Client Basic
///
///
/// ```yaml
/// resources:
///   pool:
///     type: gcp:iam:WorkforcePool
///     properties:
///       workforcePoolId: example-pool
///       parent: organizations/123456789
///       location: global
///   example:
///     type: gcp:iam:WorkforcePoolProvider
///     properties:
///       workforcePoolId: ${pool.workforcePoolId}
///       location: ${pool.location}
///       providerId: example-prvdr
///       attributeMapping:
///         google.subject: assertion.sub
///       oidc:
///         issuerUri: https://sts.windows.net/826602fe-2101-470c-9d71-ee1343668989/
///         clientId: https://analysis.windows.net/powerbi/connector/GoogleBigQuery
///         webSsoConfig:
///           responseType: CODE
///           assertionClaimsBehavior: MERGE_USER_INFO_OVER_ID_TOKEN_CLAIMS
///         clientSecret:
///           value:
///             plainText: client-secret
///       extraAttributesOauth2Client:
///         issuerUri: https://login.microsoftonline.com/826602fe-2101-470c-9d71-ee1343668989/v2.0
///         clientId: client-id
///         clientSecret:
///           value:
///             plainText: client-secret
///         attributesType: AZURE_AD_GROUPS_MAIL
/// ```
/// ### Iam Workforce Pool Provider Extra Attributes Oauth2 Config Client Full
///
///
/// ```yaml
/// resources:
///   pool:
///     type: gcp:iam:WorkforcePool
///     properties:
///       workforcePoolId: example-pool
///       parent: organizations/123456789
///       location: global
///   example:
///     type: gcp:iam:WorkforcePoolProvider
///     properties:
///       workforcePoolId: ${pool.workforcePoolId}
///       location: ${pool.location}
///       providerId: example-prvdr
///       attributeMapping:
///         google.subject: assertion.sub
///       oidc:
///         issuerUri: https://sts.windows.net/826602fe-2101-470c-9d71-ee1343668989/
///         clientId: https://analysis.windows.net/powerbi/connector/GoogleBigQuery
///         clientSecret:
///           value:
///             plainText: client-secret
///         webSsoConfig:
///           responseType: CODE
///           assertionClaimsBehavior: MERGE_USER_INFO_OVER_ID_TOKEN_CLAIMS
///       extraAttributesOauth2Client:
///         issuerUri: https://login.microsoftonline.com/826602fe-2101-470c-9d71-ee1343668989/v2.0
///         clientId: client-id
///         clientSecret:
///           value:
///             plainText: client-secret
///         attributesType: AZURE_AD_GROUPS_MAIL
///         queryParameters:
///           filter: mail:gcp
/// ```
///
/// ## Import
///
/// WorkforcePoolProvider can be imported using any of these accepted formats:
///
/// * `locations/{{location}}/workforcePools/{{workforce_pool_id}}/providers/{{provider_id}}`
///
/// * `{{location}}/{{workforce_pool_id}}/{{provider_id}}`
///
/// When using the `pulumi import` command, WorkforcePoolProvider can be imported using one of the formats above. For example:
///
/// ```sh
/// $ pulumi import gcp:iam/workforcePoolProvider:WorkforcePoolProvider default locations/{{location}}/workforcePools/{{workforce_pool_id}}/providers/{{provider_id}}
/// ```
///
/// ```sh
/// $ pulumi import gcp:iam/workforcePoolProvider:WorkforcePoolProvider default {{location}}/{{workforce_pool_id}}/{{provider_id}}
/// ```
///
pub mod workforce_pool_provider {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct WorkforcePoolProviderArgs {
        /// A [Common Expression Language](https://opensource.google/projects/cel) expression, in
        /// plain text, to restrict what otherwise valid authentication credentials issued by the
        /// provider should not be accepted.
        /// The expression must output a boolean representing whether to allow the federation.
        /// The following keywords may be referenced in the expressions:
        #[builder(into, default)]
        pub attribute_condition: pulumi_wasm_rust::Output<Option<String>>,
        /// Maps attributes from the authentication credentials issued by an external identity provider
        /// to Google Cloud attributes, such as `subject` and `segment`.
        /// Each key must be a string specifying the Google Cloud IAM attribute to map to.
        /// The following keys are supported:
        /// * `google.subject`: The principal IAM is authenticating. You can reference this value in IAM bindings.
        /// This is also the subject that appears in Cloud Logging logs. This is a required field and
        /// the mapped subject cannot exceed 127 bytes.
        /// * `google.groups`: Groups the authenticating user belongs to. You can grant groups access to
        /// resources using an IAM `principalSet` binding; access applies to all members of the group.
        /// * `google.display_name`: The name of the authenticated user. This is an optional field and
        /// the mapped display name cannot exceed 100 bytes. If not set, `google.subject` will be displayed instead.
        /// This attribute cannot be referenced in IAM bindings.
        /// * `google.profile_photo`: The URL that specifies the authenticated user's thumbnail photo.
        /// This is an optional field. When set, the image will be visible as the user's profile picture.
        /// If not set, a generic user icon will be displayed instead.
        /// This attribute cannot be referenced in IAM bindings.
        /// You can also provide custom attributes by specifying `attribute.{custom_attribute}`, where {custom_attribute}
        /// is the name of the custom attribute to be mapped. You can define a maximum of 50 custom attributes.
        /// The maximum length of a mapped attribute key is 100 characters, and the key may only contain the characters [a-z0-9_].
        /// You can reference these attributes in IAM policies to define fine-grained access for a workforce pool
        /// to Google Cloud resources. For example:
        /// * `google.subject`:
        /// `principal://iam.googleapis.com/locations/{location}/workforcePools/{pool}/subject/{value}`
        /// * `google.groups`:
        /// `principalSet://iam.googleapis.com/locations/{location}/workforcePools/{pool}/group/{value}`
        /// * `attribute.{custom_attribute}`:
        /// `principalSet://iam.googleapis.com/locations/{location}/workforcePools/{pool}/attribute.{custom_attribute}/{value}`
        /// Each value must be a [Common Expression Language](https://opensource.google/projects/cel)
        /// function that maps an identity provider credential to the normalized attribute specified
        /// by the corresponding map key.
        /// You can use the `assertion` keyword in the expression to access a JSON representation of
        /// the authentication credential issued by the provider.
        /// The maximum length of an attribute mapping expression is 2048 characters. When evaluated,
        /// the total size of all mapped attributes must not exceed 8KB.
        /// For OIDC providers, you must supply a custom mapping that includes the `google.subject` attribute.
        /// For example, the following maps the sub claim of the incoming credential to the `subject` attribute
        /// on a Google token:
        /// ```sh
        /// {"google.subject": "assertion.sub"}
        /// ```
        /// An object containing a list of `"key": value` pairs.
        /// Example: `{ "name": "wrench", "mass": "1.3kg", "count": "3" }`.
        #[builder(into, default)]
        pub attribute_mapping: pulumi_wasm_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// A user-specified description of the provider. Cannot exceed 256 characters.
        #[builder(into, default)]
        pub description: pulumi_wasm_rust::Output<Option<String>>,
        /// Whether the provider is disabled. You cannot use a disabled provider to exchange tokens.
        /// However, existing tokens still grant access.
        #[builder(into, default)]
        pub disabled: pulumi_wasm_rust::Output<Option<bool>>,
        /// A user-specified display name for the provider. Cannot exceed 32 characters.
        #[builder(into, default)]
        pub display_name: pulumi_wasm_rust::Output<Option<String>>,
        /// The configuration for OAuth 2.0 client used to get the additional user
        /// attributes. This should be used when users can't get the desired claims
        /// in authentication credentials. Currently this configuration is only
        /// supported with OIDC protocol.
        /// Structure is documented below.
        #[builder(into, default)]
        pub extra_attributes_oauth2_client: pulumi_wasm_rust::Output<
            Option<
                super::super::types::iam::WorkforcePoolProviderExtraAttributesOauth2Client,
            >,
        >,
        /// The location for the resource.
        #[builder(into)]
        pub location: pulumi_wasm_rust::Output<String>,
        /// Represents an OpenId Connect 1.0 identity provider.
        /// Structure is documented below.
        #[builder(into, default)]
        pub oidc: pulumi_wasm_rust::Output<
            Option<super::super::types::iam::WorkforcePoolProviderOidc>,
        >,
        /// The ID for the provider, which becomes the final component of the resource name.
        /// This value must be 4-32 characters, and may contain the characters [a-z0-9-].
        /// The prefix `gcp-` is reserved for use by Google, and may not be specified.
        ///
        ///
        /// - - -
        #[builder(into)]
        pub provider_id: pulumi_wasm_rust::Output<String>,
        /// Represents a SAML identity provider.
        /// Structure is documented below.
        #[builder(into, default)]
        pub saml: pulumi_wasm_rust::Output<
            Option<super::super::types::iam::WorkforcePoolProviderSaml>,
        >,
        /// The ID to use for the pool, which becomes the final component of the resource name.
        /// The IDs must be a globally unique string of 6 to 63 lowercase letters, digits, or hyphens.
        /// It must start with a letter, and cannot have a trailing hyphen.
        /// The prefix `gcp-` is reserved for use by Google, and may not be specified.
        #[builder(into)]
        pub workforce_pool_id: pulumi_wasm_rust::Output<String>,
    }
    #[allow(dead_code)]
    pub struct WorkforcePoolProviderResult {
        /// A [Common Expression Language](https://opensource.google/projects/cel) expression, in
        /// plain text, to restrict what otherwise valid authentication credentials issued by the
        /// provider should not be accepted.
        /// The expression must output a boolean representing whether to allow the federation.
        /// The following keywords may be referenced in the expressions:
        pub attribute_condition: pulumi_wasm_rust::Output<Option<String>>,
        /// Maps attributes from the authentication credentials issued by an external identity provider
        /// to Google Cloud attributes, such as `subject` and `segment`.
        /// Each key must be a string specifying the Google Cloud IAM attribute to map to.
        /// The following keys are supported:
        /// * `google.subject`: The principal IAM is authenticating. You can reference this value in IAM bindings.
        /// This is also the subject that appears in Cloud Logging logs. This is a required field and
        /// the mapped subject cannot exceed 127 bytes.
        /// * `google.groups`: Groups the authenticating user belongs to. You can grant groups access to
        /// resources using an IAM `principalSet` binding; access applies to all members of the group.
        /// * `google.display_name`: The name of the authenticated user. This is an optional field and
        /// the mapped display name cannot exceed 100 bytes. If not set, `google.subject` will be displayed instead.
        /// This attribute cannot be referenced in IAM bindings.
        /// * `google.profile_photo`: The URL that specifies the authenticated user's thumbnail photo.
        /// This is an optional field. When set, the image will be visible as the user's profile picture.
        /// If not set, a generic user icon will be displayed instead.
        /// This attribute cannot be referenced in IAM bindings.
        /// You can also provide custom attributes by specifying `attribute.{custom_attribute}`, where {custom_attribute}
        /// is the name of the custom attribute to be mapped. You can define a maximum of 50 custom attributes.
        /// The maximum length of a mapped attribute key is 100 characters, and the key may only contain the characters [a-z0-9_].
        /// You can reference these attributes in IAM policies to define fine-grained access for a workforce pool
        /// to Google Cloud resources. For example:
        /// * `google.subject`:
        /// `principal://iam.googleapis.com/locations/{location}/workforcePools/{pool}/subject/{value}`
        /// * `google.groups`:
        /// `principalSet://iam.googleapis.com/locations/{location}/workforcePools/{pool}/group/{value}`
        /// * `attribute.{custom_attribute}`:
        /// `principalSet://iam.googleapis.com/locations/{location}/workforcePools/{pool}/attribute.{custom_attribute}/{value}`
        /// Each value must be a [Common Expression Language](https://opensource.google/projects/cel)
        /// function that maps an identity provider credential to the normalized attribute specified
        /// by the corresponding map key.
        /// You can use the `assertion` keyword in the expression to access a JSON representation of
        /// the authentication credential issued by the provider.
        /// The maximum length of an attribute mapping expression is 2048 characters. When evaluated,
        /// the total size of all mapped attributes must not exceed 8KB.
        /// For OIDC providers, you must supply a custom mapping that includes the `google.subject` attribute.
        /// For example, the following maps the sub claim of the incoming credential to the `subject` attribute
        /// on a Google token:
        /// ```sh
        /// {"google.subject": "assertion.sub"}
        /// ```
        /// An object containing a list of `"key": value` pairs.
        /// Example: `{ "name": "wrench", "mass": "1.3kg", "count": "3" }`.
        pub attribute_mapping: pulumi_wasm_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// A user-specified description of the provider. Cannot exceed 256 characters.
        pub description: pulumi_wasm_rust::Output<Option<String>>,
        /// Whether the provider is disabled. You cannot use a disabled provider to exchange tokens.
        /// However, existing tokens still grant access.
        pub disabled: pulumi_wasm_rust::Output<Option<bool>>,
        /// A user-specified display name for the provider. Cannot exceed 32 characters.
        pub display_name: pulumi_wasm_rust::Output<Option<String>>,
        /// The configuration for OAuth 2.0 client used to get the additional user
        /// attributes. This should be used when users can't get the desired claims
        /// in authentication credentials. Currently this configuration is only
        /// supported with OIDC protocol.
        /// Structure is documented below.
        pub extra_attributes_oauth2_client: pulumi_wasm_rust::Output<
            Option<
                super::super::types::iam::WorkforcePoolProviderExtraAttributesOauth2Client,
            >,
        >,
        /// The location for the resource.
        pub location: pulumi_wasm_rust::Output<String>,
        /// Output only. The resource name of the provider.
        /// Format: `locations/{location}/workforcePools/{workforcePoolId}/providers/{providerId}`
        pub name: pulumi_wasm_rust::Output<String>,
        /// Represents an OpenId Connect 1.0 identity provider.
        /// Structure is documented below.
        pub oidc: pulumi_wasm_rust::Output<
            Option<super::super::types::iam::WorkforcePoolProviderOidc>,
        >,
        /// The ID for the provider, which becomes the final component of the resource name.
        /// This value must be 4-32 characters, and may contain the characters [a-z0-9-].
        /// The prefix `gcp-` is reserved for use by Google, and may not be specified.
        ///
        ///
        /// - - -
        pub provider_id: pulumi_wasm_rust::Output<String>,
        /// Represents a SAML identity provider.
        /// Structure is documented below.
        pub saml: pulumi_wasm_rust::Output<
            Option<super::super::types::iam::WorkforcePoolProviderSaml>,
        >,
        /// The current state of the provider.
        /// * STATE_UNSPECIFIED: State unspecified.
        /// * ACTIVE: The provider is active and may be used to validate authentication credentials.
        /// * DELETED: The provider is soft-deleted. Soft-deleted providers are permanently
        /// deleted after approximately 30 days. You can restore a soft-deleted provider using
        /// [providers.undelete](https://cloud.google.com/iam/docs/reference/rest/v1/locations.workforcePools.providers/undelete#google.iam.admin.v1.WorkforcePools.UndeleteWorkforcePoolProvider).
        pub state: pulumi_wasm_rust::Output<String>,
        /// The ID to use for the pool, which becomes the final component of the resource name.
        /// The IDs must be a globally unique string of 6 to 63 lowercase letters, digits, or hyphens.
        /// It must start with a letter, and cannot have a trailing hyphen.
        /// The prefix `gcp-` is reserved for use by Google, and may not be specified.
        pub workforce_pool_id: pulumi_wasm_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        name: &str,
        args: WorkforcePoolProviderArgs,
    ) -> WorkforcePoolProviderResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let attribute_condition_binding = args.attribute_condition.get_inner();
        let attribute_mapping_binding = args.attribute_mapping.get_inner();
        let description_binding = args.description.get_inner();
        let disabled_binding = args.disabled.get_inner();
        let display_name_binding = args.display_name.get_inner();
        let extra_attributes_oauth2_client_binding = args
            .extra_attributes_oauth2_client
            .get_inner();
        let location_binding = args.location.get_inner();
        let oidc_binding = args.oidc.get_inner();
        let provider_id_binding = args.provider_id.get_inner();
        let saml_binding = args.saml.get_inner();
        let workforce_pool_id_binding = args.workforce_pool_id.get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "gcp:iam/workforcePoolProvider:WorkforcePoolProvider".into(),
            name: name.to_string(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "attributeCondition".into(),
                    value: &attribute_condition_binding,
                },
                register_interface::ObjectField {
                    name: "attributeMapping".into(),
                    value: &attribute_mapping_binding,
                },
                register_interface::ObjectField {
                    name: "description".into(),
                    value: &description_binding,
                },
                register_interface::ObjectField {
                    name: "disabled".into(),
                    value: &disabled_binding,
                },
                register_interface::ObjectField {
                    name: "displayName".into(),
                    value: &display_name_binding,
                },
                register_interface::ObjectField {
                    name: "extraAttributesOauth2Client".into(),
                    value: &extra_attributes_oauth2_client_binding,
                },
                register_interface::ObjectField {
                    name: "location".into(),
                    value: &location_binding,
                },
                register_interface::ObjectField {
                    name: "oidc".into(),
                    value: &oidc_binding,
                },
                register_interface::ObjectField {
                    name: "providerId".into(),
                    value: &provider_id_binding,
                },
                register_interface::ObjectField {
                    name: "saml".into(),
                    value: &saml_binding,
                },
                register_interface::ObjectField {
                    name: "workforcePoolId".into(),
                    value: &workforce_pool_id_binding,
                },
            ]),
            results: Vec::from([
                register_interface::ResultField {
                    name: "attributeCondition".into(),
                },
                register_interface::ResultField {
                    name: "attributeMapping".into(),
                },
                register_interface::ResultField {
                    name: "description".into(),
                },
                register_interface::ResultField {
                    name: "disabled".into(),
                },
                register_interface::ResultField {
                    name: "displayName".into(),
                },
                register_interface::ResultField {
                    name: "extraAttributesOauth2Client".into(),
                },
                register_interface::ResultField {
                    name: "location".into(),
                },
                register_interface::ResultField {
                    name: "name".into(),
                },
                register_interface::ResultField {
                    name: "oidc".into(),
                },
                register_interface::ResultField {
                    name: "providerId".into(),
                },
                register_interface::ResultField {
                    name: "saml".into(),
                },
                register_interface::ResultField {
                    name: "state".into(),
                },
                register_interface::ResultField {
                    name: "workforcePoolId".into(),
                },
            ]),
        };
        let o = register_interface::register(&request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        WorkforcePoolProviderResult {
            attribute_condition: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("attributeCondition").unwrap(),
            ),
            attribute_mapping: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("attributeMapping").unwrap(),
            ),
            description: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("description").unwrap(),
            ),
            disabled: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("disabled").unwrap(),
            ),
            display_name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("displayName").unwrap(),
            ),
            extra_attributes_oauth2_client: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("extraAttributesOauth2Client").unwrap(),
            ),
            location: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("location").unwrap(),
            ),
            name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("name").unwrap(),
            ),
            oidc: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("oidc").unwrap(),
            ),
            provider_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("providerId").unwrap(),
            ),
            saml: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("saml").unwrap(),
            ),
            state: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("state").unwrap(),
            ),
            workforce_pool_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("workforcePoolId").unwrap(),
            ),
        }
    }
}
