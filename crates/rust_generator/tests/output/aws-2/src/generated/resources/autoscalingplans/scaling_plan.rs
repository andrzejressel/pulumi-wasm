/// Manages an AWS Auto Scaling scaling plan.
/// More information can be found in the [AWS Auto Scaling User Guide](https://docs.aws.amazon.com/autoscaling/plans/userguide/what-is-aws-auto-scaling.html).
///
/// > **NOTE:** The AWS Auto Scaling service uses an AWS IAM service-linked role to manage predictive scaling of Amazon EC2 Auto Scaling groups. The service attempts to automatically create this role the first time a scaling plan with predictive scaling enabled is created.
/// An `aws.iam.ServiceLinkedRole` resource can be used to manually manage this role.
/// See the [AWS documentation](https://docs.aws.amazon.com/autoscaling/plans/userguide/aws-auto-scaling-service-linked-roles.html#create-service-linked-role-manual) for more details.
///
/// ## Example Usage
///
/// ## Import
///
/// Using `pulumi import`, import Auto Scaling scaling plans using the `name`. For example:
///
/// ```sh
/// $ pulumi import aws:autoscalingplans/scalingPlan:ScalingPlan example MyScale1
/// ```
pub mod scaling_plan {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct ScalingPlanArgs {
        /// CloudFormation stack or set of tags. You can create one scaling plan per application source.
        #[builder(into)]
        pub application_source: pulumi_gestalt_rust::InputOrOutput<
            super::super::types::autoscalingplans::ScalingPlanApplicationSource,
        >,
        /// Name of the scaling plan. Names cannot contain vertical bars, colons, or forward slashes.
        #[builder(into, default)]
        pub name: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Scaling instructions. More details can be found in the [AWS Auto Scaling API Reference](https://docs.aws.amazon.com/autoscaling/plans/APIReference/API_ScalingInstruction.html).
        #[builder(into)]
        pub scaling_instructions: pulumi_gestalt_rust::InputOrOutput<
            Vec<super::super::types::autoscalingplans::ScalingPlanScalingInstruction>,
        >,
    }
    #[allow(dead_code)]
    pub struct ScalingPlanResult {
        /// CloudFormation stack or set of tags. You can create one scaling plan per application source.
        pub application_source: pulumi_gestalt_rust::Output<
            super::super::types::autoscalingplans::ScalingPlanApplicationSource,
        >,
        /// Name of the scaling plan. Names cannot contain vertical bars, colons, or forward slashes.
        pub name: pulumi_gestalt_rust::Output<String>,
        /// Scaling instructions. More details can be found in the [AWS Auto Scaling API Reference](https://docs.aws.amazon.com/autoscaling/plans/APIReference/API_ScalingInstruction.html).
        pub scaling_instructions: pulumi_gestalt_rust::Output<
            Vec<super::super::types::autoscalingplans::ScalingPlanScalingInstruction>,
        >,
        /// The version number of the scaling plan. This value is always 1.
        pub scaling_plan_version: pulumi_gestalt_rust::Output<i32>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::PulumiContext,
        name: &str,
        args: ScalingPlanArgs,
    ) -> ScalingPlanResult {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::register_interface;
        use std::collections::HashMap;
        let application_source_binding = args
            .application_source
            .get_output(context)
            .get_inner();
        let name_binding = args.name.get_output(context).get_inner();
        let scaling_instructions_binding = args
            .scaling_instructions
            .get_output(context)
            .get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "aws:autoscalingplans/scalingPlan:ScalingPlan".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "applicationSource".into(),
                    value: &application_source_binding,
                },
                register_interface::ObjectField {
                    name: "name".into(),
                    value: &name_binding,
                },
                register_interface::ObjectField {
                    name: "scalingInstructions".into(),
                    value: &scaling_instructions_binding,
                },
            ]),
        };
        let o = register_interface::register(context.get_inner(), &request);
        ScalingPlanResult {
            application_source: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("applicationSource"),
            ),
            name: pulumi_gestalt_rust::__private::into_domain(o.extract_field("name")),
            scaling_instructions: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("scalingInstructions"),
            ),
            scaling_plan_version: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("scalingPlanVersion"),
            ),
        }
    }
}
