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

    let op_code_variant_iter_match_arms = variants.iter().map(|v| &v.ident);

    let generated_variants = variants.iter().map(|variant| {
        let ident = &variant.ident;
        let args_ident = syn::Ident::new(&format!("{}Args", ident), ident.span());

        quote! {
            #ident(#args_ident)
        }
    });

    let decode_match_arms = variants.iter().map(|variant| {
        let ident = &variant.ident;
        let args_ident = syn::Ident::new(&format!("{}Args", ident), ident.span());

        quote! {
            OpCode::#ident => DecodedInstruction::#ident(#args_ident::parse_args(bytes)),
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
        }

        #[derive(Debug, Copy, Clone, PartialEq, PartialOrd)]
        pub enum DecodedInstruction {
            #(#generated_variants,)*
        }

        impl DecodedInstruction {
            pub fn decode(opcode: OpCode, bytes: &[u8]) -> Self {
                match opcode {
                    #(#decode_match_arms)*
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
