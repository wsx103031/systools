#[derive(PartialEq, Eq, PartialOrd, Ord, Debug, Clone, Copy)]
pub enum Status {
    Inactive,
    Ready,
    Running,
    Stopping,
    Terminating,
}
