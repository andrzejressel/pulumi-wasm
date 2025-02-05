/// ## Example Usage
///
/// ### Basic usage
///
/// ```ignore
/// use pulumi_wasm_rust::Output;
/// use pulumi_wasm_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let cisAwsFoundationsBenchmark = standards_subscription::create(
///         "cisAwsFoundationsBenchmark",
///         StandardsSubscriptionArgs::builder()
///             .standards_arn(
///                 "arn:aws:securityhub:::ruleset/cis-aws-foundations-benchmark/v/1.2.0",
///             )
///             .build_struct(),
///     );
///     let cisAwsFoundationsBenchmarkDisableIam1 = standards_control_association::create(
///         "cisAwsFoundationsBenchmarkDisableIam1",
///         StandardsControlAssociationArgs::builder()
///             .association_status("DISABLED")
///             .security_control_id("IAM.1")
///             .standards_arn("${cisAwsFoundationsBenchmark.standardsArn}")
///             .updated_reason("Not needed")
///             .build_struct(),
///     );
///     let example = account::create("example", AccountArgs::builder().build_struct());
/// }
/// ```
///
pub mod standards_control_association {
    #[derive(pulumi_wasm_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct StandardsControlAssociationArgs {
        /// The desired enablement status of the control in the standard. Valid values: `ENABLED`, `DISABLED`.
        #[builder(into)]
        pub association_status: pulumi_wasm_rust::InputOrOutput<String>,
        /// The unique identifier for the security control whose enablement status you want to update.
        #[builder(into)]
        pub security_control_id: pulumi_wasm_rust::InputOrOutput<String>,
        /// The Amazon Resource Name (ARN) of the standard in which you want to update the control's enablement status.
        ///
        /// The following arguments are optional:
        #[builder(into)]
        pub standards_arn: pulumi_wasm_rust::InputOrOutput<String>,
        /// The reason for updating the control's enablement status in the standard. Required when `association_status` is `DISABLED`.
        #[builder(into, default)]
        pub updated_reason: pulumi_wasm_rust::InputOrOutput<Option<String>>,
    }
    #[allow(dead_code)]
    pub struct StandardsControlAssociationResult {
        /// The desired enablement status of the control in the standard. Valid values: `ENABLED`, `DISABLED`.
        pub association_status: pulumi_wasm_rust::Output<String>,
        /// The unique identifier for the security control whose enablement status you want to update.
        pub security_control_id: pulumi_wasm_rust::Output<String>,
        /// The Amazon Resource Name (ARN) of the standard in which you want to update the control's enablement status.
        ///
        /// The following arguments are optional:
        pub standards_arn: pulumi_wasm_rust::Output<String>,
        /// The reason for updating the control's enablement status in the standard. Required when `association_status` is `DISABLED`.
        pub updated_reason: pulumi_wasm_rust::Output<Option<String>>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_wasm_rust::PulumiContext,
        name: &str,
        args: StandardsControlAssociationArgs,
    ) -> StandardsControlAssociationResult {
        use pulumi_wasm_rust::__private::pulumi_gestalt_adapter_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let association_status_binding = args
            .association_status
            .get_output(context)
            .get_inner();
        let security_control_id_binding = args
            .security_control_id
            .get_output(context)
            .get_inner();
        let standards_arn_binding = args.standards_arn.get_output(context).get_inner();
        let updated_reason_binding = args.updated_reason.get_output(context).get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "aws:securityhub/standardsControlAssociation:StandardsControlAssociation"
                .into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "associationStatus".into(),
                    value: &association_status_binding,
                },
                register_interface::ObjectField {
                    name: "securityControlId".into(),
                    value: &security_control_id_binding,
                },
                register_interface::ObjectField {
                    name: "standardsArn".into(),
                    value: &standards_arn_binding,
                },
                register_interface::ObjectField {
                    name: "updatedReason".into(),
                    value: &updated_reason_binding,
                },
            ]),
        };
        let o = register_interface::register(context.get_inner(), &request);
        StandardsControlAssociationResult {
            association_status: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("associationStatus"),
            ),
            security_control_id: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("securityControlId"),
            ),
            standards_arn: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("standardsArn"),
            ),
            updated_reason: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("updatedReason"),
            ),
        }
    }
}
