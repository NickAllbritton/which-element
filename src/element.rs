#[derive(Clone)]
pub struct Element {
    pub atomic_number: String, //u8
    pub symbol: String,
    pub name: String,
    pub group: String, //u8
    pub mass: String, //f32
    pub natural: bool
    //category: String,
}

impl Element {
    pub fn new(atomic_number: String,
                symbol: String,
                name: String,
                group: String,
                mass: String,
                natural: bool) -> Self {
        Self {
            atomic_number: atomic_number,
            symbol: symbol,
            name: name,
            group: group,
            mass: mass,
            natural: natural
        }
    }
}
