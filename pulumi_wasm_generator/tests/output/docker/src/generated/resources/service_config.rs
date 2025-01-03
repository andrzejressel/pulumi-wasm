///
///
/// ## Import
///
/// ### Example
///
/// Assuming you created a `config` as follows
///
/// ```shell
/// printf '{"a":"b"}' | docker config create foo -
/// ```
///
/// prints the id
///
/// ```text
/// 08c26c477474478d971139f750984775a7f019dbe8a2e7f09d66a187c009e66d
/// ```
///
/// you provide the definition for the resource as follows
///
/// ```ignore
/// use pulumi_wasm_rust::Output;
/// use pulumi_wasm_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let foo = service_config::create(
///         "foo",
///         ServiceConfigArgs::builder()
///             .data("base64encode(\"{\\\"a\\\": \\\"b\\\"}\")")
///             .build_struct(),
///     );
/// }
/// ```
///
/// then the import command is as follows
///
/// ```sh
/// $ pulumi import docker:index/serviceConfig:ServiceConfig foo 08c26c477474478d971139f750984775a7f019dbe8a2e7f09d66a187c009e66d
/// ```
///
pub mod service_config {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct ServiceConfigArgs {
        /// Base64-url-safe-encoded config data
        #[builder(into)]
        pub data: pulumi_wasm_rust::Output<String>,
        /// User-defined name of the config
        #[builder(into, default)]
        pub name: pulumi_wasm_rust::Output<Option<String>>,
    }
    #[allow(dead_code)]
    pub struct ServiceConfigResult {
        /// Base64-url-safe-encoded config data
        pub data: pulumi_wasm_rust::Output<String>,
        /// User-defined name of the config
        pub name: pulumi_wasm_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(name: &str, args: ServiceConfigArgs) -> ServiceConfigResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let data_binding = args.data.get_inner();
        let name_binding = args.name.get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "docker:index/serviceConfig:ServiceConfig".into(),
            name: name.to_string(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "data".into(),
                    value: &data_binding,
                },
                register_interface::ObjectField {
                    name: "name".into(),
                    value: &name_binding,
                },
            ]),
            results: Vec::from([
                register_interface::ResultField {
                    name: "data".into(),
                },
                register_interface::ResultField {
                    name: "name".into(),
                },
            ]),
        };
        let o = register_interface::register(&request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        ServiceConfigResult {
            data: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("data").unwrap(),
            ),
            name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("name").unwrap(),
            ),
        }
    }
}
