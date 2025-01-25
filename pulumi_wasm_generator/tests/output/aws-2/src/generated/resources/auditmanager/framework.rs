/// Resource for managing an AWS Audit Manager Framework.
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
pub mod framework {
    #[derive(pulumi_wasm_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct FrameworkArgs {
        /// Compliance type that the new custom framework supports, such as `CIS` or `HIPAA`.
        #[builder(into, default)]
        pub compliance_type: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        /// Configuration block(s) for the control sets that are associated with the framework. See `control_sets` Block below for details.
        ///
        /// The following arguments are optional:
        #[builder(into, default)]
        pub control_sets: pulumi_wasm_rust::InputOrOutput<
            Option<Vec<super::super::types::auditmanager::FrameworkControlSet>>,
        >,
        /// Description of the framework.
        #[builder(into, default)]
        pub description: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        /// Name of the framework.
        #[builder(into, default)]
        pub name: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        /// A map of tags to assign to the framework. If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
        #[builder(into, default)]
        pub tags: pulumi_wasm_rust::InputOrOutput<
            Option<std::collections::HashMap<String, String>>,
        >,
    }
    #[allow(dead_code)]
    pub struct FrameworkResult {
        /// Amazon Resource Name (ARN) of the framework.
        /// * `control_sets[*].id` - Unique identifier for the framework control set.
        pub arn: pulumi_wasm_rust::Output<String>,
        /// Compliance type that the new custom framework supports, such as `CIS` or `HIPAA`.
        pub compliance_type: pulumi_wasm_rust::Output<Option<String>>,
        /// Configuration block(s) for the control sets that are associated with the framework. See `control_sets` Block below for details.
        ///
        /// The following arguments are optional:
        pub control_sets: pulumi_wasm_rust::Output<
            Option<Vec<super::super::types::auditmanager::FrameworkControlSet>>,
        >,
        /// Description of the framework.
        pub description: pulumi_wasm_rust::Output<Option<String>>,
        /// Framework type, such as a custom framework or a standard framework.
        pub framework_type: pulumi_wasm_rust::Output<String>,
        /// Name of the framework.
        pub name: pulumi_wasm_rust::Output<String>,
        /// A map of tags to assign to the framework. If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
        pub tags: pulumi_wasm_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        pub tags_all: pulumi_wasm_rust::Output<
            std::collections::HashMap<String, String>,
        >,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_wasm_rust::PulumiContext,
        name: &str,
        args: FrameworkArgs,
    ) -> FrameworkResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let compliance_type_binding = args
            .compliance_type
            .get_output(context)
            .get_inner();
        let control_sets_binding = args.control_sets.get_output(context).get_inner();
        let description_binding = args.description.get_output(context).get_inner();
        let name_binding = args.name.get_output(context).get_inner();
        let tags_binding = args.tags.get_output(context).get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "aws:auditmanager/framework:Framework".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "complianceType".into(),
                    value: &compliance_type_binding,
                },
                register_interface::ObjectField {
                    name: "controlSets".into(),
                    value: &control_sets_binding,
                },
                register_interface::ObjectField {
                    name: "description".into(),
                    value: &description_binding,
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
            results: Vec::from([
                register_interface::ResultField {
                    name: "arn".into(),
                },
                register_interface::ResultField {
                    name: "complianceType".into(),
                },
                register_interface::ResultField {
                    name: "controlSets".into(),
                },
                register_interface::ResultField {
                    name: "description".into(),
                },
                register_interface::ResultField {
                    name: "frameworkType".into(),
                },
                register_interface::ResultField {
                    name: "name".into(),
                },
                register_interface::ResultField {
                    name: "tags".into(),
                },
                register_interface::ResultField {
                    name: "tagsAll".into(),
                },
            ]),
        };
        let o = register_interface::register(context.get_inner(), &request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        FrameworkResult {
            arn: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("arn").unwrap(),
            ),
            compliance_type: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("complianceType").unwrap(),
            ),
            control_sets: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("controlSets").unwrap(),
            ),
            description: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("description").unwrap(),
            ),
            framework_type: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("frameworkType").unwrap(),
            ),
            name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("name").unwrap(),
            ),
            tags: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("tags").unwrap(),
            ),
            tags_all: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("tagsAll").unwrap(),
            ),
        }
    }
}
