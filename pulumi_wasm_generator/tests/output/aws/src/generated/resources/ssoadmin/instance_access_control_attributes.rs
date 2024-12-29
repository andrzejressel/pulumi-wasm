/// Provides a Single Sign-On (SSO) ABAC Resource: https://docs.aws.amazon.com/singlesignon/latest/userguide/abac.html
///
/// ## Example Usage
///
/// ```ignore
/// use pulumi_wasm_rust::Output;
/// use pulumi_wasm_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let example = get_instances::invoke(GetInstancesArgs::builder().build_struct());
///     let exampleInstanceAccessControlAttributes = instance_access_control_attributes::create(
///         "exampleInstanceAccessControlAttributes",
///         InstanceAccessControlAttributesArgs::builder()
///             .attributes(
///                 vec![
///                     InstanceAccessControlAttributesAttribute::builder().key("name")
///                     .values(vec![InstanceAccessControlAttributesAttributeValue::builder()
///                     .sources(vec!["${path:name.givenName}",]).build_struct(),])
///                     .build_struct(), InstanceAccessControlAttributesAttribute::builder()
///                     .key("last")
///                     .values(vec![InstanceAccessControlAttributesAttributeValue::builder()
///                     .sources(vec!["${path:name.familyName}",]).build_struct(),])
///                     .build_struct(),
///                 ],
///             )
///             .instance_arn("${example.arns[0]}")
///             .build_struct(),
///     );
/// }
/// ```
///
/// ## Import
///
/// Using `pulumi import`, import SSO Account Assignments using the `instance_arn`. For example:
///
/// ```sh
/// $ pulumi import aws:ssoadmin/instanceAccessControlAttributes:InstanceAccessControlAttributes example arn:aws:sso:::instance/ssoins-0123456789abcdef
/// ```
pub mod instance_access_control_attributes {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct InstanceAccessControlAttributesArgs {
        /// See AccessControlAttribute for more details.
        #[builder(into)]
        pub attributes: pulumi_wasm_rust::Output<
            Vec<super::super::types::ssoadmin::InstanceAccessControlAttributesAttribute>,
        >,
        /// The Amazon Resource Name (ARN) of the SSO Instance.
        #[builder(into)]
        pub instance_arn: pulumi_wasm_rust::Output<String>,
    }
    #[allow(dead_code)]
    pub struct InstanceAccessControlAttributesResult {
        /// See AccessControlAttribute for more details.
        pub attributes: pulumi_wasm_rust::Output<
            Vec<super::super::types::ssoadmin::InstanceAccessControlAttributesAttribute>,
        >,
        /// The Amazon Resource Name (ARN) of the SSO Instance.
        pub instance_arn: pulumi_wasm_rust::Output<String>,
        pub status: pulumi_wasm_rust::Output<String>,
        pub status_reason: pulumi_wasm_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        name: &str,
        args: InstanceAccessControlAttributesArgs,
    ) -> InstanceAccessControlAttributesResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let attributes_binding = args.attributes.get_inner();
        let instance_arn_binding = args.instance_arn.get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "aws:ssoadmin/instanceAccessControlAttributes:InstanceAccessControlAttributes"
                .into(),
            name: name.to_string(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "attributes".into(),
                    value: &attributes_binding,
                },
                register_interface::ObjectField {
                    name: "instanceArn".into(),
                    value: &instance_arn_binding,
                },
            ]),
            results: Vec::from([
                register_interface::ResultField {
                    name: "attributes".into(),
                },
                register_interface::ResultField {
                    name: "instanceArn".into(),
                },
                register_interface::ResultField {
                    name: "status".into(),
                },
                register_interface::ResultField {
                    name: "statusReason".into(),
                },
            ]),
        };
        let o = register_interface::register(&request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        InstanceAccessControlAttributesResult {
            attributes: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("attributes").unwrap(),
            ),
            instance_arn: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("instanceArn").unwrap(),
            ),
            status: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("status").unwrap(),
            ),
            status_reason: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("statusReason").unwrap(),
            ),
        }
    }
}
