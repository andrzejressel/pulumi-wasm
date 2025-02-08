/// ## Example Usage
///
/// Example creating a RuntimeConfig variable.
///
/// ```yaml
/// resources:
///   my-runtime-config:
///     type: gcp:runtimeconfig:Config
///     properties:
///       name: my-service-runtime-config
///       description: Runtime configuration values for my service
///   environment:
///     type: gcp:runtimeconfig:Variable
///     properties:
///       parent: ${["my-runtime-config"].name}
///       name: prod-variables/hostname
///       text: example.com
/// ```
///
/// You can also encode binary content using the `value` argument instead. The
/// value must be base64 encoded.
///
/// Example of using the `value` argument.
///
/// ```yaml
/// resources:
///   my-runtime-config:
///     type: gcp:runtimeconfig:Config
///     properties:
///       name: my-service-runtime-config
///       description: Runtime configuration values for my service
///   my-secret:
///     type: gcp:runtimeconfig:Variable
///     properties:
///       parent: ${["my-runtime-config"].name}
///       name: secret
///       value:
///         fn::invoke:
///           function: std:filebase64
///           arguments:
///             input: my-encrypted-secret.dat
///           return: result
/// ```
///
/// ## Import
///
/// Runtime Config Variables can be imported using the `name` or full variable name, e.g.
///
/// * `projects/my-gcp-project/configs/{{config_id}}/variables/{{name}}`
///
/// * `{{config_id}}/{{name}}`
///
/// When using the `pulumi import` command, Runtime Config Variables can be imported using one of the formats above. For example:
///
/// ```sh
/// $ pulumi import gcp:runtimeconfig/variable:Variable default projects/my-gcp-project/configs/{{config_id}}/variables/{{name}}
/// ```
///
/// ```sh
/// $ pulumi import gcp:runtimeconfig/variable:Variable default {{config_id}}/{{name}}
/// ```
///
/// When importing using only the name, the provider project must be set.
///
#[allow(clippy::doc_lazy_continuation)]
pub mod variable {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct VariableArgs {
        /// The name of the variable to manage. Note that variable
        /// names can be hierarchical using slashes (e.g. "prod-variables/hostname").
        #[builder(into, default)]
        pub name: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The name of the RuntimeConfig resource containing this
        /// variable.
        #[builder(into)]
        pub parent: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The ID of the project in which the resource belongs. If it
        /// is not provided, the provider project is used.
        #[builder(into, default)]
        pub project: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// or `value` - (Required) The content to associate with the variable.
        /// Exactly one of `text` or `variable` must be specified. If `text` is specified,
        /// it must be a valid UTF-8 string and less than 4096 bytes in length. If `value`
        /// is specified, it must be base64 encoded and less than 4096 bytes in length.
        ///
        /// - - -
        #[builder(into, default)]
        pub text: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        #[builder(into, default)]
        pub value: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
    }
    #[allow(dead_code)]
    pub struct VariableResult {
        /// The name of the variable to manage. Note that variable
        /// names can be hierarchical using slashes (e.g. "prod-variables/hostname").
        pub name: pulumi_gestalt_rust::Output<String>,
        /// The name of the RuntimeConfig resource containing this
        /// variable.
        pub parent: pulumi_gestalt_rust::Output<String>,
        /// The ID of the project in which the resource belongs. If it
        /// is not provided, the provider project is used.
        pub project: pulumi_gestalt_rust::Output<String>,
        /// or `value` - (Required) The content to associate with the variable.
        /// Exactly one of `text` or `variable` must be specified. If `text` is specified,
        /// it must be a valid UTF-8 string and less than 4096 bytes in length. If `value`
        /// is specified, it must be base64 encoded and less than 4096 bytes in length.
        ///
        /// - - -
        pub text: pulumi_gestalt_rust::Output<Option<String>>,
        /// (Computed) The timestamp in RFC3339 UTC "Zulu" format,
        /// accurate to nanoseconds, representing when the variable was last updated.
        /// Example: "2016-10-09T12:33:37.578138407Z".
        pub update_time: pulumi_gestalt_rust::Output<String>,
        pub value: pulumi_gestalt_rust::Output<Option<String>>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::PulumiContext,
        name: &str,
        args: VariableArgs,
    ) -> VariableResult {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::register_interface;
        use std::collections::HashMap;
        let name_binding = args.name.get_output(context).get_inner();
        let parent_binding = args.parent.get_output(context).get_inner();
        let project_binding = args.project.get_output(context).get_inner();
        let text_binding = args.text.get_output(context).get_inner();
        let value_binding = args.value.get_output(context).get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "gcp:runtimeconfig/variable:Variable".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "name".into(),
                    value: &name_binding,
                },
                register_interface::ObjectField {
                    name: "parent".into(),
                    value: &parent_binding,
                },
                register_interface::ObjectField {
                    name: "project".into(),
                    value: &project_binding,
                },
                register_interface::ObjectField {
                    name: "text".into(),
                    value: &text_binding,
                },
                register_interface::ObjectField {
                    name: "value".into(),
                    value: &value_binding,
                },
            ]),
        };
        let o = register_interface::register(context.get_inner(), &request);
        VariableResult {
            name: pulumi_gestalt_rust::__private::into_domain(o.extract_field("name")),
            parent: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("parent"),
            ),
            project: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("project"),
            ),
            text: pulumi_gestalt_rust::__private::into_domain(o.extract_field("text")),
            update_time: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("updateTime"),
            ),
            value: pulumi_gestalt_rust::__private::into_domain(o.extract_field("value")),
        }
    }
}
