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
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
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
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct ServiceConfigArgs {
        /// Base64-url-safe-encoded config data
        #[builder(into)]
        pub data: pulumi_gestalt_rust::InputOrOutput<String>,
        /// User-defined name of the config
        #[builder(into, default)]
        pub name: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
    }
    #[allow(dead_code)]
    pub struct ServiceConfigResult {
        /// Base64-url-safe-encoded config data
        pub data: pulumi_gestalt_rust::Output<String>,
        /// User-defined name of the config
        pub name: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::PulumiContext,
        name: &str,
        args: ServiceConfigArgs,
    ) -> ServiceConfigResult {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::register_interface;
        use std::collections::HashMap;
        let data_binding = args.data.get_output(context).get_inner();
        let name_binding = args.name.get_output(context).get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "docker:index/serviceConfig:ServiceConfig".into(),
            name: name.to_string(),
            version: super::get_version(),
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
        };
        let o = register_interface::register(context.get_inner(), &request);
        ServiceConfigResult {
            data: pulumi_gestalt_rust::__private::into_domain(o.extract_field("data")),
            name: pulumi_gestalt_rust::__private::into_domain(o.extract_field("name")),
        }
    }
}
