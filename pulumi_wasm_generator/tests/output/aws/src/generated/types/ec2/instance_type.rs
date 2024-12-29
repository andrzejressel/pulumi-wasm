#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, Debug, PartialEq, Clone)]
#[allow(dead_code)]
pub enum InstanceType {
    #[serde(rename = "a1.2xlarge")]
    A1_2XLarge,
    #[serde(rename = "a1.4xlarge")]
    A1_4XLarge,
    #[serde(rename = "a1.large")]
    A1_Large,
    #[serde(rename = "a1.medium")]
    A1_Medium,
    #[serde(rename = "a1.metal")]
    A1_Metal,
    #[serde(rename = "a1.xlarge")]
    A1_XLarge,
    #[serde(rename = "c1.medium")]
    C1_Medium,
    #[serde(rename = "c1.xlarge")]
    C1_XLarge,
    #[serde(rename = "c3.2xlarge")]
    C3_2XLarge,
    #[serde(rename = "c3.4xlarge")]
    C3_4XLarge,
    #[serde(rename = "c3.8xlarge")]
    C3_8XLarge,
    #[serde(rename = "c3.large")]
    C3_Large,
    #[serde(rename = "c3.xlarge")]
    C3_XLarge,
    #[serde(rename = "c4.2xlarge")]
    C4_2XLarge,
    #[serde(rename = "c4.4xlarge")]
    C4_4XLarge,
    #[serde(rename = "c4.8xlarge")]
    C4_8XLarge,
    #[serde(rename = "c4.large")]
    C4_Large,
    #[serde(rename = "c4.xlarge")]
    C4_XLarge,
    #[serde(rename = "c5.12xlarge")]
    C5_12XLarge,
    #[serde(rename = "c5.18xlarge")]
    C5_18XLarge,
    #[serde(rename = "c5.24xlarge")]
    C5_24XLarge,
    #[serde(rename = "c5.2xlarge")]
    C5_2XLarge,
    #[serde(rename = "c5.4xlarge")]
    C5_4XLarge,
    #[serde(rename = "c5.9xlarge")]
    C5_9XLarge,
    #[serde(rename = "c5.large")]
    C5_Large,
    #[serde(rename = "c5.metal")]
    C5_Metal,
    #[serde(rename = "c5.xlarge")]
    C5_XLarge,
    #[serde(rename = "c5a.12xlarge")]
    C5a_12XLarge,
    #[serde(rename = "c5a.16xlarge")]
    C5a_16XLarge,
    #[serde(rename = "c5a.24xlarge")]
    C5a_24XLarge,
    #[serde(rename = "c5a.2xlarge")]
    C5a_2XLarge,
    #[serde(rename = "c5a.4xlarge")]
    C5a_4XLarge,
    #[serde(rename = "c5a.8xlarge")]
    C5a_8XLarge,
    #[serde(rename = "c5a.large")]
    C5a_Large,
    #[serde(rename = "c5a.xlarge")]
    C5a_XLarge,
    #[serde(rename = "c5ad.12xlarge")]
    C5ad_12XLarge,
    #[serde(rename = "c5ad.16xlarge")]
    C5ad_16XLarge,
    #[serde(rename = "c5ad.24xlarge")]
    C5ad_24XLarge,
    #[serde(rename = "c5ad.2xlarge")]
    C5ad_2XLarge,
    #[serde(rename = "c5ad.4xlarge")]
    C5ad_4XLarge,
    #[serde(rename = "c5ad.8xlarge")]
    C5ad_8XLarge,
    #[serde(rename = "c5ad.large")]
    C5ad_Large,
    #[serde(rename = "c5ad.xlarge")]
    C5ad_XLarge,
    #[serde(rename = "c5d.12xlarge")]
    C5d_12XLarge,
    #[serde(rename = "c5d.18xlarge")]
    C5d_18XLarge,
    #[serde(rename = "c5d.24xlarge")]
    C5d_24XLarge,
    #[serde(rename = "c5d.2xlarge")]
    C5d_2XLarge,
    #[serde(rename = "c5d.4xlarge")]
    C5d_4XLarge,
    #[serde(rename = "c5d.9xlarge")]
    C5d_9XLarge,
    #[serde(rename = "c5d.large")]
    C5d_Large,
    #[serde(rename = "c5d.metal")]
    C5d_Metal,
    #[serde(rename = "c5d.xlarge")]
    C5d_XLarge,
    #[serde(rename = "c5n.18xlarge")]
    C5n_18XLarge,
    #[serde(rename = "c5n.2xlarge")]
    C5n_2XLarge,
    #[serde(rename = "c5n.4xlarge")]
    C5n_4XLarge,
    #[serde(rename = "c5n.9xlarge")]
    C5n_9XLarge,
    #[serde(rename = "c5n.large")]
    C5n_Large,
    #[serde(rename = "c5n.metal")]
    C5n_Metal,
    #[serde(rename = "c5n.xlarge")]
    C5n_XLarge,
    #[serde(rename = "c6a.12xlarge")]
    C6a_12XLarge,
    #[serde(rename = "c6a.16xlarge")]
    C6a_16XLarge,
    #[serde(rename = "c6a.24xlarge")]
    C6a_24XLarge,
    #[serde(rename = "c6a.2xlarge")]
    C6a_2XLarge,
    #[serde(rename = "c6a.32xlarge")]
    C6a_32XLarge,
    #[serde(rename = "c6a.48xlarge")]
    C6a_48XLarge,
    #[serde(rename = "c6a.4xlarge")]
    C6a_4XLarge,
    #[serde(rename = "c6a.8xlarge")]
    C6a_8XLarge,
    #[serde(rename = "c6a.large")]
    C6a_Large,
    #[serde(rename = "c6a.metal")]
    C6a_Metal,
    #[serde(rename = "c6a.xlarge")]
    C6a_XLarge,
    #[serde(rename = "c6g.12xlarge")]
    C6g_12XLarge,
    #[serde(rename = "c6g.16xlarge")]
    C6g_16XLarge,
    #[serde(rename = "c6g.2xlarge")]
    C6g_2XLarge,
    #[serde(rename = "c6g.4xlarge")]
    C6g_4XLarge,
    #[serde(rename = "c6g.8xlarge")]
    C6g_8XLarge,
    #[serde(rename = "c6g.large")]
    C6g_Large,
    #[serde(rename = "c6g.medium")]
    C6g_Medium,
    #[serde(rename = "c6g.metal")]
    C6g_Metal,
    #[serde(rename = "c6g.xlarge")]
    C6g_XLarge,
    #[serde(rename = "c6gd.12xlarge")]
    C6gd_12XLarge,
    #[serde(rename = "c6gd.16xlarge")]
    C6gd_16XLarge,
    #[serde(rename = "c6gd.2xlarge")]
    C6gd_2XLarge,
    #[serde(rename = "c6gd.4xlarge")]
    C6gd_4XLarge,
    #[serde(rename = "c6gd.8xlarge")]
    C6gd_8XLarge,
    #[serde(rename = "c6gd.large")]
    C6gd_Large,
    #[serde(rename = "c6gd.medium")]
    C6gd_Medium,
    #[serde(rename = "c6gd.metal")]
    C6gd_Metal,
    #[serde(rename = "c6gd.xlarge")]
    C6gd_XLarge,
    #[serde(rename = "c6gn.12xlarge")]
    C6gn_12XLarge,
    #[serde(rename = "c6gn.16xlarge")]
    C6gn_16XLarge,
    #[serde(rename = "c6gn.2xlarge")]
    C6gn_2XLarge,
    #[serde(rename = "c6gn.4xlarge")]
    C6gn_4XLarge,
    #[serde(rename = "c6gn.8xlarge")]
    C6gn_8XLarge,
    #[serde(rename = "c6gn.large")]
    C6gn_Large,
    #[serde(rename = "c6gn.medium")]
    C6gn_Medium,
    #[serde(rename = "c6gn.xlarge")]
    C6gn_XLarge,
    #[serde(rename = "c6i.12xlarge")]
    C6i_12XLarge,
    #[serde(rename = "c6i.16xlarge")]
    C6i_16XLarge,
    #[serde(rename = "c6i.24xlarge")]
    C6i_24XLarge,
    #[serde(rename = "c6i.2xlarge")]
    C6i_2XLarge,
    #[serde(rename = "c6i.32xlarge")]
    C6i_32XLarge,
    #[serde(rename = "c6i.4xlarge")]
    C6i_4XLarge,
    #[serde(rename = "c6i.8xlarge")]
    C6i_8XLarge,
    #[serde(rename = "c6i.large")]
    C6i_Large,
    #[serde(rename = "c6i.metal")]
    C6i_Metal,
    #[serde(rename = "c6i.xlarge")]
    C6i_XLarge,
    #[serde(rename = "c6id.12xlarge")]
    C6id_12XLarge,
    #[serde(rename = "c6id.16xlarge")]
    C6id_16XLarge,
    #[serde(rename = "c6id.24xlarge")]
    C6id_24XLarge,
    #[serde(rename = "c6id.2xlarge")]
    C6id_2XLarge,
    #[serde(rename = "c6id.32xlarge")]
    C6id_32XLarge,
    #[serde(rename = "c6id.4xlarge")]
    C6id_4XLarge,
    #[serde(rename = "c6id.8xlarge")]
    C6id_8XLarge,
    #[serde(rename = "c6id.large")]
    C6id_Large,
    #[serde(rename = "c6id.metal")]
    C6id_Metal,
    #[serde(rename = "c6id.xlarge")]
    C6id_XLarge,
    #[serde(rename = "c6in.12xlarge")]
    C6in_12XLarge,
    #[serde(rename = "c6in.16xlarge")]
    C6in_16XLarge,
    #[serde(rename = "c6in.24xlarge")]
    C6in_24XLarge,
    #[serde(rename = "c6in.2xlarge")]
    C6in_2XLarge,
    #[serde(rename = "c6in.32xlarge")]
    C6in_32XLarge,
    #[serde(rename = "c6in.4xlarge")]
    C6in_4XLarge,
    #[serde(rename = "c6in.8xlarge")]
    C6in_8XLarge,
    #[serde(rename = "c6in.large")]
    C6in_Large,
    #[serde(rename = "c6in.metal")]
    C6in_Metal,
    #[serde(rename = "c6in.xlarge")]
    C6in_XLarge,
    #[serde(rename = "c7a.12xlarge")]
    C7a_12XLarge,
    #[serde(rename = "c7a.16xlarge")]
    C7a_16XLarge,
    #[serde(rename = "c7a.24xlarge")]
    C7a_24XLarge,
    #[serde(rename = "c7a.2xlarge")]
    C7a_2XLarge,
    #[serde(rename = "c7a.32xlarge")]
    C7a_32XLarge,
    #[serde(rename = "c7a.48xlarge")]
    C7a_48XLarge,
    #[serde(rename = "c7a.4xlarge")]
    C7a_4XLarge,
    #[serde(rename = "c7a.8xlarge")]
    C7a_8XLarge,
    #[serde(rename = "c7a.large")]
    C7a_Large,
    #[serde(rename = "c7a.medium")]
    C7a_Medium,
    #[serde(rename = "c7a.metal-48xl")]
    C7a_Metal_48xl,
    #[serde(rename = "c7a.xlarge")]
    C7a_XLarge,
    #[serde(rename = "c7g.12xlarge")]
    C7g_12XLarge,
    #[serde(rename = "c7g.16xlarge")]
    C7g_16XLarge,
    #[serde(rename = "c7g.2xlarge")]
    C7g_2XLarge,
    #[serde(rename = "c7g.4xlarge")]
    C7g_4XLarge,
    #[serde(rename = "c7g.8xlarge")]
    C7g_8XLarge,
    #[serde(rename = "c7g.large")]
    C7g_Large,
    #[serde(rename = "c7g.medium")]
    C7g_Medium,
    #[serde(rename = "c7g.metal")]
    C7g_Metal,
    #[serde(rename = "c7g.xlarge")]
    C7g_XLarge,
    #[serde(rename = "c7gd.12xlarge")]
    C7gd_12XLarge,
    #[serde(rename = "c7gd.16xlarge")]
    C7gd_16XLarge,
    #[serde(rename = "c7gd.2xlarge")]
    C7gd_2XLarge,
    #[serde(rename = "c7gd.4xlarge")]
    C7gd_4XLarge,
    #[serde(rename = "c7gd.8xlarge")]
    C7gd_8XLarge,
    #[serde(rename = "c7gd.large")]
    C7gd_Large,
    #[serde(rename = "c7gd.medium")]
    C7gd_Medium,
    #[serde(rename = "c7gd.metal")]
    C7gd_Metal,
    #[serde(rename = "c7gd.xlarge")]
    C7gd_XLarge,
    #[serde(rename = "c7gn.12xlarge")]
    C7gn_12XLarge,
    #[serde(rename = "c7gn.16xlarge")]
    C7gn_16XLarge,
    #[serde(rename = "c7gn.2xlarge")]
    C7gn_2XLarge,
    #[serde(rename = "c7gn.4xlarge")]
    C7gn_4XLarge,
    #[serde(rename = "c7gn.8xlarge")]
    C7gn_8XLarge,
    #[serde(rename = "c7gn.large")]
    C7gn_Large,
    #[serde(rename = "c7gn.medium")]
    C7gn_Medium,
    #[serde(rename = "c7gn.metal")]
    C7gn_Metal,
    #[serde(rename = "c7gn.xlarge")]
    C7gn_XLarge,
    #[serde(rename = "c7i.12xlarge")]
    C7i_12XLarge,
    #[serde(rename = "c7i.16xlarge")]
    C7i_16XLarge,
    #[serde(rename = "c7i.24xlarge")]
    C7i_24XLarge,
    #[serde(rename = "c7i.2xlarge")]
    C7i_2XLarge,
    #[serde(rename = "c7i.48xlarge")]
    C7i_48XLarge,
    #[serde(rename = "c7i.4xlarge")]
    C7i_4XLarge,
    #[serde(rename = "c7i.8xlarge")]
    C7i_8XLarge,
    #[serde(rename = "c7i.large")]
    C7i_Large,
    #[serde(rename = "c7i.metal-24xl")]
    C7i_Metal_24xl,
    #[serde(rename = "c7i.metal-48xl")]
    C7i_Metal_48xl,
    #[serde(rename = "c7i.xlarge")]
    C7i_XLarge,
    #[serde(rename = "d2.2xlarge")]
    D2_2XLarge,
    #[serde(rename = "d2.4xlarge")]
    D2_4XLarge,
    #[serde(rename = "d2.8xlarge")]
    D2_8XLarge,
    #[serde(rename = "d2.xlarge")]
    D2_XLarge,
    #[serde(rename = "d3.2xlarge")]
    D3_2XLarge,
    #[serde(rename = "d3.4xlarge")]
    D3_4XLarge,
    #[serde(rename = "d3.8xlarge")]
    D3_8XLarge,
    #[serde(rename = "d3.xlarge")]
    D3_XLarge,
    #[serde(rename = "d3en.12xlarge")]
    D3en_12XLarge,
    #[serde(rename = "d3en.2xlarge")]
    D3en_2XLarge,
    #[serde(rename = "d3en.4xlarge")]
    D3en_4XLarge,
    #[serde(rename = "d3en.6xlarge")]
    D3en_6XLarge,
    #[serde(rename = "d3en.8xlarge")]
    D3en_8XLarge,
    #[serde(rename = "d3en.xlarge")]
    D3en_XLarge,
    #[serde(rename = "dl1.24xlarge")]
    Dl1_24XLarge,
    #[serde(rename = "dl2q.24xlarge")]
    Dl2q_24XLarge,
    #[serde(rename = "f1.16xlarge")]
    F1_16XLarge,
    #[serde(rename = "f1.2xlarge")]
    F1_2XLarge,
    #[serde(rename = "f1.4xlarge")]
    F1_4XLarge,
    #[serde(rename = "g3.16xlarge")]
    G3_16XLarge,
    #[serde(rename = "g3.4xlarge")]
    G3_4XLarge,
    #[serde(rename = "g3.8xlarge")]
    G3_8XLarge,
    #[serde(rename = "g3s.xlarge")]
    G3s_XLarge,
    #[serde(rename = "g4ad.16xlarge")]
    G4ad_16XLarge,
    #[serde(rename = "g4ad.2xlarge")]
    G4ad_2XLarge,
    #[serde(rename = "g4ad.4xlarge")]
    G4ad_4XLarge,
    #[serde(rename = "g4ad.8xlarge")]
    G4ad_8XLarge,
    #[serde(rename = "g4ad.xlarge")]
    G4ad_XLarge,
    #[serde(rename = "g4dn.12xlarge")]
    G4dn_12XLarge,
    #[serde(rename = "g4dn.16xlarge")]
    G4dn_16XLarge,
    #[serde(rename = "g4dn.2xlarge")]
    G4dn_2XLarge,
    #[serde(rename = "g4dn.4xlarge")]
    G4dn_4XLarge,
    #[serde(rename = "g4dn.8xlarge")]
    G4dn_8XLarge,
    #[serde(rename = "g4dn.metal")]
    G4dn_Metal,
    #[serde(rename = "g4dn.xlarge")]
    G4dn_XLarge,
    #[serde(rename = "g5.12xlarge")]
    G5_12XLarge,
    #[serde(rename = "g5.16xlarge")]
    G5_16XLarge,
    #[serde(rename = "g5.24xlarge")]
    G5_24XLarge,
    #[serde(rename = "g5.2xlarge")]
    G5_2XLarge,
    #[serde(rename = "g5.48xlarge")]
    G5_48XLarge,
    #[serde(rename = "g5.4xlarge")]
    G5_4XLarge,
    #[serde(rename = "g5.8xlarge")]
    G5_8XLarge,
    #[serde(rename = "g5.xlarge")]
    G5_XLarge,
    #[serde(rename = "g5g.16xlarge")]
    G5g_16XLarge,
    #[serde(rename = "g5g.2xlarge")]
    G5g_2XLarge,
    #[serde(rename = "g5g.4xlarge")]
    G5g_4XLarge,
    #[serde(rename = "g5g.8xlarge")]
    G5g_8XLarge,
    #[serde(rename = "g5g.metal")]
    G5g_Metal,
    #[serde(rename = "g5g.xlarge")]
    G5g_XLarge,
    #[serde(rename = "g6.12xlarge")]
    G6_12XLarge,
    #[serde(rename = "g6.16xlarge")]
    G6_16XLarge,
    #[serde(rename = "g6.24xlarge")]
    G6_24XLarge,
    #[serde(rename = "g6.2xlarge")]
    G6_2XLarge,
    #[serde(rename = "g6.48xlarge")]
    G6_48XLarge,
    #[serde(rename = "g6.4xlarge")]
    G6_4XLarge,
    #[serde(rename = "g6.8xlarge")]
    G6_8XLarge,
    #[serde(rename = "g6.xlarge")]
    G6_XLarge,
    #[serde(rename = "gr6.4xlarge")]
    Gr6_4XLarge,
    #[serde(rename = "gr6.8xlarge")]
    Gr6_8XLarge,
    #[serde(rename = "h1.16xlarge")]
    H1_16XLarge,
    #[serde(rename = "h1.2xlarge")]
    H1_2XLarge,
    #[serde(rename = "h1.4xlarge")]
    H1_4XLarge,
    #[serde(rename = "h1.8xlarge")]
    H1_8XLarge,
    #[serde(rename = "i2.2xlarge")]
    I2_2XLarge,
    #[serde(rename = "i2.4xlarge")]
    I2_4XLarge,
    #[serde(rename = "i2.8xlarge")]
    I2_8XLarge,
    #[serde(rename = "i2.xlarge")]
    I2_XLarge,
    #[serde(rename = "i3.16xlarge")]
    I3_16XLarge,
    #[serde(rename = "i3.2xlarge")]
    I3_2XLarge,
    #[serde(rename = "i3.4xlarge")]
    I3_4XLarge,
    #[serde(rename = "i3.8xlarge")]
    I3_8XLarge,
    #[serde(rename = "i3.large")]
    I3_Large,
    #[serde(rename = "i3.metal")]
    I3_Metal,
    #[serde(rename = "i3.xlarge")]
    I3_XLarge,
    #[serde(rename = "i3en.12xlarge")]
    I3en_12XLarge,
    #[serde(rename = "i3en.24xlarge")]
    I3en_24XLarge,
    #[serde(rename = "i3en.2xlarge")]
    I3en_2XLarge,
    #[serde(rename = "i3en.3xlarge")]
    I3en_3XLarge,
    #[serde(rename = "i3en.6xlarge")]
    I3en_6XLarge,
    #[serde(rename = "i3en.large")]
    I3en_Large,
    #[serde(rename = "i3en.metal")]
    I3en_Metal,
    #[serde(rename = "i3en.xlarge")]
    I3en_XLarge,
    #[serde(rename = "i4g.16xlarge")]
    I4g_16XLarge,
    #[serde(rename = "i4g.2xlarge")]
    I4g_2XLarge,
    #[serde(rename = "i4g.4xlarge")]
    I4g_4XLarge,
    #[serde(rename = "i4g.8xlarge")]
    I4g_8XLarge,
    #[serde(rename = "i4g.large")]
    I4g_Large,
    #[serde(rename = "i4g.xlarge")]
    I4g_XLarge,
    #[serde(rename = "i4i.12xlarge")]
    I4i_12XLarge,
    #[serde(rename = "i4i.16xlarge")]
    I4i_16XLarge,
    #[serde(rename = "i4i.24xlarge")]
    I4i_24XLarge,
    #[serde(rename = "i4i.2xlarge")]
    I4i_2XLarge,
    #[serde(rename = "i4i.32xlarge")]
    I4i_32XLarge,
    #[serde(rename = "i4i.4xlarge")]
    I4i_4XLarge,
    #[serde(rename = "i4i.8xlarge")]
    I4i_8XLarge,
    #[serde(rename = "i4i.large")]
    I4i_Large,
    #[serde(rename = "i4i.metal")]
    I4i_Metal,
    #[serde(rename = "i4i.xlarge")]
    I4i_XLarge,
    #[serde(rename = "im4gn.16xlarge")]
    Im4gn_16XLarge,
    #[serde(rename = "im4gn.2xlarge")]
    Im4gn_2XLarge,
    #[serde(rename = "im4gn.4xlarge")]
    Im4gn_4XLarge,
    #[serde(rename = "im4gn.8xlarge")]
    Im4gn_8XLarge,
    #[serde(rename = "im4gn.large")]
    Im4gn_Large,
    #[serde(rename = "im4gn.xlarge")]
    Im4gn_XLarge,
    #[serde(rename = "inf1.24xlarge")]
    Inf1_24XLarge,
    #[serde(rename = "inf1.2xlarge")]
    Inf1_2XLarge,
    #[serde(rename = "inf1.6xlarge")]
    Inf1_6XLarge,
    #[serde(rename = "inf1.xlarge")]
    Inf1_XLarge,
    #[serde(rename = "inf2.24xlarge")]
    Inf2_24XLarge,
    #[serde(rename = "inf2.48xlarge")]
    Inf2_48XLarge,
    #[serde(rename = "inf2.8xlarge")]
    Inf2_8XLarge,
    #[serde(rename = "inf2.xlarge")]
    Inf2_XLarge,
    #[serde(rename = "is4gen.2xlarge")]
    Is4gen_2XLarge,
    #[serde(rename = "is4gen.4xlarge")]
    Is4gen_4XLarge,
    #[serde(rename = "is4gen.8xlarge")]
    Is4gen_8XLarge,
    #[serde(rename = "is4gen.large")]
    Is4gen_Large,
    #[serde(rename = "is4gen.medium")]
    Is4gen_Medium,
    #[serde(rename = "is4gen.xlarge")]
    Is4gen_XLarge,
    #[serde(rename = "m1.large")]
    M1_Large,
    #[serde(rename = "m1.medium")]
    M1_Medium,
    #[serde(rename = "m1.small")]
    M1_Small,
    #[serde(rename = "m1.xlarge")]
    M1_XLarge,
    #[serde(rename = "m2.2xlarge")]
    M2_2XLarge,
    #[serde(rename = "m2.4xlarge")]
    M2_4XLarge,
    #[serde(rename = "m2.xlarge")]
    M2_XLarge,
    #[serde(rename = "m3.2xlarge")]
    M3_2XLarge,
    #[serde(rename = "m3.large")]
    M3_Large,
    #[serde(rename = "m3.medium")]
    M3_Medium,
    #[serde(rename = "m3.xlarge")]
    M3_XLarge,
    #[serde(rename = "m4.10xlarge")]
    M4_10XLarge,
    #[serde(rename = "m4.16xlarge")]
    M4_16XLarge,
    #[serde(rename = "m4.2xlarge")]
    M4_2XLarge,
    #[serde(rename = "m4.4xlarge")]
    M4_4XLarge,
    #[serde(rename = "m4.large")]
    M4_Large,
    #[serde(rename = "m4.xlarge")]
    M4_XLarge,
    #[serde(rename = "m5.12xlarge")]
    M5_12XLarge,
    #[serde(rename = "m5.16xlarge")]
    M5_16XLarge,
    #[serde(rename = "m5.24xlarge")]
    M5_24XLarge,
    #[serde(rename = "m5.2xlarge")]
    M5_2XLarge,
    #[serde(rename = "m5.4xlarge")]
    M5_4XLarge,
    #[serde(rename = "m5.8xlarge")]
    M5_8XLarge,
    #[serde(rename = "m5.large")]
    M5_Large,
    #[serde(rename = "m5.metal")]
    M5_Metal,
    #[serde(rename = "m5.xlarge")]
    M5_XLarge,
    #[serde(rename = "m5a.12xlarge")]
    M5a_12XLarge,
    #[serde(rename = "m5a.16xlarge")]
    M5a_16XLarge,
    #[serde(rename = "m5a.24xlarge")]
    M5a_24XLarge,
    #[serde(rename = "m5a.2xlarge")]
    M5a_2XLarge,
    #[serde(rename = "m5a.4xlarge")]
    M5a_4XLarge,
    #[serde(rename = "m5a.8xlarge")]
    M5a_8XLarge,
    #[serde(rename = "m5a.large")]
    M5a_Large,
    #[serde(rename = "m5a.xlarge")]
    M5a_XLarge,
    #[serde(rename = "m5ad.12xlarge")]
    M5ad_12XLarge,
    #[serde(rename = "m5ad.16xlarge")]
    M5ad_16XLarge,
    #[serde(rename = "m5ad.24xlarge")]
    M5ad_24XLarge,
    #[serde(rename = "m5ad.2xlarge")]
    M5ad_2XLarge,
    #[serde(rename = "m5ad.4xlarge")]
    M5ad_4XLarge,
    #[serde(rename = "m5ad.8xlarge")]
    M5ad_8XLarge,
    #[serde(rename = "m5ad.large")]
    M5ad_Large,
    #[serde(rename = "m5ad.xlarge")]
    M5ad_XLarge,
    #[serde(rename = "m5d.12xlarge")]
    M5d_12XLarge,
    #[serde(rename = "m5d.16xlarge")]
    M5d_16XLarge,
    #[serde(rename = "m5d.24xlarge")]
    M5d_24XLarge,
    #[serde(rename = "m5d.2xlarge")]
    M5d_2XLarge,
    #[serde(rename = "m5d.4xlarge")]
    M5d_4XLarge,
    #[serde(rename = "m5d.8xlarge")]
    M5d_8XLarge,
    #[serde(rename = "m5d.large")]
    M5d_Large,
    #[serde(rename = "m5d.metal")]
    M5d_Metal,
    #[serde(rename = "m5d.xlarge")]
    M5d_XLarge,
    #[serde(rename = "m5dn.12xlarge")]
    M5dn_12XLarge,
    #[serde(rename = "m5dn.16xlarge")]
    M5dn_16XLarge,
    #[serde(rename = "m5dn.24xlarge")]
    M5dn_24XLarge,
    #[serde(rename = "m5dn.2xlarge")]
    M5dn_2XLarge,
    #[serde(rename = "m5dn.4xlarge")]
    M5dn_4XLarge,
    #[serde(rename = "m5dn.8xlarge")]
    M5dn_8XLarge,
    #[serde(rename = "m5dn.large")]
    M5dn_Large,
    #[serde(rename = "m5dn.metal")]
    M5dn_Metal,
    #[serde(rename = "m5dn.xlarge")]
    M5dn_XLarge,
    #[serde(rename = "m5n.12xlarge")]
    M5n_12XLarge,
    #[serde(rename = "m5n.16xlarge")]
    M5n_16XLarge,
    #[serde(rename = "m5n.24xlarge")]
    M5n_24XLarge,
    #[serde(rename = "m5n.2xlarge")]
    M5n_2XLarge,
    #[serde(rename = "m5n.4xlarge")]
    M5n_4XLarge,
    #[serde(rename = "m5n.8xlarge")]
    M5n_8XLarge,
    #[serde(rename = "m5n.large")]
    M5n_Large,
    #[serde(rename = "m5n.metal")]
    M5n_Metal,
    #[serde(rename = "m5n.xlarge")]
    M5n_XLarge,
    #[serde(rename = "m5zn.12xlarge")]
    M5zn_12XLarge,
    #[serde(rename = "m5zn.2xlarge")]
    M5zn_2XLarge,
    #[serde(rename = "m5zn.3xlarge")]
    M5zn_3XLarge,
    #[serde(rename = "m5zn.6xlarge")]
    M5zn_6XLarge,
    #[serde(rename = "m5zn.large")]
    M5zn_Large,
    #[serde(rename = "m5zn.metal")]
    M5zn_Metal,
    #[serde(rename = "m5zn.xlarge")]
    M5zn_XLarge,
    #[serde(rename = "m6a.12xlarge")]
    M6a_12XLarge,
    #[serde(rename = "m6a.16xlarge")]
    M6a_16XLarge,
    #[serde(rename = "m6a.24xlarge")]
    M6a_24XLarge,
    #[serde(rename = "m6a.2xlarge")]
    M6a_2XLarge,
    #[serde(rename = "m6a.32xlarge")]
    M6a_32XLarge,
    #[serde(rename = "m6a.48xlarge")]
    M6a_48XLarge,
    #[serde(rename = "m6a.4xlarge")]
    M6a_4XLarge,
    #[serde(rename = "m6a.8xlarge")]
    M6a_8XLarge,
    #[serde(rename = "m6a.large")]
    M6a_Large,
    #[serde(rename = "m6a.metal")]
    M6a_Metal,
    #[serde(rename = "m6a.xlarge")]
    M6a_XLarge,
    #[serde(rename = "m6g.12xlarge")]
    M6g_12XLarge,
    #[serde(rename = "m6g.16xlarge")]
    M6g_16XLarge,
    #[serde(rename = "m6g.2xlarge")]
    M6g_2XLarge,
    #[serde(rename = "m6g.4xlarge")]
    M6g_4XLarge,
    #[serde(rename = "m6g.8xlarge")]
    M6g_8XLarge,
    #[serde(rename = "m6g.large")]
    M6g_Large,
    #[serde(rename = "m6g.medium")]
    M6g_Medium,
    #[serde(rename = "m6g.metal")]
    M6g_Metal,
    #[serde(rename = "m6g.xlarge")]
    M6g_XLarge,
    #[serde(rename = "m6gd.12xlarge")]
    M6gd_12XLarge,
    #[serde(rename = "m6gd.16xlarge")]
    M6gd_16XLarge,
    #[serde(rename = "m6gd.2xlarge")]
    M6gd_2XLarge,
    #[serde(rename = "m6gd.4xlarge")]
    M6gd_4XLarge,
    #[serde(rename = "m6gd.8xlarge")]
    M6gd_8XLarge,
    #[serde(rename = "m6gd.large")]
    M6gd_Large,
    #[serde(rename = "m6gd.medium")]
    M6gd_Medium,
    #[serde(rename = "m6gd.metal")]
    M6gd_Metal,
    #[serde(rename = "m6gd.xlarge")]
    M6gd_XLarge,
    #[serde(rename = "m6i.12xlarge")]
    M6i_12XLarge,
    #[serde(rename = "m6i.16xlarge")]
    M6i_16XLarge,
    #[serde(rename = "m6i.24xlarge")]
    M6i_24XLarge,
    #[serde(rename = "m6i.2xlarge")]
    M6i_2XLarge,
    #[serde(rename = "m6i.32xlarge")]
    M6i_32XLarge,
    #[serde(rename = "m6i.4xlarge")]
    M6i_4XLarge,
    #[serde(rename = "m6i.8xlarge")]
    M6i_8XLarge,
    #[serde(rename = "m6i.large")]
    M6i_Large,
    #[serde(rename = "m6i.metal")]
    M6i_Metal,
    #[serde(rename = "m6i.xlarge")]
    M6i_XLarge,
    #[serde(rename = "m6id.12xlarge")]
    M6id_12XLarge,
    #[serde(rename = "m6id.16xlarge")]
    M6id_16XLarge,
    #[serde(rename = "m6id.24xlarge")]
    M6id_24XLarge,
    #[serde(rename = "m6id.2xlarge")]
    M6id_2XLarge,
    #[serde(rename = "m6id.32xlarge")]
    M6id_32XLarge,
    #[serde(rename = "m6id.4xlarge")]
    M6id_4XLarge,
    #[serde(rename = "m6id.8xlarge")]
    M6id_8XLarge,
    #[serde(rename = "m6id.large")]
    M6id_Large,
    #[serde(rename = "m6id.metal")]
    M6id_Metal,
    #[serde(rename = "m6id.xlarge")]
    M6id_XLarge,
    #[serde(rename = "m6idn.12xlarge")]
    M6idn_12XLarge,
    #[serde(rename = "m6idn.16xlarge")]
    M6idn_16XLarge,
    #[serde(rename = "m6idn.24xlarge")]
    M6idn_24XLarge,
    #[serde(rename = "m6idn.2xlarge")]
    M6idn_2XLarge,
    #[serde(rename = "m6idn.32xlarge")]
    M6idn_32XLarge,
    #[serde(rename = "m6idn.4xlarge")]
    M6idn_4XLarge,
    #[serde(rename = "m6idn.8xlarge")]
    M6idn_8XLarge,
    #[serde(rename = "m6idn.large")]
    M6idn_Large,
    #[serde(rename = "m6idn.metal")]
    M6idn_Metal,
    #[serde(rename = "m6idn.xlarge")]
    M6idn_XLarge,
    #[serde(rename = "m6in.12xlarge")]
    M6in_12XLarge,
    #[serde(rename = "m6in.16xlarge")]
    M6in_16XLarge,
    #[serde(rename = "m6in.24xlarge")]
    M6in_24XLarge,
    #[serde(rename = "m6in.2xlarge")]
    M6in_2XLarge,
    #[serde(rename = "m6in.32xlarge")]
    M6in_32XLarge,
    #[serde(rename = "m6in.4xlarge")]
    M6in_4XLarge,
    #[serde(rename = "m6in.8xlarge")]
    M6in_8XLarge,
    #[serde(rename = "m6in.large")]
    M6in_Large,
    #[serde(rename = "m6in.metal")]
    M6in_Metal,
    #[serde(rename = "m6in.xlarge")]
    M6in_XLarge,
    #[serde(rename = "m7a.12xlarge")]
    M7a_12XLarge,
    #[serde(rename = "m7a.16xlarge")]
    M7a_16XLarge,
    #[serde(rename = "m7a.24xlarge")]
    M7a_24XLarge,
    #[serde(rename = "m7a.2xlarge")]
    M7a_2XLarge,
    #[serde(rename = "m7a.32xlarge")]
    M7a_32XLarge,
    #[serde(rename = "m7a.48xlarge")]
    M7a_48XLarge,
    #[serde(rename = "m7a.4xlarge")]
    M7a_4XLarge,
    #[serde(rename = "m7a.8xlarge")]
    M7a_8XLarge,
    #[serde(rename = "m7a.large")]
    M7a_Large,
    #[serde(rename = "m7a.medium")]
    M7a_Medium,
    #[serde(rename = "m7a.metal-48xl")]
    M7a_Metal_48xl,
    #[serde(rename = "m7a.xlarge")]
    M7a_XLarge,
    #[serde(rename = "m7g.12xlarge")]
    M7g_12XLarge,
    #[serde(rename = "m7g.16xlarge")]
    M7g_16XLarge,
    #[serde(rename = "m7g.2xlarge")]
    M7g_2XLarge,
    #[serde(rename = "m7g.4xlarge")]
    M7g_4XLarge,
    #[serde(rename = "m7g.8xlarge")]
    M7g_8XLarge,
    #[serde(rename = "m7g.large")]
    M7g_Large,
    #[serde(rename = "m7g.medium")]
    M7g_Medium,
    #[serde(rename = "m7g.metal")]
    M7g_Metal,
    #[serde(rename = "m7g.xlarge")]
    M7g_XLarge,
    #[serde(rename = "m7gd.12xlarge")]
    M7gd_12XLarge,
    #[serde(rename = "m7gd.16xlarge")]
    M7gd_16XLarge,
    #[serde(rename = "m7gd.2xlarge")]
    M7gd_2XLarge,
    #[serde(rename = "m7gd.4xlarge")]
    M7gd_4XLarge,
    #[serde(rename = "m7gd.8xlarge")]
    M7gd_8XLarge,
    #[serde(rename = "m7gd.large")]
    M7gd_Large,
    #[serde(rename = "m7gd.medium")]
    M7gd_Medium,
    #[serde(rename = "m7gd.metal")]
    M7gd_Metal,
    #[serde(rename = "m7gd.xlarge")]
    M7gd_XLarge,
    #[serde(rename = "m7i-flex.2xlarge")]
    M7i_flex_2XLarge,
    #[serde(rename = "m7i-flex.4xlarge")]
    M7i_flex_4XLarge,
    #[serde(rename = "m7i-flex.8xlarge")]
    M7i_flex_8XLarge,
    #[serde(rename = "m7i-flex.large")]
    M7i_flex_Large,
    #[serde(rename = "m7i-flex.xlarge")]
    M7i_flex_XLarge,
    #[serde(rename = "m7i.12xlarge")]
    M7i_12XLarge,
    #[serde(rename = "m7i.16xlarge")]
    M7i_16XLarge,
    #[serde(rename = "m7i.24xlarge")]
    M7i_24XLarge,
    #[serde(rename = "m7i.2xlarge")]
    M7i_2XLarge,
    #[serde(rename = "m7i.48xlarge")]
    M7i_48XLarge,
    #[serde(rename = "m7i.4xlarge")]
    M7i_4XLarge,
    #[serde(rename = "m7i.8xlarge")]
    M7i_8XLarge,
    #[serde(rename = "m7i.large")]
    M7i_Large,
    #[serde(rename = "m7i.metal-24xl")]
    M7i_Metal_24xl,
    #[serde(rename = "m7i.metal-48xl")]
    M7i_Metal_48xl,
    #[serde(rename = "m7i.xlarge")]
    M7i_XLarge,
    #[serde(rename = "mac1.metal")]
    Mac1_Metal,
    #[serde(rename = "mac2-m2.metal")]
    Mac2_m2_Metal,
    #[serde(rename = "mac2-m2pro.metal")]
    Mac2_m2pro_Metal,
    #[serde(rename = "mac2.metal")]
    Mac2_Metal,
    #[serde(rename = "p2.16xlarge")]
    P2_16XLarge,
    #[serde(rename = "p2.8xlarge")]
    P2_8XLarge,
    #[serde(rename = "p2.xlarge")]
    P2_XLarge,
    #[serde(rename = "p3.16xlarge")]
    P3_16XLarge,
    #[serde(rename = "p3.2xlarge")]
    P3_2XLarge,
    #[serde(rename = "p3.8xlarge")]
    P3_8XLarge,
    #[serde(rename = "p3dn.24xlarge")]
    P3dn_24XLarge,
    #[serde(rename = "p4d.24xlarge")]
    P4d_24XLarge,
    #[serde(rename = "p5.48xlarge")]
    P5_48XLarge,
    #[serde(rename = "r3.2xlarge")]
    R3_2XLarge,
    #[serde(rename = "r3.4xlarge")]
    R3_4XLarge,
    #[serde(rename = "r3.8xlarge")]
    R3_8XLarge,
    #[serde(rename = "r3.large")]
    R3_Large,
    #[serde(rename = "r3.xlarge")]
    R3_XLarge,
    #[serde(rename = "r4.16xlarge")]
    R4_16XLarge,
    #[serde(rename = "r4.2xlarge")]
    R4_2XLarge,
    #[serde(rename = "r4.4xlarge")]
    R4_4XLarge,
    #[serde(rename = "r4.8xlarge")]
    R4_8XLarge,
    #[serde(rename = "r4.large")]
    R4_Large,
    #[serde(rename = "r4.xlarge")]
    R4_XLarge,
    #[serde(rename = "r5.12xlarge")]
    R5_12XLarge,
    #[serde(rename = "r5.16xlarge")]
    R5_16XLarge,
    #[serde(rename = "r5.24xlarge")]
    R5_24XLarge,
    #[serde(rename = "r5.2xlarge")]
    R5_2XLarge,
    #[serde(rename = "r5.4xlarge")]
    R5_4XLarge,
    #[serde(rename = "r5.8xlarge")]
    R5_8XLarge,
    #[serde(rename = "r5.large")]
    R5_Large,
    #[serde(rename = "r5.metal")]
    R5_Metal,
    #[serde(rename = "r5.xlarge")]
    R5_XLarge,
    #[serde(rename = "r5a.12xlarge")]
    R5a_12XLarge,
    #[serde(rename = "r5a.16xlarge")]
    R5a_16XLarge,
    #[serde(rename = "r5a.24xlarge")]
    R5a_24XLarge,
    #[serde(rename = "r5a.2xlarge")]
    R5a_2XLarge,
    #[serde(rename = "r5a.4xlarge")]
    R5a_4XLarge,
    #[serde(rename = "r5a.8xlarge")]
    R5a_8XLarge,
    #[serde(rename = "r5a.large")]
    R5a_Large,
    #[serde(rename = "r5a.xlarge")]
    R5a_XLarge,
    #[serde(rename = "r5ad.12xlarge")]
    R5ad_12XLarge,
    #[serde(rename = "r5ad.16xlarge")]
    R5ad_16XLarge,
    #[serde(rename = "r5ad.24xlarge")]
    R5ad_24XLarge,
    #[serde(rename = "r5ad.2xlarge")]
    R5ad_2XLarge,
    #[serde(rename = "r5ad.4xlarge")]
    R5ad_4XLarge,
    #[serde(rename = "r5ad.8xlarge")]
    R5ad_8XLarge,
    #[serde(rename = "r5ad.large")]
    R5ad_Large,
    #[serde(rename = "r5ad.xlarge")]
    R5ad_XLarge,
    #[serde(rename = "r5b.12xlarge")]
    R5b_12XLarge,
    #[serde(rename = "r5b.16xlarge")]
    R5b_16XLarge,
    #[serde(rename = "r5b.24xlarge")]
    R5b_24XLarge,
    #[serde(rename = "r5b.2xlarge")]
    R5b_2XLarge,
    #[serde(rename = "r5b.4xlarge")]
    R5b_4XLarge,
    #[serde(rename = "r5b.8xlarge")]
    R5b_8XLarge,
    #[serde(rename = "r5b.large")]
    R5b_Large,
    #[serde(rename = "r5b.metal")]
    R5b_Metal,
    #[serde(rename = "r5b.xlarge")]
    R5b_XLarge,
    #[serde(rename = "r5d.12xlarge")]
    R5d_12XLarge,
    #[serde(rename = "r5d.16xlarge")]
    R5d_16XLarge,
    #[serde(rename = "r5d.24xlarge")]
    R5d_24XLarge,
    #[serde(rename = "r5d.2xlarge")]
    R5d_2XLarge,
    #[serde(rename = "r5d.4xlarge")]
    R5d_4XLarge,
    #[serde(rename = "r5d.8xlarge")]
    R5d_8XLarge,
    #[serde(rename = "r5d.large")]
    R5d_Large,
    #[serde(rename = "r5d.metal")]
    R5d_Metal,
    #[serde(rename = "r5d.xlarge")]
    R5d_XLarge,
    #[serde(rename = "r5dn.12xlarge")]
    R5dn_12XLarge,
    #[serde(rename = "r5dn.16xlarge")]
    R5dn_16XLarge,
    #[serde(rename = "r5dn.24xlarge")]
    R5dn_24XLarge,
    #[serde(rename = "r5dn.2xlarge")]
    R5dn_2XLarge,
    #[serde(rename = "r5dn.4xlarge")]
    R5dn_4XLarge,
    #[serde(rename = "r5dn.8xlarge")]
    R5dn_8XLarge,
    #[serde(rename = "r5dn.large")]
    R5dn_Large,
    #[serde(rename = "r5dn.metal")]
    R5dn_Metal,
    #[serde(rename = "r5dn.xlarge")]
    R5dn_XLarge,
    #[serde(rename = "r5n.12xlarge")]
    R5n_12XLarge,
    #[serde(rename = "r5n.16xlarge")]
    R5n_16XLarge,
    #[serde(rename = "r5n.24xlarge")]
    R5n_24XLarge,
    #[serde(rename = "r5n.2xlarge")]
    R5n_2XLarge,
    #[serde(rename = "r5n.4xlarge")]
    R5n_4XLarge,
    #[serde(rename = "r5n.8xlarge")]
    R5n_8XLarge,
    #[serde(rename = "r5n.large")]
    R5n_Large,
    #[serde(rename = "r5n.metal")]
    R5n_Metal,
    #[serde(rename = "r5n.xlarge")]
    R5n_XLarge,
    #[serde(rename = "r6a.12xlarge")]
    R6a_12XLarge,
    #[serde(rename = "r6a.16xlarge")]
    R6a_16XLarge,
    #[serde(rename = "r6a.24xlarge")]
    R6a_24XLarge,
    #[serde(rename = "r6a.2xlarge")]
    R6a_2XLarge,
    #[serde(rename = "r6a.32xlarge")]
    R6a_32XLarge,
    #[serde(rename = "r6a.48xlarge")]
    R6a_48XLarge,
    #[serde(rename = "r6a.4xlarge")]
    R6a_4XLarge,
    #[serde(rename = "r6a.8xlarge")]
    R6a_8XLarge,
    #[serde(rename = "r6a.large")]
    R6a_Large,
    #[serde(rename = "r6a.metal")]
    R6a_Metal,
    #[serde(rename = "r6a.xlarge")]
    R6a_XLarge,
    #[serde(rename = "r6g.12xlarge")]
    R6g_12XLarge,
    #[serde(rename = "r6g.16xlarge")]
    R6g_16XLarge,
    #[serde(rename = "r6g.2xlarge")]
    R6g_2XLarge,
    #[serde(rename = "r6g.4xlarge")]
    R6g_4XLarge,
    #[serde(rename = "r6g.8xlarge")]
    R6g_8XLarge,
    #[serde(rename = "r6g.large")]
    R6g_Large,
    #[serde(rename = "r6g.medium")]
    R6g_Medium,
    #[serde(rename = "r6g.metal")]
    R6g_Metal,
    #[serde(rename = "r6g.xlarge")]
    R6g_XLarge,
    #[serde(rename = "r6gd.12xlarge")]
    R6gd_12XLarge,
    #[serde(rename = "r6gd.16xlarge")]
    R6gd_16XLarge,
    #[serde(rename = "r6gd.2xlarge")]
    R6gd_2XLarge,
    #[serde(rename = "r6gd.4xlarge")]
    R6gd_4XLarge,
    #[serde(rename = "r6gd.8xlarge")]
    R6gd_8XLarge,
    #[serde(rename = "r6gd.large")]
    R6gd_Large,
    #[serde(rename = "r6gd.medium")]
    R6gd_Medium,
    #[serde(rename = "r6gd.metal")]
    R6gd_Metal,
    #[serde(rename = "r6gd.xlarge")]
    R6gd_XLarge,
    #[serde(rename = "r6i.12xlarge")]
    R6i_12XLarge,
    #[serde(rename = "r6i.16xlarge")]
    R6i_16XLarge,
    #[serde(rename = "r6i.24xlarge")]
    R6i_24XLarge,
    #[serde(rename = "r6i.2xlarge")]
    R6i_2XLarge,
    #[serde(rename = "r6i.32xlarge")]
    R6i_32XLarge,
    #[serde(rename = "r6i.4xlarge")]
    R6i_4XLarge,
    #[serde(rename = "r6i.8xlarge")]
    R6i_8XLarge,
    #[serde(rename = "r6i.large")]
    R6i_Large,
    #[serde(rename = "r6i.metal")]
    R6i_Metal,
    #[serde(rename = "r6i.xlarge")]
    R6i_XLarge,
    #[serde(rename = "r6id.12xlarge")]
    R6id_12XLarge,
    #[serde(rename = "r6id.16xlarge")]
    R6id_16XLarge,
    #[serde(rename = "r6id.24xlarge")]
    R6id_24XLarge,
    #[serde(rename = "r6id.2xlarge")]
    R6id_2XLarge,
    #[serde(rename = "r6id.32xlarge")]
    R6id_32XLarge,
    #[serde(rename = "r6id.4xlarge")]
    R6id_4XLarge,
    #[serde(rename = "r6id.8xlarge")]
    R6id_8XLarge,
    #[serde(rename = "r6id.large")]
    R6id_Large,
    #[serde(rename = "r6id.metal")]
    R6id_Metal,
    #[serde(rename = "r6id.xlarge")]
    R6id_XLarge,
    #[serde(rename = "r6idn.12xlarge")]
    R6idn_12XLarge,
    #[serde(rename = "r6idn.16xlarge")]
    R6idn_16XLarge,
    #[serde(rename = "r6idn.24xlarge")]
    R6idn_24XLarge,
    #[serde(rename = "r6idn.2xlarge")]
    R6idn_2XLarge,
    #[serde(rename = "r6idn.32xlarge")]
    R6idn_32XLarge,
    #[serde(rename = "r6idn.4xlarge")]
    R6idn_4XLarge,
    #[serde(rename = "r6idn.8xlarge")]
    R6idn_8XLarge,
    #[serde(rename = "r6idn.large")]
    R6idn_Large,
    #[serde(rename = "r6idn.metal")]
    R6idn_Metal,
    #[serde(rename = "r6idn.xlarge")]
    R6idn_XLarge,
    #[serde(rename = "r6in.12xlarge")]
    R6in_12XLarge,
    #[serde(rename = "r6in.16xlarge")]
    R6in_16XLarge,
    #[serde(rename = "r6in.24xlarge")]
    R6in_24XLarge,
    #[serde(rename = "r6in.2xlarge")]
    R6in_2XLarge,
    #[serde(rename = "r6in.32xlarge")]
    R6in_32XLarge,
    #[serde(rename = "r6in.4xlarge")]
    R6in_4XLarge,
    #[serde(rename = "r6in.8xlarge")]
    R6in_8XLarge,
    #[serde(rename = "r6in.large")]
    R6in_Large,
    #[serde(rename = "r6in.metal")]
    R6in_Metal,
    #[serde(rename = "r6in.xlarge")]
    R6in_XLarge,
    #[serde(rename = "r7a.12xlarge")]
    R7a_12XLarge,
    #[serde(rename = "r7a.16xlarge")]
    R7a_16XLarge,
    #[serde(rename = "r7a.24xlarge")]
    R7a_24XLarge,
    #[serde(rename = "r7a.2xlarge")]
    R7a_2XLarge,
    #[serde(rename = "r7a.32xlarge")]
    R7a_32XLarge,
    #[serde(rename = "r7a.48xlarge")]
    R7a_48XLarge,
    #[serde(rename = "r7a.4xlarge")]
    R7a_4XLarge,
    #[serde(rename = "r7a.8xlarge")]
    R7a_8XLarge,
    #[serde(rename = "r7a.large")]
    R7a_Large,
    #[serde(rename = "r7a.medium")]
    R7a_Medium,
    #[serde(rename = "r7a.metal-48xl")]
    R7a_Metal_48xl,
    #[serde(rename = "r7a.xlarge")]
    R7a_XLarge,
    #[serde(rename = "r7g.12xlarge")]
    R7g_12XLarge,
    #[serde(rename = "r7g.16xlarge")]
    R7g_16XLarge,
    #[serde(rename = "r7g.2xlarge")]
    R7g_2XLarge,
    #[serde(rename = "r7g.4xlarge")]
    R7g_4XLarge,
    #[serde(rename = "r7g.8xlarge")]
    R7g_8XLarge,
    #[serde(rename = "r7g.large")]
    R7g_Large,
    #[serde(rename = "r7g.medium")]
    R7g_Medium,
    #[serde(rename = "r7g.metal")]
    R7g_Metal,
    #[serde(rename = "r7g.xlarge")]
    R7g_XLarge,
    #[serde(rename = "r7gd.12xlarge")]
    R7gd_12XLarge,
    #[serde(rename = "r7gd.16xlarge")]
    R7gd_16XLarge,
    #[serde(rename = "r7gd.2xlarge")]
    R7gd_2XLarge,
    #[serde(rename = "r7gd.4xlarge")]
    R7gd_4XLarge,
    #[serde(rename = "r7gd.8xlarge")]
    R7gd_8XLarge,
    #[serde(rename = "r7gd.large")]
    R7gd_Large,
    #[serde(rename = "r7gd.medium")]
    R7gd_Medium,
    #[serde(rename = "r7gd.metal")]
    R7gd_Metal,
    #[serde(rename = "r7gd.xlarge")]
    R7gd_XLarge,
    #[serde(rename = "r7i.12xlarge")]
    R7i_12XLarge,
    #[serde(rename = "r7i.16xlarge")]
    R7i_16XLarge,
    #[serde(rename = "r7i.24xlarge")]
    R7i_24XLarge,
    #[serde(rename = "r7i.2xlarge")]
    R7i_2XLarge,
    #[serde(rename = "r7i.48xlarge")]
    R7i_48XLarge,
    #[serde(rename = "r7i.4xlarge")]
    R7i_4XLarge,
    #[serde(rename = "r7i.8xlarge")]
    R7i_8XLarge,
    #[serde(rename = "r7i.large")]
    R7i_Large,
    #[serde(rename = "r7i.metal-24xl")]
    R7i_Metal_24xl,
    #[serde(rename = "r7i.metal-48xl")]
    R7i_Metal_48xl,
    #[serde(rename = "r7i.xlarge")]
    R7i_XLarge,
    #[serde(rename = "r7iz.12xlarge")]
    R7iz_12XLarge,
    #[serde(rename = "r7iz.16xlarge")]
    R7iz_16XLarge,
    #[serde(rename = "r7iz.2xlarge")]
    R7iz_2XLarge,
    #[serde(rename = "r7iz.32xlarge")]
    R7iz_32XLarge,
    #[serde(rename = "r7iz.4xlarge")]
    R7iz_4XLarge,
    #[serde(rename = "r7iz.8xlarge")]
    R7iz_8XLarge,
    #[serde(rename = "r7iz.large")]
    R7iz_Large,
    #[serde(rename = "r7iz.metal-16xl")]
    R7iz_Metal_16xl,
    #[serde(rename = "r7iz.metal-32xl")]
    R7iz_Metal_32xl,
    #[serde(rename = "r7iz.xlarge")]
    R7iz_XLarge,
    #[serde(rename = "t1.micro")]
    T1_Micro,
    #[serde(rename = "t2.2xlarge")]
    T2_2XLarge,
    #[serde(rename = "t2.large")]
    T2_Large,
    #[serde(rename = "t2.medium")]
    T2_Medium,
    #[serde(rename = "t2.micro")]
    T2_Micro,
    #[serde(rename = "t2.nano")]
    T2_Nano,
    #[serde(rename = "t2.small")]
    T2_Small,
    #[serde(rename = "t2.xlarge")]
    T2_XLarge,
    #[serde(rename = "t3.2xlarge")]
    T3_2XLarge,
    #[serde(rename = "t3.large")]
    T3_Large,
    #[serde(rename = "t3.medium")]
    T3_Medium,
    #[serde(rename = "t3.micro")]
    T3_Micro,
    #[serde(rename = "t3.nano")]
    T3_Nano,
    #[serde(rename = "t3.small")]
    T3_Small,
    #[serde(rename = "t3.xlarge")]
    T3_XLarge,
    #[serde(rename = "t3a.2xlarge")]
    T3a_2XLarge,
    #[serde(rename = "t3a.large")]
    T3a_Large,
    #[serde(rename = "t3a.medium")]
    T3a_Medium,
    #[serde(rename = "t3a.micro")]
    T3a_Micro,
    #[serde(rename = "t3a.nano")]
    T3a_Nano,
    #[serde(rename = "t3a.small")]
    T3a_Small,
    #[serde(rename = "t3a.xlarge")]
    T3a_XLarge,
    #[serde(rename = "t4g.2xlarge")]
    T4g_2XLarge,
    #[serde(rename = "t4g.large")]
    T4g_Large,
    #[serde(rename = "t4g.medium")]
    T4g_Medium,
    #[serde(rename = "t4g.micro")]
    T4g_Micro,
    #[serde(rename = "t4g.nano")]
    T4g_Nano,
    #[serde(rename = "t4g.small")]
    T4g_Small,
    #[serde(rename = "t4g.xlarge")]
    T4g_XLarge,
    #[serde(rename = "trn1.2xlarge")]
    Trn1_2XLarge,
    #[serde(rename = "trn1.32xlarge")]
    Trn1_32XLarge,
    #[serde(rename = "trn1n.32xlarge")]
    Trn1n_32XLarge,
    #[serde(rename = "u-12tb1.112xlarge")]
    U_12tb1_112XLarge,
    #[serde(rename = "u-18tb1.112xlarge")]
    U_18tb1_112XLarge,
    #[serde(rename = "u-24tb1.112xlarge")]
    U_24tb1_112XLarge,
    #[serde(rename = "u-3tb1.56xlarge")]
    U_3tb1_56XLarge,
    #[serde(rename = "u-6tb1.112xlarge")]
    U_6tb1_112XLarge,
    #[serde(rename = "u-6tb1.56xlarge")]
    U_6tb1_56XLarge,
    #[serde(rename = "u-9tb1.112xlarge")]
    U_9tb1_112XLarge,
    #[serde(rename = "vt1.24xlarge")]
    Vt1_24XLarge,
    #[serde(rename = "vt1.3xlarge")]
    Vt1_3XLarge,
    #[serde(rename = "vt1.6xlarge")]
    Vt1_6XLarge,
    #[serde(rename = "x1.16xlarge")]
    X1_16XLarge,
    #[serde(rename = "x1.32xlarge")]
    X1_32XLarge,
    #[serde(rename = "x1e.16xlarge")]
    X1e_16XLarge,
    #[serde(rename = "x1e.2xlarge")]
    X1e_2XLarge,
    #[serde(rename = "x1e.32xlarge")]
    X1e_32XLarge,
    #[serde(rename = "x1e.4xlarge")]
    X1e_4XLarge,
    #[serde(rename = "x1e.8xlarge")]
    X1e_8XLarge,
    #[serde(rename = "x1e.xlarge")]
    X1e_XLarge,
    #[serde(rename = "x2gd.12xlarge")]
    X2gd_12XLarge,
    #[serde(rename = "x2gd.16xlarge")]
    X2gd_16XLarge,
    #[serde(rename = "x2gd.2xlarge")]
    X2gd_2XLarge,
    #[serde(rename = "x2gd.4xlarge")]
    X2gd_4XLarge,
    #[serde(rename = "x2gd.8xlarge")]
    X2gd_8XLarge,
    #[serde(rename = "x2gd.large")]
    X2gd_Large,
    #[serde(rename = "x2gd.medium")]
    X2gd_Medium,
    #[serde(rename = "x2gd.metal")]
    X2gd_Metal,
    #[serde(rename = "x2gd.xlarge")]
    X2gd_XLarge,
    #[serde(rename = "x2idn.16xlarge")]
    X2idn_16XLarge,
    #[serde(rename = "x2idn.24xlarge")]
    X2idn_24XLarge,
    #[serde(rename = "x2idn.32xlarge")]
    X2idn_32XLarge,
    #[serde(rename = "x2idn.metal")]
    X2idn_Metal,
    #[serde(rename = "x2iedn.16xlarge")]
    X2iedn_16XLarge,
    #[serde(rename = "x2iedn.24xlarge")]
    X2iedn_24XLarge,
    #[serde(rename = "x2iedn.2xlarge")]
    X2iedn_2XLarge,
    #[serde(rename = "x2iedn.32xlarge")]
    X2iedn_32XLarge,
    #[serde(rename = "x2iedn.4xlarge")]
    X2iedn_4XLarge,
    #[serde(rename = "x2iedn.8xlarge")]
    X2iedn_8XLarge,
    #[serde(rename = "x2iedn.metal")]
    X2iedn_Metal,
    #[serde(rename = "x2iedn.xlarge")]
    X2iedn_XLarge,
    #[serde(rename = "x2iezn.12xlarge")]
    X2iezn_12XLarge,
    #[serde(rename = "x2iezn.2xlarge")]
    X2iezn_2XLarge,
    #[serde(rename = "x2iezn.4xlarge")]
    X2iezn_4XLarge,
    #[serde(rename = "x2iezn.6xlarge")]
    X2iezn_6XLarge,
    #[serde(rename = "x2iezn.8xlarge")]
    X2iezn_8XLarge,
    #[serde(rename = "x2iezn.metal")]
    X2iezn_Metal,
    #[serde(rename = "z1d.12xlarge")]
    Z1d_12XLarge,
    #[serde(rename = "z1d.2xlarge")]
    Z1d_2XLarge,
    #[serde(rename = "z1d.3xlarge")]
    Z1d_3XLarge,
    #[serde(rename = "z1d.6xlarge")]
    Z1d_6XLarge,
    #[serde(rename = "z1d.large")]
    Z1d_Large,
    #[serde(rename = "z1d.metal")]
    Z1d_Metal,
    #[serde(rename = "z1d.xlarge")]
    Z1d_XLarge,
    #[serde(rename = "u-12tb1.metal")]
    U_12tb1Metal,
    #[serde(rename = "u-6tb1.metal")]
    U_6tb1Metal,
    #[serde(rename = "u-9tb1.metal")]
    U_9tb1Metal,
    #[serde(rename = "hs1.8xlarge")]
    Hs1_8XLarge,
    #[serde(rename = "m5ad.xlarge")]
    M5as_XLarge,
    #[serde(rename = "c7a.metal-48xl")]
    C7a_Metal,
    #[serde(rename = "m7a.metal-48xl")]
    M7a_Metal,
    #[serde(rename = "cc2.8xlarge")]
    Cc2_8XLarge,
    #[serde(rename = "g2.2xlarge")]
    G2_2XLarge,
    #[serde(rename = "g2.8xlarge")]
    G2_8XLarge,
}
