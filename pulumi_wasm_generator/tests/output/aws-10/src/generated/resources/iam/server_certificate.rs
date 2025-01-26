/// Provides an IAM Server Certificate resource to upload Server Certificates.
/// Certs uploaded to IAM can easily work with other AWS services such as:
///
/// - AWS Elastic Beanstalk
/// - Elastic Load Balancing
/// - CloudFront
/// - AWS OpsWorks
///
/// For information about server certificates in IAM, see [Managing Server
/// Certificates][2] in AWS Documentation.
///
/// ## Example Usage
///
/// **Using certs on file:**
///
/// ```yaml
/// resources:
///   testCert:
///     type: aws:iam:ServerCertificate
///     name: test_cert
///     properties:
///       name: some_test_cert
///       certificateBody:
///         fn::invoke:
///           function: std:file
///           arguments:
///             input: self-ca-cert.pem
///           return: result
///       privateKey:
///         fn::invoke:
///           function: std:file
///           arguments:
///             input: test-key.pem
///           return: result
/// ```
///
/// **Example with cert in-line:**
///
/// ```ignore
/// use pulumi_wasm_rust::Output;
/// use pulumi_wasm_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let testCertAlt = server_certificate::create(
///         "testCertAlt",
///         ServerCertificateArgs::builder()
///             .certificate_body(
///                 "-----BEGIN CERTIFICATE-----\n[......] # cert contents\n-----END CERTIFICATE-----\n",
///             )
///             .name("alt_test_cert")
///             .private_key(
///                 "-----BEGIN RSA PRIVATE KEY-----\n[......] # cert contents\n-----END RSA PRIVATE KEY-----",
///             )
///             .build_struct(),
///     );
/// }
/// ```
///
/// **Use in combination with an AWS ELB resource:**
///
/// Some properties of an IAM Server Certificates cannot be updated while they are
/// in use. In order for the provider to effectively manage a Certificate in this situation, it is
/// recommended you utilize the `name_prefix` attribute and enable the
/// `create_before_destroy`. This will allow this provider
/// to create a new, updated `aws.iam.ServerCertificate` resource and replace it in
/// dependant resources before attempting to destroy the old version.
///
/// ## Import
///
/// Using `pulumi import`, import IAM Server Certificates using the `name`. For example:
///
/// ```sh
/// $ pulumi import aws:iam/serverCertificate:ServerCertificate certificate example.com-certificate-until-2018
/// ```
pub mod server_certificate {
    #[derive(pulumi_wasm_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct ServerCertificateArgs {
        /// The contents of the public key certificate in
        /// PEM-encoded format.
        #[builder(into)]
        pub certificate_body: pulumi_wasm_rust::InputOrOutput<String>,
        /// The contents of the certificate chain.
        /// This is typically a concatenation of the PEM-encoded public key certificates
        /// of the chain.
        #[builder(into, default)]
        pub certificate_chain: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        /// The name of the Server Certificate. Do not include the
        /// path in this value. If omitted, the provider will assign a random, unique name.
        #[builder(into, default)]
        pub name: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        /// Creates a unique name beginning with the specified
        /// prefix. Conflicts with `name`.
        #[builder(into, default)]
        pub name_prefix: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        /// The IAM path for the server certificate.  If it is not
        /// included, it defaults to a slash (/). If this certificate is for use with
        /// AWS CloudFront, the path must be in format `/cloudfront/your_path_here`.
        /// See [IAM Identifiers](https://docs.aws.amazon.com/IAM/latest/UserGuide/Using_Identifiers.html) for more details on IAM Paths.
        #[builder(into, default)]
        pub path: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        /// The contents of the private key in PEM-encoded format.
        #[builder(into)]
        pub private_key: pulumi_wasm_rust::InputOrOutput<String>,
        /// Map of resource tags for the server certificate. If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
        ///
        /// > **NOTE:** AWS performs behind-the-scenes modifications to some certificate files if they do not adhere to a specific format. These modifications will result in this provider forever believing that it needs to update the resources since the local and AWS file contents will not match after theses modifications occur. In order to prevent this from happening you must ensure that all your PEM-encoded files use UNIX line-breaks and that `certificate_body` contains only one certificate. All other certificates should go in `certificate_chain`. It is common for some Certificate Authorities to issue certificate files that have DOS line-breaks and that are actually multiple certificates concatenated together in order to form a full certificate chain.
        #[builder(into, default)]
        pub tags: pulumi_wasm_rust::InputOrOutput<
            Option<std::collections::HashMap<String, String>>,
        >,
    }
    #[allow(dead_code)]
    pub struct ServerCertificateResult {
        /// The Amazon Resource Name (ARN) specifying the server certificate.
        pub arn: pulumi_wasm_rust::Output<String>,
        /// The contents of the public key certificate in
        /// PEM-encoded format.
        pub certificate_body: pulumi_wasm_rust::Output<String>,
        /// The contents of the certificate chain.
        /// This is typically a concatenation of the PEM-encoded public key certificates
        /// of the chain.
        pub certificate_chain: pulumi_wasm_rust::Output<Option<String>>,
        /// Date and time in [RFC3339 format](https://tools.ietf.org/html/rfc3339#section-5.8) on which the certificate is set to expire.
        pub expiration: pulumi_wasm_rust::Output<String>,
        /// The name of the Server Certificate. Do not include the
        /// path in this value. If omitted, the provider will assign a random, unique name.
        pub name: pulumi_wasm_rust::Output<String>,
        /// Creates a unique name beginning with the specified
        /// prefix. Conflicts with `name`.
        pub name_prefix: pulumi_wasm_rust::Output<String>,
        /// The IAM path for the server certificate.  If it is not
        /// included, it defaults to a slash (/). If this certificate is for use with
        /// AWS CloudFront, the path must be in format `/cloudfront/your_path_here`.
        /// See [IAM Identifiers](https://docs.aws.amazon.com/IAM/latest/UserGuide/Using_Identifiers.html) for more details on IAM Paths.
        pub path: pulumi_wasm_rust::Output<Option<String>>,
        /// The contents of the private key in PEM-encoded format.
        pub private_key: pulumi_wasm_rust::Output<String>,
        /// Map of resource tags for the server certificate. If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
        ///
        /// > **NOTE:** AWS performs behind-the-scenes modifications to some certificate files if they do not adhere to a specific format. These modifications will result in this provider forever believing that it needs to update the resources since the local and AWS file contents will not match after theses modifications occur. In order to prevent this from happening you must ensure that all your PEM-encoded files use UNIX line-breaks and that `certificate_body` contains only one certificate. All other certificates should go in `certificate_chain`. It is common for some Certificate Authorities to issue certificate files that have DOS line-breaks and that are actually multiple certificates concatenated together in order to form a full certificate chain.
        pub tags: pulumi_wasm_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// A map of tags assigned to the resource, including those inherited from the provider `default_tags` configuration block.
        pub tags_all: pulumi_wasm_rust::Output<
            std::collections::HashMap<String, String>,
        >,
        /// Date and time in [RFC3339 format](https://tools.ietf.org/html/rfc3339#section-5.8) when the server certificate was uploaded.
        pub upload_date: pulumi_wasm_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_wasm_rust::PulumiContext,
        name: &str,
        args: ServerCertificateArgs,
    ) -> ServerCertificateResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let certificate_body_binding = args
            .certificate_body
            .get_output(context)
            .get_inner();
        let certificate_chain_binding = args
            .certificate_chain
            .get_output(context)
            .get_inner();
        let name_binding = args.name.get_output(context).get_inner();
        let name_prefix_binding = args.name_prefix.get_output(context).get_inner();
        let path_binding = args.path.get_output(context).get_inner();
        let private_key_binding = args.private_key.get_output(context).get_inner();
        let tags_binding = args.tags.get_output(context).get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "aws:iam/serverCertificate:ServerCertificate".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "certificateBody".into(),
                    value: &certificate_body_binding,
                },
                register_interface::ObjectField {
                    name: "certificateChain".into(),
                    value: &certificate_chain_binding,
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
                    name: "path".into(),
                    value: &path_binding,
                },
                register_interface::ObjectField {
                    name: "privateKey".into(),
                    value: &private_key_binding,
                },
                register_interface::ObjectField {
                    name: "tags".into(),
                    value: &tags_binding,
                },
            ]),
        };
        let o = register_interface::register(context.get_inner(), &request);
        ServerCertificateResult {
            arn: pulumi_wasm_rust::__private::into_domain(o.extract_field("arn")),
            certificate_body: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("certificateBody"),
            ),
            certificate_chain: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("certificateChain"),
            ),
            expiration: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("expiration"),
            ),
            name: pulumi_wasm_rust::__private::into_domain(o.extract_field("name")),
            name_prefix: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("namePrefix"),
            ),
            path: pulumi_wasm_rust::__private::into_domain(o.extract_field("path")),
            private_key: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("privateKey"),
            ),
            tags: pulumi_wasm_rust::__private::into_domain(o.extract_field("tags")),
            tags_all: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("tagsAll"),
            ),
            upload_date: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("uploadDate"),
            ),
        }
    }
}
