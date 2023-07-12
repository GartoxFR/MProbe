use std::collections::BTreeMap;

use plotpy::{Curve, Plot};

use crate::utils::{convert_kb, convert_time_micro};

pub fn plot_memory(
    path: Option<&str>,
    points: &[(usize, usize)],
    title: &str,
    show: bool,
) -> Result<(), &'static str> {
    let path = path.unwrap_or("memory.svg");

    let max_time = points.iter().map(|(x, _)| x).copied().max().unwrap_or(0);
    let (_, time_unit, time_divisor) = convert_time_micro(max_time as f64);

    let max_memory = points.iter().map(|(_, y)| y).copied().max().unwrap_or(0);
    let (_, memory_unit, memory_divisor) = convert_kb(max_memory as f64);

    let time_divisor = f64::min(time_divisor, 1_000_000.0);

    let mut curve = Curve::new();
    curve.points_begin();
    for (x, y) in points.iter().copied() {
        let x = x as f64 / time_divisor;
        let y = y as f64 / memory_divisor;
        curve.points_add(x, y);
    }
    curve.points_end();

    let mut plot = Plot::new();
    if let "hh:mm:ss" = time_unit {
        plot.extra("import time\nplt.axes().xaxis.set_major_formatter(tck.FuncFormatter(lambda x, pos: time.strftime('%H:%M:%S', time.gmtime(x))))\n");
    }
    plot.add(&curve);

    plot.grid_and_labels(
        &format!("Temps ({})", time_unit),
        &format!("Mémoire utilisée ({})", memory_unit),
    );
    plot.set_title(title);


    match show {
        false => plot.save(path)?,
        true => plot.save_and_show(path)?,
    }

    Ok(())
}

pub fn _plot_process_duration(
    path: Option<&str>,
    points: &[(usize, usize)],
    title: &str,
    show: bool,
) -> Result<(), &'static str> {
    let path = path.unwrap_or("duration.svg");
    let mut curve = Curve::new();

    let max_time = points.iter().map(|(_, y)| y).copied().max().unwrap_or(0);
    let (_, unit, divisor) = convert_time_micro(max_time as f64);

    curve.set_line_style("None");
    curve.set_marker_style("+");

    let mut map: BTreeMap<usize, Vec<usize>> = BTreeMap::new();

    curve.points_begin();
    for (x, y) in points.iter().copied() {
        map.entry(x).or_default().push(y);
        let y = y as f64 / divisor;

        curve.points_add(x as f64, y);
    }
    curve.points_end();

    for (_, vec) in map.iter_mut() {
        vec.sort_unstable();
    }

    let mut curve_mediane = Curve::new();
    let mut curve_percentile = Curve::new();
    curve_mediane
        .set_label("Médiane")
        .set_line_style("--")
        .set_marker_style("o");
    curve_percentile
        .set_label("90e percentile")
        .set_line_style("--")
        .set_marker_style("o");

    curve_mediane.points_begin();
    curve_percentile.points_begin();
    for (x, points_sorted) in map.iter() {
        let mediane = points_sorted[points_sorted.len() / 2];
        let percentile = points_sorted[9 * points_sorted.len() / 10];
        curve_mediane.points_add(*x as f64, mediane as f64 / divisor);
        curve_percentile.points_add(*x as f64, percentile as f64 / divisor);
    }
    curve_mediane.points_end();
    curve_percentile.points_end();

    let mut plot = Plot::new();

    plot.add(&curve);
    plot.add(&curve_mediane);
    plot.add(&curve_percentile);

    plot.set_title(title);
    plot.grid_labels_legend(
        "Nombre de processus",
        &format!("Durée du round de mesure ({})", unit),
    );

    match show {
        false => plot.save(path)?,
        true => plot.save_and_show(path)?,
    }

    Ok(())
}
