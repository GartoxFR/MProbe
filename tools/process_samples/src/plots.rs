use std::collections::BTreeMap;
use std::fmt::Display;

use plotpy::{Curve, Plot};

pub fn plot_memory<T: Display>(
    path: Option<&str>,
    points: impl Iterator<Item = (T, T)>,
    show: bool,
) -> Result<(), &'static str> {
    let path = path.unwrap_or("memory.svg");
    let mut curve = Curve::new();
    curve.points_begin();
    for (x, y) in points {
        curve.points_add(x, y);
    }
    curve.points_end();

    let mut plot = Plot::new();
    plot.add(&curve);
    
    plot.grid_and_labels("Temps (μs)", "Mémoire utilisée (kB)");

    match show {
        false => plot.save(path)?,
        true => plot.save_and_show(path)?,
    }

    Ok(())
}

pub fn plot_process_duration(
    path: Option<&str>,
    points: impl Iterator<Item = (usize, usize)>,
    show: bool,
) -> Result<(), &'static str> {
    let path = path.unwrap_or("duration.svg");
    let mut curve = Curve::new();

    curve.set_line_style("None");
    curve.set_marker_style("+");

    let mut map: BTreeMap<usize, (usize, usize)> = BTreeMap::new();

    curve.points_begin();
    for (x, y) in points {
        let entry = map.entry(x).or_default();
        entry.0 += y;
        entry.1 += 1;

        curve.points_add(x, y);
    }
    curve.points_end();

    let mut curve_moyenne = Curve::new();
    curve_moyenne
        .set_label("Moyenne")
        .set_line_style("--")
        .set_marker_style("");

    curve_moyenne.points_begin();
    for (x, (sum, count)) in map.iter() {
        curve_moyenne.points_add(*x, sum / count);
    }
    curve_moyenne.points_end();

    let mut plot = Plot::new();

    plot.add(&curve);
    plot.add(&curve_moyenne);
    plot.grid_labels_legend("Nombre de processus", "Durée du round de mesure (μs)");

    match show {
        false => plot.save(path)?,
        true => plot.save_and_show(path)?,
    }

    Ok(())
}
