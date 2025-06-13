extern crate proc_macro;

use proc_macro::TokenStream;
use quote::quote;
use syn::{DeriveInput, parse_macro_input};

//TODO: first pass enum (label variant, jumps use labels)

#[proc_macro_derive(DecodedInstructionEnum)]
pub fn derive_decoded_instruction_enum(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);

    let name: &syn::Ident = &input.ident;

    let variants = if let syn::Data::Enum(data_enum) = &input.data {
        &data_enum.variants
    } else {
        panic!("DecodedInstructionEnum can only be derived for enums");
    };

    let jumpy_instructions = ["Jump", "JumpIf", "JumpIfFalse", "CallFunction"];

    let variants_no_jump = if let syn::Data::Enum(data_enum) = &input.data {
        &data_enum
            .variants
            .iter()
            .filter(|&v| !jumpy_instructions.contains(&v.ident.to_string().as_str()))
            .cloned()
            .collect::<Vec<_>>()
    } else {
        panic!("DecodedInstructionEnum can only be derived for enums");
    };

    let op_code_variant_iter_match_arms = variants.iter().map(|v| &v.ident);

    let generated_variants = variants.iter().map(|variant| {
        let ident = &variant.ident;
        let args_ident = syn::Ident::new(&format!("{}Args", ident), ident.span());

        quote! {
            #ident(#args_ident)
        }
    });

    let generated_variants_no_jump = variants_no_jump.iter().map(|variant| {
        let ident = &variant.ident;
        let args_ident = syn::Ident::new(&format!("{}Args", ident), ident.span());

        quote! {
            #ident(#args_ident)
        }
    });

    let generated_variants_no_jump_to_decoded_collect = variants_no_jump.iter().map(|variant| {
        let ident = &variant.ident;

        quote! {
            UnprocessedInstruction::#ident(args) =>{
                let new_instruction = DecodedInstruction::#ident(*args);
                final_instructions.push(new_instruction);
            },
        }
    });

    let generated_variants_no_jump_to_decoded_count = variants_no_jump.iter().map(|variant| {
        let ident = &variant.ident;
        let args_ident = syn::Ident::new(&format!("{}Instruction", ident), ident.span());

        quote! {
            UnprocessedInstruction::#ident(args) =>{
                byte_count += #args_ident::instr_size() as u64;
            },
        }
    });

    let decode_match_arms = variants.iter().map(|variant| {
        let ident = &variant.ident;
        let args_ident = syn::Ident::new(&format!("{}Args", ident), ident.span());

        quote! {
            OpCode::#ident => DecodedInstruction::#ident(#args_ident::parse_args(bytes)),
        }
    });

    let encode_match_arms = variants.iter().map(|variant| {
        let ident = &variant.ident;
        let instr_ident = syn::Ident::new(&format!("{}Instruction", ident), ident.span());

        quote! {
            DecodedInstruction::#ident(args) => #instr_ident::encode(*args),
        }
    });

    let instr_size_match_arms = variants.iter().map(|variant| {
        let ident = &variant.ident;
        let instr_ident = syn::Ident::new(&format!("{}Instruction", ident), ident.span());

        quote! {
            DecodedInstruction::#ident(_) => #instr_ident::instr_size(),
        }
    });

    let op_code_instr_size_match_arms = variants.iter().map(|variant| {
        let ident = &variant.ident;
        let instr_ident = syn::Ident::new(&format!("{}Instruction", ident), ident.span());

        quote! {
            OpCode::#ident => #instr_ident::instr_size(),
        }
    });

    let args_size_match_arms = variants.iter().map(|variant| {
        let ident = &variant.ident;
        let instr_ident = syn::Ident::new(&format!("{}Instruction", ident), ident.span());

        quote! {
            DecodedInstruction::#ident(_) => #instr_ident::args_size(),
        }
    });

    let op_code_args_size_match_arms = variants.iter().map(|variant| {
        let ident = &variant.ident;
        let instr_ident = syn::Ident::new(&format!("{}Instruction", ident), ident.span());

        quote! {
            OpCode::#ident => #instr_ident::args_size(),
        }
    });

    let exec_match_arms = variants.iter().map(|variant| {
        let ident = &variant.ident;
        let instr_ident = syn::Ident::new(&format!("{}Instruction", ident), ident.span());

        quote! {
            DecodedInstruction::#ident(args) => #instr_ident::decoded_handler(executor, args),
        }
    });

    let exec_instr_match_arms = variants.iter().map(|variant| {
        let ident = &variant.ident;
        let instr_ident = syn::Ident::new(&format!("Executable{}", ident), ident.span());

        quote! {
            DecodedInstruction::#ident(args) => Box::new(#instr_ident::new(args)),
        }
    });

    let exec_instr_fn_match_arms = variants.iter().map(|variant| {
        let ident = &variant.ident;
        let instr_ident = syn::Ident::new(&format!("{}Instruction", ident), ident.span());

        quote! {
            DecodedInstruction::#ident(args) => Box::new(move |executor: &mut VmInterpretedExecutor| -> Result<(), VmExecutionError> {
                #instr_ident::decoded_handler(executor, args)
            }),
        }
    });

    let op_code_handler_arms = variants.iter().map(|variant| {
        let ident = &variant.ident;
        let instr_ident = syn::Ident::new(&format!("{}Instruction", ident), ident.span());

        quote! {
            OpCode::#ident => #instr_ident::handler(executor, bytes),
        }
    });

    let op_code_dispatch_table_arms = variants.iter().map(|variant| {
        let ident = &variant.ident;
        let instr_ident = syn::Ident::new(&format!("{}Instruction", ident), ident.span());
        let args_ident = syn::Ident::new(&format!("{}Args", ident), ident.span());

        quote! {
            dispatch_table[OpCode::#ident as usize] = (#instr_ident::handler, <#args_ident as InstructionArgsFromStr>::encode_from_strs, #args_ident::args_size());
        }
    });

    let get_doc_arms = variants.iter().map(|variant| {
        let ident = &variant.ident;
        let instr_ident = syn::Ident::new(&format!("{}Instruction", ident), ident.span());

        quote! {
            OpCode::#ident => #instr_ident::docs(),
        }
    });

    // TODO encode for Decoded (turn to bytecode)
    // TODO auto generate Unprocessed to Decoded
    // TODO .. better names?
    let expanded = quote! {
        impl #name {
            pub fn instr_size(self) -> usize {
                match self {
                    #(#op_code_instr_size_match_arms)*
                }
            }

            pub fn init_dispatch_table(dispatch_table: &mut [(OpcodeHandler, ParseHandler, usize)]) {
                #(#op_code_dispatch_table_arms)*
            }

            pub fn args_size(self) -> usize {
                match self {
                    #(#op_code_args_size_match_arms)*
                }
            }

            pub fn variant_iter() -> impl Iterator<Item = Self> {
                [ #( Self::#op_code_variant_iter_match_arms ),* ].into_iter()
            }

            pub fn exec_fn(self, executor: &mut VmInterpretedExecutor, bytes: &[u8]) -> Result<(), VmExecutionError> {
                match self {
                    #(#op_code_handler_arms)*
                }
            }

            pub fn get_doc(self) -> InstructionDocsEntry {
                match self {
                    #(#get_doc_arms)*
                }
            }
        }

        #[derive(Debug, Copy, Clone, PartialEq, PartialOrd)]
        pub enum DecodedInstruction {
            #(#generated_variants,)*
        }

        #[derive(Debug, Clone, PartialEq, PartialOrd)]
        pub enum UnprocessedInstruction {
            Label((String,)),
            Jump((String,)),
            JumpIf((String, RegisterType)),
            JumpIfFalse((String, RegisterType)),
            CallFunction((String)),
            #(#generated_variants_no_jump,)*
        }

        impl UnprocessedInstruction {
            pub fn process_instructions(instructions: &[Self]) -> Vec<DecodedInstruction> {
                use std::collections::HashMap;
                let mut byte_count: u64 = 0;
                let mut label_to_byte_offset: HashMap<String, u64> = HashMap::new();
                let mut final_instructions = Vec::new();
                for instr in instructions {
                    match instr {
                        #(#generated_variants_no_jump_to_decoded_count)*
                        UnprocessedInstruction::Label((label,)) => {
                            label_to_byte_offset.insert(label.to_string(), byte_count);
                        },
                        UnprocessedInstruction::Jump((String,)) => {
                            byte_count += JumpInstruction::instr_size() as u64;
                        },
                        UnprocessedInstruction::JumpIf((String, RegisterType)) => {
                            byte_count += JumpIfInstruction::instr_size() as u64;
                        },
                        UnprocessedInstruction::JumpIfFalse((String, RegisterType)) => {
                            byte_count += JumpIfFalseInstruction::instr_size() as u64;
                        },
                        UnprocessedInstruction::CallFunction((String)) => {
                            byte_count += CallFunctionInstruction::instr_size() as u64;
                        },
                    }
                }
                for instr in instructions {
                    match instr {
                        #(#generated_variants_no_jump_to_decoded_collect)*
                        UnprocessedInstruction::Label((label,)) => {
                        },
                        UnprocessedInstruction::Jump((label,)) => {
                            let jump_offset = label_to_byte_offset[label];
                            final_instructions.push(DecodedInstruction::Jump((jump_offset,)))
                        },
                        UnprocessedInstruction::JumpIf((label, reg)) => {
                            let jump_offset = label_to_byte_offset[label];
                            final_instructions.push(DecodedInstruction::JumpIf((jump_offset, *reg)))
                        },
                        UnprocessedInstruction::JumpIfFalse((label, reg)) => {
                            let jump_offset = label_to_byte_offset[label];
                            final_instructions.push(DecodedInstruction::JumpIfFalse((jump_offset, *reg)))
                        },
                        UnprocessedInstruction::CallFunction((label)) => {
                            let jump_offset = label_to_byte_offset[label];
                            final_instructions.push(DecodedInstruction::CallFunction((jump_offset,)))
                        },
                    }
                }

                final_instructions
            }
        }

        impl DecodedInstruction {
            pub fn decode(opcode: OpCode, bytes: &[u8]) -> Self {
                match opcode {
                    #(#decode_match_arms)*
                }
            }

            pub fn encode(&self) -> Vec<u8> {
                match self {
                    #(#encode_match_arms)*
                }
            }

            pub fn exec(self, executor: &mut VmInterpretedExecutor) -> Result<(), VmExecutionError> {
                match self {
                    #(#exec_match_arms)*
                }
            }

            pub fn exec_instr(self) -> Box<dyn ExecutableInstruction> {
                match self {
                    #(#exec_instr_match_arms)*
                }
            }

            pub fn exec_instr_fn(self) -> ExecutableInstructionFn {
                match self {
                    #(#exec_instr_fn_match_arms)*
                }
            }

            pub fn instr_size(self) -> usize {
                match self {
                    #(#instr_size_match_arms)*
                }
            }

            pub fn args_size(self) -> usize {
                match self {
                    #(#args_size_match_arms)*
                }
            }
        }
    };

    TokenStream::from(expanded)
}
