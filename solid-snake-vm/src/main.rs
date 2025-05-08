use std::{error::Error, time::Instant};

use bytecode_parser::parse_byte_code_from_txt;
use executor::{
    ext::VmExecutorExt,
    interpreted::{
        implimentation::{RegisterFileExt, VmInterpretedExecutor},
        opcode_decoder::RegisterType,
    },
};

use opcodes::OpCode;
use rustc_version_runtime::version;
use sysinfo::{System, get_current_pid};

mod asm_internal;
mod bytecode_parser;
mod executor;
mod opcodes;

fn main() -> Result<(), Box<dyn Error>> {
    env_logger::init();

    println!("🐍 Solid Snake starting..");

    // vm_tests();

    let fib_n = 80;
    let iter_n = 1000;
    bench_vm_fib(fib_n, iter_n);
    bench_native_fib(fib_n, iter_n);
    for n in 0..20 {
        println!("{}", n);
        bench_native_fib_recursive(n, 200);
    }
    for n in 0..20 {
        println!("{}", n);
        bench_vm_fib_recursive(n, 200);
    }

    let mut system = System::new();
    system.refresh_all();
    let rust_info = version();
    const VERSION: &str = env!("CARGO_PKG_VERSION");

    println!();
    println!("🐍 Solid Snake {} Repl", VERSION);
    println!();

    // Display system information:
    println!(
        "System:            {} {}",
        sysinfo::System::name().unwrap_or_else(|| "Unknown".to_string()),
        sysinfo::System::os_version().unwrap_or_else(|| "unknown".to_string())
    );
    println!(
        "Kernel             {}",
        sysinfo::System::kernel_version().unwrap_or_else(|| "Unknown".to_string())
    );
    println!(
        "Rust version:      {}.{}.{}",
        rust_info.major, rust_info.minor, rust_info.patch
    );
    if let Ok(current_pid) = get_current_pid() {
        let current_process_opt = system.process(current_pid);
        if let Some(current_process) = current_process_opt {
            let mem_bytes = current_process.memory();
            println!(
                "Memory used:       {}.{}mb",
                mem_bytes / (1024 * 1024),
                (mem_bytes / 1024) % 1024
            );
        }
    }

    println!();

    let mut stdin = std::io::stdin();
    let _ = std::io::Read::read(&mut stdin, &mut [0u8]).unwrap();

    Ok(())
}

fn bench_vm_fib(fib_n: i64, n_iter: usize) {
    use std::time::Instant;

    let code = format!(
        "
; Setup
LoadImmediateI64 R10 0        ; a = 0
LoadImmediateI64 R11 1        ; b = 1
LoadImmediateI64 R12 0        ; i = 0
LoadImmediateI64 R13 {fib_n}  ; n = {fib_n}
DecrementI64 R13 1            ; We need to start at n - 1

loop:
MoveI64 R14 R11               ; tmp = b
AddI64 R11 R10 R11            ; b = a + b
MoveI64 R10 R14               ; a = tmp
IncrementI64 R12 1            ; i++
LessThanI64 R15 R12 R13       ; i < n
JumpIf loop R15
Halt                          ; R10 = F(80)
",
    );

    let (bc, _lined_code) = parse_byte_code_from_txt(&code);

    let mut vm = VmInterpretedExecutor::new();

    let mut total = 0u128;
    let bc = vm.preprocess_bytecode(&bc).unwrap();
    for _ in 0..n_iter {
        let start = Instant::now();
        vm.execute_processeded_bytecode(&bc).unwrap();
        let duration: std::time::Duration = start.elapsed();
        total += duration.as_nanos();
    }

    let res: i64 = vm.registers().get_register_value(11_u8).unwrap();
    println!(
        "VM avg over {n_iter} runs: {} ns ({})",
        total / (n_iter as u128),
        res
    );
}

