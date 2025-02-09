/// An SslCertificate resource, used for HTTPS load balancing. This resource
/// provides a mechanism to upload an SSL key and certificate to
/// the load balancer to serve secure connections from the user.
///
///
/// To get more information about SslCertificate, see:
///
/// * [API documentation](https://cloud.google.com/compute/docs/reference/rest/v1/sslCertificates)
/// * How-to Guides
///     * [Official Documentation](https://cloud.google.com/load-balancing/docs/ssl-certificates)
///
///
///
/// ## Example Usage
///
/// ## Import
///
/// SslCertificate can be imported using any of these accepted formats:
///
/// * `projects/{{project}}/global/sslCertificates/{{name}}`
///
/// * `{{project}}/{{name}}`
///
/// * `{{name}}`
///
/// When using the `pulumi import` command, SslCertificate can be imported using one of the formats above. For example:
///
/// ```sh
/// $ pulumi import gcp:compute/sSLCertificate:SSLCertificate default projects/{{project}}/global/sslCertificates/{{name}}
/// ```
///
/// ```sh
/// $ pulumi import gcp:compute/sSLCertificate:SSLCertificate default {{project}}/{{name}}
/// ```
///
/// ```sh
/// $ pulumi import gcp:compute/sSLCertificate:SSLCertificate default {{name}}
/// ```
///
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod ssl_certificate {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct SSLCertificateArgs {
        /// The certificate in PEM format.
        /// The certificate chain must be no greater than 5 certs long.
        /// The chain must include at least one intermediate cert.
        /// **Note**: This property is sensitive and will not be displayed in the plan.
        #[builder(into)]
        pub certificate: pulumi_gestalt_rust::InputOrOutput<String>,
        /// An optional description of this resource.
        #[builder(into, default)]
        pub description: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Name of the resource. Provided by the client when the resource is
        /// created. The name must be 1-63 characters long, and comply with
        /// RFC1035. Specifically, the name must be 1-63 characters long and match
        /// the regular expression `a-z?` which means the
        /// first character must be a lowercase letter, and all following
        /// characters must be a dash, lowercase letter, or digit, except the last
        /// character, which cannot be a dash.
        /// These are in the same namespace as the managed SSL certificates.
        #[builder(into, default)]
        pub name: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Creates a unique name beginning with the
        /// specified prefix. Conflicts with `name`. Max length is 54 characters.
        /// Prefixes with lengths longer than 37 characters will use a shortened
        /// UUID that will be more prone to collisions.
        /// Resulting name for a `name_prefix` <= 37 characters:
        /// `name_prefix` + YYYYmmddHHSSssss + 8 digit incremental counter
        /// Resulting name for a `name_prefix` 38 - 54 characters:
        /// `name_prefix` + YYmmdd + 3 digit incremental counter
        #[builder(into, default)]
        pub name_prefix: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The write-only private key in PEM format.
        /// **Note**: This property is sensitive and will not be displayed in the plan.
        ///
        ///
        /// - - -
        #[builder(into)]
        pub private_key: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The ID of the project in which the resource belongs.
        /// If it is not provided, the provider project is used.
        #[builder(into, default)]
        pub project: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
    }
    #[allow(dead_code)]
    pub struct SSLCertificateResult {
        /// The certificate in PEM format.
        /// The certificate chain must be no greater than 5 certs long.
        /// The chain must include at least one intermediate cert.
        /// **Note**: This property is sensitive and will not be displayed in the plan.
        pub certificate: pulumi_gestalt_rust::Output<String>,
        /// The unique identifier for the resource.
        pub certificate_id: pulumi_gestalt_rust::Output<i32>,
        /// Creation timestamp in RFC3339 text format.
        pub creation_timestamp: pulumi_gestalt_rust::Output<String>,
        /// An optional description of this resource.
        pub description: pulumi_gestalt_rust::Output<Option<String>>,
        /// Expire time of the certificate in RFC3339 text format.
        pub expire_time: pulumi_gestalt_rust::Output<String>,
        /// Name of the resource. Provided by the client when the resource is
        /// created. The name must be 1-63 characters long, and comply with
        /// RFC1035. Specifically, the name must be 1-63 characters long and match
        /// the regular expression `a-z?` which means the
        /// first character must be a lowercase letter, and all following
        /// characters must be a dash, lowercase letter, or digit, except the last
        /// character, which cannot be a dash.
        /// These are in the same namespace as the managed SSL certificates.
        pub name: pulumi_gestalt_rust::Output<String>,
        /// Creates a unique name beginning with the
        /// specified prefix. Conflicts with `name`. Max length is 54 characters.
        /// Prefixes with lengths longer than 37 characters will use a shortened
        /// UUID that will be more prone to collisions.
        /// Resulting name for a `name_prefix` <= 37 characters:
        /// `name_prefix` + YYYYmmddHHSSssss + 8 digit incremental counter
        /// Resulting name for a `name_prefix` 38 - 54 characters:
        /// `name_prefix` + YYmmdd + 3 digit incremental counter
        pub name_prefix: pulumi_gestalt_rust::Output<String>,
        /// The write-only private key in PEM format.
        /// **Note**: This property is sensitive and will not be displayed in the plan.
        ///
        ///
        /// - - -
        pub private_key: pulumi_gestalt_rust::Output<String>,
        /// The ID of the project in which the resource belongs.
        /// If it is not provided, the provider project is used.
        pub project: pulumi_gestalt_rust::Output<String>,
        /// The URI of the created resource.
        pub self_link: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::PulumiContext,
        name: &str,
        args: SSLCertificateArgs,
    ) -> SSLCertificateResult {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::register_interface;
        use std::collections::HashMap;
        let certificate_binding_1 = args.certificate.get_output(context);
        let certificate_binding = certificate_binding_1.get_inner();
        let description_binding_1 = args.description.get_output(context);
        let description_binding = description_binding_1.get_inner();
        let name_binding_1 = args.name.get_output(context);
        let name_binding = name_binding_1.get_inner();
        let name_prefix_binding_1 = args.name_prefix.get_output(context);
        let name_prefix_binding = name_prefix_binding_1.get_inner();
        let private_key_binding_1 = args.private_key.get_output(context);
        let private_key_binding = private_key_binding_1.get_inner();
        let project_binding_1 = args.project.get_output(context);
        let project_binding = project_binding_1.get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "gcp:compute/sSLCertificate:SSLCertificate".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "certificate".into(),
                    value: &certificate_binding,
                },
                register_interface::ObjectField {
                    name: "description".into(),
                    value: &description_binding,
                },
                register_interface::ObjectField {
                    name: "name".into(),
                    value: &name_binding,
                },
                register_interface::ObjectField {
                    name: "namePrefix".into(),
                    value: &name_prefix_binding,
                },
                register_interface::ObjectField {
                    name: "privateKey".into(),
                    value: &private_key_binding,
                },
                register_interface::ObjectField {
                    name: "project".into(),
                    value: &project_binding,
                },
            ]),
        };
        let o = register_interface::register(context.get_inner(), &request);
        SSLCertificateResult {
            certificate: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("certificate"),
            ),
            certificate_id: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("certificateId"),
            ),
            creation_timestamp: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("creationTimestamp"),
            ),
            description: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("description"),
            ),
            expire_time: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("expireTime"),
            ),
            name: pulumi_gestalt_rust::__private::into_domain(o.extract_field("name")),
            name_prefix: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("namePrefix"),
            ),
            private_key: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("privateKey"),
            ),
            project: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("project"),
            ),
            self_link: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("selfLink"),
            ),
        }
    }
}
