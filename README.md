# Rustacor

This is an assembler and VM written in Rust for the platform used in Eric Wastl (@topaz)'s lovely [Synacor Challenge](https://challenge.synacor.com/). The challenge describes the architecture of a fictional 16-bit mini-processor, and includes a binary containing the rest of the challenge.

## Building

Run `cargo build`. Binaries will drop in `target/debug/`.  
For release mode (runs faster), run `cargo build --release`, which will drop binaries in `target/release/`.

## Assembler

Usage: `synasm <input_source> --out <output_binary>`.

Example program:
```
; Simplified fibonacci program 

set $0 0            ; Init variables
set $1 1

out $0
out $1

loop:
    add $2 $0 $1    ; Add
    set $0 $1
    set $1 $2       ; Shift
    
    out $2          ; Print
        
    jmp :loop       ; Loop
```

## VM

The VM contains the assembler, so you can pass an asm file and it'll assemble as well as execute it.
Or you can just pass the binary directly.

Usage: `synvm <binary_file|--asm <asm_file>> [--input_str text] [--input_file file]`