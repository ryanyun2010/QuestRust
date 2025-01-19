use std::{error::Error, fmt};

#[derive(Debug, Clone)]
pub enum PE {
    NotFound(ErrorDescriptor),
    Invalid(ErrorDescriptor),
    InputFailed(ErrorDescriptor),
    Error(ErrorDescriptor),
    UnwrapFailure(ErrorDescriptor),
    MissingExpectedGlobalSprite(ErrorDescriptor),
    SurfaceError(wgpu::SurfaceError)
}

impl PE {
    pub fn as_string(&self) -> String {
        match self {
            PE::NotFound(e) => format!("Not Found: {}", e.as_string()),
            PE::InputFailed(e) => format!("Input Failed: {}", e.as_string()),
            PE::Error(e) => format!("Error: {}",e.as_string()),
            PE::Invalid(e) => format!("Invalid: {}", e.as_string()),
            PE::UnwrapFailure(e) => format!("Unwrap Failure: {}", e.as_string()),
            PE::MissingExpectedGlobalSprite(e) => format!("Missing Expected Global Sprite: {}", e.as_string()),
            PE::SurfaceError(e) => format!("Surface Error: {}", e)
        }
    }
}
impl ErrorDescriptor {
    pub fn as_string(&self) -> String {
        format!("{} at {}:{}", self.desc, self.location.file, self.location.line)
    }
}

#[derive(Debug, Clone)]
pub struct PError {
    pub error: PE,
    pub trace: Vec<String>
}

impl Error for PError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        None
    }
}

impl PError{
    pub fn new(error: PE, trace: Vec<String>) -> Self{
        Self{
            error,
            trace
        }
    }
    pub fn as_trace(&self) -> Vec<String> {
        let mut trace = vec![];
        trace.push(self.error.as_string());
        for s in self.trace.iter(){
            trace.push(s.clone());
        }
        trace
    }
}
#[macro_export]
macro_rules! perror {
    ($error_variant:ident, $desc:expr, $($args:tt)*) => {{
        crate::error::PError::new(
            crate::error::PE::$error_variant(crate::error::ErrorDescriptor {
                desc: format!($desc, $($args)*),
                location: crate::error::Location {
                    file: file!().to_string(),
                    line: line!(),
                },
            }),
            vec![]
        )
    }};
    ($error_variant:ident, $desc:expr) => {{
        crate::error::PError::new(
            crate::error::PE::$error_variant(crate::error::ErrorDescriptor {
                desc: format!($desc),
                location: crate::error::Location {
                    file: file!().to_string(),
                    line: line!(),
                },
            }),
            vec![]
        )
    }};
    ($desc:expr) => {{
        crate::error::PError::new(
            crate::error::PE::Error(crate::error::ErrorDescriptor {
                desc: format!($desc),
                location: crate::error::Location {
                    file: file!().to_string(),
                    line: line!(),
                },
            }),
            vec![]
        )
    }};
    ($desc:expr, $($args:tt)*) => {{
        crate::error::PError::new(
            crate::error::PE::Error(crate::error::ErrorDescriptor {
                desc: format!($desc, $($args)*),
                location: crate::error::Location {
                    file: file!().to_string(),
                    line: line!(),
                },
            }),
            vec![]
        )
    }};
    ($error_variant:ident) => {{
        crate::error::PError::new(
            crate::error::PE::Error(crate::error::ErrorDescriptor {
                desc: Strgin::from(""),
                location: crate::error::Location {
                    file: file!().to_string(),
                    line: line!(),
                },
            }),
            vec![]
        )
    }};
}
// TODO, MAKE THIS NOT USE FORMAT IF IT DOESNT HAVE TO + LET IT RETURN JUST AN ERROR IF ONLY 1 ARG


impl fmt::Display for PError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {


        let mut string = self.error.as_string();
        


        write!(f, "\n\n{}", self.error.as_string())?;
        write!(f, "{}", "\n\nCaused by: \n\n");

        let mut i =0;
        for s in self.trace.iter(){
            write!(f, "{}. {} \n", i, s)?;
            i += 1;
        }
        Ok(())
    }
}

#[derive(Debug, Clone)]
pub struct ErrorDescriptor {
    pub desc: String,
    pub location: Location
}


#[derive(Debug, Clone)]
pub struct Location {
    pub file: String,
    pub line: u32
}

