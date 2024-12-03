use anyhow::Result;

use crate::{
    machine::{parse_program, Machine, Op},
    Runner,
};

pub struct Day08;

impl Runner for Day08 {
    type Input = Vec<Op>;
    type Output = i32;

    fn day() -> usize {
        8
    }

    fn get_input(input: &str) -> Result<Self::Input> {
        let (_, ops) = parse_program(input).unwrap();
        Ok(ops)
    }

    fn part1(input: &Self::Input) -> Result<Self::Output> {
        let mut machine = Machine::new(input);

        machine.run().unwrap();
        Ok(machine.get())
    }

    fn part2(input: &Self::Input) -> Result<Self::Output> {
        let machine = Machine::new(input);
        let mut machines = input
            .iter()
            .enumerate()
            .rev()
            .filter(|(_, &op)| matches!(op, Op::Jmp(_) | Op::Nop(_)))
            .filter_map(|(idx, _)| {
                let mut machine = machine.clone();
                machine.flip(idx);

                machine.run().unwrap();

                if machine.curr_ip() == input.len() {
                    Some(machine)
                } else {
                    None
                }
            });

        Ok(machines.next().unwrap().get())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sample1() -> Result<()> {
        let input = "nop +0\n\
                     acc +1\n\
                     jmp +4\n\
                     acc +3\n\
                     jmp -3\n\
                     acc -99\n\
                     acc +1\n\
                     jmp -4\n\
                     acc +6";

        let input = Day08::get_input(input)?;
        println!("{:#?}", input);
        assert_eq!(5, Day08::part1(&input)?);
        assert_eq!(8, Day08::part2(&input)?);
        Ok(())
    }
}
