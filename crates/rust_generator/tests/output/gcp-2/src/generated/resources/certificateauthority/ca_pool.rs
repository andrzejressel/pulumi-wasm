/// A CaPool represents a group of CertificateAuthorities that form a trust anchor. A CaPool can be used to manage
/// issuance policies for one or more CertificateAuthority resources and to rotate CA certificates in and out of the
/// trust anchor.
///
///
///
/// ## Example Usage
///
/// ### Privateca Capool Basic
///
///
/// ```yaml
/// resources:
///   default:
///     type: gcp:certificateauthority:CaPool
///     properties:
///       name: my-pool
///       location: us-central1
///       tier: ENTERPRISE
///       publishingOptions:
///         publishCaCert: true
///         publishCrl: true
///       labels:
///         foo: bar
/// ```
/// ### Privateca Capool All Fields
///
///
/// ```yaml
/// resources:
///   default:
///     type: gcp:certificateauthority:CaPool
///     properties:
///       name: my-pool
///       location: us-central1
///       tier: ENTERPRISE
///       publishingOptions:
///         publishCaCert: false
///         publishCrl: true
///         encodingFormat: PEM
///       labels:
///         foo: bar
///       issuancePolicy:
///         allowedKeyTypes:
///           - ellipticCurve:
///               signatureAlgorithm: ECDSA_P256
///           - rsa:
///               minModulusSize: 5
///               maxModulusSize: 10
///         maximumLifetime: 50000s
///         allowedIssuanceModes:
///           allowCsrBasedIssuance: true
///           allowConfigBasedIssuance: true
///         identityConstraints:
///           allowSubjectPassthrough: true
///           allowSubjectAltNamesPassthrough: true
///           celExpression:
///             expression: subject_alt_names.all(san, san.type == DNS || san.type == EMAIL )
///             title: My title
///         baselineValues:
///           aiaOcspServers:
///             - example.com
///           additionalExtensions:
///             - critical: true
///               value: asdf
///               objectId:
///                 objectIdPaths:
///                   - 1
///                   - 7
///           policyIds:
///             - objectIdPaths:
///                 - 1
///                 - 5
///             - objectIdPaths:
///                 - 1
///                 - 5
///                 - 7
///           caOptions:
///             isCa: true
///             maxIssuerPathLength: 10
///           keyUsage:
///             baseKeyUsage:
///               digitalSignature: true
///               contentCommitment: true
///               keyEncipherment: false
///               dataEncipherment: true
///               keyAgreement: true
///               certSign: false
///               crlSign: true
///               decipherOnly: true
///             extendedKeyUsage:
///               serverAuth: true
///               clientAuth: false
///               emailProtection: true
///               codeSigning: true
///               timeStamping: true
///           nameConstraints:
///             critical: true
///             permittedDnsNames:
///               - '*.example1.com'
///               - '*.example2.com'
///             excludedDnsNames:
///               - '*.deny.example1.com'
///               - '*.deny.example2.com'
///             permittedIpRanges:
///               - 10.0.0.0/8
///               - 11.0.0.0/8
///             excludedIpRanges:
///               - 10.1.1.0/24
///               - 11.1.1.0/24
///             permittedEmailAddresses:
///               - .example1.com
///               - .example2.com
///             excludedEmailAddresses:
///               - .deny.example1.com
///               - .deny.example2.com
///             permittedUris:
///               - .example1.com
///               - .example2.com
///             excludedUris:
///               - .deny.example1.com
///               - .deny.example2.com
/// ```
///
/// ## Import
///
/// CaPool can be imported using any of these accepted formats:
///
/// * `projects/{{project}}/locations/{{location}}/caPools/{{name}}`
///
/// * `{{project}}/{{location}}/{{name}}`
///
/// * `{{location}}/{{name}}`
///
/// When using the `pulumi import` command, CaPool can be imported using one of the formats above. For example:
///
/// ```sh
/// $ pulumi import gcp:certificateauthority/caPool:CaPool default projects/{{project}}/locations/{{location}}/caPools/{{name}}
/// ```
///
/// ```sh
/// $ pulumi import gcp:certificateauthority/caPool:CaPool default {{project}}/{{location}}/{{name}}
/// ```
///
/// ```sh
/// $ pulumi import gcp:certificateauthority/caPool:CaPool default {{location}}/{{name}}
/// ```
///
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod ca_pool {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct CaPoolArgs {
        /// The IssuancePolicy to control how Certificates will be issued from this CaPool.
        /// Structure is documented below.
        #[builder(into, default)]
        pub issuance_policy: pulumi_gestalt_rust::InputOrOutput<
            Option<super::super::types::certificateauthority::CaPoolIssuancePolicy>,
        >,
        /// Labels with user-defined metadata.
        /// An object containing a list of "key": value pairs. Example: { "name": "wrench", "mass":
        /// "1.3kg", "count": "3" }.
        ///
        /// **Note**: This field is non-authoritative, and will only manage the labels present in your configuration.
        /// Please refer to the field `effective_labels` for all of the labels present on the resource.
        #[builder(into, default)]
        pub labels: pulumi_gestalt_rust::InputOrOutput<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// Location of the CaPool. A full list of valid locations can be found by
        /// running `gcloud privateca locations list`.
        ///
        ///
        /// - - -
        #[builder(into)]
        pub location: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The name for this CaPool.
        #[builder(into, default)]
        pub name: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The ID of the project in which the resource belongs.
        /// If it is not provided, the provider project is used.
        #[builder(into, default)]
        pub project: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The PublishingOptions to follow when issuing Certificates from any CertificateAuthority in this CaPool.
        /// Structure is documented below.
        #[builder(into, default)]
        pub publishing_options: pulumi_gestalt_rust::InputOrOutput<
            Option<super::super::types::certificateauthority::CaPoolPublishingOptions>,
        >,
        /// The Tier of this CaPool.
        /// Possible values are: `ENTERPRISE`, `DEVOPS`.
        #[builder(into)]
        pub tier: pulumi_gestalt_rust::InputOrOutput<String>,
    }
    #[allow(dead_code)]
    pub struct CaPoolResult {
        /// All of labels (key/value pairs) present on the resource in GCP, including the labels configured through Pulumi, other clients and services.
        pub effective_labels: pulumi_gestalt_rust::Output<
            std::collections::HashMap<String, String>,
        >,
        /// The IssuancePolicy to control how Certificates will be issued from this CaPool.
        /// Structure is documented below.
        pub issuance_policy: pulumi_gestalt_rust::Output<
            Option<super::super::types::certificateauthority::CaPoolIssuancePolicy>,
        >,
        /// Labels with user-defined metadata.
        /// An object containing a list of "key": value pairs. Example: { "name": "wrench", "mass":
        /// "1.3kg", "count": "3" }.
        ///
        /// **Note**: This field is non-authoritative, and will only manage the labels present in your configuration.
        /// Please refer to the field `effective_labels` for all of the labels present on the resource.
        pub labels: pulumi_gestalt_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// Location of the CaPool. A full list of valid locations can be found by
        /// running `gcloud privateca locations list`.
        ///
        ///
        /// - - -
        pub location: pulumi_gestalt_rust::Output<String>,
        /// The name for this CaPool.
        pub name: pulumi_gestalt_rust::Output<String>,
        /// The ID of the project in which the resource belongs.
        /// If it is not provided, the provider project is used.
        pub project: pulumi_gestalt_rust::Output<String>,
        /// The PublishingOptions to follow when issuing Certificates from any CertificateAuthority in this CaPool.
        /// Structure is documented below.
        pub publishing_options: pulumi_gestalt_rust::Output<
            Option<super::super::types::certificateauthority::CaPoolPublishingOptions>,
        >,
        /// The combination of labels configured directly on the resource
        /// and default labels configured on the provider.
        pub pulumi_labels: pulumi_gestalt_rust::Output<
            std::collections::HashMap<String, String>,
        >,
        /// The Tier of this CaPool.
        /// Possible values are: `ENTERPRISE`, `DEVOPS`.
        pub tier: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::PulumiContext,
        name: &str,
        args: CaPoolArgs,
    ) -> CaPoolResult {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::register_interface;
        use std::collections::HashMap;
        let issuance_policy_binding = args
            .issuance_policy
            .get_output(context)
            .get_inner();
        let labels_binding = args.labels.get_output(context).get_inner();
        let location_binding = args.location.get_output(context).get_inner();
        let name_binding = args.name.get_output(context).get_inner();
        let project_binding = args.project.get_output(context).get_inner();
        let publishing_options_binding = args
            .publishing_options
            .get_output(context)
            .get_inner();
        let tier_binding = args.tier.get_output(context).get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "gcp:certificateauthority/caPool:CaPool".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "issuancePolicy".into(),
                    value: &issuance_policy_binding,
                },
                register_interface::ObjectField {
                    name: "labels".into(),
                    value: &labels_binding,
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
                    name: "publishingOptions".into(),
                    value: &publishing_options_binding,
                },
                register_interface::ObjectField {
                    name: "tier".into(),
                    value: &tier_binding,
                },
            ]),
        };
        let o = register_interface::register(context.get_inner(), &request);
        CaPoolResult {
            effective_labels: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("effectiveLabels"),
            ),
            issuance_policy: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("issuancePolicy"),
            ),
            labels: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("labels"),
            ),
            location: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("location"),
            ),
            name: pulumi_gestalt_rust::__private::into_domain(o.extract_field("name")),
            project: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("project"),
            ),
            publishing_options: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("publishingOptions"),
            ),
            pulumi_labels: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("pulumiLabels"),
            ),
            tier: pulumi_gestalt_rust::__private::into_domain(o.extract_field("tier")),
        }
    }
}
