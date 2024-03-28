

pub struct CommandArgs<'a> {
    pub name: &'a str,
    pub archivePaths: Vec,
    pub assetPaths: Vec,
    pub create: String,
    pub delete: String,
    pub dir: String,
    pub environment: Object,
    pub interpreter: Vec,
    pub stdin: String,
    pub triggers: Vec,
    pub update: String,

}