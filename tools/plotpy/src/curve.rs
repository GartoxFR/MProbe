use super::{vector_to_array, AsVector, GraphMaker};
use std::fmt::Write;

/// Holds either the second point coordinates of a ray or the slope of the ray
#[derive(Clone, Debug)]
pub enum RayEndpoint {
    /// Coordinates of the second point
    Coords(f64, f64),

    /// Slope of the ray
    Slope(f64),

    /// Indicates a horizontal ray
    Horizontal,

    /// Indicates a vertical ray
    Vertical,
}

/// Generates a curve (aka line-plot) given two arrays (x,y)
///
/// # Notes
///
/// * This struct corresponds to the **plot** function of Matplotlib.
/// * You may plot a Scatter plot by setting line_style = "None"
///
/// # Examples
///
/// ## Using methods to set the points
///
/// ```
/// use plotpy::{Curve, Plot, StrError};
/// use std::f64::consts::PI;
///
/// fn main() -> Result<(), StrError> {
///     // configure curve
///     let mut curve = Curve::new();
///     curve.set_line_width(2.0);
///
///     // add points
///     const N: usize = 30;
///     curve.points_begin();
///     for i in 0..N {
///         let x = (i as f64) * 2.0 * PI / ((N - 1) as f64);
///         let y = f64::sin(x);
///         curve.points_add(x, y);
///     }
///     curve.points_end();
///
///     // add curve to plot
///     let mut plot = Plot::new();
///     plot.add(&curve).grid_and_labels("x", "y");
///
///     // configure multiple-of-pi formatter
///     let minor_every = PI / 12.0;
///     plot.set_ticks_x_multiple_of_pi(minor_every);
///
///     // save figure
///     plot.save("/tmp/plotpy/doc_tests/doc_curve_methods.svg")?;
///     Ok(())
/// }
/// ```
///
/// ![doc_curve_methods.svg](https://raw.githubusercontent.com/cpmech/plotpy/main/figures/doc_curve_methods.svg)
///
/// ## Using Vector with point data
///
/// ```
/// use plotpy::{Curve, Plot, StrError};
/// use russell_lab::Vector;
///
/// fn main() -> Result<(), StrError> {
///     // generate (x,y) points
///     let x = Vector::linspace(-1.0, 1.0, 21)?;
///     let y = x.get_mapped(|v| 1.0 / (1.0 + f64::exp(-5.0 * v)));
///
///     // configure curve
///     let mut curve = Curve::new();
///     curve
///         .set_label("logistic function")
///         .set_line_alpha(0.8)
///         .set_line_color("#5f9cd8")
///         .set_line_style("-")
///         .set_line_width(5.0)
///         .set_marker_color("#eeea83")
///         .set_marker_every(5)
///         .set_marker_line_color("#da98d1")
///         .set_marker_line_width(2.5)
///         .set_marker_size(20.0)
///         .set_marker_style("*");
///
///     // draw curve
///     curve.draw(&x, &y);
///
///     // add curve to plot
///     let mut plot = Plot::new();
///     plot.add(&curve).set_num_ticks_y(11).grid_labels_legend("x", "y");
///
///     // save figure
///     plot.save("/tmp/plotpy/doc_tests/doc_curve.svg")?;
///     Ok(())
/// }
/// ```
///
/// ![doc_curve_vector.svg](https://raw.githubusercontent.com/cpmech/plotpy/main/figures/doc_curve_vector.svg)
///
/// See also integration tests in the [tests directory](https://github.com/cpmech/plotpy/tree/main/tests)
///
/// Output from some integration tests:
///
/// ![integ_curve.svg](https://raw.githubusercontent.com/cpmech/plotpy/main/figures/integ_curve.svg)
///
/// ![integ_curve_3d.svg](https://raw.githubusercontent.com/cpmech/plotpy/main/figures/integ_curve_3d.svg)
pub struct Curve {
    label: String,             // Name of this curve in the legend
    line_alpha: f64,           // Opacity of lines (0, 1]. A<1e-14 => A=1.0
    line_color: String,        // Color of lines
    line_style: String,        // Style of lines
    line_width: f64,           // Width of lines
    marker_color: String,      // Color of markers
    marker_every: usize,       // Increment of data points to use when drawing markers
    marker_void: bool,         // Draw a void marker (draw edge only)
    marker_line_color: String, // Edge color of markers
    marker_line_width: f64,    // Edge width of markers
    marker_size: f64,          // Size of markers
    marker_style: String,      // Style of markers, e.g., "`o`", "`+`"
    stop_clip: bool,           // Stop clipping features within margins
    buffer: String,            // buffer
}

impl Curve {
    /// Creates new Curve object
    pub fn new() -> Self {
        Curve {
            label: String::new(),
            line_alpha: 0.0,
            line_color: String::new(),
            line_style: String::new(),
            line_width: 0.0,
            marker_color: String::new(),
            marker_every: 0,
            marker_void: false,
            marker_line_color: String::new(),
            marker_line_width: 0.0,
            marker_size: 0.0,
            marker_style: String::new(),
            stop_clip: false,
            buffer: String::new(),
        }
    }

