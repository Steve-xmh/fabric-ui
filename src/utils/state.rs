struct State<T> {
    pub value: T,
    pub changed: bool,
}

impl<T: Default> Default for State<T> {
    fn default() -> Self {
        Self {
            value: Default::default(),
            changed: false,
        }
    }
}
/*
impl<T: Default> State<T> {
    pub fn new() -> Self {
        Default::default()
    }
}
*/
