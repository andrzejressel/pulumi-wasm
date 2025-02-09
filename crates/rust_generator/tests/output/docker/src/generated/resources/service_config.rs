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
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
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
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: ServiceConfigArgs,
    ) -> ServiceConfigResult {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::register_interface;
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let data_binding = args.data.get_output(context);
        let name_binding = args.name.get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "docker:index/serviceConfig:ServiceConfig".into(),
            name: name.to_string(),
            version: super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "data".into(),
                    value: data_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "name".into(),
                    value: name_binding.get_id(),
                },
            ],
        };
        let o = context.register_resource(request);
        ServiceConfigResult {
            data: o.get_field("data"),
            name: o.get_field("name"),
        }
    }
}