#[macro_export]
macro_rules! ptry {
    ($result:expr, $error_variant:ident,  $desc:expr, $($args:tt)*) => {{
        match $result {
            Ok(value) => value,
            Err(mut perror) => {
                return Err(crate::error::PError::new(
                    crate::error::PE::$error_variant(
                        crate::error::ErrorDescriptor {
                            desc: format!($desc, $($args)*),
                            location: crate::error::Location {
                                file: file!().to_string(),
                                line: line!()
                            }
                        }
                    ),
                    perror.as_trace()
                ));
            }
        }
    }};
    ($result:expr, $error_variant:ident,  $desc:expr) => {{
        match $result {
            Ok(value) => value,
            Err(mut perror) => {
                return Err(crate::PError::new(
                    PE::$error_variant(
                        ErrorDescriptor {
                            desc: format!($desc),
                            location: Location {
                                file: file!().to_string(),
                                line: line!()
                            }
                        }
                    ),
                    perror.as_trace()
                ));
            }
        }
    }};
    ($result:expr, $desc:expr, $($args:tt)*) => {{
        match $result {
            Ok(value) => value,
            Err(mut perror) => {
                return Err(crate::PError::new(
                    PE::Error(
                        ErrorDescriptor {
                            desc: format!($desc, $($args)*),
                            location: Location {
                                file: file!().to_string(),
                                line: line!()
                            }
                        }
                    ),
                    perror.as_trace()
                ));
            }
        }
    }};
    ($result:expr) => {{
        match $result {
            Ok(value) => value,
            Err(mut perror) => {
                return Err(crate::error::PError::new(
                    crate::error::PE::Error(
                        crate::error::ErrorDescriptor {
                            desc: format!(""),
                            location: crate::error::Location {
                                file: file!().to_string(),
                                line: line!()
                            }
                        }
                    ),
                    perror.as_trace()
                ));
            }
        }
    }};
    ($result:expr, $desc:expr) => {{
        match $result {
            Ok(value) => value,
            Err(mut perror) => {
                return Err(crate::error::PError::new(
                    crate::error::PE::Error(
                        crate::error::ErrorDescriptor {
                            desc: format!($desc),
                            location: crate::error::Location {
                                file: file!().to_string(),
                                line: line!()
                            }
                        }
                    ),
                    perror.as_trace()
                ));
            }
        }
    }};
    ($result:expr, $error_variant:ident) => {{
        match $result {
            Ok(value) => value,
            Err(mut perror) => {
                return Err(crate::PError::new(
                    PE::$error_variant(
                        ErrorDescriptor {
                            desc: format!(""),
                            location: Location {
                                file: file!().to_string(),
                                line: line!()
                            }
                        }
                    ),
                    perror.as_trace()
                ));
            }
        }
    }};
   
}

#[macro_export]
macro_rules! punwrap {
    ($option:expr) => {{
        match $option {
            Some(value) => value,
            None => {
                return Err(crate::error::PError::new(
                    crate::error::PE::UnwrapFailure(
                        crate::error::ErrorDescriptor {
                            desc: format!(""),
                            location: crate::error::Location {
                                file: file!().to_string(),
                                line: line!(),
                            },
                        },
                    ),
                    vec![],
                ));
            }
        }
    }};
    ($option:expr, $error_variant:ident, $desc:expr, $($args:tt)*) => {{
        match $option {
            Some(value) => value,
            None => {
                return Err(crate::error::PError::new(
                    crate::error::PE::$error_variant(
                        crate::error::ErrorDescriptor {
                            desc: format!($desc, $($args)*),
                            location: crate::error::Location {
                                file: file!().to_string(),
                                line: line!()
                            }
                        }
                    ),
                    vec![]
                ));
            }
        }
    }};
    ($option:expr, $desc:expr, $($args:tt)*) => {{
        match $option {
            Some(value) => value,
            None => {
                return Err(crate::error::PError::new(
                    crate::error::PE::UnwrapFailure(
                        crate::error::ErrorDescriptor {
                            desc: format!($desc, $($args)*),
                            location: crate::error::Location {
                                file: file!().to_string(),
                                line: line!(),
                            },
                        },
                    ),
                    vec![],
                ));
            }
        }
    }};
    ($option:expr, $desc:expr) => {{
        match $option {
            Some(value) => value,
            None => {
                return Err(crate::error::PError::new(
                    crate::error::PE::UnwrapFailure(
                        crate::error::ErrorDescriptor {
                            desc: format!($desc),
                            location: crate::error::Location {
                                file: file!().to_string(),
                                line: line!(),
                            },
                        },
                    ),
                    vec![],
                ));
            }
        }
    }};
    ($option:expr, $error_variant:ident) => {{
        match $option {
            Some(value) => value,
            None => {
                return crate::PError::new(
                    PE::$error_variant(
                        ErrorDescriptor {
                            desc: format!(""),
                            location: Location {
                                file: file!().to_string(),
                                line: line!(),
                            },
                        },
                    ),
                    vec![],
                );
            }
        }
    }};
    ($option:expr, $error_variant:ident, $desc:expr, $($args:tt)*) => {{
        match $option {
            Some(value) => value,
            None => {
                return crate::PError::new(
                    PE::$error_variant(
                        ErrorDescriptor {
                            desc: format!($desc, $($args)*),
                            location: Location {
                                file: file!().to_string(),
                                line: line!(),
                            },
                        },
                    ),
                    vec![],
                );
            }
        }
    }};
}

#[macro_export]
macro_rules! error_prolif {
    ($error:expr) => {{
        return Err(crate::error::PError::new(
            crate::error::PE::Error(
                crate::error::ErrorDescriptor {
                    desc: format!(""),
                    location: crate::error::Location {
                        file: file!().to_string(),
                        line: line!()
                    }
                }
            ),
            $error.as_trace()
        )
    );
    }} 
}

#[macro_export]
macro_rules! error_prolif_allow {
    ($result:expr, $($error_variant:ident)*) => {{
        match $result {
            Ok(value) => $result,
            Err(mut perror) => {
                match perror.error {
                    $(
                        crate::error::PE::$error_variant(_) => $result,
                    )*
                    _ => crate::error_prolif!(perror),
                }
            }
        }
    }} 
}