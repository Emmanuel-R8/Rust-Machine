# Open Genera Lisp Machine emulator in Rust

**Why?** Because what can be better than writing an emulator as a way to learn Rust.

**But why?** Because there are already many NES or GameBoy emulators.


# Current status

This version comes after several other stabs when learning Nim and Go.  The starting point is the C
version based on last commit at which Brad's version still compiles is
[https://github.com/Manny8888/vlm/commit/da42b63bdc4eb1b18f816f7fbb93bccef0f8b028]()

## Current environment

- Spacemacs + slime + SBCL for the lisp development. The Lisp code generates some C code and the Alpha
  assembly
- VSCode or KDevelop for the C development
- VSCode for Rust
- cmake is used in VSCode to build the c-emulator (add the relevant cmake extensions in VSCode if desired)

## Steps to run the C version.

### Step 1: recreate the C files from the ALPHA assembly

From an SBCL prompt:
- load/compile stub/convert-asm-to-c.lisp
  *WARNING*: output4.c had to be patched following the supplied diff file. Checked this is OK!

- go into: (in-package :alpha-axp-internals)
- run: (build)


### Step 2: compile the Genera runtime

Ensure the following is available: usual build packages, clang, libpcap-dev

In the top directory:
- make clean
- make genera

Cross fingers...


### Alternative Step 2: compile the Genera_c runtime

Ensure the following is available: usual build packages, clang, libpcap-dev, cmake, ninja
Click the VSCode build button...


# Structure of the emulator

The best way to imagine this is to recall that the Open Genera mimicks the MacIntosh Ivory system. The Ivory
was a addon NuBus card plugged into a Mac.

The emulator is split into 3 parts:
- the "life support" that emulates the hardware environment (console, FEP, disks, network)
- the emulator proper which emulates everything but the CPU
- the CPU

## Life support

The life support is common to all emulators and is coded in C.

It grew out of the Mac Ivory environment where the life support package was
software available on the Mac to interface with the Ivory add-on hardware card
providing the Lisp Machine.

In the Genera version, the emulator replaces the physical add-on card, the
purpose of life support remains the same.

## Emulator

The emulator comes in different flavours: Alpha assembly, C version started but
unfinished by Symbolics, C version by Brian PARKER. The Alpha version is written
in Lisp. The assembly instructions are Lisp functions/macros that are then
assembled by an assembler in Lisp. The Brian P. C version basically takes the Alpha
version and assembles it into C instructions (instead of generating Alpha
binary).

The CPU is provided by the Aplha assembly, the Alpha assembly compiled to C, or
the C emulator (emulator.c).

# Build system

## .sid files

No idea where `.sid` files extension comes from.

They are manipulated by assembler / `alphadsdl.lisp` (Does DSDL mean Document Schema
Definition Language?). As best as I can see, DSDL is a LM facility that
describes a file type and methods to convert it into other file types. Here, the
`.sid` files are converted into `.as` (Alpha assembly), `.c`, `.h` and `.lisp` files.

Note that I have completely ignored the Power G5 version.

## BP Emulator

(BP = Brian PARKER)

`stub/process.lisp`: converts Alpha ISA into C statements, then converts assembly
(written in lisp) files to c files. It has been renamed `convert-asm-to-c.lisp`.

## Various notes

The assembly file describe the instructions as _list of symbols_. Each item in a
list must be a symbol; therefore no item must be evaluated past read time. This
explains why many values are prefixed with the '#.' reader macro which forces a
read-time evaluation (see
<http://www.lispworks.com/documentation/HyperSpec/Body/02_dh.htm>). (Not
entirely sure that this is a correct explanation, but the point remains.)

Far example `fcallmac.lisp` line 42: `#.1_22` forces the read-time
evaluation of `1_22` which is a constant defined as `2^22`.