fn bench_vm_fib_recursive(fib_n: i64, n_iter: usize) {
    use std::time::Instant;

    let code = format!(
        "
; Entry Point (Main)
LoadImmediateI64 R1 {fib_n}      ; R1 = n
CallFunction fib
Halt

; Function fib(n) -- expects n in R1, returns in R0
fib:
    LoadImmediateI64 R2 1
    LessThanOrEqualI64 R3 R1 R2
    JumpIf fib_base_case R3

    ; Store original R1 (n) to temp
    MoveI64 R10 R1

    ; fib(n-1)
    LoadImmediateI64 R2 1
    SubtractI64 R1 R10 R2         ; R1 = n - 1
    CallFunction fib
    MoveI64 R11 R0           ; fib(n-1)

    ; fib(n-2)
    LoadImmediateI64 R2 2
    SubtractI64 R1 R10 R2         ; R1 = n - 2
    CallFunction fib
    MoveI64 R12 R0           ; fib(n-2)

    AddI64 R0 R11 R12
    Return

fib_base_case:
    MoveI64 R0 R1
    Return

"
    );

    let (bc, _lined_code) = parse_byte_code_from_txt(&code);

    // _lined_code.iter().for_each(|(line, idx)| {
    //     println!("{idx:0>3} : {line}",)
    // });

    let mut vm = VmInterpretedExecutor::new();

    let mut total1 = 0u128;
    for _ in 0..n_iter {
        let start = Instant::now();
        vm.preprocess_bytecode(&bc).unwrap();
        let duration: std::time::Duration = start.elapsed();
        total1 += duration.as_nanos();
    }
    println!(
        "VM recursive preprocess avg over {n_iter} runs: {} ns",
        total1 / (n_iter as u128)
    );

    let mut total = 0u128;
    let pbc = vm.preprocess_bytecode(&bc).unwrap();
    for _ in 0..n_iter {
        let start = Instant::now();
        vm.execute_processeded_bytecode(&pbc).unwrap();
        let duration: std::time::Duration = start.elapsed();
        total += duration.as_nanos();
    }

    let res: i64 = vm.registers().get_register_value(0_u8).unwrap();
    println!(
        "VM recursive avg over {n_iter} runs: {} ns ({})",
        total / (n_iter as u128),
        res
    );
}

fn bench_native_fib(fib_n: i64, n_iter: usize) {
    use std::hint::black_box;

    #[inline(never)]
    fn fibonacci(n: i64) -> i64 {
        let mut a: i64 = 0;
        let mut b: i64 = 1;
        for _ in 0..black_box(n) {
            let tmp = black_box(b);
            b = black_box(a) + black_box(b);
            a = black_box(tmp);
        }
        a
    }

    let mut result = 0;
    let start = Instant::now();
    for _ in 0..n_iter {
        result = black_box(fibonacci(fib_n));
    }
    let native_avg = start.elapsed().as_nanos() / (n_iter as u128);
    println!(
        "Native avg over {n_iter} runs: {} ns (total: {})",
        native_avg, result
    );
}

fn bench_native_fib_recursive(fib_n: i64, n_iter: usize) {
    use std::hint::black_box;
    use std::time::Instant;

    #[inline(never)]
    fn fibonacci(n: i64) -> i64 {
        if n <= 1 {
            return n;
        }
        black_box(fibonacci(n - 1)) + black_box(fibonacci(n - 2))
    }

    let mut result = 0;
    let start = Instant::now();
    for _ in 0..n_iter {
        result = black_box(fibonacci(fib_n));
    }
    let native_avg = start.elapsed().as_nanos() / (n_iter as u128);
    println!(
        "Native recursive avg over {n_iter} runs: {} ns (result: {})",
        native_avg, result
    );
}

fn vm_tests() {
    let code = "
LoadImmediateU64 R1 8
Allocate R2 R1       ; R2 = dest
Allocate R3 R1       ; R3 = src

LoadImmediateU64 R4 0xAB
MemSet R3 R4 R1      ; Fill src with 0xAB 

Memcpy R2 R3 R1      ; Copy to dest

; Add some debug instructions to verify contents
LoadImmediateU64 R5 0     ; offset
LoadIndirectWithOffsetU64 R6 R2 R5
DebugPrintU64 R6

Deallocate R2
Deallocate R3
Halt
    ";

    let (bc, _lined_code) = parse_byte_code_from_txt(code);

    let mut vm = VmInterpretedExecutor::new();

    let bytecode = asm! {
        LoadImmediateI64, R1, (-123);
        LoadImmediateI64, R2, 456;
        AddI64, R3, R1, R2;
        DebugPrintI64, R3;
        Halt;
    };

    let bc = vm.preprocess_bytecode(&bytecode).unwrap();
    vm.execute_processeded_bytecode(&bc).unwrap();

    let bytecode = asm!(
        LoadImmediateI64, R1, (-42);
        LoadImmediateI64, R2, 100;
        AddI64, R3, R1, R2;
        DebugPrintI64, R3;
        Halt;
    );
    let bc = vm.preprocess_bytecode(&bytecode).unwrap();
    vm.execute_processeded_bytecode(&bc).unwrap();

    let bytecode = asm!(
        LoadImmediateI64, R1, (-123);
        LoadImmediateI64, R2, 999;
        AddI64, R3, R1, R2;
        DebugPrintI64, R3;
        Halt;
    );

    let bc = vm.preprocess_bytecode(&bytecode).unwrap();
    vm.execute_processeded_bytecode(&bc).unwrap();

    let bytecode = asm_!(LoadImmediateI64 R1, -123);

    _lined_code
        .iter()
        .for_each(|(line, idx)| println!("{idx:0>3} : {line}",));

    let bc = vm.preprocess_bytecode(&bytecode).unwrap();
    vm.execute_processeded_bytecode(&bc).unwrap();

    std::process::exit(0);
}
