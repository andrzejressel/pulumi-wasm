/// The GatewaySecurityPolicy resource contains a collection of GatewaySecurityPolicyRules and associated metadata.
///
///
/// To get more information about GatewaySecurityPolicy, see:
///
/// * [API documentation](https://cloud.google.com/secure-web-proxy/docs/reference/network-security/rest/v1/projects.locations.gatewaySecurityPolicies)
///
/// ## Example Usage
///
/// ### Network Security Gateway Security Policy Basic
///
///
/// ```ignore
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let default = gateway_security_policy::create(
///         "default",
///         GatewaySecurityPolicyArgs::builder()
///             .description("my description")
///             .location("us-central1")
///             .name("my-gateway-security-policy")
///             .build_struct(),
///     );
/// }
/// ```
/// ### Network Security Gateway Security Policy Tls Inspection Basic
///
///
/// ```yaml
/// resources:
///   default:
///     type: gcp:certificateauthority:CaPool
///     properties:
///       name: my-basic-ca-pool
///       location: us-central1
///       tier: DEVOPS
///       publishingOptions:
///         publishCaCert: false
///         publishCrl: false
///       issuancePolicy:
///         maximumLifetime: 1209600s
///         baselineValues:
///           caOptions:
///             isCa: false
///           keyUsage:
///             baseKeyUsage: {}
///             extendedKeyUsage:
///               serverAuth: true
///   defaultAuthority:
///     type: gcp:certificateauthority:Authority
///     name: default
///     properties:
///       pool: ${default.name}
///       certificateAuthorityId: my-basic-certificate-authority
///       location: us-central1
///       lifetime: 86400s
///       type: SELF_SIGNED
///       deletionProtection: false
///       skipGracePeriod: true
///       ignoreActiveCertificatesOnDeletion: true
///       config:
///         subjectConfig:
///           subject:
///             organization: Test LLC
///             commonName: my-ca
///         x509Config:
///           caOptions:
///             isCa: true
///           keyUsage:
///             baseKeyUsage:
///               certSign: true
///               crlSign: true
///             extendedKeyUsage:
///               serverAuth: false
///       keySpec:
///         algorithm: RSA_PKCS1_4096_SHA256
///   tlsInspectionPermission:
///     type: gcp:certificateauthority:CaPoolIamMember
///     name: tls_inspection_permission
///     properties:
///       caPool: ${default.id}
///       role: roles/privateca.certificateManager
///       member: serviceAccount:service-${project.number}@gcp-sa-networksecurity.iam.gserviceaccount.com
///   defaultTlsInspectionPolicy:
///     type: gcp:networksecurity:TlsInspectionPolicy
///     name: default
///     properties:
///       name: my-tls-inspection-policy
///       location: us-central1
///       caPool: ${default.id}
///     options:
///       dependsOn:
///         - ${default}
///         - ${defaultAuthority}
///         - ${tlsInspectionPermission}
///   defaultGatewaySecurityPolicy:
///     type: gcp:networksecurity:GatewaySecurityPolicy
///     name: default
///     properties:
///       name: my-gateway-security-policy
///       location: us-central1
///       description: my description
///       tlsInspectionPolicy: ${defaultTlsInspectionPolicy.id}
///     options:
///       dependsOn:
///         - ${defaultTlsInspectionPolicy}
/// variables:
///   project:
///     fn::invoke:
///       function: gcp:organizations:getProject
///       arguments: {}
/// ```
///
/// ## Import
///
/// GatewaySecurityPolicy can be imported using any of these accepted formats:
///
/// * `projects/{{project}}/locations/{{location}}/gatewaySecurityPolicies/{{name}}`
///
/// * `{{project}}/{{location}}/{{name}}`
///
/// * `{{location}}/{{name}}`
///
/// When using the `pulumi import` command, GatewaySecurityPolicy can be imported using one of the formats above. For example:
///
/// ```sh
/// $ pulumi import gcp:networksecurity/gatewaySecurityPolicy:GatewaySecurityPolicy default projects/{{project}}/locations/{{location}}/gatewaySecurityPolicies/{{name}}
/// ```
///
/// ```sh
/// $ pulumi import gcp:networksecurity/gatewaySecurityPolicy:GatewaySecurityPolicy default {{project}}/{{location}}/{{name}}
/// ```
///
/// ```sh
/// $ pulumi import gcp:networksecurity/gatewaySecurityPolicy:GatewaySecurityPolicy default {{location}}/{{name}}
/// ```
///
#[allow(clippy::doc_lazy_continuation)]
pub mod gateway_security_policy {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GatewaySecurityPolicyArgs {
        /// A free-text description of the resource. Max length 1024 characters.
        #[builder(into, default)]
        pub description: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The location of the gateway security policy.
        /// The default value is `global`.
        #[builder(into, default)]
        pub location: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Name of the resource. Name is of the form projects/{project}/locations/{location}/gatewaySecurityPolicies/{gatewaySecurityPolicy}
        /// gatewaySecurityPolicy should match the pattern:(^a-z?$).
        ///
        ///
        /// - - -
        #[builder(into, default)]
        pub name: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The ID of the project in which the resource belongs.
        /// If it is not provided, the provider project is used.
        #[builder(into, default)]
        pub project: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Name of a TlsInspectionPolicy resource that defines how TLS inspection is performed for any rule that enables it.
        /// Note: gcp.networksecurity.TlsInspectionPolicy resource is still in Beta therefore it will need to import the provider.
        #[builder(into, default)]
        pub tls_inspection_policy: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
    }
    #[allow(dead_code)]
    pub struct GatewaySecurityPolicyResult {
        /// The timestamp when the resource was created.
        /// A timestamp in RFC3339 UTC "Zulu" format, with nanosecond resolution and up to nine fractional digits.
        /// Examples: "2014-10-02T15:01:23Z" and "2014-10-02T15:01:23.045123456Z"
        pub create_time: pulumi_gestalt_rust::Output<String>,
        /// A free-text description of the resource. Max length 1024 characters.
        pub description: pulumi_gestalt_rust::Output<Option<String>>,
        /// The location of the gateway security policy.
        /// The default value is `global`.
        pub location: pulumi_gestalt_rust::Output<Option<String>>,
        /// Name of the resource. Name is of the form projects/{project}/locations/{location}/gatewaySecurityPolicies/{gatewaySecurityPolicy}
        /// gatewaySecurityPolicy should match the pattern:(^a-z?$).
        ///
        ///
        /// - - -
        pub name: pulumi_gestalt_rust::Output<String>,
        /// The ID of the project in which the resource belongs.
        /// If it is not provided, the provider project is used.
        pub project: pulumi_gestalt_rust::Output<String>,
        /// Server-defined URL of this resource.
        pub self_link: pulumi_gestalt_rust::Output<String>,
        /// Name of a TlsInspectionPolicy resource that defines how TLS inspection is performed for any rule that enables it.
        /// Note: gcp.networksecurity.TlsInspectionPolicy resource is still in Beta therefore it will need to import the provider.
        pub tls_inspection_policy: pulumi_gestalt_rust::Output<Option<String>>,
        /// The timestamp when the resource was updated.
        /// A timestamp in RFC3339 UTC "Zulu" format, with nanosecond resolution and up to nine fractional digits.
        /// Examples: "2014-10-02T15:01:23Z" and "2014-10-02T15:01:23.045123456Z".
        pub update_time: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::PulumiContext,
        name: &str,
        args: GatewaySecurityPolicyArgs,
    ) -> GatewaySecurityPolicyResult {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::register_interface;
        use std::collections::HashMap;
        let description_binding = args.description.get_output(context).get_inner();
        let location_binding = args.location.get_output(context).get_inner();
        let name_binding = args.name.get_output(context).get_inner();
        let project_binding = args.project.get_output(context).get_inner();
        let tls_inspection_policy_binding = args
            .tls_inspection_policy
            .get_output(context)
            .get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "gcp:networksecurity/gatewaySecurityPolicy:GatewaySecurityPolicy"
                .into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "description".into(),
                    value: &description_binding,
                },
                register_interface::ObjectField {
                    name: "location".into(),
                    value: &location_binding,
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
                    name: "tlsInspectionPolicy".into(),
                    value: &tls_inspection_policy_binding,
                },
            ]),
        };
        let o = register_interface::register(context.get_inner(), &request);
        GatewaySecurityPolicyResult {
            create_time: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("createTime"),
            ),
            description: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("description"),
            ),
            location: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("location"),
            ),
            name: pulumi_gestalt_rust::__private::into_domain(o.extract_field("name")),
            project: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("project"),
            ),
            self_link: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("selfLink"),
            ),
            tls_inspection_policy: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("tlsInspectionPolicy"),
            ),
            update_time: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("updateTime"),
            ),
        }
    }
}
