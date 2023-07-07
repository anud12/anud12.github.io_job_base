pub struct RequestList {
    pub name: Option<String>,
    pub size: Option<usize>,
    pub fixed: Option<bool>,
    pub parent: Option<String>,
}

impl Default for RequestList {
    fn default() -> Self {
        Self {
            name: Default::default(),
            size: Default::default(),
            fixed: Default::default(),
            parent: Default::default(),
        }
    }
}
