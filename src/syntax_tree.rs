pub struct FractranInput {
    pub init: u128,
    pub frac_list: Vec<(u128, u128)>,
}

impl FractranInput {
    pub fn new(init: u128, frac_list: Vec<(u128, u128)>) -> Self {
        Self { init, frac_list }
    }
}
