#[derive(PartialEq, Eq, PartialOrd, Ord, Debug)]
pub enum Status {
    Inactive,
    Ready,
    Running,
    Stopping,
    Terminating,
}

/// 先創建必要物件
pub trait Begin {
    fn prepare(&mut self) -> &mut Self;
    fn run(&mut self) -> std::io::Result<()>;
}
/// 開新執行緒監控鍵盤輸入、更新console資訊
pub trait Running {
    fn receive_keycode(&mut self) -> std::io::Result<()>;
    fn update(&mut self);
    fn refresh_screen(&mut self);
}

pub trait Terminating {
    fn terminate(&mut self) -> std::io::Result<()>;
}