    /// Begins adding points to the curve (2D only)
    ///
    /// # Warning
    ///
    /// This function must be followed by [Curve::points_add] and [Curve::points_end],
    /// otherwise Python/Matplotlib will fail.
    pub fn points_begin(&mut self) -> &mut Self {
        write!(&mut self.buffer, "xy=np.array([").unwrap();
        self
    }

    /// Adds point to the curve (2D only)
    ///
    /// # Warning
    ///
    /// This function must be called after [Curve::points_begin] and must be followed by [Curve::points_end],
    /// otherwise Python/Matplotlib will fail.
    pub fn points_add<T>(&mut self, x: T, y: T) -> &mut Self
    where
        T: std::fmt::Display,
    {
        write!(&mut self.buffer, "[{},{}],", x, y).unwrap();
        self
    }

    /// Ends adding points to the curve (2D only)
    ///
    /// # Warning
    ///
    /// This function must be called after [Curve::points_begin] and [Curve::points_add],
    /// otherwise Python/Matplotlib will fail.
    pub fn points_end(&mut self) -> &mut Self {
        let opt = self.options();
        write!(&mut self.buffer, "])\nplt.plot(xy[:,0],xy[:,1]{})\n", &opt).unwrap();
        self
    }

    /// Begins adding 3D points to the curve
    ///
    /// # Warning
    ///
    /// This function must be followed by [Curve::points_3d_add] and [Curve::points_3d_end],
    /// otherwise Python/Matplotlib will fail
    pub fn points_3d_begin(&mut self) -> &mut Self {
        write!(&mut self.buffer, "maybe_create_ax3d()\nxyz=np.array([").unwrap();
        self
    }

    /// Adds 3D point to the curve
    ///
    /// # Warning
    ///
    /// This function must be called after [Curve::points_3d_begin] and must be followed by [Curve::points_3d_end],
    /// otherwise Python/Matplotlib will fail.
    pub fn points_3d_add<T>(&mut self, x: T, y: T, z: T) -> &mut Self
    where
        T: std::fmt::Display,
    {
        write!(&mut self.buffer, "[{},{},{}],", x, y, z).unwrap();
        self
    }

    /// Ends adding 3D points to the curve
    ///
    /// # Warning
    ///
    /// This function must be called after [Curve::points_3d_begin] and [Curve::points_3d_add],
    /// otherwise Python/Matplotlib will fail.
    pub fn points_3d_end(&mut self) -> &mut Self {
        let opt = self.options();
        write!(&mut self.buffer, "])\nAX3D.plot(xyz[:,0],xyz[:,1],xyz[:,2]{})\n", &opt).unwrap();
        self
    }

    /// Draws curve
    ///
    /// # Input
    ///
    /// * `x` - abscissa values
    /// * `y` - ordinate values
    ///
    /// # Notes
    ///
    /// * The type `U` of the input array must be a number.
    ///
    pub fn draw<'a, T, U>(&mut self, x: &'a T, y: &'a T)
    where
        T: AsVector<'a, U>,
        U: 'a + std::fmt::Display,
    {
        vector_to_array(&mut self.buffer, "x", x);
        vector_to_array(&mut self.buffer, "y", y);
        let opt = self.options();
        write!(&mut self.buffer, "plt.plot(x,y{})\n", &opt).unwrap();
    }

