/// Resource for managing an AWS Audit Manager Framework Share.
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
///     let example = framework_share::create(
///         "example",
///         FrameworkShareArgs::builder()
///             .destination_account("123456789012")
///             .destination_region("us-east-1")
///             .framework_id("${exampleAwsAuditmanagerFramework.id}")
///             .build_struct(),
///     );
/// }
/// ```
///
/// ## Import
///
/// Using `pulumi import`, import Audit Manager Framework Share using the `id`. For example:
///
/// ```sh
/// $ pulumi import aws:auditmanager/frameworkShare:FrameworkShare example abcdef-123456
/// ```
pub mod framework_share {
    #[derive(pulumi_wasm_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct FrameworkShareArgs {
        /// Comment from the sender about the share request.
        #[builder(into, default)]
        pub comment: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        /// Amazon Web Services account of the recipient.
        #[builder(into)]
        pub destination_account: pulumi_wasm_rust::InputOrOutput<String>,
        /// Amazon Web Services region of the recipient.
        #[builder(into)]
        pub destination_region: pulumi_wasm_rust::InputOrOutput<String>,
        /// Unique identifier for the shared custom framework.
        ///
        /// The following arguments are optional:
        #[builder(into)]
        pub framework_id: pulumi_wasm_rust::InputOrOutput<String>,
    }
    #[allow(dead_code)]
    pub struct FrameworkShareResult {
        /// Comment from the sender about the share request.
        pub comment: pulumi_wasm_rust::Output<Option<String>>,
        /// Amazon Web Services account of the recipient.
        pub destination_account: pulumi_wasm_rust::Output<String>,
        /// Amazon Web Services region of the recipient.
        pub destination_region: pulumi_wasm_rust::Output<String>,
        /// Unique identifier for the shared custom framework.
        ///
        /// The following arguments are optional:
        pub framework_id: pulumi_wasm_rust::Output<String>,
        /// Status of the share request.
        pub status: pulumi_wasm_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_wasm_rust::PulumiContext,
        name: &str,
        args: FrameworkShareArgs,
    ) -> FrameworkShareResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let comment_binding = args.comment.get_output(context).get_inner();
        let destination_account_binding = args
            .destination_account
            .get_output(context)
            .get_inner();
        let destination_region_binding = args
            .destination_region
            .get_output(context)
            .get_inner();
        let framework_id_binding = args.framework_id.get_output(context).get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "aws:auditmanager/frameworkShare:FrameworkShare".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "comment".into(),
                    value: &comment_binding,
                },
                register_interface::ObjectField {
                    name: "destinationAccount".into(),
                    value: &destination_account_binding,
                },
                register_interface::ObjectField {
                    name: "destinationRegion".into(),
                    value: &destination_region_binding,
                },
                register_interface::ObjectField {
                    name: "frameworkId".into(),
                    value: &framework_id_binding,
                },
            ]),
        };
        let o = register_interface::register(context.get_inner(), &request);
        FrameworkShareResult {
            comment: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("comment"),
            ),
            destination_account: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("destinationAccount"),
            ),
            destination_region: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("destinationRegion"),
            ),
            framework_id: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("frameworkId"),
            ),
            status: pulumi_wasm_rust::__private::into_domain(o.extract_field("status")),
        }
    }
}
