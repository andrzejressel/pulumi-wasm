/// Represents a PublicAdvertisedPrefix for use with bring your own IP addresses (BYOIP).
///
///
/// To get more information about PublicAdvertisedPrefix, see:
///
/// * [API documentation](https://cloud.google.com/compute/docs/reference/rest/v1/publicAdvertisedPrefixes)
/// * How-to Guides
///     * [Using bring your own IP](https://cloud.google.com/vpc/docs/using-bring-your-own-ip)
///
/// ## Example Usage
///
/// ### Public Advertised Prefixes Basic
///
///
/// ```ignore
/// use pulumi_wasm_rust::Output;
/// use pulumi_wasm_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let prefixes = public_advertised_prefix::create(
///         "prefixes",
///         PublicAdvertisedPrefixArgs::builder()
///             .description("description")
///             .dns_verification_ip("127.127.0.0")
///             .ip_cidr_range("127.127.0.0/16")
///             .name("my-prefix")
///             .build_struct(),
///     );
/// }
/// ```
///
/// ## Import
///
/// PublicAdvertisedPrefix can be imported using any of these accepted formats:
///
/// * `projects/{{project}}/global/publicAdvertisedPrefixes/{{name}}`
///
/// * `{{project}}/{{name}}`
///
/// * `{{name}}`
///
/// When using the `pulumi import` command, PublicAdvertisedPrefix can be imported using one of the formats above. For example:
///
/// ```sh
/// $ pulumi import gcp:compute/publicAdvertisedPrefix:PublicAdvertisedPrefix default projects/{{project}}/global/publicAdvertisedPrefixes/{{name}}
/// ```
///
/// ```sh
/// $ pulumi import gcp:compute/publicAdvertisedPrefix:PublicAdvertisedPrefix default {{project}}/{{name}}
/// ```
///
/// ```sh
/// $ pulumi import gcp:compute/publicAdvertisedPrefix:PublicAdvertisedPrefix default {{name}}
/// ```
///
pub mod public_advertised_prefix {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct PublicAdvertisedPrefixArgs {
        /// An optional description of this resource.
        #[builder(into, default)]
        pub description: pulumi_wasm_rust::Output<Option<String>>,
        /// The IPv4 address to be used for reverse DNS verification.
        #[builder(into)]
        pub dns_verification_ip: pulumi_wasm_rust::Output<String>,
        /// The IPv4 address range, in CIDR format, represented by this public advertised prefix.
        ///
        ///
        /// - - -
        #[builder(into)]
        pub ip_cidr_range: pulumi_wasm_rust::Output<String>,
        /// Name of the resource. The name must be 1-63 characters long, and
        /// comply with RFC1035. Specifically, the name must be 1-63 characters
        /// long and match the regular expression `a-z?`
        /// which means the first character must be a lowercase letter, and all
        /// following characters must be a dash, lowercase letter, or digit,
        /// except the last character, which cannot be a dash.
        #[builder(into, default)]
        pub name: pulumi_wasm_rust::Output<Option<String>>,
        /// The ID of the project in which the resource belongs.
        /// If it is not provided, the provider project is used.
        #[builder(into, default)]
        pub project: pulumi_wasm_rust::Output<Option<String>>,
    }
    #[allow(dead_code)]
    pub struct PublicAdvertisedPrefixResult {
        /// An optional description of this resource.
        pub description: pulumi_wasm_rust::Output<Option<String>>,
        /// The IPv4 address to be used for reverse DNS verification.
        pub dns_verification_ip: pulumi_wasm_rust::Output<String>,
        /// The IPv4 address range, in CIDR format, represented by this public advertised prefix.
        ///
        ///
        /// - - -
        pub ip_cidr_range: pulumi_wasm_rust::Output<String>,
        /// Name of the resource. The name must be 1-63 characters long, and
        /// comply with RFC1035. Specifically, the name must be 1-63 characters
        /// long and match the regular expression `a-z?`
        /// which means the first character must be a lowercase letter, and all
        /// following characters must be a dash, lowercase letter, or digit,
        /// except the last character, which cannot be a dash.
        pub name: pulumi_wasm_rust::Output<String>,
        /// The ID of the project in which the resource belongs.
        /// If it is not provided, the provider project is used.
        pub project: pulumi_wasm_rust::Output<String>,
        /// The URI of the created resource.
        pub self_link: pulumi_wasm_rust::Output<String>,
        /// Output Only. The shared secret to be used for reverse DNS verification.
        pub shared_secret: pulumi_wasm_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        name: &str,
        args: PublicAdvertisedPrefixArgs,
    ) -> PublicAdvertisedPrefixResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let description_binding = args.description.get_inner();
        let dns_verification_ip_binding = args.dns_verification_ip.get_inner();
        let ip_cidr_range_binding = args.ip_cidr_range.get_inner();
        let name_binding = args.name.get_inner();
        let project_binding = args.project.get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "gcp:compute/publicAdvertisedPrefix:PublicAdvertisedPrefix".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "description".into(),
                    value: &description_binding,
                },
                register_interface::ObjectField {
                    name: "dnsVerificationIp".into(),
                    value: &dns_verification_ip_binding,
                },
                register_interface::ObjectField {
                    name: "ipCidrRange".into(),
                    value: &ip_cidr_range_binding,
                },
                register_interface::ObjectField {
                    name: "name".into(),
                    value: &name_binding,
                },
                register_interface::ObjectField {
                    name: "project".into(),
                    value: &project_binding,
                },
            ]),
            results: Vec::from([
                register_interface::ResultField {
                    name: "description".into(),
                },
                register_interface::ResultField {
                    name: "dnsVerificationIp".into(),
                },
                register_interface::ResultField {
                    name: "ipCidrRange".into(),
                },
                register_interface::ResultField {
                    name: "name".into(),
                },
                register_interface::ResultField {
                    name: "project".into(),
                },
                register_interface::ResultField {
                    name: "selfLink".into(),
                },
                register_interface::ResultField {
                    name: "sharedSecret".into(),
                },
            ]),
        };
        let o = register_interface::register(&request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        PublicAdvertisedPrefixResult {
            description: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("description").unwrap(),
            ),
            dns_verification_ip: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("dnsVerificationIp").unwrap(),
            ),
            ip_cidr_range: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("ipCidrRange").unwrap(),
            ),
            name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("name").unwrap(),
            ),
            project: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("project").unwrap(),
            ),
            self_link: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("selfLink").unwrap(),
            ),
            shared_secret: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("sharedSecret").unwrap(),
            ),
        }
    }
}
