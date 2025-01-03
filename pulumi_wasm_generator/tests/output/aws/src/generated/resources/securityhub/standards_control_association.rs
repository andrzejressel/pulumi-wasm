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
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct StandardsControlAssociationArgs {
        /// The desired enablement status of the control in the standard. Valid values: `ENABLED`, `DISABLED`.
        #[builder(into)]
        pub association_status: pulumi_wasm_rust::Output<String>,
        /// The unique identifier for the security control whose enablement status you want to update.
        #[builder(into)]
        pub security_control_id: pulumi_wasm_rust::Output<String>,
        /// The Amazon Resource Name (ARN) of the standard in which you want to update the control's enablement status.
        ///
        /// The following arguments are optional:
        #[builder(into)]
        pub standards_arn: pulumi_wasm_rust::Output<String>,
        /// The reason for updating the control's enablement status in the standard. Required when `association_status` is `DISABLED`.
        #[builder(into, default)]
        pub updated_reason: pulumi_wasm_rust::Output<Option<String>>,
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
        name: &str,
        args: StandardsControlAssociationArgs,
    ) -> StandardsControlAssociationResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let association_status_binding = args.association_status.get_inner();
        let security_control_id_binding = args.security_control_id.get_inner();
        let standards_arn_binding = args.standards_arn.get_inner();
        let updated_reason_binding = args.updated_reason.get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "aws:securityhub/standardsControlAssociation:StandardsControlAssociation"
                .into(),
            name: name.to_string(),
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
            results: Vec::from([
                register_interface::ResultField {
                    name: "associationStatus".into(),
                },
                register_interface::ResultField {
                    name: "securityControlId".into(),
                },
                register_interface::ResultField {
                    name: "standardsArn".into(),
                },
                register_interface::ResultField {
                    name: "updatedReason".into(),
                },
            ]),
        };
        let o = register_interface::register(&request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        StandardsControlAssociationResult {
            association_status: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("associationStatus").unwrap(),
            ),
            security_control_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("securityControlId").unwrap(),
            ),
            standards_arn: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("standardsArn").unwrap(),
            ),
            updated_reason: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("updatedReason").unwrap(),
            ),
        }
    }
}
