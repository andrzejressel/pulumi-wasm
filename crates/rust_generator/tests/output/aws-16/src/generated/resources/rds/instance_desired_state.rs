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
        context: &pulumi_gestalt_rust::PulumiContext,
        name: &str,
        args: InstanceDesiredStateArgs,
    ) -> InstanceDesiredStateResult {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::register_interface;
        use std::collections::HashMap;
        let identifier_binding_1 = args.identifier.get_output(context);
        let identifier_binding = identifier_binding_1.get_inner();
        let state_binding_1 = args.state.get_output(context);
        let state_binding = state_binding_1.get_inner();
        let timeouts_binding_1 = args.timeouts.get_output(context);
        let timeouts_binding = timeouts_binding_1.get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "aws:rds/instanceDesiredState:InstanceDesiredState".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "identifier".into(),
                    value: &identifier_binding,
                },
                register_interface::ObjectField {
                    name: "state".into(),
                    value: &state_binding,
                },
                register_interface::ObjectField {
                    name: "timeouts".into(),
                    value: &timeouts_binding,
                },
            ]),
        };
        let o = register_interface::register(context.get_inner(), &request);
        InstanceDesiredStateResult {
            identifier: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("identifier"),
            ),
            state: pulumi_gestalt_rust::__private::into_domain(o.extract_field("state")),
            timeouts: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("timeouts"),
            ),
        }
    }
}
