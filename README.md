# Advent of Code Projects

This is a collection of my Advent of Code solutions.

## Setup

### Adding a new year

To add a new year, copy the `template` folder and rename it to the year.

```sh
cp -r template aoc2024
```

Then, edit the `.env` file in the new folder. Though this is not required if it is still a 
valid session token from adventofcode.com

### Notes for starting with new computer.

This project uses [git-crypt](https://github.com/AGWA/git-crypt) to encrypt my secrets with gpg.
If you don't have access to the private keys anymore, you can just generate a new `.env` file by 
copying the `dot-env.example` file.

```sh
cp dot-env.example .env
```
