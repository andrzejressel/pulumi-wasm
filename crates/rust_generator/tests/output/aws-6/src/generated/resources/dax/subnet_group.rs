/// Provides a DAX Subnet Group resource.
///
/// ## Example Usage
///
/// ```ignore
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let example = subnet_group::create(
///         "example",
///         SubnetGroupArgs::builder()
///             .name("example")
///             .subnet_ids(vec!["${example1.id}", "${example2.id}",])
///             .build_struct(),
///     );
/// }
/// ```
///
/// ## Import
///
/// Using `pulumi import`, import DAX Subnet Group using the `name`. For example:
///
/// ```sh
/// $ pulumi import aws:dax/subnetGroup:SubnetGroup example my_dax_sg
/// ```
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod subnet_group {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct SubnetGroupArgs {
        /// A description of the subnet group.
        #[builder(into, default)]
        pub description: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The name of the subnet group.
        #[builder(into, default)]
        pub name: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// A list of VPC subnet IDs for the subnet group.
        #[builder(into)]
        pub subnet_ids: pulumi_gestalt_rust::InputOrOutput<Vec<String>>,
    }
    #[allow(dead_code)]
    pub struct SubnetGroupResult {
        /// A description of the subnet group.
        pub description: pulumi_gestalt_rust::Output<Option<String>>,
        /// The name of the subnet group.
        pub name: pulumi_gestalt_rust::Output<String>,
        /// A list of VPC subnet IDs for the subnet group.
        pub subnet_ids: pulumi_gestalt_rust::Output<Vec<String>>,
        /// VPC ID of the subnet group.
        pub vpc_id: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::PulumiContext,
        name: &str,
        args: SubnetGroupArgs,
    ) -> SubnetGroupResult {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::register_interface;
        use std::collections::HashMap;
        let description_binding_1 = args.description.get_output(context);
        let description_binding = description_binding_1.get_inner();
        let name_binding_1 = args.name.get_output(context);
        let name_binding = name_binding_1.get_inner();
        let subnet_ids_binding_1 = args.subnet_ids.get_output(context);
        let subnet_ids_binding = subnet_ids_binding_1.get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "aws:dax/subnetGroup:SubnetGroup".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "description".into(),
                    value: &description_binding,
                },
                register_interface::ObjectField {
                    name: "name".into(),
                    value: &name_binding,
                },
                register_interface::ObjectField {
                    name: "subnetIds".into(),
                    value: &subnet_ids_binding,
                },
            ]),
        };
        let o = register_interface::register(context.get_inner(), &request);
        SubnetGroupResult {
            description: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("description"),
            ),
            name: pulumi_gestalt_rust::__private::into_domain(o.extract_field("name")),
            subnet_ids: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("subnetIds"),
            ),
            vpc_id: pulumi_gestalt_rust::__private::into_domain(o.extract_field("vpcId")),
        }
    }
}
