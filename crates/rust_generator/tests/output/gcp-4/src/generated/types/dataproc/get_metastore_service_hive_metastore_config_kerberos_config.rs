#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct GetMetastoreServiceHiveMetastoreConfigKerberosConfig {
    /// A Kerberos keytab file that can be used to authenticate a service principal with a Kerberos Key Distribution Center (KDC).
    #[builder(into)]
    #[serde(rename = "keytabs")]
    pub r#keytabs: Box<Vec<super::super::types::dataproc::GetMetastoreServiceHiveMetastoreConfigKerberosConfigKeytab>>,
    /// A Cloud Storage URI that specifies the path to a krb5.conf file. It is of the form gs://{bucket_name}/path/to/krb5.conf, although the file does not need to be named krb5.conf explicitly.
    #[builder(into)]
    #[serde(rename = "krb5ConfigGcsUri")]
    pub r#krb_5_config_gcs_uri: Box<String>,
    /// A Kerberos principal that exists in the both the keytab the KDC to authenticate as. A typical principal is of the form "primary/instance@REALM", but there is no exact format.
    #[builder(into)]
    #[serde(rename = "principal")]
    pub r#principal: Box<String>,
}
