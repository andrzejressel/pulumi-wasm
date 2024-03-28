

pub struct RandomIntegerArgs<'a> {
    pub name: &'a str,
    pub keepers: Object,
    pub max: int,
    pub min: int,
    pub seed: String,

}