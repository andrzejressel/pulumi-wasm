/// Creates an LF-Tag with the specified name and values. Each key must have at least one value. The maximum number of values permitted is 1000.
///
/// ## Example Usage
///
/// ```ignore
/// use pulumi_wasm_rust::Output;
/// use pulumi_wasm_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let example = lf_tag::create(
///         "example",
///         LfTagArgs::builder()
///             .key("module")
///             .values(vec!["Orders", "Sales", "Customers",])
///             .build_struct(),
///     );
/// }
/// ```
///
/// ## Import
///
/// Using `pulumi import`, import Lake Formation LF-Tags using the `catalog_id:key`. If you have not set a Catalog ID specify the AWS Account ID that the database is in. For example:
///
/// ```sh
/// $ pulumi import aws:lakeformation/lfTag:LfTag example 123456789012:some_key
/// ```
pub mod lf_tag {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct LfTagArgs {
        /// ID of the Data Catalog to create the tag in. If omitted, this defaults to the AWS Account ID.
        #[builder(into, default)]
        pub catalog_id: pulumi_wasm_rust::Output<Option<String>>,
        /// Key-name for the tag.
        #[builder(into)]
        pub key: pulumi_wasm_rust::Output<String>,
        /// List of possible values an attribute can take.
        #[builder(into)]
        pub values: pulumi_wasm_rust::Output<Vec<String>>,
    }
    #[allow(dead_code)]
    pub struct LfTagResult {
        /// ID of the Data Catalog to create the tag in. If omitted, this defaults to the AWS Account ID.
        pub catalog_id: pulumi_wasm_rust::Output<String>,
        /// Key-name for the tag.
        pub key: pulumi_wasm_rust::Output<String>,
        /// List of possible values an attribute can take.
        pub values: pulumi_wasm_rust::Output<Vec<String>>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(name: &str, args: LfTagArgs) -> LfTagResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let catalog_id_binding = args.catalog_id.get_inner();
        let key_binding = args.key.get_inner();
        let values_binding = args.values.get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "aws:lakeformation/lfTag:LfTag".into(),
            name: name.to_string(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "catalogId".into(),
                    value: &catalog_id_binding,
                },
                register_interface::ObjectField {
                    name: "key".into(),
                    value: &key_binding,
                },
                register_interface::ObjectField {
                    name: "values".into(),
                    value: &values_binding,
                },
            ]),
            results: Vec::from([
                register_interface::ResultField {
                    name: "catalogId".into(),
                },
                register_interface::ResultField {
                    name: "key".into(),
                },
                register_interface::ResultField {
                    name: "values".into(),
                },
            ]),
        };
        let o = register_interface::register(&request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        LfTagResult {
            catalog_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("catalogId").unwrap(),
            ),
            key: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("key").unwrap(),
            ),
            values: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("values").unwrap(),
            ),
        }
    }
}