/// Resource for managing an AWS Backup Restore Testing Selection.
///
/// ## Example Usage
///
/// ### Basic Usage
///
/// ```ignore
/// use pulumi_wasm_rust::Output;
/// use pulumi_wasm_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let example = restore_testing_selection::create(
///         "example",
///         RestoreTestingSelectionArgs::builder()
///             .iam_role_arn("${exampleAwsIamRole.arn}")
///             .name("ec2_selection")
///             .protected_resource_arns(vec!["*",])
///             .protected_resource_type("EC2")
///             .restore_testing_plan_name("${exampleAwsBackupRestoreTestingPlan.name}")
///             .build_struct(),
///     );
/// }
/// ```
///
/// ### Advanced Usage
///
/// ```yaml
/// resources:
///   example:
///     type: aws:backup:RestoreTestingSelection
///     properties:
///       name: ec2_selection
///       restoreTestingPlanName: ${exampleAwsBackupRestoreTestingPlan.name}
///       protectedResourceType: EC2
///       iamRoleArn: ${exampleAwsIamRole.arn}
///       protectedResourceConditions:
///         stringEquals:
///           - key: aws:ResourceTag/backup
///             value: true
/// ```
///
/// ## Import
///
/// Using `pulumi import`, import Backup Restore Testing Selection using `name:restore_testing_plan_name`. For example:
///
/// ```sh
/// $ pulumi import aws:backup/restoreTestingSelection:RestoreTestingSelection example restore_testing_selection_12345678:restore_testing_plan_12345678
/// ```
pub mod restore_testing_selection {
    #[derive(pulumi_wasm_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct RestoreTestingSelectionArgs {
        /// The ARN of the IAM role.
        #[builder(into)]
        pub iam_role_arn: pulumi_wasm_rust::InputOrOutput<String>,
        /// The name of the backup restore testing selection.
        #[builder(into, default)]
        pub name: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        /// The ARNs for the protected resources.
        #[builder(into, default)]
        pub protected_resource_arns: pulumi_wasm_rust::InputOrOutput<
            Option<Vec<String>>,
        >,
        /// The conditions for the protected resource.
        #[builder(into, default)]
        pub protected_resource_conditions: pulumi_wasm_rust::InputOrOutput<
            Option<
                super::super::types::backup::RestoreTestingSelectionProtectedResourceConditions,
            >,
        >,
        /// The type of the protected resource.
        #[builder(into)]
        pub protected_resource_type: pulumi_wasm_rust::InputOrOutput<String>,
        /// Override certain restore metadata keys. See the complete list of [restore testing inferred metadata](https://docs.aws.amazon.com/aws-backup/latest/devguide/restore-testing-inferred-metadata.html) .
        #[builder(into, default)]
        pub restore_metadata_overrides: pulumi_wasm_rust::InputOrOutput<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// The name of the restore testing plan.
        #[builder(into)]
        pub restore_testing_plan_name: pulumi_wasm_rust::InputOrOutput<String>,
        /// The amount of hours available to run a validation script on the data. Valid range is `1` to `168`.
        #[builder(into, default)]
        pub validation_window_hours: pulumi_wasm_rust::InputOrOutput<Option<i32>>,
    }
    #[allow(dead_code)]
    pub struct RestoreTestingSelectionResult {
        /// The ARN of the IAM role.
        pub iam_role_arn: pulumi_wasm_rust::Output<String>,
        /// The name of the backup restore testing selection.
        pub name: pulumi_wasm_rust::Output<String>,
        /// The ARNs for the protected resources.
        pub protected_resource_arns: pulumi_wasm_rust::Output<Vec<String>>,
        /// The conditions for the protected resource.
        pub protected_resource_conditions: pulumi_wasm_rust::Output<
            Option<
                super::super::types::backup::RestoreTestingSelectionProtectedResourceConditions,
            >,
        >,
        /// The type of the protected resource.
        pub protected_resource_type: pulumi_wasm_rust::Output<String>,
        /// Override certain restore metadata keys. See the complete list of [restore testing inferred metadata](https://docs.aws.amazon.com/aws-backup/latest/devguide/restore-testing-inferred-metadata.html) .
        pub restore_metadata_overrides: pulumi_wasm_rust::Output<
            std::collections::HashMap<String, String>,
        >,
        /// The name of the restore testing plan.
        pub restore_testing_plan_name: pulumi_wasm_rust::Output<String>,
        /// The amount of hours available to run a validation script on the data. Valid range is `1` to `168`.
        pub validation_window_hours: pulumi_wasm_rust::Output<i32>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_wasm_rust::PulumiContext,
        name: &str,
        args: RestoreTestingSelectionArgs,
    ) -> RestoreTestingSelectionResult {
        use pulumi_wasm_rust::__private::pulumi_gestalt_adapter_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let iam_role_arn_binding = args.iam_role_arn.get_output(context).get_inner();
        let name_binding = args.name.get_output(context).get_inner();
        let protected_resource_arns_binding = args
            .protected_resource_arns
            .get_output(context)
            .get_inner();
        let protected_resource_conditions_binding = args
            .protected_resource_conditions
            .get_output(context)
            .get_inner();
        let protected_resource_type_binding = args
            .protected_resource_type
            .get_output(context)
            .get_inner();
        let restore_metadata_overrides_binding = args
            .restore_metadata_overrides
            .get_output(context)
            .get_inner();
        let restore_testing_plan_name_binding = args
            .restore_testing_plan_name
            .get_output(context)
            .get_inner();
        let validation_window_hours_binding = args
            .validation_window_hours
            .get_output(context)
            .get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "aws:backup/restoreTestingSelection:RestoreTestingSelection".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "iamRoleArn".into(),
                    value: &iam_role_arn_binding,
                },
                register_interface::ObjectField {
                    name: "name".into(),
                    value: &name_binding,
                },
                register_interface::ObjectField {
                    name: "protectedResourceArns".into(),
                    value: &protected_resource_arns_binding,
                },
                register_interface::ObjectField {
                    name: "protectedResourceConditions".into(),
                    value: &protected_resource_conditions_binding,
                },
                register_interface::ObjectField {
                    name: "protectedResourceType".into(),
                    value: &protected_resource_type_binding,
                },
                register_interface::ObjectField {
                    name: "restoreMetadataOverrides".into(),
                    value: &restore_metadata_overrides_binding,
                },
                register_interface::ObjectField {
                    name: "restoreTestingPlanName".into(),
                    value: &restore_testing_plan_name_binding,
                },
                register_interface::ObjectField {
                    name: "validationWindowHours".into(),
                    value: &validation_window_hours_binding,
                },
            ]),
        };
        let o = register_interface::register(context.get_inner(), &request);
        RestoreTestingSelectionResult {
            iam_role_arn: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("iamRoleArn"),
            ),
            name: pulumi_wasm_rust::__private::into_domain(o.extract_field("name")),
            protected_resource_arns: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("protectedResourceArns"),
            ),
            protected_resource_conditions: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("protectedResourceConditions"),
            ),
            protected_resource_type: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("protectedResourceType"),
            ),
            restore_metadata_overrides: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("restoreMetadataOverrides"),
            ),
            restore_testing_plan_name: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("restoreTestingPlanName"),
            ),
            validation_window_hours: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("validationWindowHours"),
            ),
        }
    }
}
