pub struct RequestOne {
    pub name: Option<String>,
    pub id: Option<String>,
    pub parent: Option<String>,
}

impl Default for RequestOne {
    fn default() -> Self {
        Self {
            name: Default::default(),
            id: Default::default(),
            parent: Default::default(),
        }
    }
}
