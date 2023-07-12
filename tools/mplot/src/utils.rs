pub fn convert_time_micro(mut time: f64) -> (f64, &'static str, f64) {
    static UNITS: [(&str, f64); 3] = [("ms", 1000.0), ("s", 1000.0), ("hh:mm:ss", 60.0)];
    let mut unit = "μs";
    let mut global_divisor = 1.0;

    for (new_unit, divisor) in UNITS {
        if time < divisor {
            break;
        }

        time /= divisor;
        unit = new_unit;
        global_divisor *= divisor;
    }

    (time, unit, global_divisor)
}

pub fn convert_kb(mut memory: f64) -> (f64, &'static str, f64) {
    static UNITS: [(&str, f64); 2] = [("MiB", 1024.0), ("GiB", 1024.0)];
    let mut unit = "KiB";
    let mut global_divisor = 1.0;

    for (new_unit, divisor) in UNITS {
        if memory < divisor {
            break;
        }

        memory /= divisor;
        unit = new_unit;
        global_divisor *= divisor;
    }

    (memory, unit, global_divisor)
}
