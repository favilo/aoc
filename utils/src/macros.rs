#[macro_export]
#[allow(clippy::crate_in_macro_def)]
macro_rules! run_days {
    ($day:ident = $id:expr, $($days:ident = $ids:expr),* $(,)?) => {
        use aoc_utils::{run, Heap, StageTime};
        use heapless::{binary_heap::Max, BinaryHeap};

        pub mod $day;
        $(pub mod $days;)*
        pub fn run_all(days: Vec<usize>, track: bool) -> miette::Result<Heap> {
            let mut heap = BinaryHeap::<StageTime, Max, 125>::new();
            if days.is_empty() {
                run::<$day::Day, _, _>(crate::YEAR, track, &mut heap)?;
                $(run::<$days::Day, _, _>(crate::YEAR, track, &mut heap)?;)*
            } else {
                for day in days {
                    match day {
                        $id => run::<$day::Day, _, _>(crate::YEAR, track, &mut heap)?,
                        $($ids => run::<$days::Day, _, _>(crate::YEAR, track, &mut heap)?,)*
                        _ => panic!("Invalid day passed"),
                    };
                }
            }

            Ok(heap)
        }
    };
    () => {
        pub fn run_all(_days: Vec<usize>, _track: bool) -> miette::Result<Heap> {
            miette::bail!("No days specified")
        }
    };
}

#[macro_export]
macro_rules! sample_case {
        ($id:ident => preamble = $preamble:expr; input1 = $input1:expr; part1_r = $part1:expr; input2 = $input2:expr; part2_r = $part2:expr;) => {
            mod $id {
                use super::*;

                #[test]
                fn part1() -> miette::Result<()> {
                    let _ = env_logger::try_init();
                    #[allow(clippy::unused_unit)]
                    {
                        $preamble;
                    };
                    let input = $input1;
                    println!("{}", input);
                    let input = Day::get_input(input)?;
                    println!("{:#?}", input);
                    let output = Day::part1(&input);
                    if $part1.is_err() {
                        assert!(output.is_err());
                        assert_eq!($part1.unwrap_err().to_string(), output.unwrap_err().to_string())
                    } else {
                        assert_eq!($part1.unwrap(), output?);
                    }
                    Ok(())
                }

                #[test]
                fn part2() -> miette::Result<()> {
                    let _ = env_logger::try_init();
                    #[allow(clippy::unused_unit)]
                    {
                        $preamble;
                    };
                    let input = $input2;
                    println!("{}", input);
                    let input = Day::get_input(input)?;
                    println!("{:#?}", input);
                    if $part2.is_err() {
                        assert!(Day::part2(&input).is_err());
                    } else {
                        assert_eq!($part2.unwrap(), Day::part2(&input)?);
                    }
                    Ok(())
                }
            }
        };
        ($id:ident => input1 = $input1:expr; part1_e = $part1_e:expr; input2 = $input2:expr; part2_e = $part2_e:expr;) => {
            sample_case! { $id =>  preamble = {()}; input1 = $input1; part1_r = Err::<_, &'static str>($part1_e); input2 = $input2; part2_r = Err::<_, &'static str>($part2_e); }
        };
        ($id:ident => preamble = $preamble:expr; input1 = $input1:expr; part1 = $part1:expr; input2 = $input2:expr; part2 = $part2:expr;) => {
            sample_case! { $id =>  preamble = $preamble; input1 = $input1; part1_r = Ok::<_, &'static str>($part1); input2 = $input2; part2_r = Ok::<_, &'static str>($part2); }
        };
        ($id:ident => input1 = $input1:expr; part1 = $part1:expr; input2 = $input2:expr; part2 = $part2:expr;) => {
            sample_case! { $id =>  preamble = {()}; input1 = $input1; part1_r = Ok::<_, &'static str>($part1); input2 = $input2; part2_r = Ok::<_, &'static str>($part2); }
        };
        ($id:ident => preamble = $preamble:expr; input = $input:expr; part1 = $part1:expr; part2 = $part2:expr;) => {
            sample_case! { $id =>  preamble = $preamble; input1 = $input; part1_r = Ok::<_, &'static str>($part1); input2 = $input; part2_r = Ok::<_, &'static str>($part2); }
        };
        ($id:ident => input = $input:expr; part1 = $part1:expr; part2 = $part2:expr;) => {
            sample_case! { $id =>  preamble = (); input = $input; part1 = $part1; part2 = $part2; }
        };
    }

#[macro_export]
#[allow(clippy::crate_in_macro_def)]
macro_rules! prod_case {
    (part1 = $part1:expr; part2 = $part2:expr;) => {
        mod prod {
            use super::*;
            use aoc_utils::utils::file::get_input_path;
            use miette::{IntoDiagnostic, WrapErr};
            use std::fs::read_to_string;

            #[test]
            fn part1() -> miette::Result<()> {
                let _ = env_logger::try_init();
                let input_path = get_input_path(crate::YEAR, Day::day())?;
                let input = read_to_string(input_path)
                    .into_diagnostic()
                    .wrap_err("failed to read input")?;
                let input = Day::get_input(&input)?;
                assert_eq!($part1, Day::part1(&input)?);
                Ok(())
            }

            #[test]
            fn part2() -> miette::Result<()> {
                let _ = env_logger::try_init();
                let input_path = get_input_path(crate::YEAR, Day::day())?;
                let input = read_to_string(input_path)
                    .into_diagnostic()
                    .wrap_err("failed to read input")?;
                let input = Day::get_input(&input)?;
                assert_eq!($part2, Day::part2(&input)?);
                Ok(())
            }
        }
    };
}
