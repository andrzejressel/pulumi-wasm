#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod get_outpost_instance_type {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GetOutpostInstanceTypeArgs {
        /// Outpost ARN.
        ///
        /// The following arguments are optional:
        #[builder(into)]
        pub arn: pulumi_gestalt_rust::InputOrOutput<String>,
        /// Desired instance type. Conflicts with `preferred_instance_types`.
        #[builder(into, default)]
        pub instance_type: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Ordered list of preferred instance types. The first match in this list will be returned. If no preferred matches are found and the original search returned more than one result, an error is returned. Conflicts with `instance_type`.
        #[builder(into, default)]
        pub preferred_instance_types: pulumi_gestalt_rust::InputOrOutput<
            Option<Vec<String>>,
        >,
    }
    #[allow(dead_code)]
    pub struct GetOutpostInstanceTypeResult {
        pub arn: pulumi_gestalt_rust::Output<String>,
        /// The provider-assigned unique ID for this managed resource.
        pub id: pulumi_gestalt_rust::Output<String>,
        pub instance_type: pulumi_gestalt_rust::Output<String>,
        pub preferred_instance_types: pulumi_gestalt_rust::Output<Option<Vec<String>>>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn invoke(
        context: &pulumi_gestalt_rust::Context,
        args: GetOutpostInstanceTypeArgs,
    ) -> GetOutpostInstanceTypeResult {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::register_interface;
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let arn_binding = args.arn.get_output(context);
        let instance_type_binding = args.instance_type.get_output(context);
        let preferred_instance_types_binding = args
            .preferred_instance_types
            .get_output(context);
        let request = pulumi_gestalt_rust::InvokeResourceRequest {
            token: "aws:outposts/getOutpostInstanceType:getOutpostInstanceType".into(),
            version: super::super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "arn".into(),
                    value: arn_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "instanceType".into(),
                    value: instance_type_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "preferredInstanceTypes".into(),
                    value: preferred_instance_types_binding.get_id(),
                },
            ],
        };
        let o = context.invoke_resource(request);
        GetOutpostInstanceTypeResult {
            arn: o.get_field("arn"),
            id: o.get_field("id"),
            instance_type: o.get_field("instanceType"),
            preferred_instance_types: o.get_field("preferredInstanceTypes"),
        }
    }
}
