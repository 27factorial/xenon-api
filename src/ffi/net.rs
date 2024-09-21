use super::io::Handle;

#[repr(C)]
pub struct TcpStream(Handle);

#[repr(C)]
pub struct TcpListener(Handle);

#[repr(C)]
pub struct UdpSocket(Handle);
