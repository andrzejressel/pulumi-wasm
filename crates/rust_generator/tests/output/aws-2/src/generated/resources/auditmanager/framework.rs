/// Resource for managing an AWS Audit Manager Framework.
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
///     let test = framework::create(
///         "test",
///         FrameworkArgs::builder()
///             .control_sets(
///                 vec![
///                     FrameworkControlSet::builder()
///                     .controls(vec![FrameworkControlSetControl::builder()
///                     .id("${test1.id}").build_struct(),
///                     FrameworkControlSetControl::builder().id("${test2.id}")
///                     .build_struct(),]).name("example").build_struct(),
///                 ],
///             )
///             .name("example")
///             .build_struct(),
///     );
/// }
/// ```
///
/// ## Import
///
/// Using `pulumi import`, import Audit Manager Framework using the framework `id`. For example:
///
/// ```sh
/// $ pulumi import aws:auditmanager/framework:Framework example abc123-de45
/// ```
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod framework {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct FrameworkArgs {
        /// Compliance type that the new custom framework supports, such as `CIS` or `HIPAA`.
        #[builder(into, default)]
        pub compliance_type: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Configuration block(s) for the control sets that are associated with the framework. See `control_sets` Block below for details.
        ///
        /// The following arguments are optional:
        #[builder(into, default)]
        pub control_sets: pulumi_gestalt_rust::InputOrOutput<
            Option<Vec<super::super::types::auditmanager::FrameworkControlSet>>,
        >,
        /// Description of the framework.
        #[builder(into, default)]
        pub description: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Name of the framework.
        #[builder(into, default)]
        pub name: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// A map of tags to assign to the framework. If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
        #[builder(into, default)]
        pub tags: pulumi_gestalt_rust::InputOrOutput<
            Option<std::collections::HashMap<String, String>>,
        >,
    }
    #[allow(dead_code)]
    pub struct FrameworkResult {
        /// Amazon Resource Name (ARN) of the framework.
        /// * `control_sets[*].id` - Unique identifier for the framework control set.
        pub arn: pulumi_gestalt_rust::Output<String>,
        /// Compliance type that the new custom framework supports, such as `CIS` or `HIPAA`.
        pub compliance_type: pulumi_gestalt_rust::Output<Option<String>>,
        /// Configuration block(s) for the control sets that are associated with the framework. See `control_sets` Block below for details.
        ///
        /// The following arguments are optional:
        pub control_sets: pulumi_gestalt_rust::Output<
            Option<Vec<super::super::types::auditmanager::FrameworkControlSet>>,
        >,
        /// Description of the framework.
        pub description: pulumi_gestalt_rust::Output<Option<String>>,
        /// Framework type, such as a custom framework or a standard framework.
        pub framework_type: pulumi_gestalt_rust::Output<String>,
        /// Name of the framework.
        pub name: pulumi_gestalt_rust::Output<String>,
        /// A map of tags to assign to the framework. If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
        pub tags: pulumi_gestalt_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        pub tags_all: pulumi_gestalt_rust::Output<
            std::collections::HashMap<String, String>,
        >,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: FrameworkArgs,
    ) -> FrameworkResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let compliance_type_binding = args.compliance_type.get_output(context);
        let control_sets_binding = args.control_sets.get_output(context);
        let description_binding = args.description.get_output(context);
        let name_binding = args.name.get_output(context);
        let tags_binding = args.tags.get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "aws:auditmanager/framework:Framework".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "complianceType".into(),
                    value: compliance_type_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "controlSets".into(),
                    value: control_sets_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "description".into(),
                    value: description_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "name".into(),
                    value: name_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "tags".into(),
                    value: tags_binding.get_id(),
                },
            ],
        };
        let o = context.register_resource(request);
        FrameworkResult {
            arn: o.get_field("arn"),
            compliance_type: o.get_field("complianceType"),
            control_sets: o.get_field("controlSets"),
            description: o.get_field("description"),
            framework_type: o.get_field("frameworkType"),
            name: o.get_field("name"),
            tags: o.get_field("tags"),
            tags_all: o.get_field("tagsAll"),
        }
    }
}
