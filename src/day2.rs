use std::fs::File;
use std::io;
use std::io::{BufRead, BufReader};

#[derive(Debug, PartialEq)]
struct Position {
    first: usize,
    second: usize,
    result: usize,
}

impl Position {
    fn from<'a, I>(iterator: &mut I) -> Position
        where I: Iterator<Item=&'a i32>
    {
        let first = *iterator.next().expect("first position") as usize;
        let second = *iterator.next().expect("second position") as usize;
        let result_position = *iterator.next().expect("result position") as usize;
        Position { first, second, result: result_position }
    }
}

#[derive(Debug, PartialEq)]
enum Instruction {
    Add(Position),
    Mul(Position),
    Halt,
}

impl Instruction {
    fn execute(&self, memory: &mut Vec<i32>) {
        match self {
            Instruction::Add(position) =>
                memory[position.result] = memory[position.first] + memory[position.second],
            Instruction::Mul(position) =>
                memory[position.result] = memory[position.first] * memory[position.second],
            Instruction::Halt => (),
        }
    }
}

#[derive(Debug, PartialEq)]
struct Program {
    program: Vec<Instruction>
}

impl Program {
    fn run(&self, memory: &mut Vec<i32>) {
        self.program.iter().for_each(|instruction| instruction.execute(memory));
    }
}

fn read_program_content(filename: &str) -> io::Result<Vec<i32>> {
    let file = File::open(filename)?;
    let mut reader = BufReader::new(file);
    let mut buffer = String::new();

    reader.read_line(&mut buffer)?;
    Ok(transform_program_into_readable_sequence(&buffer))
}

fn transform_program_into_readable_sequence(program: &str) -> Vec<i32> {
    program.split(',').map(|elem| elem.trim().parse().expect("number")).collect()
}

fn parse_program(program: &Vec<i32>) -> Program {
    let mut result = vec![];
    let mut iterator = program.iter();

    while let Some(instruction) = iterator.next() {
        match instruction {
            1 => result.push(Instruction::Add(Position::from(&mut iterator))),
            2 => result.push(Instruction::Mul(Position::from(&mut iterator))),
            99 => {
                result.push(Instruction::Halt);
                break;
            }
            _ => panic!("program does not having HALT(99) instruction")
        }
    }
    Program { program: result }
}

pub fn program_first_place_value_during_1202(filename: &str) -> io::Result<i32> {
    let mut memory = read_program_content(filename)?;
    reset_memory_before_1202(&mut memory);
    let program = parse_program(&memory);
    program.run(&mut memory);
    Ok(memory[0])
}

fn reset_memory_before_1202(memory: &mut Vec<i32>) {
    memory[1] = 12;
    memory[2] = 2;
}

#[cfg(test)]
mod test {
    use super::*;

    mod transform_program_into_readable_sequence {
        use super::*;

        #[test]
        fn should_parse_tokens_correctly() {
            let program = "1,9,10,3,2,3,11,0,99,30,40,50";
            assert_eq!(vec![1, 9, 10, 3, 2, 3, 11, 0, 99, 30, 40, 50],
                       transform_program_into_readable_sequence(program));
        }

        #[test]
        fn should_parse_tokens_correctly_with_spaces() {
            let program = "1, 9, 10, 3, 2, 3, 11, 0, 99, 30, 40, 50";
            assert_eq!(vec![1, 9, 10, 3, 2, 3, 11, 0, 99, 30, 40, 50],
                       transform_program_into_readable_sequence(program));
        }
    }

    mod parse_program {
        use super::*;

        #[test]
        fn should_parse_add_sequence() {
            let program = vec![1, 9, 10, 3];
            assert_eq!(vec![Instruction::Add(Position { first: 9, second: 10, result: 3 })],
                       parse_program(&program).program);
        }

        #[test]
        fn should_parse_mul_sequence() {
            let program = vec![2, 9, 10, 3];
            assert_eq!(vec![Instruction::Mul(Position { first: 9, second: 10, result: 3 })],
                       parse_program(&program).program);
        }

        #[test]
        fn should_parse_halt_instruction() {
            let program = vec![99];
            assert_eq!(vec![Instruction::Halt],
                       parse_program(&program).program);
        }

        #[test]
        fn should_parse_integer_sequence() {
            let program = vec![1, 9, 10, 3, 2, 3, 11, 0, 99, 30, 40, 50];
            let expected_ast = vec![
                Instruction::Add(Position { first: 9, second: 10, result: 3 }),
                Instruction::Mul(Position { first: 3, second: 11, result: 0 }),
                Instruction::Halt
            ];
            assert_eq!(expected_ast, parse_program(&program).program);
        }
    }

    #[test]
    fn program_should_execute() {
        let mut memory = vec![1, 9, 10, 3, 2, 3, 11, 0, 99, 30, 40, 50];
        let program = Program {
            program: vec![
                Instruction::Add(Position { first: 9, second: 10, result: 3 }),
                Instruction::Mul(Position { first: 3, second: 11, result: 0 }),
                Instruction::Halt
            ]
        };
        program.run(&mut memory);
        assert_eq!(vec![3500, 9, 10, 70, 2, 3, 11, 0, 99, 30, 40, 50], memory);
    }
}