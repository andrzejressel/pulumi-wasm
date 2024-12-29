/// Adds a launch permission to an Amazon Machine Image (AMI).
///
/// ## Example Usage
///
/// ### AWS Account ID
///
/// ```ignore
/// use pulumi_wasm_rust::Output;
/// use pulumi_wasm_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let example = ami_launch_permission::create(
///         "example",
///         AmiLaunchPermissionArgs::builder()
///             .account_id("123456789012")
///             .image_id("ami-12345678")
///             .build_struct(),
///     );
/// }
/// ```
///
/// ### Public Access
///
/// ```ignore
/// use pulumi_wasm_rust::Output;
/// use pulumi_wasm_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let example = ami_launch_permission::create(
///         "example",
///         AmiLaunchPermissionArgs::builder()
///             .group("all")
///             .image_id("ami-12345678")
///             .build_struct(),
///     );
/// }
/// ```
///
/// ### Organization Access
///
/// ```ignore
/// use pulumi_wasm_rust::Output;
/// use pulumi_wasm_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let current = get_organization::invoke(
///         GetOrganizationArgs::builder().build_struct(),
///     );
///     let example = ami_launch_permission::create(
///         "example",
///         AmiLaunchPermissionArgs::builder()
///             .image_id("ami-12345678")
///             .organization_arn("${current.arn}")
///             .build_struct(),
///     );
/// }
/// ```
///
/// ## Import
///
/// Using `pulumi import`, import AMI Launch Permissions using `[ACCOUNT-ID|GROUP-NAME|ORGANIZATION-ARN|ORGANIZATIONAL-UNIT-ARN]/IMAGE-ID`. For example:
///
/// ```sh
/// $ pulumi import aws:ec2/amiLaunchPermission:AmiLaunchPermission example 123456789012/ami-12345678
/// ```
pub mod ami_launch_permission {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct AmiLaunchPermissionArgs {
        /// AWS account ID for the launch permission.
        #[builder(into, default)]
        pub account_id: pulumi_wasm_rust::Output<Option<String>>,
        /// Name of the group for the launch permission. Valid values: `"all"`.
        #[builder(into, default)]
        pub group: pulumi_wasm_rust::Output<Option<String>>,
        /// ID of the AMI.
        #[builder(into)]
        pub image_id: pulumi_wasm_rust::Output<String>,
        /// ARN of an organization for the launch permission.
        #[builder(into, default)]
        pub organization_arn: pulumi_wasm_rust::Output<Option<String>>,
        /// ARN of an organizational unit for the launch permission.
        #[builder(into, default)]
        pub organizational_unit_arn: pulumi_wasm_rust::Output<Option<String>>,
    }
    #[allow(dead_code)]
    pub struct AmiLaunchPermissionResult {
        /// AWS account ID for the launch permission.
        pub account_id: pulumi_wasm_rust::Output<Option<String>>,
        /// Name of the group for the launch permission. Valid values: `"all"`.
        pub group: pulumi_wasm_rust::Output<Option<String>>,
        /// ID of the AMI.
        pub image_id: pulumi_wasm_rust::Output<String>,
        /// ARN of an organization for the launch permission.
        pub organization_arn: pulumi_wasm_rust::Output<Option<String>>,
        /// ARN of an organizational unit for the launch permission.
        pub organizational_unit_arn: pulumi_wasm_rust::Output<Option<String>>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        name: &str,
        args: AmiLaunchPermissionArgs,
    ) -> AmiLaunchPermissionResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let account_id_binding = args.account_id.get_inner();
        let group_binding = args.group.get_inner();
        let image_id_binding = args.image_id.get_inner();
        let organization_arn_binding = args.organization_arn.get_inner();
        let organizational_unit_arn_binding = args.organizational_unit_arn.get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "aws:ec2/amiLaunchPermission:AmiLaunchPermission".into(),
            name: name.to_string(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "accountId".into(),
                    value: &account_id_binding,
                },
                register_interface::ObjectField {
                    name: "group".into(),
                    value: &group_binding,
                },
                register_interface::ObjectField {
                    name: "imageId".into(),
                    value: &image_id_binding,
                },
                register_interface::ObjectField {
                    name: "organizationArn".into(),
                    value: &organization_arn_binding,
                },
                register_interface::ObjectField {
                    name: "organizationalUnitArn".into(),
                    value: &organizational_unit_arn_binding,
                },
            ]),
            results: Vec::from([
                register_interface::ResultField {
                    name: "accountId".into(),
                },
                register_interface::ResultField {
                    name: "group".into(),
                },
                register_interface::ResultField {
                    name: "imageId".into(),
                },
                register_interface::ResultField {
                    name: "organizationArn".into(),
                },
                register_interface::ResultField {
                    name: "organizationalUnitArn".into(),
                },
            ]),
        };
        let o = register_interface::register(&request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        AmiLaunchPermissionResult {
            account_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("accountId").unwrap(),
            ),
            group: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("group").unwrap(),
            ),
            image_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("imageId").unwrap(),
            ),
            organization_arn: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("organizationArn").unwrap(),
            ),
            organizational_unit_arn: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("organizationalUnitArn").unwrap(),
            ),
        }
    }
}
