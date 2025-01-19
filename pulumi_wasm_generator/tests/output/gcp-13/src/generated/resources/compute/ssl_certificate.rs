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
pub mod ssl_certificate {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct SSLCertificateArgs {
        /// The certificate in PEM format.
        /// The certificate chain must be no greater than 5 certs long.
        /// The chain must include at least one intermediate cert.
        /// **Note**: This property is sensitive and will not be displayed in the plan.
        #[builder(into)]
        pub certificate: pulumi_wasm_rust::Output<String>,
        /// An optional description of this resource.
        #[builder(into, default)]
        pub description: pulumi_wasm_rust::Output<Option<String>>,
        /// Name of the resource. Provided by the client when the resource is
        /// created. The name must be 1-63 characters long, and comply with
        /// RFC1035. Specifically, the name must be 1-63 characters long and match
        /// the regular expression `a-z?` which means the
        /// first character must be a lowercase letter, and all following
        /// characters must be a dash, lowercase letter, or digit, except the last
        /// character, which cannot be a dash.
        /// These are in the same namespace as the managed SSL certificates.
        #[builder(into, default)]
        pub name: pulumi_wasm_rust::Output<Option<String>>,
        /// Creates a unique name beginning with the
        /// specified prefix. Conflicts with `name`. Max length is 54 characters.
        /// Prefixes with lengths longer than 37 characters will use a shortened
        /// UUID that will be more prone to collisions.
        /// Resulting name for a `name_prefix` <= 37 characters:
        /// `name_prefix` + YYYYmmddHHSSssss + 8 digit incremental counter
        /// Resulting name for a `name_prefix` 38 - 54 characters:
        /// `name_prefix` + YYmmdd + 3 digit incremental counter
        #[builder(into, default)]
        pub name_prefix: pulumi_wasm_rust::Output<Option<String>>,
        /// The write-only private key in PEM format.
        /// **Note**: This property is sensitive and will not be displayed in the plan.
        ///
        ///
        /// - - -
        #[builder(into)]
        pub private_key: pulumi_wasm_rust::Output<String>,
        /// The ID of the project in which the resource belongs.
        /// If it is not provided, the provider project is used.
        #[builder(into, default)]
        pub project: pulumi_wasm_rust::Output<Option<String>>,
    }
    #[allow(dead_code)]
    pub struct SSLCertificateResult {
        /// The certificate in PEM format.
        /// The certificate chain must be no greater than 5 certs long.
        /// The chain must include at least one intermediate cert.
        /// **Note**: This property is sensitive and will not be displayed in the plan.
        pub certificate: pulumi_wasm_rust::Output<String>,
        /// The unique identifier for the resource.
        pub certificate_id: pulumi_wasm_rust::Output<i32>,
        /// Creation timestamp in RFC3339 text format.
        pub creation_timestamp: pulumi_wasm_rust::Output<String>,
        /// An optional description of this resource.
        pub description: pulumi_wasm_rust::Output<Option<String>>,
        /// Expire time of the certificate in RFC3339 text format.
        pub expire_time: pulumi_wasm_rust::Output<String>,
        /// Name of the resource. Provided by the client when the resource is
        /// created. The name must be 1-63 characters long, and comply with
        /// RFC1035. Specifically, the name must be 1-63 characters long and match
        /// the regular expression `a-z?` which means the
        /// first character must be a lowercase letter, and all following
        /// characters must be a dash, lowercase letter, or digit, except the last
        /// character, which cannot be a dash.
        /// These are in the same namespace as the managed SSL certificates.
        pub name: pulumi_wasm_rust::Output<String>,
        /// Creates a unique name beginning with the
        /// specified prefix. Conflicts with `name`. Max length is 54 characters.
        /// Prefixes with lengths longer than 37 characters will use a shortened
        /// UUID that will be more prone to collisions.
        /// Resulting name for a `name_prefix` <= 37 characters:
        /// `name_prefix` + YYYYmmddHHSSssss + 8 digit incremental counter
        /// Resulting name for a `name_prefix` 38 - 54 characters:
        /// `name_prefix` + YYmmdd + 3 digit incremental counter
        pub name_prefix: pulumi_wasm_rust::Output<String>,
        /// The write-only private key in PEM format.
        /// **Note**: This property is sensitive and will not be displayed in the plan.
        ///
        ///
        /// - - -
        pub private_key: pulumi_wasm_rust::Output<String>,
        /// The ID of the project in which the resource belongs.
        /// If it is not provided, the provider project is used.
        pub project: pulumi_wasm_rust::Output<String>,
        /// The URI of the created resource.
        pub self_link: pulumi_wasm_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(name: &str, args: SSLCertificateArgs) -> SSLCertificateResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let certificate_binding = args.certificate.get_inner();
        let description_binding = args.description.get_inner();
        let name_binding = args.name.get_inner();
        let name_prefix_binding = args.name_prefix.get_inner();
        let private_key_binding = args.private_key.get_inner();
        let project_binding = args.project.get_inner();
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
            results: Vec::from([
                register_interface::ResultField {
                    name: "certificate".into(),
                },
                register_interface::ResultField {
                    name: "certificateId".into(),
                },
                register_interface::ResultField {
                    name: "creationTimestamp".into(),
                },
                register_interface::ResultField {
                    name: "description".into(),
                },
                register_interface::ResultField {
                    name: "expireTime".into(),
                },
                register_interface::ResultField {
                    name: "name".into(),
                },
                register_interface::ResultField {
                    name: "namePrefix".into(),
                },
                register_interface::ResultField {
                    name: "privateKey".into(),
                },
                register_interface::ResultField {
                    name: "project".into(),
                },
                register_interface::ResultField {
                    name: "selfLink".into(),
                },
            ]),
        };
        let o = register_interface::register(&request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        SSLCertificateResult {
            certificate: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("certificate").unwrap(),
            ),
            certificate_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("certificateId").unwrap(),
            ),
            creation_timestamp: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("creationTimestamp").unwrap(),
            ),
            description: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("description").unwrap(),
            ),
            expire_time: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("expireTime").unwrap(),
            ),
            name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("name").unwrap(),
            ),
            name_prefix: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("namePrefix").unwrap(),
            ),
            private_key: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("privateKey").unwrap(),
            ),
            project: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("project").unwrap(),
            ),
            self_link: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("selfLink").unwrap(),
            ),
        }
    }
}
