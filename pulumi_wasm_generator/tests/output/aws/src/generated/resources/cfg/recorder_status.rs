/// Manages status (recording / stopped) of an AWS Config Configuration Recorder.
///
/// > **Note:** Starting Configuration Recorder requires a Delivery Channel to be present. Use of `depends_on` (as shown below) is recommended to avoid race conditions.
///
/// ## Example Usage
///
/// ```ignore
/// use pulumi_wasm_rust::Output;
/// use pulumi_wasm_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let assumeRole = get_policy_document::invoke(
///         GetPolicyDocumentArgs::builder()
///             .statements(
///                 vec![
///                     GetPolicyDocumentStatement::builder()
///                     .actions(vec!["sts:AssumeRole",]).effect("Allow")
///                     .principals(vec![GetPolicyDocumentStatementPrincipal::builder()
///                     .identifiers(vec!["config.amazonaws.com",]). type ("Service")
///                     .build_struct(),]).build_struct(),
///                 ],
///             )
///             .build_struct(),
///     );
///     let p = get_policy_document::invoke(
///         GetPolicyDocumentArgs::builder()
///             .statements(
///                 vec![
///                     GetPolicyDocumentStatement::builder().actions(vec!["s3:*",])
///                     .effect("Allow").resources(vec!["${b.arn}", "${b.arn}/*",])
///                     .build_struct(),
///                 ],
///             )
///             .build_struct(),
///     );
///     let a = role_policy_attachment::create(
///         "a",
///         RolePolicyAttachmentArgs::builder()
///             .policy_arn("arn:aws:iam::aws:policy/service-role/AWS_ConfigRole")
///             .role("${r.name}")
///             .build_struct(),
///     );
///     let b = bucket_v_2::create(
///         "b",
///         BucketV2Args::builder().bucket("awsconfig-example").build_struct(),
///     );
///     let foo = recorder_status::create(
///         "foo",
///         RecorderStatusArgs::builder()
///             .is_enabled(true)
///             .name("${fooRecorder.name}")
///             .build_struct(),
///     );
///     let fooDeliveryChannel = delivery_channel::create(
///         "fooDeliveryChannel",
///         DeliveryChannelArgs::builder()
///             .name("example")
///             .s_3_bucket_name("${b.bucket}")
///             .build_struct(),
///     );
///     let fooRecorder = recorder::create(
///         "fooRecorder",
///         RecorderArgs::builder().name("example").role_arn("${r.arn}").build_struct(),
///     );
///     let pRolePolicy = role_policy::create(
///         "pRolePolicy",
///         RolePolicyArgs::builder()
///             .name("awsconfig-example")
///             .policy("${p.json}")
///             .role("${r.id}")
///             .build_struct(),
///     );
///     let r = role::create(
///         "r",
///         RoleArgs::builder()
///             .assume_role_policy("${assumeRole.json}")
///             .name("example-awsconfig")
///             .build_struct(),
///     );
/// }
/// ```
///
/// ## Import
///
/// Using `pulumi import`, import Configuration Recorder Status using the name of the Configuration Recorder. For example:
///
/// ```sh
/// $ pulumi import aws:cfg/recorderStatus:RecorderStatus foo example
/// ```
pub mod recorder_status {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct RecorderStatusArgs {
        /// Whether the configuration recorder should be enabled or disabled.
        #[builder(into)]
        pub is_enabled: pulumi_wasm_rust::Output<bool>,
        /// The name of the recorder
        #[builder(into, default)]
        pub name: pulumi_wasm_rust::Output<Option<String>>,
    }
    #[allow(dead_code)]
    pub struct RecorderStatusResult {
        /// Whether the configuration recorder should be enabled or disabled.
        pub is_enabled: pulumi_wasm_rust::Output<bool>,
        /// The name of the recorder
        pub name: pulumi_wasm_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(name: &str, args: RecorderStatusArgs) -> RecorderStatusResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let is_enabled_binding = args.is_enabled.get_inner();
        let name_binding = args.name.get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "aws:cfg/recorderStatus:RecorderStatus".into(),
            name: name.to_string(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "isEnabled".into(),
                    value: &is_enabled_binding,
                },
                register_interface::ObjectField {
                    name: "name".into(),
                    value: &name_binding,
                },
            ]),
            results: Vec::from([
                register_interface::ResultField {
                    name: "isEnabled".into(),
                },
                register_interface::ResultField {
                    name: "name".into(),
                },
            ]),
        };
        let o = register_interface::register(&request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        RecorderStatusResult {
            is_enabled: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("isEnabled").unwrap(),
            ),
            name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("name").unwrap(),
            ),
        }
    }
}
