
/// Defines a trait to handle Vector-like data
///
/// # Example
///
/// ```
/// use plotpy::AsVector;
/// use russell_lab::Vector;
///
/// fn sum<'a, T, U>(array: &'a T) -> f64
/// where
///     T: AsVector<'a, U>,
///     U: 'a + Into<f64>,
/// {
///     let mut res = 0.0;
///     let m = array.vec_size();
///     for i in 0..m {
///         res += array.vec_at(i).into();
///     }
///     res
/// }
///
/// // heap-allocated 1D array (vector)
/// let x = vec![1.0, 2.0, 3.0];
/// assert_eq!(sum(&x), 6.0);
///
/// // heap-allocated 1D array (slice)
/// let y: &[f64] = &[10.0, 20.0, 30.0];
/// assert_eq!(sum(&y), 60.0);
///
/// // stack-allocated (fixed-size) 2D array
/// let z = [100.0, 200.0, 300.0];
/// assert_eq!(sum(&z), 600.0);
///
/// // using Vector
/// let w = Vector::from(&[5.0, 5.0, 5.0]);
/// assert_eq!(sum(&w), 15.0);
/// ```
pub trait AsVector<'a, U: 'a> {
    /// Returns the size of the vector
    fn vec_size(&self) -> usize;

    /// Returns the value at index i
    fn vec_at(&self, i: usize) -> U;
}

/// Defines a heap-allocated 1D array (vector)
impl<'a, U: 'a> AsVector<'a, U> for Vec<U>
where
    U: 'a + Copy,
{
    fn vec_size(&self) -> usize {
        self.len()
    }
    fn vec_at(&self, i: usize) -> U {
        self[i]
    }
}

/// Defines a heap-allocated 1D array (slice)
impl<'a, U> AsVector<'a, U> for &'a [U]
where
    U: 'a + Copy,
{
    fn vec_size(&self) -> usize {
        self.len()
    }
    fn vec_at(&self, i: usize) -> U {
        self[i]
    }
}

/// Defines a stack-allocated (fixed-size) 1D array
impl<'a, U, const M: usize> AsVector<'a, U> for [U; M]
where
    U: 'a + Copy,
{
    fn vec_size(&self) -> usize {
        self.len()
    }
    fn vec_at(&self, i: usize) -> U {
        self[i]
    }
}
