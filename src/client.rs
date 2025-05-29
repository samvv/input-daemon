use zbus::proxy;

#[proxy(
    interface = "be.samvervaeck.InputDaemon.Events",
    default_service = "be.samvervaeck.InputDaemon",
    default_path = "/be/samvervaeck/InputDaemon"
)]
pub trait Service {
    fn send_key(&mut self, code: u32, state: i32) -> zbus::Result<bool>;
    fn send_wheel(&mut self, vertical: i32, horizontal: i32) -> zbus::Result<bool>;
    fn send_motion(&mut self, x: i32, y: i32, z: i32) -> zbus::Result<bool>;
}
