/// Provides a Cloudflare Access Mutual TLS Certificate resource.
/// Mutual TLS authentication ensures that the traffic is secure and
/// trusted in both directions between a client and server and can be
///  used with Access to only allows requests from devices with a
///  corresponding client certificate.
///
/// > It's required that an `account_id` or `zone_id` is provided and in
///    most cases using either is fine. However, if you're using a scoped
///    access token, you must provide the argument that matches the token's
///    scope. For example, an access token that is scoped to the "example.com"
///    zone needs to use the `zone_id` argument.
///
/// ## Example Usage
///
/// ```ignore
/// use pulumi_wasm_rust::Output;
/// use pulumi_wasm_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let myCert = access_mutual_tls_certificate::create(
///         "myCert",
///         AccessMutualTlsCertificateArgs::builder()
///             .associated_hostnames(vec!["staging.example.com",])
///             .certificate("${caPem}")
///             .name("My Root Cert")
///             .zone_id("0da42c8d2132a9ddaf714f9e7c920711")
///             .build_struct(),
///     );
/// }
/// ```
///
/// ## Import
///
/// Account level import.
///
/// ```sh
/// $ pulumi import cloudflare:index/accessMutualTlsCertificate:AccessMutualTlsCertificate example account/<account_id>/<mutual_tls_certificate_id>
/// ```
///
/// Zone level import.
///
/// ```sh
/// $ pulumi import cloudflare:index/accessMutualTlsCertificate:AccessMutualTlsCertificate example zone/<zone_id>/<mutual_tls_certificate_id>
/// ```
///
pub mod access_mutual_tls_certificate {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct AccessMutualTlsCertificateArgs {
        /// The account identifier to target for the resource. Conflicts with `zone_id`.
        #[builder(into, default)]
        pub account_id: pulumi_wasm_rust::Output<Option<String>>,
        /// The hostnames that will be prompted for this certificate.
        #[builder(into, default)]
        pub associated_hostnames: pulumi_wasm_rust::Output<Option<Vec<String>>>,
        /// The Root CA for your certificates.
        #[builder(into, default)]
        pub certificate: pulumi_wasm_rust::Output<Option<String>>,
        /// The name of the certificate.
        #[builder(into)]
        pub name: pulumi_wasm_rust::Output<String>,
        /// The zone identifier to target for the resource. Conflicts with `account_id`.
        #[builder(into, default)]
        pub zone_id: pulumi_wasm_rust::Output<Option<String>>,
    }
    #[allow(dead_code)]
    pub struct AccessMutualTlsCertificateResult {
        /// The account identifier to target for the resource. Conflicts with `zone_id`.
        pub account_id: pulumi_wasm_rust::Output<String>,
        /// The hostnames that will be prompted for this certificate.
        pub associated_hostnames: pulumi_wasm_rust::Output<Option<Vec<String>>>,
        /// The Root CA for your certificates.
        pub certificate: pulumi_wasm_rust::Output<Option<String>>,
        pub fingerprint: pulumi_wasm_rust::Output<String>,
        /// The name of the certificate.
        pub name: pulumi_wasm_rust::Output<String>,
        /// The zone identifier to target for the resource. Conflicts with `account_id`.
        pub zone_id: pulumi_wasm_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        name: &str,
        args: AccessMutualTlsCertificateArgs,
    ) -> AccessMutualTlsCertificateResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let account_id_binding = args.account_id.get_inner();
        let associated_hostnames_binding = args.associated_hostnames.get_inner();
        let certificate_binding = args.certificate.get_inner();
        let name_binding = args.name.get_inner();
        let zone_id_binding = args.zone_id.get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "cloudflare:index/accessMutualTlsCertificate:AccessMutualTlsCertificate"
                .into(),
            name: name.to_string(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "accountId".into(),
                    value: &account_id_binding,
                },
                register_interface::ObjectField {
                    name: "associatedHostnames".into(),
                    value: &associated_hostnames_binding,
                },
                register_interface::ObjectField {
                    name: "certificate".into(),
                    value: &certificate_binding,
                },
                register_interface::ObjectField {
                    name: "name".into(),
                    value: &name_binding,
                },
                register_interface::ObjectField {
                    name: "zoneId".into(),
                    value: &zone_id_binding,
                },
            ]),
            results: Vec::from([
                register_interface::ResultField {
                    name: "accountId".into(),
                },
                register_interface::ResultField {
                    name: "associatedHostnames".into(),
                },
                register_interface::ResultField {
                    name: "certificate".into(),
                },
                register_interface::ResultField {
                    name: "fingerprint".into(),
                },
                register_interface::ResultField {
                    name: "name".into(),
                },
                register_interface::ResultField {
                    name: "zoneId".into(),
                },
            ]),
        };
        let o = register_interface::register(&request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        AccessMutualTlsCertificateResult {
            account_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("accountId").unwrap(),
            ),
            associated_hostnames: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("associatedHostnames").unwrap(),
            ),
            certificate: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("certificate").unwrap(),
            ),
            fingerprint: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("fingerprint").unwrap(),
            ),
            name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("name").unwrap(),
            ),
            zone_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("zoneId").unwrap(),
            ),
        }
    }
}