    /// Draws curve in 3D plot
    ///
    /// # Input
    ///
    /// * `x` - x values
    /// * `y` - y values
    /// * `z` - z values
    ///
    /// # Notes
    ///
    /// * The type `U` of the input array must be a number.
    ///
    pub fn draw_3d<'a, T, U>(&mut self, x: &'a T, y: &'a T, z: &'a T)
    where
        T: AsVector<'a, U>,
        U: 'a + std::fmt::Display,
    {
        vector_to_array(&mut self.buffer, "x", x);
        vector_to_array(&mut self.buffer, "y", y);
        vector_to_array(&mut self.buffer, "z", z);
        let opt = self.options();
        write!(&mut self.buffer, "maybe_create_ax3d()\n").unwrap();
        write!(&mut self.buffer, "AX3D.plot(x,y,z{})\n", &opt).unwrap();
    }

    /// Sets the name of this curve in the legend
    pub fn set_label(&mut self, label: &str) -> &mut Self {
        self.label = String::from(label);
        self
    }

    /// Sets the opacity of lines (0, 1]. A<1e-14 => A=1.0
    pub fn set_line_alpha(&mut self, alpha: f64) -> &mut Self {
        self.line_alpha = alpha;
        self
    }

    /// Sets the color of lines
    pub fn set_line_color(&mut self, color: &str) -> &mut Self {
        self.line_color = String::from(color);
        self
    }

    /// Draws a ray (an infinite line)
    ///
    /// * For horizontal rays, only `ya` is used
    /// * For vertical rays, only `xa` is used
    pub fn draw_ray(&mut self, xa: f64, ya: f64, endpoint: RayEndpoint) {
        let opt = self.options();
        match endpoint {
            RayEndpoint::Coords(xb, yb) => write!(
                &mut self.buffer,
                "plt.axline(({},{}),({},{}){})\n",
                xa, ya, xb, yb, &opt
            )
            .unwrap(),
            RayEndpoint::Slope(m) => write!(
                &mut self.buffer,
                "plt.axline(({},{}),None,slope={}{})\n",
                xa, ya, m, &opt
            )
            .unwrap(),
            RayEndpoint::Horizontal => write!(&mut self.buffer, "plt.axhline({}{})\n", ya, &opt).unwrap(),
            RayEndpoint::Vertical => write!(&mut self.buffer, "plt.axvline({}{})\n", xa, &opt).unwrap(),
        }
    }

    /// Sets the style of lines
    ///
    /// Options:
    ///
    /// * "`-`", `:`", "`--`", "`-.`", or "`None`"
    /// * As defined in <https://matplotlib.org/stable/gallery/lines_bars_and_markers/linestyles.html>
    pub fn set_line_style(&mut self, style: &str) -> &mut Self {
        self.line_style = String::from(style);
        self
    }

    /// Sets the width of lines
    pub fn set_line_width(&mut self, width: f64) -> &mut Self {
        self.line_width = width;
        self
    }

    /// Sets the color of markers
    pub fn set_marker_color(&mut self, color: &str) -> &mut Self {
        self.marker_color = String::from(color);
        self
    }

    /// Sets the increment of data points to use when drawing markers
    pub fn set_marker_every(&mut self, every: usize) -> &mut Self {
        self.marker_every = every;
        self
    }

    /// Sets the option to draw a void marker (draw edge only)
    pub fn set_marker_void(&mut self, flag: bool) -> &mut Self {
        self.marker_void = flag;
        self
    }

    /// Sets the edge color of markers
    pub fn set_marker_line_color(&mut self, color: &str) -> &mut Self {
        self.marker_line_color = String::from(color);
        self
    }

    /// Sets the edge width of markers
    pub fn set_marker_line_width(&mut self, width: f64) -> &mut Self {
        self.marker_line_width = width;
        self
    }

    /// Sets the size of markers
    pub fn set_marker_size(&mut self, size: f64) -> &mut Self {
        self.marker_size = size;
        self
    }

    /// Sets the style of markers
    ///
    /// Examples:
    ///
    /// * "`o`", "`+`"
    /// * As defined in <https://matplotlib.org/stable/api/markers_api.html>
    pub fn set_marker_style(&mut self, style: &str) -> &mut Self {
        self.marker_style = String::from(style);
        self
    }

    /// Sets the flag to stop clipping features within margins
    pub fn set_stop_clip(&mut self, flag: bool) -> &mut Self {
        self.stop_clip = flag;
        self
    }

    /// Returns options for curve
    fn options(&self) -> String {
        // fix color if marker is void
        let line_color = if self.marker_void && self.line_color == "" {
            "red"
        } else {
            &self.line_color
        };

        // output
        let mut opt = String::new();

        // label
        if self.label != "" {
            write!(&mut opt, ",label='{}'", self.label).unwrap();
        }

        // lines
        if self.line_alpha > 0.0 {
            write!(&mut opt, ",alpha={}", self.line_alpha).unwrap();
        }
        if line_color != "" {
            write!(&mut opt, ",color='{}'", line_color).unwrap();
        }
        if self.line_style != "" {
            write!(&mut opt, ",linestyle='{}'", self.line_style).unwrap();
        }
        if self.line_width > 0.0 {
            write!(&mut opt, ",linewidth={}", self.line_width).unwrap();
        }

        // markers
        if !self.marker_void && self.marker_color != "" {
            write!(&mut opt, ",markerfacecolor='{}'", self.marker_color).unwrap();
        }
        if self.marker_every > 0 {
            write!(&mut opt, ",markevery={}", self.marker_every).unwrap();
        }
        if self.marker_void {
            write!(&mut opt, ",markerfacecolor='none'").unwrap();
        }
        if self.marker_line_color != "" {
            write!(&mut opt, ",markeredgecolor='{}'", self.marker_line_color).unwrap();
        }
        if self.marker_line_width > 0.0 {
            write!(&mut opt, ",markeredgewidth={}", self.marker_line_width).unwrap();
        }
        if self.marker_size > 0.0 {
            write!(&mut opt, ",markersize={}", self.marker_size).unwrap();
        }
        if self.marker_style != "" {
            write!(&mut opt, ",marker='{}'", self.marker_style).unwrap();
        }

        // clipping
        if self.stop_clip {
            write!(&mut opt, ",clip_on=False").unwrap();
        }

        opt
    }
}

impl GraphMaker for Curve {
    fn get_buffer<'a>(&'a self) -> &'a String {
        &self.buffer
    }
    fn clear_buffer(&mut self) {
        self.buffer.clear();
    }
}

