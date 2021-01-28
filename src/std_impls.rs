use std::{
    io::ErrorKind,
    net::Shutdown,
    sync::mpsc::{RecvTimeoutError, TryRecvError},
};

use super::*;

foreign_derive_var_iter! {
    ErrorKind [
        ErrorKind::NotFound,
        ErrorKind::PermissionDenied,
        ErrorKind::ConnectionRefused,
        ErrorKind::ConnectionReset,
        ErrorKind::ConnectionAborted,
        ErrorKind::NotConnected,
        ErrorKind::AddrInUse,
        ErrorKind::AddrNotAvailable,
        ErrorKind::BrokenPipe,
        ErrorKind::AlreadyExists,
        ErrorKind::WouldBlock,
        ErrorKind::InvalidInput,
        ErrorKind::InvalidData,
        ErrorKind::TimedOut,
        ErrorKind::WriteZero,
        ErrorKind::Interrupted,
        ErrorKind::Other,
        ErrorKind::UnexpectedEof,
    ]
    Shutdown [Shutdown::Read, Shutdown::Write, Shutdown::Both]
    RecvTimeoutError [RecvTimeoutError::Timeout, RecvTimeoutError::Disconnected]
    TryRecvError [TryRecvError::Empty, TryRecvError::Disconnected]

}
