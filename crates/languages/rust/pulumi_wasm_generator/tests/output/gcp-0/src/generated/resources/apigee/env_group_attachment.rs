/// An `Environment Group attachment` in Apigee.
///
///
/// To get more information about EnvgroupAttachment, see:
///
/// * [API documentation](https://cloud.google.com/apigee/docs/reference/apis/apigee/rest/v1/organizations.envgroups.attachments/create)
/// * How-to Guides
///     * [Creating an environment](https://cloud.google.com/apigee/docs/api-platform/get-started/create-environment)
///
/// ## Example Usage
///
/// ## Import
///
/// EnvgroupAttachment can be imported using any of these accepted formats:
///
/// * `{{envgroup_id}}/attachments/{{name}}`
///
/// * `{{envgroup_id}}/{{name}}`
///
/// When using the `pulumi import` command, EnvgroupAttachment can be imported using one of the formats above. For example:
///
/// ```sh
/// $ pulumi import gcp:apigee/envGroupAttachment:EnvGroupAttachment default {{envgroup_id}}/attachments/{{name}}
/// ```
///
/// ```sh
/// $ pulumi import gcp:apigee/envGroupAttachment:EnvGroupAttachment default {{envgroup_id}}/{{name}}
/// ```
///
pub mod env_group_attachment {
    #[derive(pulumi_wasm_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct EnvGroupAttachmentArgs {
        /// The Apigee environment group associated with the Apigee environment,
        /// in the format `organizations/{{org_name}}/envgroups/{{envgroup_name}}`.
        ///
        ///
        /// - - -
        #[builder(into)]
        pub envgroup_id: pulumi_wasm_rust::InputOrOutput<String>,
        /// The resource ID of the environment.
        #[builder(into)]
        pub environment: pulumi_wasm_rust::InputOrOutput<String>,
    }
    #[allow(dead_code)]
    pub struct EnvGroupAttachmentResult {
        /// The Apigee environment group associated with the Apigee environment,
        /// in the format `organizations/{{org_name}}/envgroups/{{envgroup_name}}`.
        ///
        ///
        /// - - -
        pub envgroup_id: pulumi_wasm_rust::Output<String>,
        /// The resource ID of the environment.
        pub environment: pulumi_wasm_rust::Output<String>,
        /// The name of the newly created  attachment (output parameter).
        pub name: pulumi_wasm_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_wasm_rust::PulumiContext,
        name: &str,
        args: EnvGroupAttachmentArgs,
    ) -> EnvGroupAttachmentResult {
        use pulumi_wasm_rust::__private::pulumi_gestalt_adapter_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let envgroup_id_binding = args.envgroup_id.get_output(context).get_inner();
        let environment_binding = args.environment.get_output(context).get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "gcp:apigee/envGroupAttachment:EnvGroupAttachment".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "envgroupId".into(),
                    value: &envgroup_id_binding,
                },
                register_interface::ObjectField {
                    name: "environment".into(),
                    value: &environment_binding,
                },
            ]),
        };
        let o = register_interface::register(context.get_inner(), &request);
        EnvGroupAttachmentResult {
            envgroup_id: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("envgroupId"),
            ),
            environment: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("environment"),
            ),
            name: pulumi_wasm_rust::__private::into_domain(o.extract_field("name")),
        }
    }
}
