/// Creates a domain entry resource
///
/// > **NOTE on `id`:** In an effort to simplify imports, this resource `id` field has been updated to the standard resource id separator, a comma (`,`). For backward compatibility, the previous separator (underscore `_`) can still be used to read and import existing resources. When state is refreshed, the `id` will be updated to use the new standard separator. The previous separator will be deprecated in a future major release.
///
/// ## Example Usage
///
/// ```ignore
/// use pulumi_wasm_rust::Output;
/// use pulumi_wasm_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let test = domain::create(
///         "test",
///         DomainArgs::builder().domain_name("mydomain.com").build_struct(),
///     );
///     let testDomainEntry = domain_entry::create(
///         "testDomainEntry",
///         DomainEntryArgs::builder()
///             .domain_name("${domainTest.domainName}")
///             .name("www")
///             .target("127.0.0.1")
///             .type_("A")
///             .build_struct(),
///     );
/// }
/// ```
///
/// ## Import
///
/// Using `pulumi import`, import `aws_lightsail_domain_entry` using the id attribute. For example:
///
/// ```sh
/// $ pulumi import aws:lightsail/domainEntry:DomainEntry example www,mydomain.com,A,127.0.0.1
/// ```
pub mod domain_entry {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct DomainEntryArgs {
        /// The name of the Lightsail domain in which to create the entry
        #[builder(into)]
        pub domain_name: pulumi_wasm_rust::Output<String>,
        /// If the entry should be an alias Defaults to `false`
        #[builder(into, default)]
        pub is_alias: pulumi_wasm_rust::Output<Option<bool>>,
        /// Name of the entry record
        #[builder(into, default)]
        pub name: pulumi_wasm_rust::Output<Option<String>>,
        /// Target of the domain entry
        #[builder(into)]
        pub target: pulumi_wasm_rust::Output<String>,
        /// Type of record
        #[builder(into)]
        pub type_: pulumi_wasm_rust::Output<String>,
    }
    #[allow(dead_code)]
    pub struct DomainEntryResult {
        /// The name of the Lightsail domain in which to create the entry
        pub domain_name: pulumi_wasm_rust::Output<String>,
        /// If the entry should be an alias Defaults to `false`
        pub is_alias: pulumi_wasm_rust::Output<Option<bool>>,
        /// Name of the entry record
        pub name: pulumi_wasm_rust::Output<String>,
        /// Target of the domain entry
        pub target: pulumi_wasm_rust::Output<String>,
        /// Type of record
        pub type_: pulumi_wasm_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(name: &str, args: DomainEntryArgs) -> DomainEntryResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let domain_name_binding = args.domain_name.get_inner();
        let is_alias_binding = args.is_alias.get_inner();
        let name_binding = args.name.get_inner();
        let target_binding = args.target.get_inner();
        let type__binding = args.type_.get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "aws:lightsail/domainEntry:DomainEntry".into(),
            name: name.to_string(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "domainName".into(),
                    value: &domain_name_binding,
                },
                register_interface::ObjectField {
                    name: "isAlias".into(),
                    value: &is_alias_binding,
                },
                register_interface::ObjectField {
                    name: "name".into(),
                    value: &name_binding,
                },
                register_interface::ObjectField {
                    name: "target".into(),
                    value: &target_binding,
                },
                register_interface::ObjectField {
                    name: "type".into(),
                    value: &type__binding,
                },
            ]),
            results: Vec::from([
                register_interface::ResultField {
                    name: "domainName".into(),
                },
                register_interface::ResultField {
                    name: "isAlias".into(),
                },
                register_interface::ResultField {
                    name: "name".into(),
                },
                register_interface::ResultField {
                    name: "target".into(),
                },
                register_interface::ResultField {
                    name: "type".into(),
                },
            ]),
        };
        let o = register_interface::register(&request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        DomainEntryResult {
            domain_name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("domainName").unwrap(),
            ),
            is_alias: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("isAlias").unwrap(),
            ),
            name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("name").unwrap(),
            ),
            target: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("target").unwrap(),
            ),
            type_: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("type").unwrap(),
            ),
        }
    }
}