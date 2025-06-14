use crate::executor::{
    ext::{VmExecutionError, VmExecutorExt},
    interpreted::{
        implimentation::{RegisterFileExt, VmInterpretedExecutor},
        opcode_decoder::{RegisterType, RegisterValue, VmErrorCode},
    },
};

pub fn reg_index(s: &str) -> u8 {
    s.trim_start_matches('R').parse::<u8>().unwrap()
}

#[macro_export]
macro_rules! R {
    ($n:expr) => {
        $crate::executor::interpreted::opcode_decoder::RegisterType::from($n)
    };
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
            [<$instr Instruction>]::encode(())
        }
    }};

    // Halt
    (Halt, $exit_code:literal) => {{
        paste::paste! {
            use $crate::executor::interpreted::opcode_impl::all::*;
            HaltInstruction::encode(($exit_code,))
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
                [<$instr Instruction>]::encode(args)
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
            [<$instr Instruction>]::encode(args)
        }
    }};
}

#[macro_export]
macro_rules! asm_ {
    ($instr:ident $($reg:ident),* $(,)? ) => {{
        paste::paste! {
            {
                use $crate::executor::interpreted::opcode_impl::all::*;
                let args = (
                    $( $crate::RegisterType::from( $crate::asm_internal::reg_index(stringify!($reg)) ), )*
                );
                [<$instr Instruction>]::encode(args)
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
                [<$instr Instruction>]::encode(args)
            }
        }
    }};
}

#[macro_export]
macro_rules! parse_arg {
    (R$reg:literal) => {
        RegisterType::from($crate::asm_internal::reg_index(concat!("R", $reg)))
    };
    ($val:literal) => {
        $val
    };
}

pub struct VmTest {
    vm: VmInterpretedExecutor,
    expectations: Vec<Box<dyn Fn(&VmInterpretedExecutor)>>,
    expects_error: bool,
}

impl VmTest {
    pub fn new() -> Self {
        Self {
            vm: VmInterpretedExecutor::new(None),
            expectations: Vec::new(),
            expects_error: false,
        }
    }

    pub fn setup<T: RegisterValue + 'static>(mut self, value: T, reg: RegisterType) -> Self {
        self.vm
            .registers_mut()
            .set_register_value(reg, value.to_u64())
            .expect("setup failed");
        self
    }

    pub fn expect<T: RegisterValue + PartialEq + std::fmt::Debug + 'static>(
        mut self,
        expected: T,
        reg: RegisterType,
    ) -> Self {
        self.expectations.push(Box::new(move |vm| {
            let got: T = vm.registers().get_register_value(reg).unwrap();
            assert_eq!(got, expected, "Expected {:?}, got {:?}", expected, got);
        }));
        self
    }

    pub fn expect_error(mut self, expected: VmErrorCode) -> Self {
        self.expectations.push(Box::new(move |vm| {
            let expected_i = expected as i64;
            assert_eq!(
                vm.error_code, expected_i,
                "Expected error code {}, got {}",
                expected_i, vm.error_code
            );
        }));
        self.expects_error = true;
        self
    }

    pub fn run(mut self, bytecode: Vec<u8>) {
        let bc = self.vm.preprocess_bytecode(&bytecode).unwrap();
        let _ = self.vm.execute_processeded_bytecode(&bc);
        for check in self.expectations {
            check(&self.vm);
        }
        if !self.expects_error && self.vm.error_code != 0 {
            panic!("Expected success, got error code: {}", self.vm.error_code)
        }
    }
}

pub struct VmProgramTest {
    vm: VmInterpretedExecutor,
    program: Vec<u8>,
    expectations: Vec<Box<dyn Fn(&VmInterpretedExecutor)>>,
}

impl VmProgramTest {
    pub fn new() -> Self {
        Self {
            vm: VmInterpretedExecutor::new(None),
            program: Vec::new(),
            expectations: Vec::new(),
        }
    }

    pub fn vm_mut(&mut self) -> &mut VmInterpretedExecutor {
        &mut self.vm
    }

    pub fn setup_register<T: RegisterValue>(mut self, value: T, reg: RegisterType) -> Self {
        self.vm
            .registers_mut()
            .set_register_value(reg, value.to_u64())
            .expect("setup_register failed");
        self
    }

    pub fn with_program(mut self, bytecode: Vec<Vec<u8>>) -> Self {
        self.program = bytecode.into_iter().flatten().collect();
        self
    }

    pub fn expect_register<T: RegisterValue + PartialEq + Copy + std::fmt::Debug + 'static>(
        mut self,
        reg: RegisterType,
        expected: T,
    ) -> Self {
        self.expectations.push(Box::new(move |vm| {
            let got: T = vm.registers().get_register_value(reg).unwrap();
            assert_eq!(
                got, expected,
                "Expected R{} to be {:?}, got {:?}",
                reg, expected, got
            );
        }));
        self
    }

    pub fn expect_pc(mut self, expected_pc: usize) -> Self {
        self.expectations.push(Box::new(move |vm| {
            assert_eq!(
                vm.get_program_counter().unwrap(),
                expected_pc,
                "Expected PC {}, got {}",
                expected_pc,
                vm.get_program_counter().unwrap()
            );
        }));
        self
    }

    pub fn expect_error(mut self, expected_err: VmErrorCode) -> Self {
        self.expectations.push(Box::new(move |vm| {
            assert_eq!(
                vm.error_code, expected_err as i64,
                "Expected error {:?}, got {}",
                expected_err, vm.error_code
            );
        }));
        self
    }

    pub fn run(mut self) -> Result<(), VmExecutionError> {
        let bc = self.vm.preprocess_bytecode(&self.program)?;
        self.vm.execute_processeded_bytecode(&bc)?;

        for check in self.expectations {
            check(&self.vm);
        }

        Ok(())
    }
}

#[macro_export]
macro_rules! define_vm_program_test {
    (
        $name:ident,
        $(setup => [$($setup:expr),*],)?
        program => [$($program:expr),* $(,)?],
        $(expect_register => [$(($reg:expr, $val:expr)),*],)?
        $(expect_error => $err:expr,)?
        $(expect_fail => $fail:pat,)?
        $(expect_pc => $pc:expr,)?
    ) => {
        #[test]
        fn $name() {
            use $crate::executor::interpreted::opcode_impl::all::*;
            let mut test = $crate::asm_internal::VmProgramTest::new();
            $(
                $(
                    test = test.setup_register($setup.0, $setup.1);
                )*
            )?
            test = test.with_program(vec![
                $(
                    $program
                ),*
            ]);

            $(
                $(
                    test = test.expect_register($reg, $val);
                )*
            )?

            $(
                test = test.expect_error($err);
            )?

            $(
                test = test.expect_pc($pc);
            )?

            let result = test.run();

            // If we expected a fail, assert that.
            $(
                let err = result.unwrap_err();
                assert!(matches!(err, $fail));
                return; // if failure expected and asserted, stop here
            )?

            // Otherwise, unwrap normally (success path)
            let _ = result.unwrap();
        }
    };
}
