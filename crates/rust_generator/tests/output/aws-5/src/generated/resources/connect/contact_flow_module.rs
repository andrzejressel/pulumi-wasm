/// Provides an Amazon Connect Contact Flow Module resource. For more information see
/// [Amazon Connect: Getting Started](https://docs.aws.amazon.com/connect/latest/adminguide/amazon-connect-get-started.html)
///
/// This resource embeds or references Contact Flows Modules specified in Amazon Connect Contact Flow Language. For more information see
/// [Amazon Connect Flow language](https://docs.aws.amazon.com/connect/latest/adminguide/flow-language.html)
///
/// !> **WARN:** Contact Flow Modules exported from the Console [See Contact Flow import/export which is the same for Contact Flow Modules](https://docs.aws.amazon.com/connect/latest/adminguide/contact-flow-import-export.html) are not in the Amazon Connect Contact Flow Language and can not be used with this resource. Instead, the recommendation is to use the AWS CLI [`describe-contact-flow-module`](https://docs.aws.amazon.com/cli/latest/reference/connect/describe-contact-flow-module.html).
/// See example below which uses `jq` to extract the `Content` attribute and saves it to a local file.
///
/// ## Example Usage
///
/// ### Basic
///
/// ```yaml
/// resources:
///   example:
///     type: aws:connect:ContactFlowModule
///     properties:
///       instanceId: aaaaaaaa-bbbb-cccc-dddd-111111111111
///       name: Example
///       description: Example Contact Flow Module Description
///       content:
///         fn::toJSON:
///           Version: 2019-10-30
///           StartAction: 12345678-1234-1234-1234-123456789012
///           Actions:
///             - Identifier: 12345678-1234-1234-1234-123456789012
///               Parameters:
///                 Text: Hello contact flow module
///               Transitions:
///                 NextAction: abcdef-abcd-abcd-abcd-abcdefghijkl
///                 Errors: []
///                 Conditions: []
///               Type: MessageParticipant
///             - Identifier: abcdef-abcd-abcd-abcd-abcdefghijkl
///               Type: DisconnectParticipant
///               Parameters: {}
///               Transitions: {}
///           Settings:
///             InputParameters: []
///             OutputParameters: []
///             Transitions:
///               - DisplayName: Success
///                 ReferenceName: Success
///                 Description: ""
///               - DisplayName: Error
///                 ReferenceName: Error
///                 Description: ""
///       tags:
///         Name: Example Contact Flow Module
///         Application: Example
///         Method: Create
/// ```
///
/// ### With External Content
///
/// Use the AWS CLI to extract Contact Flow Content:
///
/// ```console
/// % aws connect describe-contact-flow-module --instance-id 1b3c5d8-1b3c-1b3c-1b3c-1b3c5d81b3c5 --contact-flow-module-id c1d4e5f6-1b3c-1b3c-1b3c-c1d4e5f6c1d4e5 --region us-west-2 | jq '.ContactFlowModule.Content | fromjson' > contact_flow_module.json
/// ```
///
/// Use the generated file as input:
///
/// ```yaml
/// resources:
///   example:
///     type: aws:connect:ContactFlowModule
///     properties:
///       instanceId: aaaaaaaa-bbbb-cccc-dddd-111111111111
///       name: Example
///       description: Example Contact Flow Module Description
///       filename: contact_flow_module.json
///       contentHash:
///         fn::invoke:
///           function: std:filebase64sha256
///           arguments:
///             input: contact_flow_module.json
///           return: result
///       tags:
///         Name: Example Contact Flow Module
///         Application: Example
///         Method: Create
/// ```
///
/// ## Import
///
/// Using `pulumi import`, import Amazon Connect Contact Flow Modules using the `instance_id` and `contact_flow_module_id` separated by a colon (`:`). For example:
///
/// ```sh
/// $ pulumi import aws:connect/contactFlowModule:ContactFlowModule example f1288a1f-6193-445a-b47e-af739b2:c1d4e5f6-1b3c-1b3c-1b3c-c1d4e5f6c1d4e5
/// ```
pub mod contact_flow_module {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct ContactFlowModuleArgs {
        /// Specifies the content of the Contact Flow Module, provided as a JSON string, written in Amazon Connect Contact Flow Language. If defined, the `filename` argument cannot be used.
        #[builder(into, default)]
        pub content: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Used to trigger updates. Must be set to a base64-encoded SHA256 hash of the Contact Flow Module source specified with `filename`.
        #[builder(into, default)]
        pub content_hash: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Specifies the description of the Contact Flow Module.
        #[builder(into, default)]
        pub description: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The path to the Contact Flow Module source within the local filesystem. Conflicts with `content`.
        #[builder(into, default)]
        pub filename: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Specifies the identifier of the hosting Amazon Connect Instance.
        #[builder(into)]
        pub instance_id: pulumi_gestalt_rust::InputOrOutput<String>,
        /// Specifies the name of the Contact Flow Module.
        #[builder(into, default)]
        pub name: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Tags to apply to the Contact Flow Module. If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
        #[builder(into, default)]
        pub tags: pulumi_gestalt_rust::InputOrOutput<
            Option<std::collections::HashMap<String, String>>,
        >,
    }
    #[allow(dead_code)]
    pub struct ContactFlowModuleResult {
        /// The Amazon Resource Name (ARN) of the Contact Flow Module.
        pub arn: pulumi_gestalt_rust::Output<String>,
        /// The identifier of the Contact Flow Module.
        pub contact_flow_module_id: pulumi_gestalt_rust::Output<String>,
        /// Specifies the content of the Contact Flow Module, provided as a JSON string, written in Amazon Connect Contact Flow Language. If defined, the `filename` argument cannot be used.
        pub content: pulumi_gestalt_rust::Output<String>,
        /// Used to trigger updates. Must be set to a base64-encoded SHA256 hash of the Contact Flow Module source specified with `filename`.
        pub content_hash: pulumi_gestalt_rust::Output<Option<String>>,
        /// Specifies the description of the Contact Flow Module.
        pub description: pulumi_gestalt_rust::Output<Option<String>>,
        /// The path to the Contact Flow Module source within the local filesystem. Conflicts with `content`.
        pub filename: pulumi_gestalt_rust::Output<Option<String>>,
        /// Specifies the identifier of the hosting Amazon Connect Instance.
        pub instance_id: pulumi_gestalt_rust::Output<String>,
        /// Specifies the name of the Contact Flow Module.
        pub name: pulumi_gestalt_rust::Output<String>,
        /// Tags to apply to the Contact Flow Module. If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
        pub tags: pulumi_gestalt_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// A map of tags assigned to the resource, including those inherited from the provider `default_tags` configuration block.
        pub tags_all: pulumi_gestalt_rust::Output<
            std::collections::HashMap<String, String>,
        >,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::PulumiContext,
        name: &str,
        args: ContactFlowModuleArgs,
    ) -> ContactFlowModuleResult {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::register_interface;
        use std::collections::HashMap;
        let content_binding = args.content.get_output(context).get_inner();
        let content_hash_binding = args.content_hash.get_output(context).get_inner();
        let description_binding = args.description.get_output(context).get_inner();
        let filename_binding = args.filename.get_output(context).get_inner();
        let instance_id_binding = args.instance_id.get_output(context).get_inner();
        let name_binding = args.name.get_output(context).get_inner();
        let tags_binding = args.tags.get_output(context).get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "aws:connect/contactFlowModule:ContactFlowModule".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "content".into(),
                    value: &content_binding,
                },
                register_interface::ObjectField {
                    name: "contentHash".into(),
                    value: &content_hash_binding,
                },
                register_interface::ObjectField {
                    name: "description".into(),
                    value: &description_binding,
                },
                register_interface::ObjectField {
                    name: "filename".into(),
                    value: &filename_binding,
                },
                register_interface::ObjectField {
                    name: "instanceId".into(),
                    value: &instance_id_binding,
                },
                register_interface::ObjectField {
                    name: "name".into(),
                    value: &name_binding,
                },
                register_interface::ObjectField {
                    name: "tags".into(),
                    value: &tags_binding,
                },
            ]),
        };
        let o = register_interface::register(context.get_inner(), &request);
        ContactFlowModuleResult {
            arn: pulumi_gestalt_rust::__private::into_domain(o.extract_field("arn")),
            contact_flow_module_id: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("contactFlowModuleId"),
            ),
            content: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("content"),
            ),
            content_hash: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("contentHash"),
            ),
            description: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("description"),
            ),
            filename: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("filename"),
            ),
            instance_id: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("instanceId"),
            ),
            name: pulumi_gestalt_rust::__private::into_domain(o.extract_field("name")),
            tags: pulumi_gestalt_rust::__private::into_domain(o.extract_field("tags")),
            tags_all: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("tagsAll"),
            ),
        }
    }
}
