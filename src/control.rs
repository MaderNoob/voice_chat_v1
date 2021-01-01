use num_enum::TryFromPrimitive;
use std::convert::TryFrom;
use std::net::{SocketAddr, UdpSocket};

pub const CONTROL_PORT: u16 = 4741;

enum ControlWarningKind {
    InvalidInitialRequestCode,
}

#[derive(Debug)]
enum ControlErrorKind {
    BindControlSocket,
    Receive,
    SetNonBlocking,
}
#[derive(Debug)]
struct ControlError {
    kind: ControlErrorKind,
    inner: Option<std::io::Error>,
}
type Result<T> = std::result::Result<T, ControlError>;
#[repr(u8)]
#[derive(TryFromPrimitive, Debug)]
enum InitialRequestCode {
    StartConversation = 0,
    GetStatus = 1,
}
trait ControlResultHandler<T> {
    fn to_control_result(self, kind: ControlErrorKind) -> Result<T>;
}
trait ControlIOResultHandler<T> {
    fn io_to_control_result(self, kind: ControlErrorKind) -> Result<T>;
}
impl<T, E> ControlResultHandler<T> for std::result::Result<T, E> {
    fn to_control_result(self, kind: ControlErrorKind) -> Result<T> {
        match self {
            Ok(v) => Ok(v),
            Err(_) => Err(ControlError { kind, inner: None }),
        }
    }
}
impl<T> ControlIOResultHandler<T> for std::io::Result<T> {
    fn io_to_control_result(self, kind: ControlErrorKind) -> Result<T> {
        match self {
            Ok(v) => Ok(v),
            Err(e) => Err(ControlError {
                kind,
                inner: Some(e),
            }),
        }
    }
}
fn handle_warning(warning: ControlWarningKind) {}
pub fn control_thread() {
    start_control_thread().unwrap();
}
fn start_control_thread() -> Result<()> {
    let sock = UdpSocket::bind(SocketAddr::from(([127, 0, 0, 1], CONTROL_PORT)))
        .io_to_control_result(ControlErrorKind::BindControlSocket)?;
    sock.set_nonblocking(true).io_to_control_result(ControlErrorKind::SetNonBlocking)?;
    wait_for_conversation(sock)
}
fn wait_for_conversation(sock: UdpSocket) -> Result<()> {
    let mut buffer = [0u8; 1];
    loop {
        let (amount, ep) = sock
            .recv_from(&mut buffer)
            .io_to_control_result(ControlErrorKind::Receive)?;
        if amount == 0 {
            continue;
        }
        let request_code = match InitialRequestCode::try_from(buffer[0]) {
            Ok(code) => code,
            Err(_) => {
                handle_warning(ControlWarningKind::InvalidInitialRequestCode);
                continue;
            }
        };

    }
}
