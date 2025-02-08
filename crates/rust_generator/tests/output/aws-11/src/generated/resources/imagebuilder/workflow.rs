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
        context: &pulumi_gestalt_rust::PulumiContext,
        name: &str,
        args: WorkflowArgs,
    ) -> WorkflowResult {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::register_interface;
        use std::collections::HashMap;
        let change_description_binding = args
            .change_description
            .get_output(context)
            .get_inner();
        let data_binding = args.data.get_output(context).get_inner();
        let description_binding = args.description.get_output(context).get_inner();
        let kms_key_id_binding = args.kms_key_id.get_output(context).get_inner();
        let name_binding = args.name.get_output(context).get_inner();
        let tags_binding = args.tags.get_output(context).get_inner();
        let type__binding = args.type_.get_output(context).get_inner();
        let uri_binding = args.uri.get_output(context).get_inner();
        let version_binding = args.version.get_output(context).get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "aws:imagebuilder/workflow:Workflow".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "changeDescription".into(),
                    value: &change_description_binding,
                },
                register_interface::ObjectField {
                    name: "data".into(),
                    value: &data_binding,
                },
                register_interface::ObjectField {
                    name: "description".into(),
                    value: &description_binding,
                },
                register_interface::ObjectField {
                    name: "kmsKeyId".into(),
                    value: &kms_key_id_binding,
                },
                register_interface::ObjectField {
                    name: "name".into(),
                    value: &name_binding,
                },
                register_interface::ObjectField {
                    name: "tags".into(),
                    value: &tags_binding,
                },
                register_interface::ObjectField {
                    name: "type".into(),
                    value: &type__binding,
                },
                register_interface::ObjectField {
                    name: "uri".into(),
                    value: &uri_binding,
                },
                register_interface::ObjectField {
                    name: "version".into(),
                    value: &version_binding,
                },
            ]),
        };
        let o = register_interface::register(context.get_inner(), &request);
        WorkflowResult {
            arn: pulumi_gestalt_rust::__private::into_domain(o.extract_field("arn")),
            change_description: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("changeDescription"),
            ),
            data: pulumi_gestalt_rust::__private::into_domain(o.extract_field("data")),
            date_created: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("dateCreated"),
            ),
            description: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("description"),
            ),
            kms_key_id: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("kmsKeyId"),
            ),
            name: pulumi_gestalt_rust::__private::into_domain(o.extract_field("name")),
            owner: pulumi_gestalt_rust::__private::into_domain(o.extract_field("owner")),
            tags: pulumi_gestalt_rust::__private::into_domain(o.extract_field("tags")),
            tags_all: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("tagsAll"),
            ),
            type_: pulumi_gestalt_rust::__private::into_domain(o.extract_field("type")),
            uri: pulumi_gestalt_rust::__private::into_domain(o.extract_field("uri")),
            version: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("version"),
            ),
        }
    }
}
