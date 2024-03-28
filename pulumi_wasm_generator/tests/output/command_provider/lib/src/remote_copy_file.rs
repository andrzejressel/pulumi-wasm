

pub struct CopyFileArgs<'a> {
    pub name: &'a str,
    pub connection: Ref&lt;#/types/command:remote:Connection&gt;,
    pub localPath: String,
    pub remotePath: String,
    pub triggers: Vec,

}