use std::{
    net::Shutdown,
    sync::mpsc::{RecvTimeoutError, TryRecvError},
};

use super::*;

foreign_derive_var_iter! {
    Shutdown [Shutdown::Read, Shutdown::Write, Shutdown::Both]
    RecvTimeoutError [RecvTimeoutError::Timeout, RecvTimeoutError::Disconnected]
    TryRecvError [TryRecvError::Empty, TryRecvError::Disconnected]

}
