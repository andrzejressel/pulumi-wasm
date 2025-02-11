/// Resource for managing an AWS EC2 Image Builder Workflow.
///
/// > Image Builder manages the workflows for the distribution stage. Therefore, using the DISTRIBUTION workflow type results in an error.
///
/// ## Example Usage
///
/// ### Basic Usage
///
/// ```ignore
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let example = workflow::create(
///         "example",
///         WorkflowArgs::builder()
///             .data(
///                 "name: example\ndescription: Workflow to test an image\nschemaVersion: 1.0\n\nparameters:\n  - name: waitForActionAtEnd\n    type: boolean\n\nsteps:\n  - name: LaunchTestInstance\n    action: LaunchInstance\n    onFailure: Abort\n    inputs:\n      waitFor: \"ssmAgent\"\n\n  - name: TerminateTestInstance\n    action: TerminateInstance\n    onFailure: Continue\n    inputs:\n      instanceId.$: \"$.stepOutputs.LaunchTestInstance.instanceId\"\n\n  - name: WaitForActionAtEnd\n    action: WaitForAction\n    if:\n      booleanEquals: true\n      value: \"$.parameters.waitForActionAtEnd\"",
///             )
///             .name("example")
///             .type_("TEST")
///             .version("1.0.0")
///             .build_struct(),
///     );
/// }
/// ```
///
/// ## Import
///
/// Using `pulumi import`, import EC2 Image Builder Workflow using the `example_id_arg`. For example:
///
/// ```sh
/// $ pulumi import aws:imagebuilder/workflow:Workflow example arn:aws:imagebuilder:us-east-1:aws:workflow/test/example/1.0.1/1
/// ```
/// Certain resource arguments, such as `uri`, cannot be read via the API and imported into Terraform. Terraform will display a difference for these arguments the first run after import if declared in the Terraform configuration for an imported resource.
///
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod workflow {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct WorkflowArgs {
        /// Change description of the workflow.
        #[builder(into, default)]
        pub change_description: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Inline YAML string with data of the workflow. Exactly one of `data` and `uri` can be specified.
        #[builder(into, default)]
        pub data: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Description of the workflow.
        #[builder(into, default)]
        pub description: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Amazon Resource Name (ARN) of the Key Management Service (KMS) Key used to encrypt the workflow.
        #[builder(into, default)]
        pub kms_key_id: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Name of the workflow.
        #[builder(into, default)]
        pub name: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Key-value map of resource tags for the workflow. If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
        #[builder(into, default)]
        pub tags: pulumi_gestalt_rust::InputOrOutput<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// Type of the workflow. Valid values: `BUILD`, `TEST`.
        #[builder(into)]
        pub type_: pulumi_gestalt_rust::InputOrOutput<String>,
        /// S3 URI with data of the workflow. Exactly one of `data` and `uri` can be specified.
        #[builder(into, default)]
        pub uri: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Version of the workflow.
        ///
        /// The following arguments are optional:
        #[builder(into)]
        pub version: pulumi_gestalt_rust::InputOrOutput<String>,
    }
    #[allow(dead_code)]
    pub struct WorkflowResult {
        /// Amazon Resource Name (ARN) of the workflow.
        pub arn: pulumi_gestalt_rust::Output<String>,
        /// Change description of the workflow.
        pub change_description: pulumi_gestalt_rust::Output<Option<String>>,
        /// Inline YAML string with data of the workflow. Exactly one of `data` and `uri` can be specified.
        pub data: pulumi_gestalt_rust::Output<String>,
        /// Date the workflow was created.
        pub date_created: pulumi_gestalt_rust::Output<String>,
        /// Description of the workflow.
        pub description: pulumi_gestalt_rust::Output<Option<String>>,
        /// Amazon Resource Name (ARN) of the Key Management Service (KMS) Key used to encrypt the workflow.
        pub kms_key_id: pulumi_gestalt_rust::Output<Option<String>>,
        /// Name of the workflow.
        pub name: pulumi_gestalt_rust::Output<String>,
        /// Owner of the workflow.
        pub owner: pulumi_gestalt_rust::Output<String>,
        /// Key-value map of resource tags for the workflow. If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
        pub tags: pulumi_gestalt_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        pub tags_all: pulumi_gestalt_rust::Output<
            std::collections::HashMap<String, String>,
        >,
        /// Type of the workflow. Valid values: `BUILD`, `TEST`.
        pub type_: pulumi_gestalt_rust::Output<String>,
        /// S3 URI with data of the workflow. Exactly one of `data` and `uri` can be specified.
        pub uri: pulumi_gestalt_rust::Output<Option<String>>,
        /// Version of the workflow.
        ///
        /// The following arguments are optional:
        pub version: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: WorkflowArgs,
    ) -> WorkflowResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let change_description_binding = args.change_description.get_output(context);
        let data_binding = args.data.get_output(context);
        let description_binding = args.description.get_output(context);
        let kms_key_id_binding = args.kms_key_id.get_output(context);
        let name_binding = args.name.get_output(context);
        let tags_binding = args.tags.get_output(context);
        let type__binding = args.type_.get_output(context);
        let uri_binding = args.uri.get_output(context);
        let version_binding = args.version.get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "aws:imagebuilder/workflow:Workflow".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "changeDescription".into(),
                    value: &change_description_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "data".into(),
                    value: &data_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "description".into(),
                    value: &description_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "kmsKeyId".into(),
                    value: &kms_key_id_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "name".into(),
                    value: &name_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "tags".into(),
                    value: &tags_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "type".into(),
                    value: &type__binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "uri".into(),
                    value: &uri_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "version".into(),
                    value: &version_binding.drop_type(),
                },
            ],
        };
        let o = context.register_resource(request);
        WorkflowResult {
            arn: o.get_field("arn"),
            change_description: o.get_field("changeDescription"),
            data: o.get_field("data"),
            date_created: o.get_field("dateCreated"),
            description: o.get_field("description"),
            kms_key_id: o.get_field("kmsKeyId"),
            name: o.get_field("name"),
            owner: o.get_field("owner"),
            tags: o.get_field("tags"),
            tags_all: o.get_field("tagsAll"),
            type_: o.get_field("type"),
            uri: o.get_field("uri"),
            version: o.get_field("version"),
        }
    }
}
