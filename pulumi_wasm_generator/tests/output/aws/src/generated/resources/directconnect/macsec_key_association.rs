/// Provides a MAC Security (MACSec) secret key resource for use with Direct Connect. See [MACsec prerequisites](https://docs.aws.amazon.com/directconnect/latest/UserGuide/direct-connect-mac-sec-getting-started.html#mac-sec-prerequisites) for information about MAC Security (MACsec) prerequisites.
///
/// Creating this resource will also create a resource of type `aws.secretsmanager.Secret` which is managed by Direct Connect. While you can import this resource into your state, because this secret is managed by Direct Connect, you will not be able to make any modifications to it. See [How AWS Direct Connect uses AWS Secrets Manager](https://docs.aws.amazon.com/secretsmanager/latest/userguide/integrating_how-services-use-secrets_directconnect.html) for details.
///
/// > **Note:** All arguments including `ckn` and `cak` will be stored in the raw state as plain-text.
/// > **Note:** The `secret_arn` argument can only be used to reference a previously created MACSec key. You cannot associate a Secrets Manager secret created outside of the `aws.directconnect.MacsecKeyAssociation` resource.
///
/// ## Example Usage
///
/// ### Create MACSec key with CKN and CAK
///
/// ```ignore
/// use pulumi_wasm_rust::Output;
/// use pulumi_wasm_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let example = get_connection::invoke(
///         GetConnectionArgs::builder().name("tf-dx-connection").build_struct(),
///     );
///     let test = macsec_key_association::create(
///         "test",
///         MacsecKeyAssociationArgs::builder()
///             .cak("abcdef0123456789abcdef0123456789abcdef0123456789abcdef0123456789")
///             .ckn("0123456789abcdef0123456789abcdef0123456789abcdef0123456789abcdef")
///             .connection_id("${example.id}")
///             .build_struct(),
///     );
/// }
/// ```
///
/// ### Create MACSec key with existing Secrets Manager secret
///
/// ```ignore
/// use pulumi_wasm_rust::Output;
/// use pulumi_wasm_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let example = get_connection::invoke(
///         GetConnectionArgs::builder().name("tf-dx-connection").build_struct(),
///     );
///     let exampleGetSecret = get_secret::invoke(
///         GetSecretArgs::builder()
///             .name(
///                 "directconnect!prod/us-east-1/directconnect/0123456789abcdef0123456789abcdef0123456789abcdef0123456789abcdef",
///             )
///             .build_struct(),
///     );
///     let test = macsec_key_association::create(
///         "test",
///         MacsecKeyAssociationArgs::builder()
///             .connection_id("${example.id}")
///             .secret_arn("${exampleGetSecret.arn}")
///             .build_struct(),
///     );
/// }
/// ```
pub mod macsec_key_association {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct MacsecKeyAssociationArgs {
        /// The MAC Security (MACsec) CAK to associate with the dedicated connection. The valid values are 64 hexadecimal characters (0-9, A-E). Required if using `ckn`.
        #[builder(into, default)]
        pub cak: pulumi_wasm_rust::Output<Option<String>>,
        /// The MAC Security (MACsec) CKN to associate with the dedicated connection. The valid values are 64 hexadecimal characters (0-9, A-E). Required if using `cak`.
        #[builder(into, default)]
        pub ckn: pulumi_wasm_rust::Output<Option<String>>,
        /// The ID of the dedicated Direct Connect connection. The connection must be a dedicated connection in the `AVAILABLE` state.
        #[builder(into)]
        pub connection_id: pulumi_wasm_rust::Output<String>,
        /// The Amazon Resource Name (ARN) of the MAC Security (MACsec) secret key to associate with the dedicated connection.
        ///
        /// > **Note:** `ckn` and `cak` are mutually exclusive with `secret_arn` - these arguments cannot be used together. If you use `ckn` and `cak`, you should not use `secret_arn`. If you use the `secret_arn` argument to reference an existing MAC Security (MACSec) secret key, you should not use `ckn` or `cak`.
        #[builder(into, default)]
        pub secret_arn: pulumi_wasm_rust::Output<Option<String>>,
    }
    #[allow(dead_code)]
    pub struct MacsecKeyAssociationResult {
        /// The MAC Security (MACsec) CAK to associate with the dedicated connection. The valid values are 64 hexadecimal characters (0-9, A-E). Required if using `ckn`.
        pub cak: pulumi_wasm_rust::Output<Option<String>>,
        /// The MAC Security (MACsec) CKN to associate with the dedicated connection. The valid values are 64 hexadecimal characters (0-9, A-E). Required if using `cak`.
        pub ckn: pulumi_wasm_rust::Output<String>,
        /// The ID of the dedicated Direct Connect connection. The connection must be a dedicated connection in the `AVAILABLE` state.
        pub connection_id: pulumi_wasm_rust::Output<String>,
        /// The Amazon Resource Name (ARN) of the MAC Security (MACsec) secret key to associate with the dedicated connection.
        ///
        /// > **Note:** `ckn` and `cak` are mutually exclusive with `secret_arn` - these arguments cannot be used together. If you use `ckn` and `cak`, you should not use `secret_arn`. If you use the `secret_arn` argument to reference an existing MAC Security (MACSec) secret key, you should not use `ckn` or `cak`.
        pub secret_arn: pulumi_wasm_rust::Output<String>,
        /// The date in UTC format that the MAC Security (MACsec) secret key takes effect.
        pub start_on: pulumi_wasm_rust::Output<String>,
        /// The state of the MAC Security (MACsec) secret key. The possible values are: associating, associated, disassociating, disassociated. See [MacSecKey](https://docs.aws.amazon.com/directconnect/latest/APIReference/API_MacSecKey.html#DX-Type-MacSecKey-state) for descriptions of each state.
        pub state: pulumi_wasm_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        name: &str,
        args: MacsecKeyAssociationArgs,
    ) -> MacsecKeyAssociationResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let cak_binding = args.cak.get_inner();
        let ckn_binding = args.ckn.get_inner();
        let connection_id_binding = args.connection_id.get_inner();
        let secret_arn_binding = args.secret_arn.get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "aws:directconnect/macsecKeyAssociation:MacsecKeyAssociation".into(),
            name: name.to_string(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "cak".into(),
                    value: &cak_binding,
                },
                register_interface::ObjectField {
                    name: "ckn".into(),
                    value: &ckn_binding,
                },
                register_interface::ObjectField {
                    name: "connectionId".into(),
                    value: &connection_id_binding,
                },
                register_interface::ObjectField {
                    name: "secretArn".into(),
                    value: &secret_arn_binding,
                },
            ]),
            results: Vec::from([
                register_interface::ResultField {
                    name: "cak".into(),
                },
                register_interface::ResultField {
                    name: "ckn".into(),
                },
                register_interface::ResultField {
                    name: "connectionId".into(),
                },
                register_interface::ResultField {
                    name: "secretArn".into(),
                },
                register_interface::ResultField {
                    name: "startOn".into(),
                },
                register_interface::ResultField {
                    name: "state".into(),
                },
            ]),
        };
        let o = register_interface::register(&request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        MacsecKeyAssociationResult {
            cak: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("cak").unwrap(),
            ),
            ckn: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("ckn").unwrap(),
            ),
            connection_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("connectionId").unwrap(),
            ),
            secret_arn: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("secretArn").unwrap(),
            ),
            start_on: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("startOn").unwrap(),
            ),
            state: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("state").unwrap(),
            ),
        }
    }
}