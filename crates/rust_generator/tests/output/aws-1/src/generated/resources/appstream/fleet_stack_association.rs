/// Manages an AppStream Fleet Stack association.
///
/// ## Example Usage
///
/// ```ignore
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let example = fleet::create(
///         "example",
///         FleetArgs::builder()
///             .compute_capacity(
///                 FleetComputeCapacity::builder().desiredInstances(1).build_struct(),
///             )
///             .image_name("Amazon-AppStream2-Sample-Image-03-11-2023")
///             .instance_type("stream.standard.small")
///             .name("NAME")
///             .build_struct(),
///     );
///     let exampleFleetStackAssociation = fleet_stack_association::create(
///         "exampleFleetStackAssociation",
///         FleetStackAssociationArgs::builder()
///             .fleet_name("${example.name}")
///             .stack_name("${exampleStack.name}")
///             .build_struct(),
///     );
///     let exampleStack = stack::create(
///         "exampleStack",
///         StackArgs::builder().name("STACK NAME").build_struct(),
///     );
/// }
/// ```
///
/// ## Import
///
/// Using `pulumi import`, import AppStream Stack Fleet Association using the `fleet_name` and `stack_name` separated by a slash (`/`). For example:
///
/// ```sh
/// $ pulumi import aws:appstream/fleetStackAssociation:FleetStackAssociation example fleetName/stackName
/// ```
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod fleet_stack_association {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct FleetStackAssociationArgs {
        /// Name of the fleet.
        #[builder(into)]
        pub fleet_name: pulumi_gestalt_rust::InputOrOutput<String>,
        /// Name of the stack.
        #[builder(into)]
        pub stack_name: pulumi_gestalt_rust::InputOrOutput<String>,
    }
    #[allow(dead_code)]
    pub struct FleetStackAssociationResult {
        /// Name of the fleet.
        pub fleet_name: pulumi_gestalt_rust::Output<String>,
        /// Name of the stack.
        pub stack_name: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::PulumiContext,
        name: &str,
        args: FleetStackAssociationArgs,
    ) -> FleetStackAssociationResult {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::register_interface;
        use std::collections::HashMap;
        let fleet_name_binding_1 = args.fleet_name.get_output(context);
        let fleet_name_binding = fleet_name_binding_1.get_inner();
        let stack_name_binding_1 = args.stack_name.get_output(context);
        let stack_name_binding = stack_name_binding_1.get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "aws:appstream/fleetStackAssociation:FleetStackAssociation".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "fleetName".into(),
                    value: &fleet_name_binding,
                },
                register_interface::ObjectField {
                    name: "stackName".into(),
                    value: &stack_name_binding,
                },
            ]),
        };
        let o = register_interface::register(context.get_inner(), &request);
        FleetStackAssociationResult {
            fleet_name: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("fleetName"),
            ),
            stack_name: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("stackName"),
            ),
        }
    }
}
