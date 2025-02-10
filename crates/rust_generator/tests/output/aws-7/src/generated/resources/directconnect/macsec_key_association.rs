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
/// ```yaml
/// resources:
///   test:
///     type: aws:directconnect:MacsecKeyAssociation
///     properties:
///       connectionId: ${example.id}
///       ckn: 0123456789abcdef0123456789abcdef0123456789abcdef0123456789abcdef
///       cak: abcdef0123456789abcdef0123456789abcdef0123456789abcdef0123456789
/// variables:
///   example:
///     fn::invoke:
///       function: aws:directconnect:getConnection
///       arguments:
///         name: tf-dx-connection
/// ```
///
/// ### Create MACSec key with existing Secrets Manager secret
///
/// ```yaml
/// resources:
///   test:
///     type: aws:directconnect:MacsecKeyAssociation
///     properties:
///       connectionId: ${example.id}
///       secretArn: ${exampleGetSecret.arn}
/// variables:
///   example:
///     fn::invoke:
///       function: aws:directconnect:getConnection
///       arguments:
///         name: tf-dx-connection
///   exampleGetSecret:
///     fn::invoke:
///       function: aws:secretsmanager:getSecret
///       arguments:
///         name: directconnect!prod/us-east-1/directconnect/0123456789abcdef0123456789abcdef0123456789abcdef0123456789abcdef
/// ```
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod macsec_key_association {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct MacsecKeyAssociationArgs {
        /// The MAC Security (MACsec) CAK to associate with the dedicated connection. The valid values are 64 hexadecimal characters (0-9, A-E). Required if using `ckn`.
        #[builder(into, default)]
        pub cak: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The MAC Security (MACsec) CKN to associate with the dedicated connection. The valid values are 64 hexadecimal characters (0-9, A-E). Required if using `cak`.
        #[builder(into, default)]
        pub ckn: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The ID of the dedicated Direct Connect connection. The connection must be a dedicated connection in the `AVAILABLE` state.
        #[builder(into)]
        pub connection_id: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The Amazon Resource Name (ARN) of the MAC Security (MACsec) secret key to associate with the dedicated connection.
        ///
        /// > **Note:** `ckn` and `cak` are mutually exclusive with `secret_arn` - these arguments cannot be used together. If you use `ckn` and `cak`, you should not use `secret_arn`. If you use the `secret_arn` argument to reference an existing MAC Security (MACSec) secret key, you should not use `ckn` or `cak`.
        #[builder(into, default)]
        pub secret_arn: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
    }
    #[allow(dead_code)]
    pub struct MacsecKeyAssociationResult {
        /// The MAC Security (MACsec) CAK to associate with the dedicated connection. The valid values are 64 hexadecimal characters (0-9, A-E). Required if using `ckn`.
        pub cak: pulumi_gestalt_rust::Output<Option<String>>,
        /// The MAC Security (MACsec) CKN to associate with the dedicated connection. The valid values are 64 hexadecimal characters (0-9, A-E). Required if using `cak`.
        pub ckn: pulumi_gestalt_rust::Output<String>,
        /// The ID of the dedicated Direct Connect connection. The connection must be a dedicated connection in the `AVAILABLE` state.
        pub connection_id: pulumi_gestalt_rust::Output<String>,
        /// The Amazon Resource Name (ARN) of the MAC Security (MACsec) secret key to associate with the dedicated connection.
        ///
        /// > **Note:** `ckn` and `cak` are mutually exclusive with `secret_arn` - these arguments cannot be used together. If you use `ckn` and `cak`, you should not use `secret_arn`. If you use the `secret_arn` argument to reference an existing MAC Security (MACSec) secret key, you should not use `ckn` or `cak`.
        pub secret_arn: pulumi_gestalt_rust::Output<String>,
        /// The date in UTC format that the MAC Security (MACsec) secret key takes effect.
        pub start_on: pulumi_gestalt_rust::Output<String>,
        /// The state of the MAC Security (MACsec) secret key. The possible values are: associating, associated, disassociating, disassociated. See [MacSecKey](https://docs.aws.amazon.com/directconnect/latest/APIReference/API_MacSecKey.html#DX-Type-MacSecKey-state) for descriptions of each state.
        pub state: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: MacsecKeyAssociationArgs,
    ) -> MacsecKeyAssociationResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let cak_binding = args.cak.get_output(context);
        let ckn_binding = args.ckn.get_output(context);
        let connection_id_binding = args.connection_id.get_output(context);
        let secret_arn_binding = args.secret_arn.get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "aws:directconnect/macsecKeyAssociation:MacsecKeyAssociation".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "cak".into(),
                    value: cak_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "ckn".into(),
                    value: ckn_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "connectionId".into(),
                    value: connection_id_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "secretArn".into(),
                    value: secret_arn_binding.get_id(),
                },
            ],
        };
        let o = context.register_resource(request);
        MacsecKeyAssociationResult {
            cak: o.get_field("cak"),
            ckn: o.get_field("ckn"),
            connection_id: o.get_field("connectionId"),
            secret_arn: o.get_field("secretArn"),
            start_on: o.get_field("startOn"),
            state: o.get_field("state"),
        }
    }
}
