
pub fn reg_index(s: &str) -> u8 {
    s.trim_start_matches('R').parse::<u8>().unwrap()
}

#[macro_export]
macro_rules! asm {
    ( $( $instr:ident $(, $arg:tt)* );* $(;)?) => {{
        let mut bytecode = Vec::new();
        $(
            bytecode.extend($crate::asm_line!($instr $(, $arg)*));
        )*
        bytecode
    }};
}

#[macro_export]
macro_rules! asm_line {
    // No-arg instructions
    ($instr:ident) => {{
        paste::paste! {
            use $crate::executor::interpreted::opcode_impl::all::*;
            [<$instr Instruction>]::encode(&())
        }
    }};

    ($instr:ident, $reg:ident, $val:tt) => {{
        paste::paste! {
            {
                use $crate::executor::interpreted::opcode_impl::all::*;
                let val = $val;
                let args = (
                    $crate::RegisterType::from($crate::asm_internal::reg_index(stringify!($reg))),
                    val,
                );
                [<$instr Instruction>]::encode(&args)
            }
        }
    }};
    

    // All-register instruction (2 or more regs)
    ($instr:ident, $($reg:ident),+) => {{
        paste::paste! {
            use $crate::executor::interpreted::opcode_impl::all::*;
            let args = (
                $(
                    $crate::RegisterType::from($crate::asm_internal::reg_index(stringify!($reg))),
                )+
            );
            [<$instr Instruction>]::encode(&args)
        }
    }};
}

#[macro_export]
macro_rules! asm_old {
    ($instr:ident $($reg:ident),* $(,)? ) => {{
        paste::paste! {
            {
                use $crate::executor::interpreted::opcode_impl::all::*;
                let args = (
                    $( $crate::RegisterType::from( $crate::asm_internal::reg_index(stringify!($reg)) ), )*
                );
                [<$instr Instruction>]::encode(&args)
            }
        }
    }};
    ($instr:ident $reg1:ident, $val:literal $(,)?) => {{
        paste::paste! {
            {
                use $crate::executor::interpreted::opcode_impl::all::*;
                let args = (
                    $crate::RegisterType::from( $crate::asm_internal::reg_index(stringify!($reg1)) ),
                    $val,
                );
                [<$instr Instruction>]::encode(&args)
            }
        }
    }};
}
