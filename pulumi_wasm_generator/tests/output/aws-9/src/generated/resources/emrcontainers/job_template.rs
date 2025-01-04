/// Manages an EMR Containers (EMR on EKS) Job Template.
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
///     let example = job_template::create(
///         "example",
///         JobTemplateArgs::builder()
///             .job_template_data(
///                 JobTemplateJobTemplateData::builder()
///                     .executionRoleArn("${exampleAwsIamRole.arn}")
///                     .jobDriver(
///                         JobTemplateJobTemplateDataJobDriver::builder()
///                             .sparkSqlJobDriver(
///                                 JobTemplateJobTemplateDataJobDriverSparkSqlJobDriver::builder()
///                                     .entryPoint("default")
///                                     .build_struct(),
///                             )
///                             .build_struct(),
///                     )
///                     .releaseLabel("emr-6.10.0-latest")
///                     .build_struct(),
///             )
///             .name("example")
///             .build_struct(),
///     );
/// }
/// ```
///
/// ## Import
///
/// Using `pulumi import`, import EKS job templates using the `id`. For example:
///
/// ```sh
/// $ pulumi import aws:emrcontainers/jobTemplate:JobTemplate example a1b2c3d4e5f6g7h8i9j10k11l
/// ```
pub mod job_template {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct JobTemplateArgs {
        /// The job template data which holds values of StartJobRun API request.
        #[builder(into)]
        pub job_template_data: pulumi_wasm_rust::Output<
            super::super::types::emrcontainers::JobTemplateJobTemplateData,
        >,
        /// The KMS key ARN used to encrypt the job template.
        #[builder(into, default)]
        pub kms_key_arn: pulumi_wasm_rust::Output<Option<String>>,
        /// The specified name of the job template.
        #[builder(into, default)]
        pub name: pulumi_wasm_rust::Output<Option<String>>,
        /// Key-value mapping of resource tags. If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
        #[builder(into, default)]
        pub tags: pulumi_wasm_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
    }
    #[allow(dead_code)]
    pub struct JobTemplateResult {
        /// ARN of the job template.
        pub arn: pulumi_wasm_rust::Output<String>,
        /// The job template data which holds values of StartJobRun API request.
        pub job_template_data: pulumi_wasm_rust::Output<
            super::super::types::emrcontainers::JobTemplateJobTemplateData,
        >,
        /// The KMS key ARN used to encrypt the job template.
        pub kms_key_arn: pulumi_wasm_rust::Output<Option<String>>,
        /// The specified name of the job template.
        pub name: pulumi_wasm_rust::Output<String>,
        /// Key-value mapping of resource tags. If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
        pub tags: pulumi_wasm_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// Map of tags assigned to the resource, including those inherited from the provider `default_tags` configuration block.
        pub tags_all: pulumi_wasm_rust::Output<
            std::collections::HashMap<String, String>,
        >,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(name: &str, args: JobTemplateArgs) -> JobTemplateResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let job_template_data_binding = args.job_template_data.get_inner();
        let kms_key_arn_binding = args.kms_key_arn.get_inner();
        let name_binding = args.name.get_inner();
        let tags_binding = args.tags.get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "aws:emrcontainers/jobTemplate:JobTemplate".into(),
            name: name.to_string(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "jobTemplateData".into(),
                    value: &job_template_data_binding,
                },
                register_interface::ObjectField {
                    name: "kmsKeyArn".into(),
                    value: &kms_key_arn_binding,
                },
                register_interface::ObjectField {
                    name: "name".into(),
                    value: &name_binding,
                },
                register_interface::ObjectField {
                    name: "tags".into(),
                    value: &tags_binding,
                },
            ]),
            results: Vec::from([
                register_interface::ResultField {
                    name: "arn".into(),
                },
                register_interface::ResultField {
                    name: "jobTemplateData".into(),
                },
                register_interface::ResultField {
                    name: "kmsKeyArn".into(),
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
            ]),
        };
        let o = register_interface::register(&request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        JobTemplateResult {
            arn: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("arn").unwrap(),
            ),
            job_template_data: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("jobTemplateData").unwrap(),
            ),
            kms_key_arn: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("kmsKeyArn").unwrap(),
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
        }
    }
}
