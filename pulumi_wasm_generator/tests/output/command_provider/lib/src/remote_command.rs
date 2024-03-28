

pub struct CommandArgs<'a> {
    pub name: &'a str,
    pub connection: Ref&lt;#/types/command:remote:Connection&gt;,
    pub create: String,
    pub delete: String,
    pub environment: Object,
    pub stdin: String,
    pub triggers: Vec,
    pub update: String,

}