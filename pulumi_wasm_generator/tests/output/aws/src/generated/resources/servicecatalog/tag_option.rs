/// Manages a Service Catalog Tag Option.
///
/// ## Example Usage
///
/// ### Basic Usage
///
/// ```ignore
/// use pulumi_wasm_rust::Output;
/// use pulumi_wasm_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let example = tag_option::create(
///         "example",
///         TagOptionArgs::builder().key("nyckel").value("värde").build_struct(),
///     );
/// }
/// ```
///
/// ## Import
///
/// Using `pulumi import`, import `aws_servicecatalog_tag_option` using the tag option ID. For example:
///
/// ```sh
/// $ pulumi import aws:servicecatalog/tagOption:TagOption example tag-pjtvagohlyo3m
/// ```
pub mod tag_option {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct TagOptionArgs {
        /// Whether tag option is active. Default is `true`.
        #[builder(into, default)]
        pub active: pulumi_wasm_rust::Output<Option<bool>>,
        /// Tag option key.
        #[builder(into)]
        pub key: pulumi_wasm_rust::Output<String>,
        /// Tag option value.
        ///
        /// The following arguments are optional:
        #[builder(into)]
        pub value: pulumi_wasm_rust::Output<String>,
    }
    #[allow(dead_code)]
    pub struct TagOptionResult {
        /// Whether tag option is active. Default is `true`.
        pub active: pulumi_wasm_rust::Output<Option<bool>>,
        /// Tag option key.
        pub key: pulumi_wasm_rust::Output<String>,
        pub owner: pulumi_wasm_rust::Output<String>,
        /// Tag option value.
        ///
        /// The following arguments are optional:
        pub value: pulumi_wasm_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(name: &str, args: TagOptionArgs) -> TagOptionResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let active_binding = args.active.get_inner();
        let key_binding = args.key.get_inner();
        let value_binding = args.value.get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "aws:servicecatalog/tagOption:TagOption".into(),
            name: name.to_string(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "active".into(),
                    value: &active_binding,
                },
                register_interface::ObjectField {
                    name: "key".into(),
                    value: &key_binding,
                },
                register_interface::ObjectField {
                    name: "value".into(),
                    value: &value_binding,
                },
            ]),
            results: Vec::from([
                register_interface::ResultField {
                    name: "active".into(),
                },
                register_interface::ResultField {
                    name: "key".into(),
                },
                register_interface::ResultField {
                    name: "owner".into(),
                },
                register_interface::ResultField {
                    name: "value".into(),
                },
            ]),
        };
        let o = register_interface::register(&request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        TagOptionResult {
            active: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("active").unwrap(),
            ),
            key: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("key").unwrap(),
            ),
            owner: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("owner").unwrap(),
            ),
            value: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("value").unwrap(),
            ),
        }
    }
}