use advent_of_code::{days::*, solver::Solver};
use std::{fmt::Debug, time::Instant};

const YEAR: &str = "2022";

#[cfg(debug_assertions)]
#[global_allocator]
static ALLOCATOR: dhat::Alloc = dhat::Alloc;

macro_rules! day {
    ( $d:expr ) => {
        day!($d => None, None);
    };

    ( $d:expr, $o1:expr ) => {
        day!($d => Some($o1), None);
    };

    ( $d:expr, $o1:expr, $o2:expr ) => {
        day!($d => Some($o1), Some($o2));
    };

    ( $d:expr => $o1:expr, $o2:expr ) => {
        paste::expr! {
            solve::<_, _, [<day $d>]::[<Day $d>]>($d, $o1, $o2);
        }
    };
}

fn main() {
    #[cfg(debug_assertions)]
    let _dhat = dhat::Profiler::new_heap();

    println!("AOC {}", YEAR);
    day!(1, 68292, 203203);
    day!(2, 11063, 10349);
    day!(3, 7831, 2683);
    day!(4, 542, 900);
    day!(5, "QGTHFZBHV".to_string(), "MGDMPSZTM".to_string());
    day!(6, 1343, 2193);
    day!(7, 1325919, 2050735);
    day!(8, 1703, 496650);
    day!(9, 6498, 2531);
    day!(10, 13920, "####..##..#....#..#.###..#....####...##.\n#....#..#.#....#..#.#..#.#....#.......#.\n###..#....#....####.###..#....###.....#.\n#....#.##.#....#..#.#..#.#....#.......#.\n#....#..#.#....#..#.#..#.#....#....#..#.\n####..###.####.#..#.###..####.#.....##..\n".to_string());
    day!(11, 120056, 21816744824);
    day!(12, 504, 500);
    day!(13, 5506, 21756);
    day!(14, 1003, 25771);
    day!(15, 5511201, 11318723411840);
}

fn solve<O, O2, S: for<'a> Solver<'a, Output = O, Output2 = O2>>(
    day_number: u8,
    part1_output: Option<O>,
    part2_output: Option<O2>,
) {
    let input = std::fs::read_to_string(format!("input/{}/day{}.txt", YEAR, day_number)).unwrap();
    let trimmed = input.as_str();

    let mut args = std::env::args();
    let day_as_str = day_number.to_string();
    if args.len() > 2 {
        if args.any(|x| x == day_as_str || x == "a") {
            bench::<S>(day_number, trimmed);
        }
    } else {
        if args.any(|x| x == day_as_str) || args.len() <= 1 {
            run::<S>(day_number, trimmed, part1_output, part2_output);
        }
    }
}

fn run<'a, S: Solver<'a>>(
    day_number: u8,
    input: &'a str,
    part1_output: Option<S::Output>,
    part2_output: Option<S::Output2>,
) {
    let start_time = Instant::now();
    let parsed = S::parse(input);
    let end_time = Instant::now();

    println!("\nDay {}:", day_number);
    println!("\tparser: {:?}", (end_time - start_time));

    run_part(parsed.clone(), 1, S::part1, part1_output);
    run_part(parsed, 2, S::part2, part2_output);
}

fn run_part<P, O: Debug + PartialEq>(
    parsed: P,
    part_number: u8,
    part: impl Fn(P) -> O,
    expected_output: Option<O>,
) {
    print!("Part {}: ", part_number);

    let start_time = Instant::now();
    let result = part(parsed);
    let end_time = Instant::now();

    println!("{:?}", result);
    println!("\tsolver: {:?}", (end_time - start_time));

    if let Some(expected) = expected_output {
        assert_eq!(expected, result);
    } else {
        println!("Not checking result!");
    }
}

fn bench<'a, S: Solver<'a>>(day_number: u8, input: &'a str) {
    let mut criterion = criterion::Criterion::default().without_plots();
    let mut group = criterion.benchmark_group(format!("Day {}", day_number));

    group.bench_with_input("parser", &input, |b, i| {
        b.iter_with_large_drop(|| S::parse(i));
    });

    let parsed = S::parse(input);

    group.bench_with_input("part 1", &parsed, |b, i| {
        b.iter_batched(|| i.clone(), S::part1, criterion::BatchSize::SmallInput)
    });

    group.bench_with_input("part 2", &parsed, |b, i| {
        b.iter_batched(|| i.clone(), S::part2, criterion::BatchSize::SmallInput)
    });
}
