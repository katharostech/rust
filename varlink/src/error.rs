use std::io;

use quick_error::quick_error;
use serde_json;

use crate::Reply;

quick_error! {
    #[derive(Debug)]
    pub enum ErrorKind {
        Io(err: io::Error) {
            from()
            source(err)
            display("IO error: {}", err)
        }
        SerdeJsonSer(err: serde_json::error::Error) {
            from()
            source(err)
            display("JSON Serialization Error: {}", err)
        }
        SerdeJsonDe(json: String, err: serde_json::error::Error) {
            source(err)
            display("JSON Deserialization Error of: {}", json)
        }
        InterfaceNotFound(name: String) {
            display("Interface not found: '{}'", name)
        }
        InvalidParameter(name: String) {
            display("Invalid parameter: '{}'", name)
        }
        MethodNotFound(name: String) {
            display("Method not found: '{}'", name)
        }
        MethodNotImplemented(name: String) {
            display("Method not implemented: '{}'", name)
        }
        VarlinkErrorReply(reply: crate::Reply) {
            display("Varlink error reply: '{:#?}'", reply)
        }
        CallContinuesMismatch {
            display("Call::reply() called with continues, but without more in the request")
        }
        MethodCalledAlready { display("Varlink: method called already") }
        ConnectionBusy { display("Varlink: connection busy with other method") }
        IteratorOldReply { display("Varlink: Iterator called on old reply") }
        Server { display("Server Error") }
        Timeout { display("Timeout Error") }
        ConnectionClosed { display ("Connection Closed") }
        InvalidAddress { display ("Invalid varlink address URI") }
    }
}

impl From<Reply> for ErrorKind {
    fn from(e: Reply) -> Self {
        match e {
            Reply {
                error: Some(ref t), ..
            } if t == "org.varlink.service.InterfaceNotFound" => match e {
                Reply {
                    parameters: Some(p),
                    ..
                } => match serde_json::from_value::<crate::ErrorInterfaceNotFound>(p) {
                    Ok(v) => ErrorKind::InterfaceNotFound(v.interface.unwrap_or_default()),
                    Err(_) => ErrorKind::InterfaceNotFound(String::new()),
                },
                _ => ErrorKind::InterfaceNotFound(String::new()),
            },
            Reply {
                error: Some(ref t), ..
            } if t == "org.varlink.service.InvalidParameter" => match e {
                Reply {
                    parameters: Some(p),
                    ..
                } => match serde_json::from_value::<crate::ErrorInvalidParameter>(p) {
                    Ok(v) => ErrorKind::InvalidParameter(v.parameter.unwrap_or_default()),
                    Err(_) => ErrorKind::InvalidParameter(String::new()),
                },
                _ => ErrorKind::InvalidParameter(String::new()),
            },
            Reply {
                error: Some(ref t), ..
            } if t == "org.varlink.service.MethodNotFound" => match e {
                Reply {
                    parameters: Some(p),
                    ..
                } => match serde_json::from_value::<crate::ErrorMethodNotFound>(p) {
                    Ok(v) => ErrorKind::MethodNotFound(v.method.unwrap_or_default()),
                    Err(_) => ErrorKind::MethodNotFound(String::new()),
                },
                _ => ErrorKind::MethodNotFound(String::new()),
            },
            Reply {
                error: Some(ref t), ..
            } if t == "org.varlink.service.MethodNotImplemented" => match e {
                Reply {
                    parameters: Some(p),
                    ..
                } => match serde_json::from_value::<crate::ErrorMethodNotImplemented>(p) {
                    Ok(v) => ErrorKind::MethodNotImplemented(v.method.unwrap_or_default()),
                    Err(_) => ErrorKind::MethodNotImplemented(String::new()),
                },
                _ => ErrorKind::MethodNotImplemented(String::new()),
            },
            _ => ErrorKind::VarlinkErrorReply(e),
        }
    }
}

impl ErrorKind {
    pub fn is_error(r: &Reply) -> bool {
        match r.error {
            Some(ref t) => match t.as_ref() {
                "org.varlink.service.InvalidParameter" => true,
                "org.varlink.service.InterfaceNotFound" => true,
                "org.varlink.service.MethodNotFound" => true,
                "org.varlink.service.MethodNotImplemented" => true,
                _ => false,
            },
            _ => false,
        }
    }
}

pub struct Error {
    pub kind: ErrorKind,
    pub occurrence: &'static str,
}

impl Error {
    pub fn kind(&self) -> &ErrorKind {
        &self.kind
    }
}

impl std::error::Error for Error {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        self.kind.source()
    }
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        std::fmt::Display::fmt(&self.kind, f)
    }
}

impl std::fmt::Debug for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        use std::error::Error as StdError;

        std::fmt::Display::fmt(self.occurrence, f)?;

        std::fmt::Debug::fmt(&self.kind, f)?;
        if let Some(e) = self.source() {
            std::fmt::Display::fmt("\nCaused by:\n", f)?;
            std::fmt::Debug::fmt(&e, f)?;
        }
        Ok(())
    }
}

#[macro_export]
macro_rules! map_context {
    () => {
        |e| $crate::context!($crate::ErrorKind::from(e))
    };
}

#[macro_export]
macro_rules! context {
    ( $k:expr ) => {{
        $crate::error::Error {
            kind: $k,
            occurrence: concat!(file!(), ":", line!(), ": "),
        }
    }};
}

pub type Result<T> = std::result::Result<T, Error>;
