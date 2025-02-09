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
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: SubnetGroupArgs,
    ) -> SubnetGroupResult {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::register_interface;
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let description_binding = args.description.get_output(context);
        let name_binding = args.name.get_output(context);
        let subnet_ids_binding = args.subnet_ids.get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "aws:dax/subnetGroup:SubnetGroup".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "description".into(),
                    value: description_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "name".into(),
                    value: name_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "subnetIds".into(),
                    value: subnet_ids_binding.get_id(),
                },
            ],
        };
        let o = context.register_resource(request);
        SubnetGroupResult {
            description: o.get_field("description"),
            name: o.get_field("name"),
            subnet_ids: o.get_field("subnetIds"),
            vpc_id: o.get_field("vpcId"),
        }
    }
}
