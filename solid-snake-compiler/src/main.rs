pub mod ast;
pub mod bytecode_gen;
pub mod error_reporting;
pub mod intermediate_pass1;
pub mod new_parser;
pub mod parser;
pub mod preprocessor;
pub mod test_util;

use std::{io::Read, process::exit};

use bytecode_gen::lower_ir_to_bytecode_stage_one;
use colored::Colorize;
use error_reporting::report_error;
use intermediate_pass1::analyze_ast;
use parser::{Rule, SolidSnakeParser, build_ast};
use preprocessor::{PreprocessResult, preprocess_indentation};

use pest::Parser;
use solid_snake_vm::{
    executor::{ext::VmExecutorExt, interpreted::implimentation::VmInterpretedExecutor},
    opcodes::{DecodedInstruction, UnprocessedInstruction},
};

// TODO unary op type checking?
fn main() {
    env_logger::init();

    let file_name = "./test_data/test.ss";
    let mut source_file = std::fs::File::open(file_name).unwrap();
    let mut source = String::new();
    source_file.read_to_string(&mut source).unwrap();

    // TODO add warnings list. Same as errors but not fatal. Maybe just add them to error list tbh
    // TODO report pest errors in same way
    match preprocess_indentation(&source) {
        Ok(preprocessed) => {
            // println!("{}", preprocessed.transformed);
            match SolidSnakeParser::parse(Rule::Program, &preprocessed.transformed) {
                Ok(pairs) => {
                    // dbg!(&pairs);
                    let (ast, errors_list, _) = build_ast(pairs, 0);

                    // dbg!(&ast);

                    let mut context = analyze_ast(&ast);
                    context.errors_mut().extend(errors_list);

                    // dbg!(context.ir());
                    for ir in context.ir() {
                        // println!("{ir}");
                    }
                    // dbg!(context.var_type_map());
                    // dbg!(&scope_errors);

                    // dbg!(use_map);

                    let var_count = context.var_count();

                    for ir in context.typed_ir() {
                        println!("{ir}");
                    }

                    let (bc1, constants) =
                        match lower_ir_to_bytecode_stage_one(context.typed_ir(), var_count) {
                            Ok(v) => v,
                            Err(error) => {
                                context.errors_mut().push_error(error);
                                (Vec::new(), Vec::new())
                            }
                        };
                    for bc in &bc1 {
                        println!("{bc:?}");
                    }

                    for error in context.errors().err_iter() {
                        report_error(error, file_name, &preprocessed)
                    }

                    if context.errors().is_fatal() {
                        let error_line = format!(
                            "Could not continue due to the above {} errors.",
                            context.errors().error_count()
                        );
                        eprintln!("\n{}\n", error_line.red().bold());
                        exit(-1);
                    }

                    let final_bc = UnprocessedInstruction::process_instructions(&bc1)
                        .iter()
                        .flat_map(DecodedInstruction::encode)
                        .collect::<Vec<u8>>();

                    let mut vm = VmInterpretedExecutor::new();
                    dbg!(&constants);
                    vm.set_constants(constants);

                    let processed_bytecode = vm.preprocess_bytecode(&final_bc).unwrap();
                    let decoded = processed_bytecode
                        .iter()
                        .map(|(d, _)| *d)
                        .collect::<Vec<_>>();
                    // dbg!(&decoded);

                    vm.execute_processeded_bytecode(&processed_bytecode)
                        .unwrap();
                }
                Err(e) => {
                    // TODO : Also use ariadne here
                    eprintln!("Syntax Error: {}", e);
                }
            }
        }
        Err(errors) => {
            let preprocessed = PreprocessResult {
                original: source.clone(),
                transformed: source.clone(),
                rev_offset_map: (0..source.len()).map(Some).collect(),
            };
            for err in errors.err_iter() {
                report_error(err, file_name, &preprocessed);
            }
        }
    }
}
