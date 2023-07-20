use super::AsVector;
use std::fmt::Write;

/// Converts vector to a Python list of numbers
pub(crate) fn vector_to_numbers<T>(buf: &mut String, name: &str, vector: &[T])
where
    T: std::fmt::Display,
{
    write!(buf, "{}=[", name).unwrap();
    for val in vector.iter() {
        write!(buf, "{},", val).unwrap();
    }
    writeln!(buf, "]").unwrap();
}

/// Converts vector to a 1D NumPy array
pub(crate) fn vector_to_array<'a, T, U>(buf: &mut String, name: &str, vector: &'a T)
where
    T: AsVector<'a, U>,
    U: 'a + std::fmt::Display,
{
    write!(buf, "{}=np.array([", name).unwrap();
    let m = vector.vec_size();
    for i in 0..m {
        write!(buf, "{},", vector.vec_at(i)).unwrap();
    }
    writeln!(buf, "],dtype=float)").unwrap();
}

