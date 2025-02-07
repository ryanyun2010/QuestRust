use std::{error::Error, fmt};

use terminal_size::{Height, Width};

#[derive(Debug, Clone)]
pub enum PE {
    NotFound(ErrorDescriptor),
    Expected(ErrorDescriptor),
    Invalid(ErrorDescriptor),
    InputFailed(ErrorDescriptor),
    Error(ErrorDescriptor),
    UnwrapFailure(ErrorDescriptor),
    MissingExpectedGlobalSprite(ErrorDescriptor),
    SurfaceError(wgpu::SurfaceError),
    NoSpace(ErrorDescriptor),
    WrongItemType(ErrorDescriptor)
}


impl PE {
    pub fn as_string(&self) -> String {
        match self {
            PE::NotFound(e) => format!("Not Found Error at {}: {}", e.as_location(), e.as_string()),
            PE::InputFailed(e) => format!("Input Failed Error at {}: {}", e.as_location(), e.as_string()),
            PE::Expected(e) => format!("Expected But Not Found Error at {}: {}", e.as_location(), e.as_string()),
            PE::Error(e) => format!("Error at {}: {}",e.as_location(), e.as_string()),
            PE::Invalid(e) => format!("Invalid Error at {}: {}",e.as_location(), e.as_string()),
            PE::UnwrapFailure(e) => format!("Unwrap Failure at {}: {}",e.as_location(), e.as_string()),
            PE::MissingExpectedGlobalSprite(e) => format!("Missing Expected Global Sprite at {}: {}",e.as_location(), e.as_string()),
            PE::SurfaceError(e) => format!("Surface Error: {}", e),
            PE::NoSpace(e) => format!("No Space in Inventory Error at {}: {}", e.as_location(), e.as_string()),
            PE::WrongItemType(e) => format!("Wrong Item Type Error at {}: {}", e.as_location(), e.as_string())
        }
    }
}

impl ErrorDescriptor {
    pub fn as_string(&self) -> String {
        self.desc.clone()
    }
    pub fn as_location(&self) -> String {
        format!("{}:{}", self.location.file, self.location.line)
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
        $crate::error::PError::new(
            $crate::error::PE::$error_variant($crate::error::ErrorDescriptor {
                desc: format!($desc, $($args)*),
                location: $crate::error::Location {
                    file: file!().to_string(),
                    line: line!(),
                },
            }),
            vec![]
        )
    }};
    ($error_variant:ident, $desc:expr) => {{
        $crate::error::PError::new(
            $crate::error::PE::$error_variant($crate::error::ErrorDescriptor {
                desc: format!($desc),
                location: $crate::error::Location {
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


impl fmt::Display for PError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "\n{}", self.error.as_string())?;
        write!(f, "\n\nCaused by: \n\n")?;

        let mut i = 1;
        for s in self.trace.iter(){
            let dim = terminal_size::terminal_size();
            if let Some((Width(chunk_size), Height(h))) = dim {
        
                let chunks = split_into_chunks(s, chunk_size as usize - 12);
                let mut d = false;
                let pad = 4.0 - f32::log10(i as f32).floor();
                let pads = " ".repeat(pad as usize);
                writeln!(f, "{}:{}  →  {}", i, pads, chunks[0])?; 
                for chunk in chunks.iter(){
                    if !d {
                        d = true;
                        continue;
                    }
                    writeln!(f, "           {}", chunk.trim_start())?;
                }

                i += 1;
            }else{
                writeln!(f, "{}:  →  {}", i,s)?; 
            }
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
            Err(perror) => {
                return Err($crate::error::PError::new(
                        $crate::error::PE::$error_variant(
                            $crate::error::ErrorDescriptor {
                                desc: format!($desc, $($args)*),
                                location: $crate::error::Location {
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
                return Err($crate::error::PError::new(
                        $crate::error::PE::$error_variant(
                            $crate::error::ErrorDescriptor {
                                desc: format!($desc),
                                location: $crate::error::Location {
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
            Err(perror) => {
                return Err($crate::error::PError::new(
                        crate::error::PE::Error(
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
    ($result:expr) => {{
        match $result {
            Ok(value) => value,
            Err(perror) => {
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
            Err(perror) => {
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
                return Err($crate::error::PError::new(
                        $crate::error::PE::UnwrapFailure(
                            $crate::error::ErrorDescriptor {
                                desc: format!(""),
                                location: $crate::error::Location {
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
    ($option:expr, $error_variant:ident, $desc:expr) => {{
        match $option {
            Some(value) => value,
            None => {
                return Err($crate::error::PError::new(
                        $crate::error::PE::$error_variant(
                            $crate::error::ErrorDescriptor {
                                desc: format!($desc),
                                location: $crate::error::Location {
                                    file: file!().to_string(),
                                    line: line!(),
                                },
                            },
                        ),
                        vec![]
                ),
                );
                }
        }
    }};
    ($option:expr, $error_variant:ident, $desc:expr, $($args:tt)*) => {{
        match $option {
            Some(value) => value,
            None => {
                return Err($crate::error::PError::new(
                        $crate::error::PE::$error_variant(
                            $crate::error::ErrorDescriptor {
                                desc: format!($desc, $($args)*),
                                location: $crate::error::Location {
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
                return Err($crate::error::PError::new(
                        $crate::error::PE::UnwrapFailure(
                            $crate::error::ErrorDescriptor {
                                desc: format!($desc, $($args)*),
                                location: $crate::error::Location {
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
                return Err($crate::error::PError::new(
                        $crate::error::PE::UnwrapFailure(
                            $crate::error::ErrorDescriptor {
                                desc: format!($desc),
                                location: $crate::error::Location {
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
                return Err($crate::error::PError::new(
                        $crate::error::PE::$error_variant(
                            $crate::error::ErrorDescriptor {
                                desc: format!(""),
                                location: $crate::error::Location {
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
                return $crate::PError::new(
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
        return Err($crate::error::PError::new(
                $crate::error::PE::Error(
                    $crate::error::ErrorDescriptor {
                        desc: format!(""),
                        location: $crate::error::Location {
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
        let result = $result;
        if let Err(perror) = &result {
            match perror.error {
                $(
                    $crate::error::PE::$error_variant(_) => {
                        result
                    },
                )*
                    _ => $crate::error_prolif!(perror),
            }
        }else{
            result
        }
    }} 
}
#[macro_export]
macro_rules! print_error {
    ($e:expr) => {
        colorized::colorize_println(format!("{}", $e), colorized::Colors::BrightRedFg)
    };
}


#[macro_export]
macro_rules! panic_error {
    ($e:expr) => {
        panic!("{}", colorized::colorize_this(format!("{}", $e),colorized::Colors::BrightRedFg))
    };
}

fn split_into_chunks(input: &str, chunk_size: usize) -> Vec<String> {
    input.chars()
        .collect::<Vec<char>>()
        .chunks(chunk_size)
        .map(|chunk| chunk.iter().collect())
        .collect()
}

#[macro_export]
macro_rules! ok_or_panic {
    ($result:expr) => {
        match $result {
            Ok(v) => v,
            Err(e) => $crate::panic_error!(e)
        }
    }
}



