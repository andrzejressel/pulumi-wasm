#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct PoolCertificate {
    /// The ID of the Batch Certificate to install on the Batch Pool, which must be inside the same Batch Account.
    #[builder(into)]
    #[serde(rename = "id")]
    pub r#id: Box<String>,
    /// The location of the certificate store on the compute node into which to install the certificate. Possible values are `CurrentUser` or `LocalMachine`.
    /// 
    /// > **NOTE:** This property is applicable only for pools configured with Windows nodes (that is, created with cloudServiceConfiguration, or with virtualMachineConfiguration using a Windows image reference). For Linux compute nodes, the certificates are stored in a directory inside the task working directory and an environment variable `AZ_BATCH_CERTIFICATES_DIR` is supplied to the task to query for this location. For certificates with visibility of `remoteUser`, a 'certs' directory is created in the user's home directory (e.g., `/home/{user-name}/certs`) and certificates are placed in that directory.
    #[builder(into)]
    #[serde(rename = "storeLocation")]
    pub r#store_location: Box<String>,
    /// The name of the certificate store on the compute node into which to install the certificate. This property is applicable only for pools configured with Windows nodes (that is, created with cloudServiceConfiguration, or with virtualMachineConfiguration using a Windows image reference). Common store names include: `My`, `Root`, `CA`, `Trust`, `Disallowed`, `TrustedPeople`, `TrustedPublisher`, `AuthRoot`, `AddressBook`, but any custom store name can also be used.
    #[builder(into, default)]
    #[serde(rename = "storeName")]
    pub r#store_name: Box<Option<String>>,
    /// Which user accounts on the compute node should have access to the private data of the certificate. Possible values are `StartTask`, `Task` and `RemoteUser`.
    #[builder(into, default)]
    #[serde(rename = "visibilities")]
    pub r#visibilities: Box<Option<Vec<String>>>,
}
