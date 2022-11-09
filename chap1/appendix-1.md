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
C-L, list lines,
C-J, middle lines,

> (library-directories)
(("." . "."))

### 参考资料 How to Debug Chez Scheme Program
https://www.scheme.com/debug/debug.html


Modus operandi,
Scheme-Elisp interface makes some assumptions about the capabilites and interaction mode of the corresponding REPL.  Geiser expects the latter to support namespaces in the form of a module system, and to provide a well-defined way to establish the REPL's current namespace (or module), as well as the current file's module (or namespace). Thus , all evaluations performed by Geiser either in the REPL or in a source code buffer happen in the context  of the current namespace. Every time you switch to a different file, you'r switching namespaces automatically; at the REPL, you must request  the switch explicitly (usually just using means provided by the Scheme implementation itself).


Geiser Chez REPL

## Must needs,
Find Geiser's package in NonGNU ELPA,  MELPA, 

M-x package-install RET geiser-mit RET

C-c C-z ;; 切换到REPL,


## Basic debug techniques
R. Kent Dybvig, Aug 2002, 
* Basic debugging techniques, isolate 95% bugs,
* Advanced debugging technques,

### Printing messages,

### Tracing,
an automated mechanism for printing messages,

* Basic debugging techniques, isolate 95% bugs,
* Advanced debugging technques,

### Printing messages,

### Tracing,
an automated mechanism for printing messages,

```scheme
1. (trace name ...)
2. (untrace name null?)
3. (trace-define)
4. (trace-lambda)

```



## Advanced techniques
(debug)
?
To inspet the error contiuation, stack, at the point where the error occurred.




### Geiser debugger?



## auto complete mode
* company-mode, (global-company-mode) , 
* ac-geiser

```lisp
;; auto-complete
(add-to-list 'load-path "~/.emacs.d/")
(require 'auto-complete-config)
(add-to-list 'ac-dictionary-directories "~/.emacs.d//ac-dict")
(ac-config-default)
```


## load problem

```
> (library-directories)
(("." . "."))
> (library-extensions)
((".chezscheme.sls" . ".chezscheme.so")
(".ss" . ".so")
(".sls" . ".so")
(".scm" . ".so")
(".sch" . ".so")) 


```

如何改变呢？
export CHEZSCHEMELIBDIRS="/home/username/chez-lib:"
export CHEZSCHEMELIBEXTS=".sc::.so:"
在~/.bashrc 文件中,
测试一下这个。

emacs Exception in load no such file or directory?
emacs tries to require the session package

(load "file"), 只要将路径设置正确就可以了!
这是什么问题呢？又在Ubuntu机子上面复现了。
错误信息: Exception in load: failed for no such file or directory
解决办法: 需要定义source-directories, 来重新定义set of directories searched for source or object files, when a file is loaded via load.
(load "/mnt/data/project/git-books/eopl-gitbook/chap1/test.scm")
这样就可以了。总觉得没有完全解决这个问题。

How to execute via (load) access it's own file path?
实际上如果目录在~/下面的话，似乎就没有问题了。可以load test.scm.

Chez寻找文件的搜索路径, include, load会使用source-directories. 在REPL里load *.scm文件。


Is it expected that the file is not found despite the path being in 'library-directories'?
Is there a way to set at system level a path to be added to 'source-directories'?

library-directories,
with-source-path'
schemerc file?

```
(source-directories '("." "/location/of/types"))

```
 chez并没有提供设置soource-directories的方法，

 (current-directory)

#### compiler and loader,
eval, compile, compile-file, load,

The data in case the machine code produced by the compiler or loaded from a compiled file written by one thread is not necessarily available immediately in other threads.

##### source path handling,
list value of the source-directories parameters, How to find the source-directories? It doesnt function in other disk.

Use an absolute path for the file or use source-directories to redefine the set of directories searched for source or object files when a file is loaded via load.

The parameter source-directories determines the set of directories searched for source files not identified by absolute path names.

```scheme 
(current-directory)
;; 打印当前目录,居然能识别出项目目录，这个就非常奇怪了!是因为识别了.git目录吗?
;; 一种迂回的方式就是去生成绝对路径，使用current-directory 来保持灵活性
(string-append (current-directory) "/" "file.scm")

```
忽然发现可以了。绝对路径，

### markdown mode
C-c C-s styling text
C-c C-x, switching commands, toggling,
C-c C-h, 获取help页面,

C-h C-a, Emacs的帮助页面,

Ctrl-Tab, switch tabs in Windows Terminal,















