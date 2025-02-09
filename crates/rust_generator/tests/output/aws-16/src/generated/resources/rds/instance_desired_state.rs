/// Resource for managing an AWS RDS (Relational Database) RDS Instance State.
///
/// > Destruction of this resource is a no-op and **will not** modify the instance state
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
///     let test = instance_desired_state::create(
///         "test",
///         InstanceDesiredStateArgs::builder()
///             .identifier("${testAwsDbInstance.identifier}")
///             .state("available")
///             .build_struct(),
///     );
/// }
/// ```
///
/// ## Import
///
/// Using `pulumi import`, import RDS (Relational Database) RDS Instance State using the `example_id_arg`. For example:
///
/// ```sh
/// $ pulumi import aws:rds/instanceDesiredState:InstanceDesiredState example rds_instance_state-id-12345678
/// ```
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod instance_desired_state {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct InstanceDesiredStateArgs {
        /// DB Instance Identifier
        #[builder(into)]
        pub identifier: pulumi_gestalt_rust::InputOrOutput<String>,
        /// Configured state of the DB Instance. Valid values are `available` and `stopped`.
        #[builder(into)]
        pub state: pulumi_gestalt_rust::InputOrOutput<String>,
        #[builder(into, default)]
        pub timeouts: pulumi_gestalt_rust::InputOrOutput<
            Option<super::super::types::rds::InstanceDesiredStateTimeouts>,
        >,
    }
    #[allow(dead_code)]
    pub struct InstanceDesiredStateResult {
        /// DB Instance Identifier
        pub identifier: pulumi_gestalt_rust::Output<String>,
        /// Configured state of the DB Instance. Valid values are `available` and `stopped`.
        pub state: pulumi_gestalt_rust::Output<String>,
        pub timeouts: pulumi_gestalt_rust::Output<
            Option<super::super::types::rds::InstanceDesiredStateTimeouts>,
        >,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: InstanceDesiredStateArgs,
    ) -> InstanceDesiredStateResult {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::register_interface;
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let identifier_binding = args.identifier.get_output(context);
        let state_binding = args.state.get_output(context);
        let timeouts_binding = args.timeouts.get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "aws:rds/instanceDesiredState:InstanceDesiredState".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "identifier".into(),
                    value: identifier_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "state".into(),
                    value: state_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "timeouts".into(),
                    value: timeouts_binding.get_id(),
                },
            ],
        };
        let o = context.register_resource(request);
        InstanceDesiredStateResult {
            identifier: o.get_field("identifier"),
            state: o.get_field("state"),
            timeouts: o.get_field("timeouts"),
        }
    }
}
