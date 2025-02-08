#[allow(clippy::doc_lazy_continuation)]
pub mod get_server_certificate {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GetServerCertificateArgs {
        /// sort results by expiration date. returns the certificate with expiration date in furthest in the future.
        #[builder(into, default)]
        pub latest: pulumi_gestalt_rust::InputOrOutput<Option<bool>>,
        /// exact name of the cert to lookup
        #[builder(into, default)]
        pub name: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// prefix of cert to filter by
        #[builder(into, default)]
        pub name_prefix: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// prefix of path to filter by
        #[builder(into, default)]
        pub path_prefix: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
    }
    #[allow(dead_code)]
    pub struct GetServerCertificateResult {
        /// is set to the ARN of the IAM Server Certificate
        pub arn: pulumi_gestalt_rust::Output<String>,
        /// is the public key certificate (PEM-encoded). This is useful when [configuring back-end instance authentication](http://docs.aws.amazon.com/elasticloadbalancing/latest/classic/elb-create-https-ssl-load-balancer.html) policy for load balancer
        pub certificate_body: pulumi_gestalt_rust::Output<String>,
        /// is the public key certificate chain (PEM-encoded) if exists, empty otherwise
        pub certificate_chain: pulumi_gestalt_rust::Output<String>,
        /// is set to the expiration date of the IAM Server Certificate
        pub expiration_date: pulumi_gestalt_rust::Output<String>,
        /// The provider-assigned unique ID for this managed resource.
        pub id: pulumi_gestalt_rust::Output<String>,
        pub latest: pulumi_gestalt_rust::Output<Option<bool>>,
        pub name: pulumi_gestalt_rust::Output<String>,
        pub name_prefix: pulumi_gestalt_rust::Output<Option<String>>,
        /// is set to the path of the IAM Server Certificate
        pub path: pulumi_gestalt_rust::Output<String>,
        pub path_prefix: pulumi_gestalt_rust::Output<Option<String>>,
        /// is the date when the server certificate was uploaded
        pub upload_date: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn invoke(
        context: &pulumi_gestalt_rust::PulumiContext,
        args: GetServerCertificateArgs,
    ) -> GetServerCertificateResult {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::register_interface;
        use std::collections::HashMap;
        let latest_binding = args.latest.get_output(context).get_inner();
        let name_binding = args.name.get_output(context).get_inner();
        let name_prefix_binding = args.name_prefix.get_output(context).get_inner();
        let path_prefix_binding = args.path_prefix.get_output(context).get_inner();
        let request = register_interface::ResourceInvokeRequest {
            token: "aws:iam/getServerCertificate:getServerCertificate".into(),
            version: super::super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "latest".into(),
                    value: &latest_binding,
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
                    name: "pathPrefix".into(),
                    value: &path_prefix_binding,
                },
            ]),
        };
        let o = register_interface::invoke(context.get_inner(), &request);
        GetServerCertificateResult {
            arn: pulumi_gestalt_rust::__private::into_domain(o.extract_field("arn")),
            certificate_body: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("certificateBody"),
            ),
            certificate_chain: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("certificateChain"),
            ),
            expiration_date: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("expirationDate"),
            ),
            id: pulumi_gestalt_rust::__private::into_domain(o.extract_field("id")),
            latest: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("latest"),
            ),
            name: pulumi_gestalt_rust::__private::into_domain(o.extract_field("name")),
            name_prefix: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("namePrefix"),
            ),
            path: pulumi_gestalt_rust::__private::into_domain(o.extract_field("path")),
            path_prefix: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("pathPrefix"),
            ),
            upload_date: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("uploadDate"),
            ),
        }
    }
}
