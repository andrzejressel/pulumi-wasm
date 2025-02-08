/// Provides an Inspector Classic Assessment Target
///
/// ## Example Usage
///
/// ```yaml
/// resources:
///   bar:
///     type: aws:inspector:ResourceGroup
///     properties:
///       tags:
///         Name: foo
///         Env: bar
///   foo:
///     type: aws:inspector:AssessmentTarget
///     properties:
///       name: assessment target
///       resourceGroupArn: ${bar.arn}
/// ```
///
/// ## Import
///
/// Using `pulumi import`, import Inspector Classic Assessment Targets using their Amazon Resource Name (ARN). For example:
///
/// ```sh
/// $ pulumi import aws:inspector/assessmentTarget:AssessmentTarget example arn:aws:inspector:us-east-1:123456789012:target/0-xxxxxxx
/// ```
#[allow(clippy::doc_lazy_continuation)]
pub mod assessment_target {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct AssessmentTargetArgs {
        /// The name of the assessment target.
        #[builder(into, default)]
        pub name: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Inspector Resource Group Amazon Resource Name (ARN) stating tags for instance matching. If not specified, all EC2 instances in the current AWS account and region are included in the assessment target.
        #[builder(into, default)]
        pub resource_group_arn: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
    }
    #[allow(dead_code)]
    pub struct AssessmentTargetResult {
        /// The target assessment ARN.
        pub arn: pulumi_gestalt_rust::Output<String>,
        /// The name of the assessment target.
        pub name: pulumi_gestalt_rust::Output<String>,
        /// Inspector Resource Group Amazon Resource Name (ARN) stating tags for instance matching. If not specified, all EC2 instances in the current AWS account and region are included in the assessment target.
        pub resource_group_arn: pulumi_gestalt_rust::Output<Option<String>>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::PulumiContext,
        name: &str,
        args: AssessmentTargetArgs,
    ) -> AssessmentTargetResult {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::register_interface;
        use std::collections::HashMap;
        let name_binding = args.name.get_output(context).get_inner();
        let resource_group_arn_binding = args
            .resource_group_arn
            .get_output(context)
            .get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "aws:inspector/assessmentTarget:AssessmentTarget".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "name".into(),
                    value: &name_binding,
                },
                register_interface::ObjectField {
                    name: "resourceGroupArn".into(),
                    value: &resource_group_arn_binding,
                },
            ]),
        };
        let o = register_interface::register(context.get_inner(), &request);
        AssessmentTargetResult {
            arn: pulumi_gestalt_rust::__private::into_domain(o.extract_field("arn")),
            name: pulumi_gestalt_rust::__private::into_domain(o.extract_field("name")),
            resource_group_arn: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("resourceGroupArn"),
            ),
        }
    }
}
