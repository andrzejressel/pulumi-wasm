/// Provides a [Route53 Delegation Set](https://docs.aws.amazon.com/Route53/latest/APIReference/API-actions-by-function.html#actions-by-function-reusable-delegation-sets) resource.
///
/// ## Example Usage
///
/// ```ignore
/// use pulumi_wasm_rust::Output;
/// use pulumi_wasm_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let main = delegation_set::create(
///         "main",
///         DelegationSetArgs::builder().reference_name("DynDNS").build_struct(),
///     );
///     let primary = zone::create(
///         "primary",
///         ZoneArgs::builder()
///             .delegation_set_id("${main.id}")
///             .name("mydomain.com")
///             .build_struct(),
///     );
///     let secondary = zone::create(
///         "secondary",
///         ZoneArgs::builder()
///             .delegation_set_id("${main.id}")
///             .name("coolcompany.io")
///             .build_struct(),
///     );
/// }
/// ```
///
/// ## Import
///
/// Using `pulumi import`, import Route53 Delegation Sets using the delegation set `id`. For example:
///
/// ```sh
/// $ pulumi import aws:route53/delegationSet:DelegationSet set1 N1PA6795SAMPLE
/// ```
pub mod delegation_set {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct DelegationSetArgs {
        /// This is a reference name used in Caller Reference
        /// (helpful for identifying single delegation set amongst others)
        #[builder(into, default)]
        pub reference_name: pulumi_wasm_rust::Output<Option<String>>,
    }
    #[allow(dead_code)]
    pub struct DelegationSetResult {
        /// The Amazon Resource Name (ARN) of the Delegation Set.
        pub arn: pulumi_wasm_rust::Output<String>,
        /// A list of authoritative name servers for the hosted zone
        /// (effectively a list of NS records).
        pub name_servers: pulumi_wasm_rust::Output<Vec<String>>,
        /// This is a reference name used in Caller Reference
        /// (helpful for identifying single delegation set amongst others)
        pub reference_name: pulumi_wasm_rust::Output<Option<String>>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(name: &str, args: DelegationSetArgs) -> DelegationSetResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let reference_name_binding = args.reference_name.get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "aws:route53/delegationSet:DelegationSet".into(),
            name: name.to_string(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "referenceName".into(),
                    value: &reference_name_binding,
                },
            ]),
            results: Vec::from([
                register_interface::ResultField {
                    name: "arn".into(),
                },
                register_interface::ResultField {
                    name: "nameServers".into(),
                },
                register_interface::ResultField {
                    name: "referenceName".into(),
                },
            ]),
        };
        let o = register_interface::register(&request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        DelegationSetResult {
            arn: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("arn").unwrap(),
            ),
            name_servers: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("nameServers").unwrap(),
            ),
            reference_name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("referenceName").unwrap(),
            ),
        }
    }
}