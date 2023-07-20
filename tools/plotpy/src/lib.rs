/// Defines a type alias for the error type as a static string
pub type StrError = &'static str;

mod as_vector;
mod conversions;
mod curve;
mod fileio;
mod legend;
mod plot;
mod constants;

pub use crate::as_vector::*;
pub use crate::curve::*;
pub use crate::legend::*;
pub use crate::plot::*;

use crate::conversions::*;
use crate::fileio::*;
use crate::constants::*;
