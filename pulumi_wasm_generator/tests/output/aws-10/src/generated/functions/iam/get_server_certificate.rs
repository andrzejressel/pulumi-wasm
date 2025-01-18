pub mod get_server_certificate {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GetServerCertificateArgs {
        /// sort results by expiration date. returns the certificate with expiration date in furthest in the future.
        #[builder(into, default)]
        pub latest: pulumi_wasm_rust::Output<Option<bool>>,
        /// exact name of the cert to lookup
        #[builder(into, default)]
        pub name: pulumi_wasm_rust::Output<Option<String>>,
        /// prefix of cert to filter by
        #[builder(into, default)]
        pub name_prefix: pulumi_wasm_rust::Output<Option<String>>,
        /// prefix of path to filter by
        #[builder(into, default)]
        pub path_prefix: pulumi_wasm_rust::Output<Option<String>>,
    }
    #[allow(dead_code)]
    pub struct GetServerCertificateResult {
        /// is set to the ARN of the IAM Server Certificate
        pub arn: pulumi_wasm_rust::Output<String>,
        /// is the public key certificate (PEM-encoded). This is useful when [configuring back-end instance authentication](http://docs.aws.amazon.com/elasticloadbalancing/latest/classic/elb-create-https-ssl-load-balancer.html) policy for load balancer
        pub certificate_body: pulumi_wasm_rust::Output<String>,
        /// is the public key certificate chain (PEM-encoded) if exists, empty otherwise
        pub certificate_chain: pulumi_wasm_rust::Output<String>,
        /// is set to the expiration date of the IAM Server Certificate
        pub expiration_date: pulumi_wasm_rust::Output<String>,
        /// The provider-assigned unique ID for this managed resource.
        pub id: pulumi_wasm_rust::Output<String>,
        pub latest: pulumi_wasm_rust::Output<Option<bool>>,
        pub name: pulumi_wasm_rust::Output<String>,
        pub name_prefix: pulumi_wasm_rust::Output<Option<String>>,
        /// is set to the path of the IAM Server Certificate
        pub path: pulumi_wasm_rust::Output<String>,
        pub path_prefix: pulumi_wasm_rust::Output<Option<String>>,
        /// is the date when the server certificate was uploaded
        pub upload_date: pulumi_wasm_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn invoke(args: GetServerCertificateArgs) -> GetServerCertificateResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let latest_binding = args.latest.get_inner();
        let name_binding = args.name.get_inner();
        let name_prefix_binding = args.name_prefix.get_inner();
        let path_prefix_binding = args.path_prefix.get_inner();
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
            results: Vec::from([
                register_interface::ResultField {
                    name: "arn".into(),
                },
                register_interface::ResultField {
                    name: "certificateBody".into(),
                },
                register_interface::ResultField {
                    name: "certificateChain".into(),
                },
                register_interface::ResultField {
                    name: "expirationDate".into(),
                },
                register_interface::ResultField {
                    name: "id".into(),
                },
                register_interface::ResultField {
                    name: "latest".into(),
                },
                register_interface::ResultField {
                    name: "name".into(),
                },
                register_interface::ResultField {
                    name: "namePrefix".into(),
                },
                register_interface::ResultField {
                    name: "path".into(),
                },
                register_interface::ResultField {
                    name: "pathPrefix".into(),
                },
                register_interface::ResultField {
                    name: "uploadDate".into(),
                },
            ]),
        };
        let o = register_interface::invoke(&request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        GetServerCertificateResult {
            arn: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("arn").unwrap(),
            ),
            certificate_body: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("certificateBody").unwrap(),
            ),
            certificate_chain: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("certificateChain").unwrap(),
            ),
            expiration_date: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("expirationDate").unwrap(),
            ),
            id: pulumi_wasm_rust::__private::into_domain(hashmap.remove("id").unwrap()),
            latest: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("latest").unwrap(),
            ),
            name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("name").unwrap(),
            ),
            name_prefix: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("namePrefix").unwrap(),
            ),
            path: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("path").unwrap(),
            ),
            path_prefix: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("pathPrefix").unwrap(),
            ),
            upload_date: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("uploadDate").unwrap(),
            ),
        }
    }
}
