#[macro_export]
macro_rules! solution {
    ($day:expr, $year:expr) => {
        /// The current day.

        const DAY: $crate::cli::day::Day = $crate::day!($day, $year);

        fn main() {
            use $crate::cli::runner::*;
            let input = $crate::cli::read_inputs_file(DAY);
            run_part(part_one, &input, DAY, 1);
            run_part(part_two, &input, DAY, 2);
        }
    };
}
#[macro_export]
macro_rules! day {
    ($day:expr, $year:expr) => {
        $crate::cli::day::Day {
            day: $day,
            year: $year,
        }
    };
}
