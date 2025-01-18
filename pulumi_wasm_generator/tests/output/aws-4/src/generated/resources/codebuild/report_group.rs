/// Provides a CodeBuild Report Groups Resource.
///
/// ## Example Usage
///
/// ```yaml
/// resources:
///   exampleKey:
///     type: aws:kms:Key
///     name: example
///     properties:
///       description: my test kms key
///       deletionWindowInDays: 7
///       policy: ${example.json}
///   exampleBucketV2:
///     type: aws:s3:BucketV2
///     name: example
///     properties:
///       bucket: my-test
///   exampleReportGroup:
///     type: aws:codebuild:ReportGroup
///     name: example
///     properties:
///       name: my test report group
///       type: TEST
///       exportConfig:
///         type: S3
///         s3Destination:
///           bucket: ${exampleBucketV2.id}
///           encryptionDisabled: false
///           encryptionKey: ${exampleKey.arn}
///           packaging: NONE
///           path: /some
/// variables:
///   current:
///     fn::invoke:
///       function: aws:getCallerIdentity
///       arguments: {}
///   example:
///     fn::invoke:
///       function: aws:iam:getPolicyDocument
///       arguments:
///         statements:
///           - sid: Enable IAM User Permissions
///             effect: Allow
///             principals:
///               - type: AWS
///                 identifiers:
///                   - arn:aws:iam::${current.accountId}:root
///             actions:
///               - kms:*
///             resources:
///               - '*'
/// ```
///
/// ## Import
///
/// Using `pulumi import`, import CodeBuild Report Group using the CodeBuild Report Group arn. For example:
///
/// ```sh
/// $ pulumi import aws:codebuild/reportGroup:ReportGroup example arn:aws:codebuild:us-west-2:123456789:report-group/report-group-name
/// ```
pub mod report_group {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct ReportGroupArgs {
        /// If `true`, deletes any reports that belong to a report group before deleting the report group. If `false`, you must delete any reports in the report group before deleting it. Default value is `false`.
        #[builder(into, default)]
        pub delete_reports: pulumi_wasm_rust::Output<Option<bool>>,
        /// Information about the destination where the raw data of this Report Group is exported. see Export Config documented below.
        #[builder(into)]
        pub export_config: pulumi_wasm_rust::Output<
            super::super::types::codebuild::ReportGroupExportConfig,
        >,
        /// The name of a Report Group.
        #[builder(into, default)]
        pub name: pulumi_wasm_rust::Output<Option<String>>,
        /// Key-value mapping of resource tags. .If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
        #[builder(into, default)]
        pub tags: pulumi_wasm_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// The type of the Report Group. Valid value are `TEST` and `CODE_COVERAGE`.
        #[builder(into)]
        pub type_: pulumi_wasm_rust::Output<String>,
    }
    #[allow(dead_code)]
    pub struct ReportGroupResult {
        /// The ARN of Report Group.
        pub arn: pulumi_wasm_rust::Output<String>,
        /// The date and time this Report Group was created.
        pub created: pulumi_wasm_rust::Output<String>,
        /// If `true`, deletes any reports that belong to a report group before deleting the report group. If `false`, you must delete any reports in the report group before deleting it. Default value is `false`.
        pub delete_reports: pulumi_wasm_rust::Output<Option<bool>>,
        /// Information about the destination where the raw data of this Report Group is exported. see Export Config documented below.
        pub export_config: pulumi_wasm_rust::Output<
            super::super::types::codebuild::ReportGroupExportConfig,
        >,
        /// The name of a Report Group.
        pub name: pulumi_wasm_rust::Output<String>,
        /// Key-value mapping of resource tags. .If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
        pub tags: pulumi_wasm_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// A map of tags assigned to the resource, including those inherited from the provider `default_tags` configuration block.
        pub tags_all: pulumi_wasm_rust::Output<
            std::collections::HashMap<String, String>,
        >,
        /// The type of the Report Group. Valid value are `TEST` and `CODE_COVERAGE`.
        pub type_: pulumi_wasm_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(name: &str, args: ReportGroupArgs) -> ReportGroupResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let delete_reports_binding = args.delete_reports.get_inner();
        let export_config_binding = args.export_config.get_inner();
        let name_binding = args.name.get_inner();
        let tags_binding = args.tags.get_inner();
        let type__binding = args.type_.get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "aws:codebuild/reportGroup:ReportGroup".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "deleteReports".into(),
                    value: &delete_reports_binding,
                },
                register_interface::ObjectField {
                    name: "exportConfig".into(),
                    value: &export_config_binding,
                },
                register_interface::ObjectField {
                    name: "name".into(),
                    value: &name_binding,
                },
                register_interface::ObjectField {
                    name: "tags".into(),
                    value: &tags_binding,
                },
                register_interface::ObjectField {
                    name: "type".into(),
                    value: &type__binding,
                },
            ]),
            results: Vec::from([
                register_interface::ResultField {
                    name: "arn".into(),
                },
                register_interface::ResultField {
                    name: "created".into(),
                },
                register_interface::ResultField {
                    name: "deleteReports".into(),
                },
                register_interface::ResultField {
                    name: "exportConfig".into(),
                },
                register_interface::ResultField {
                    name: "name".into(),
                },
                register_interface::ResultField {
                    name: "tags".into(),
                },
                register_interface::ResultField {
                    name: "tagsAll".into(),
                },
                register_interface::ResultField {
                    name: "type".into(),
                },
            ]),
        };
        let o = register_interface::register(&request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        ReportGroupResult {
            arn: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("arn").unwrap(),
            ),
            created: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("created").unwrap(),
            ),
            delete_reports: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("deleteReports").unwrap(),
            ),
            export_config: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("exportConfig").unwrap(),
            ),
            name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("name").unwrap(),
            ),
            tags: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("tags").unwrap(),
            ),
            tags_all: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("tagsAll").unwrap(),
            ),
            type_: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("type").unwrap(),
            ),
        }
    }
}
