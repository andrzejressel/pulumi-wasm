/// Provides an IAM Virtual MFA Device.
///
/// > **Note:** All attributes will be stored in the raw state as plain-text.
/// > **Note:** A virtual MFA device cannot be directly associated with an IAM User from the provider.
///   To associate the virtual MFA device with a user and enable it, use the code returned in either `base_32_string_seed` or `qr_code_png` to generate TOTP authentication codes.
///   The authentication codes can then be used with the AWS CLI command [`aws iam enable-mfa-device`](https://docs.aws.amazon.com/cli/latest/reference/iam/enable-mfa-device.html) or the AWS API call [`EnableMFADevice`](https://docs.aws.amazon.com/IAM/latest/APIReference/API_EnableMFADevice.html).
///
/// ## Example Usage
///
/// **Using certs on file:**
///
/// ```ignore
/// use pulumi_wasm_rust::Output;
/// use pulumi_wasm_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let example = virtual_mfa_device::create(
///         "example",
///         VirtualMfaDeviceArgs::builder().virtual_mfa_device_name("example").build_struct(),
///     );
/// }
/// ```
///
/// ## Import
///
/// Using `pulumi import`, import IAM Virtual MFA Devices using the `arn`. For example:
///
/// ```sh
/// $ pulumi import aws:iam/virtualMfaDevice:VirtualMfaDevice example arn:aws:iam::123456789012:mfa/example
/// ```
pub mod virtual_mfa_device {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct VirtualMfaDeviceArgs {
        /// The path for the virtual MFA device.
        #[builder(into, default)]
        pub path: pulumi_wasm_rust::Output<Option<String>>,
        /// Map of resource tags for the virtual mfa device. If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
        #[builder(into, default)]
        pub tags: pulumi_wasm_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// The name of the virtual MFA device. Use with path to uniquely identify a virtual MFA device.
        #[builder(into)]
        pub virtual_mfa_device_name: pulumi_wasm_rust::Output<String>,
    }
    #[allow(dead_code)]
    pub struct VirtualMfaDeviceResult {
        /// The Amazon Resource Name (ARN) specifying the virtual mfa device.
        pub arn: pulumi_wasm_rust::Output<String>,
        /// The base32 seed defined as specified in [RFC3548](https://tools.ietf.org/html/rfc3548.txt). The `base_32_string_seed` is base64-encoded.
        pub base32_string_seed: pulumi_wasm_rust::Output<String>,
        /// The date and time when the virtual MFA device was enabled.
        pub enable_date: pulumi_wasm_rust::Output<String>,
        /// The path for the virtual MFA device.
        pub path: pulumi_wasm_rust::Output<Option<String>>,
        /// A QR code PNG image that encodes `otpauth://totp/$virtualMFADeviceName@$AccountName?secret=$Base32String` where `$virtualMFADeviceName` is one of the create call arguments. AccountName is the user name if set (otherwise, the account ID), and Base32String is the seed in base32 format.
        pub qr_code_png: pulumi_wasm_rust::Output<String>,
        /// Map of resource tags for the virtual mfa device. If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
        pub tags: pulumi_wasm_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// A map of tags assigned to the resource, including those inherited from the provider `default_tags` configuration block.
        pub tags_all: pulumi_wasm_rust::Output<
            std::collections::HashMap<String, String>,
        >,
        /// The associated IAM User name if the virtual MFA device is enabled.
        pub user_name: pulumi_wasm_rust::Output<String>,
        /// The name of the virtual MFA device. Use with path to uniquely identify a virtual MFA device.
        pub virtual_mfa_device_name: pulumi_wasm_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(name: &str, args: VirtualMfaDeviceArgs) -> VirtualMfaDeviceResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let path_binding = args.path.get_inner();
        let tags_binding = args.tags.get_inner();
        let virtual_mfa_device_name_binding = args.virtual_mfa_device_name.get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "aws:iam/virtualMfaDevice:VirtualMfaDevice".into(),
            name: name.to_string(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "path".into(),
                    value: &path_binding,
                },
                register_interface::ObjectField {
                    name: "tags".into(),
                    value: &tags_binding,
                },
                register_interface::ObjectField {
                    name: "virtualMfaDeviceName".into(),
                    value: &virtual_mfa_device_name_binding,
                },
            ]),
            results: Vec::from([
                register_interface::ResultField {
                    name: "arn".into(),
                },
                register_interface::ResultField {
                    name: "base32StringSeed".into(),
                },
                register_interface::ResultField {
                    name: "enableDate".into(),
                },
                register_interface::ResultField {
                    name: "path".into(),
                },
                register_interface::ResultField {
                    name: "qrCodePng".into(),
                },
                register_interface::ResultField {
                    name: "tags".into(),
                },
                register_interface::ResultField {
                    name: "tagsAll".into(),
                },
                register_interface::ResultField {
                    name: "userName".into(),
                },
                register_interface::ResultField {
                    name: "virtualMfaDeviceName".into(),
                },
            ]),
        };
        let o = register_interface::register(&request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        VirtualMfaDeviceResult {
            arn: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("arn").unwrap(),
            ),
            base32_string_seed: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("base32StringSeed").unwrap(),
            ),
            enable_date: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("enableDate").unwrap(),
            ),
            path: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("path").unwrap(),
            ),
            qr_code_png: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("qrCodePng").unwrap(),
            ),
            tags: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("tags").unwrap(),
            ),
            tags_all: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("tagsAll").unwrap(),
            ),
            user_name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("userName").unwrap(),
            ),
            virtual_mfa_device_name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("virtualMfaDeviceName").unwrap(),
            ),
        }
    }
}