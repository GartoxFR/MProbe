use plotpy::{Curve, Plot};

use crate::utils::{convert_kb, convert_time_micro};

pub fn add_curve_to_plot(
    points: &[(usize, usize)],
    title: &str,
    plot: &mut Plot,
    format_date: bool,
) {
    let max_time = points.iter().map(|(x, _)| x).copied().max().unwrap_or(0);
    let (_, mut time_unit, mut time_divisor) = convert_time_micro(max_time as f64);

    let max_memory = points.iter().map(|(_, y)| y).copied().max().unwrap_or(0);
    let (_, memory_unit, memory_divisor) = convert_kb(max_memory as f64);

    if format_date {
        time_divisor = 1_000_000.0;
        time_unit  = "hh:mm:ss"
    } 

    let mut curve = Curve::new();
    curve.points_begin();
    for (x, y) in points.iter().copied() {
        let x = x as f64 / time_divisor;
        let y = y as f64 / memory_divisor;
        curve.points_add(x, y);
    }
    curve.points_end();

    if let "hh:mm:ss" = time_unit {
        plot.extra("import time\nplt.axes().xaxis.set_major_formatter(tck.FuncFormatter(lambda x, pos: time.strftime('%H:%M:%S', time.gmtime(x))))\n");
    }
    plot.add(&curve);

    plot.grid_and_labels(
        &format!("Temps ({})", time_unit),
        &format!("Mémoire utilisée ({})", memory_unit),
    );
    plot.set_title(title);
}
