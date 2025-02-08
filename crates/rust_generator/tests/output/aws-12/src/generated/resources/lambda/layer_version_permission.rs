/// Provides a Lambda Layer Version Permission resource. It allows you to share you own Lambda Layers to another account by account ID, to all accounts in AWS organization or even to all AWS accounts.
///
/// For information about Lambda Layer Permissions and how to use them, see [Using Resource-based Policies for AWS Lambda][1]
///
/// > **NOTE:** Setting `skip_destroy` to `true` means that the AWS Provider will _not_ destroy any layer version permission, even when running `pulumi destroy`. Layer version permissions are thus intentional dangling resources that are _not_ managed by Pulumi and may incur extra expense in your AWS account.
///
/// ## Example Usage
///
/// ```ignore
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let lambdaLayerPermission = layer_version_permission::create(
///         "lambdaLayerPermission",
///         LayerVersionPermissionArgs::builder()
///             .action("lambda:GetLayerVersion")
///             .layer_name("arn:aws:lambda:us-west-2:123456654321:layer:test_layer1")
///             .principal("111111111111")
///             .statement_id("dev-account")
///             .version_number(1)
///             .build_struct(),
///     );
/// }
/// ```
///
/// ## Import
///
/// Using `pulumi import`, import Lambda Layer Permissions using `layer_name` and `version_number`, separated by a comma (`,`). For example:
///
/// ```sh
/// $ pulumi import aws:lambda/layerVersionPermission:LayerVersionPermission example arn:aws:lambda:us-west-2:123456654321:layer:test_layer1,1
/// ```
#[allow(clippy::doc_lazy_continuation)]
pub mod layer_version_permission {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct LayerVersionPermissionArgs {
        /// Action, which will be allowed. `lambda:GetLayerVersion` value is suggested by AWS documantation.
        #[builder(into)]
        pub action: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The name or ARN of the Lambda Layer, which you want to grant access to.
        #[builder(into)]
        pub layer_name: pulumi_gestalt_rust::InputOrOutput<String>,
        /// An identifier of AWS Organization, which should be able to use your Lambda Layer. `principal` should be equal to `*` if `organization_id` provided.
        #[builder(into, default)]
        pub organization_id: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// AWS account ID which should be able to use your Lambda Layer. `*` can be used here, if you want to share your Lambda Layer widely.
        #[builder(into)]
        pub principal: pulumi_gestalt_rust::InputOrOutput<String>,
        /// Whether to retain the old version of a previously deployed Lambda Layer. Default is `false`. When this is not set to `true`, changing any of `compatible_architectures`, `compatible_runtimes`, `description`, `filename`, `layer_name`, `license_info`, `s3_bucket`, `s3_key`, `s3_object_version`, or `source_code_hash` forces deletion of the existing layer version and creation of a new layer version.
        #[builder(into, default)]
        pub skip_destroy: pulumi_gestalt_rust::InputOrOutput<Option<bool>>,
        /// The name of Lambda Layer Permission, for example `dev-account` - human readable note about what is this permission for.
        #[builder(into)]
        pub statement_id: pulumi_gestalt_rust::InputOrOutput<String>,
        /// Version of Lambda Layer, which you want to grant access to. Note: permissions only apply to a single version of a layer.
        #[builder(into)]
        pub version_number: pulumi_gestalt_rust::InputOrOutput<i32>,
    }
    #[allow(dead_code)]
    pub struct LayerVersionPermissionResult {
        /// Action, which will be allowed. `lambda:GetLayerVersion` value is suggested by AWS documantation.
        pub action: pulumi_gestalt_rust::Output<String>,
        /// The name or ARN of the Lambda Layer, which you want to grant access to.
        pub layer_name: pulumi_gestalt_rust::Output<String>,
        /// An identifier of AWS Organization, which should be able to use your Lambda Layer. `principal` should be equal to `*` if `organization_id` provided.
        pub organization_id: pulumi_gestalt_rust::Output<Option<String>>,
        /// Full Lambda Layer Permission policy.
        pub policy: pulumi_gestalt_rust::Output<String>,
        /// AWS account ID which should be able to use your Lambda Layer. `*` can be used here, if you want to share your Lambda Layer widely.
        pub principal: pulumi_gestalt_rust::Output<String>,
        /// A unique identifier for the current revision of the policy.
        pub revision_id: pulumi_gestalt_rust::Output<String>,
        /// Whether to retain the old version of a previously deployed Lambda Layer. Default is `false`. When this is not set to `true`, changing any of `compatible_architectures`, `compatible_runtimes`, `description`, `filename`, `layer_name`, `license_info`, `s3_bucket`, `s3_key`, `s3_object_version`, or `source_code_hash` forces deletion of the existing layer version and creation of a new layer version.
        pub skip_destroy: pulumi_gestalt_rust::Output<Option<bool>>,
        /// The name of Lambda Layer Permission, for example `dev-account` - human readable note about what is this permission for.
        pub statement_id: pulumi_gestalt_rust::Output<String>,
        /// Version of Lambda Layer, which you want to grant access to. Note: permissions only apply to a single version of a layer.
        pub version_number: pulumi_gestalt_rust::Output<i32>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::PulumiContext,
        name: &str,
        args: LayerVersionPermissionArgs,
    ) -> LayerVersionPermissionResult {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::register_interface;
        use std::collections::HashMap;
        let action_binding = args.action.get_output(context).get_inner();
        let layer_name_binding = args.layer_name.get_output(context).get_inner();
        let organization_id_binding = args
            .organization_id
            .get_output(context)
            .get_inner();
        let principal_binding = args.principal.get_output(context).get_inner();
        let skip_destroy_binding = args.skip_destroy.get_output(context).get_inner();
        let statement_id_binding = args.statement_id.get_output(context).get_inner();
        let version_number_binding = args.version_number.get_output(context).get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "aws:lambda/layerVersionPermission:LayerVersionPermission".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "action".into(),
                    value: &action_binding,
                },
                register_interface::ObjectField {
                    name: "layerName".into(),
                    value: &layer_name_binding,
                },
                register_interface::ObjectField {
                    name: "organizationId".into(),
                    value: &organization_id_binding,
                },
                register_interface::ObjectField {
                    name: "principal".into(),
                    value: &principal_binding,
                },
                register_interface::ObjectField {
                    name: "skipDestroy".into(),
                    value: &skip_destroy_binding,
                },
                register_interface::ObjectField {
                    name: "statementId".into(),
                    value: &statement_id_binding,
                },
                register_interface::ObjectField {
                    name: "versionNumber".into(),
                    value: &version_number_binding,
                },
            ]),
        };
        let o = register_interface::register(context.get_inner(), &request);
        LayerVersionPermissionResult {
            action: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("action"),
            ),
            layer_name: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("layerName"),
            ),
            organization_id: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("organizationId"),
            ),
            policy: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("policy"),
            ),
            principal: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("principal"),
            ),
            revision_id: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("revisionId"),
            ),
            skip_destroy: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("skipDestroy"),
            ),
            statement_id: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("statementId"),
            ),
            version_number: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("versionNumber"),
            ),
        }
    }
}
