# Appendix 1

you can dynamically generate/send/execute new source down to the MCU. Only some Forths and luaRTOS 


## A Scheme Interpreter for ARM Microcontrollers
ARMPIT Scheme, interpreter runs on RISC microcontrollers with ARM core.

R5RS version, towards R7RS-small, 

* Designed to support multitasking and multiprocessing
* Armpit Scheme to enrich the spectrum of interpreted languages available for MCU's , Basic, forth, MCU based bytecode interpreters, to compiled languages, eg. C.
* Scheme to the metal, as turning the MCU into a rudimentary scheme machine.
* NanoPC-T3 Plus, communicating via USB, Minicom is used to communicate with the board which reads, evaluates and prints the result of the entered experssions.
* In version 080, base character are from 16-bit unicode set, 
* Cortex-M4F, or above, FPU, an MMU or MPU, SD-card for LSD bootloader,user files are stored on the SD card,
* TM4C1294XL, 120MHz, 256KB RAM
* STM32F429 Discovery, 168MHz, 8MB RAM, 

## LambdaChip,
Lightweight, open source virtual machine, designed to run on embedded systems with limited resources, for instance, an 80 MHz microcontroller with 50KB RAM, programmable with Scheme multi-paradigm programming language. For functional programming research and teaching.

LambdaChip Alonzo, STM32 Cortex-M4 development board with 512KB flash, 128KB RAM, with Bluetooth BLE connectivity,


## TinyScheme


## Chicken Scheme Compiler,


## PICOBIT
Scheme on PIC microcontroller,
As little as 7 kb of memory.

## Gambit-C


# How to Use Chez Scheme with Emacs
$ chezscheme --version
9.5

Geiser is a package that provides the ability to run several different Scheme implementations from within Emacs.
Guile is an interpreter, an Virtual Machine.

如何使用geiser, chezscheme, geiser-chez来调试，运行程序呢?

## Geiser还会支持:
- Form evaluation in the context of the current file's module;
- Macro expansion;
- File/module loading and/or compilation;
- Namespace-aware identifier completion ( local bindings, names visible in the current module, and module names)
- Autodoc: the echo area shows information about the signature of the procedure/macro around point automatically
- Jump to definition of identifier at point;
- Access to documentation (including docstrings when the implementation provides it)
- Listings of identifiers exported by a given module;
- Rudimentary support for debugging (when the REPL provides a debugger) and error navigation.
- Support for multiple , simultaneous REPLs;
- Support for image display in those Schemes that treat them as first class values;


## 支持的Scheme版本
- Chez, geiser-chez
- Chibi , geiser-chibi
- Guie, geiser-guile
- Racket, geiser-racket

## 其它的Emacs插件
- Paredit
- Company, company mode, for completion engines with pretty and automatic completion lists
- macrostep-geiser, for in-buffer macro expansion, using the macrostep package,
- ac-geiser, a Geiser plugin for popular Emacs Auto Completion Mode,

### Paredit mode,
如何安装呢？

如何使用呢？
Structured editing of S-expression data. Lisp or Scheme source code.
Keep parentheses balanced. 

Navigating
[C-M-f]
Forward sexp
[C-M-b]
Backward sexp
[C-M-u]
Go up sexp
 )

### Company mode,
Company is a text completion framework for Emacs.
t uses pluggable back-ends and front-ends to retrieve and display completion candidates.It comes with several back-ends such as Elisp, Clang, Semantic, Ispell, CMake, BBDB, Yasnippet, Dabbrev, Etags, Gtags, Files, Keywords and a few others.

```
M-x company-mode,
(add-hook 'after-init-hook 'global-company-mode)

M-x describe-function RET company-mode
```

## macrostep-geiser,

## ac-geiser,
这个可以替代company-mode

## 如何使用geiser呢
ser-font-lock-repl-prompt and geiser-font-lock-repl-input

M-x run-geiser, 默认是chez,
running a REPL server at some known port
M-p, M-n, 切换历史命令
C-c M-p, C-c M-n,
C-c C-q, 退出,

geiser’s gerbils will be scanning what you type
C-c C-d, document,



