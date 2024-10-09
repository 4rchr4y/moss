#[derive(Debug, Copy, Clone, Default, Eq, PartialEq)]
pub struct Tooltip {
    pub text: &'static str,
}

#[derive(Debug, Copy, Clone, Default, Eq, PartialEq)]
pub struct Icon {
    pub name: &'static str,
}
