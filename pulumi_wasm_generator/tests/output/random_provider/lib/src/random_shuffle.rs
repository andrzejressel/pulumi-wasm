

pub struct RandomShuffleArgs<'a> {
    pub name: &'a str,
    pub inputs: Vec,
    pub keepers: Object,
    pub resultCount: int,
    pub seed: String,

}