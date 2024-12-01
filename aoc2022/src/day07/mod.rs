use color_eyre::Result;
use id_tree::{InsertBehavior, Node, Tree};
use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{multispace0, newline, not_line_ending},
    combinator::{all_consuming, map, opt},
    error::VerboseError,
    multi::many1,
    sequence::{delimited, terminated, tuple},
    Finish, IResult,
};

use crate::{parsers::number, Runner};

pub struct Day;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum EntryType {
    Dir,
    File,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Entry<'input> {
    t: EntryType,
    name: &'input str,
    size: usize,
}

impl Entry<'_> {
    fn root() -> Self {
        Self {
            t: EntryType::Dir,
            name: "/",
            size: 0,
        }
    }

    fn is_dir(&self) -> bool {
        self.t == EntryType::Dir
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Command<'input> {
    Cd(&'input str),
    Ls(Vec<Entry<'input>>),
}

fn cd(input: &str) -> IResult<&str, Command, VerboseError<&str>> {
    delimited(
        tag("cd "),
        map(not_line_ending::<&str, _>, Command::Cd),
        newline,
    )(input)
}

fn dir(input: &str) -> IResult<&str, Entry, VerboseError<&str>> {
    map(
        tuple((tag("dir "), not_line_ending::<&str, _>)),
        |(_, name)| Entry {
            t: EntryType::Dir,
            name,
            size: 0,
        },
    )(input)
}

fn file(input: &str) -> IResult<&str, Entry, VerboseError<&str>> {
    map(
        tuple((number::<&str, usize, _>, tag(" "), not_line_ending)),
        |(size, _, name)| Entry {
            t: EntryType::File,
            name,
            size,
        },
    )(input)
}

fn entry(input: &str) -> IResult<&str, Entry, VerboseError<&str>> {
    delimited(multispace0, alt((file, dir)), opt(newline))(input)
}

fn ls(input: &str) -> IResult<&str, Command, VerboseError<&str>> {
    let (input, ()) = terminated(map(tag("ls"), drop), newline)(input)?;
    let (input, entries) = many1(entry)(input)?;
    Ok((input, Command::Ls(entries)))
}

fn command(input: &str) -> IResult<&str, Command, VerboseError<&str>> {
    delimited(tuple((multispace0, tag("$ "))), alt((ls, cd)), opt(newline))(input)
}

fn total_size(tree: &Tree<Entry>, node: &Node<Entry>) -> Result<usize> {
    Ok(node
        .children()
        .iter()
        .filter_map(|c| total_size(tree, tree.get(c).unwrap()).ok())
        .sum::<usize>()
        + node.data().size)
}

impl Runner for Day {
    type Input<'input> = Tree<Entry<'input>>;

    fn day() -> usize {
        7
    }

    fn get_input(input: &str) -> Result<Self::Input<'_>> {
        let (_, lines) = all_consuming(many1(command))(input).finish().unwrap();
        let mut tree = Tree::new();
        walk_commands(lines, &mut tree)?;
        Ok(tree)
    }

    fn part1(tree: &Self::Input<'_>) -> Result<usize> {
        let total = tree
            .traverse_pre_order(tree.root_node_id().unwrap())?
            .filter(|n| n.data().is_dir())
            .map(|n| total_size(tree, n).unwrap())
            .filter(|&size| size <= 100_000)
            .sum();
        Ok(total)
    }

    fn part2(tree: &Self::Input<'_>) -> Result<usize> {
        let total_space = 70_000_000;
        let used_space = total_size(tree, tree.get(tree.root_node_id().unwrap())?)?;
        let free_space = total_space - used_space;
        let space_to_free = 30_000_000 - free_space;

        Ok(tree
            .traverse_pre_order(tree.root_node_id().unwrap())?
            .filter(|n| n.data().is_dir())
            .map(|n| total_size(tree, n).unwrap())
            .filter(|&s| s >= space_to_free)
            .min()
            .unwrap())
    }
}

fn walk_commands<'input>(
    input: Vec<Command<'input>>,
    tree: &'_ mut Tree<Entry<'input>>,
) -> Result<()> {
    assert_eq!(input[0], Command::Cd("/"));

    let root = tree.insert(Node::new(Entry::root()), InsertBehavior::AsRoot)?;

    let mut current = root.clone();

    input.into_iter().skip(1).for_each(|command| {
        match command {
            Command::Cd(path) => match path {
                "/" => current = root.clone(),
                ".." => current = tree.get(&current).unwrap().parent().unwrap().clone(),
                path => {
                    current = tree
                        .insert(
                            Node::new(Entry {
                                t: EntryType::Dir,
                                name: path,
                                size: 0,
                            }),
                            InsertBehavior::UnderNode(&current),
                        )
                        .unwrap();
                }
            },
            Command::Ls(entries) => {
                entries
                    .into_iter()
                    // Don't worry about Directories, Cd takes care of that
                    .filter(|e| e.t == EntryType::File)
                    .for_each(|e| {
                        tree.insert(Node::new(e), InsertBehavior::UnderNode(&current))
                            .unwrap();
                    });
            }
        };
    });
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::helpers::{prod_case, sample_case};

    sample_case! {
        sample1 =>
            input = "\
                    $ cd /
                    $ ls
                    dir a
                    14848514 b.txt
                    8504156 c.dat
                    dir d
                    $ cd a
                    $ ls
                    dir e
                    29116 f
                    2557 g
                    62596 h.lst
                    $ cd e
                    $ ls
                    584 i
                    $ cd ..
                    $ cd ..
                    $ cd d
                    $ ls
                    4060174 j
                    8033020 d.log
                    5626152 d.ext
                    7214296 k";
            part1 = 95437;
            part2 = 24933642;
    }

    prod_case! {
        part1 = 1232307;
        part2 = 7268994;
    }
}
