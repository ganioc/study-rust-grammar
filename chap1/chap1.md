# 1 Tools for Symbolic Programming

符号化编程的工具

You are about to begin the study of programming language. It would be convenient if all the concepts behind programming languages could be explained using only natural language. Unfortunately, many aspcects of programming languages requre a high degree of precision, and natural language is not precise enough. In additon, we want to do more than just describe programming languages: we want to show how they are implemented. To fulfill these purposes, we use three different modes of expression:

1. For motivation and explanation, we use ordinary English.
2. When we need to be precise, we use the language of elementary mathematics: sets, functions, and elementary algebra.
3. When we need to display actual algorithms and implementations, we use the programming language Scheme.

注: 我们希望完全能够使用自然语言，如英语来描述编程语言的概念。然而自然语言缺乏所需的精确性。另一方面，我们不但要描述编程语言，我们还要描述编程语言的实现细节。因此我们选取了不同的方式来实现我们的任务:

1. 使用英语来描述意图，进行解释 。
2. 在需要精确表达的场合，我们使用初等数学: sets(集合), functions(函数), elementary algebra(初等代数).
3. 使用Scheme语言来描述具体算法和实现。

The first two chapters of this book are designed to familiarizes you with reading and writing Scheme programs. This includes a number of techniques for manipulating symbolic information of the kind found in programs and their run-time data structures. Particular emphasis is placed on techniques for functional programming, especially recursion. Along the way, we also introduce several fundamental concepts that allow us to talk precisely about programming languages. These are the basics you need to understand the rest of the book.

注: 读者将通过前2章来熟悉Scheme编程语言。包括一些技巧，操作程序中的字符，运行的数据结构。着重于函数式编程，及递归，及若干描绘编程语言的基本概念。

We begin in section 1.1 by introducitng several basic ways of building Scheme expressions. Section 1.2 introduces a few primitive data types that taken together make Scheme especially suitable for symbolic programming. This is followed in section 1.3 by a discussion of procedures, which may be used with more flexibiltiy in Scheme than in most other languages.

For every form of expression ( or other components of a program ) it si useful to distinguish between its syntax, which refers to rules governing how it is formed, and its semantics, which refers to rules that specify its meaning.

If you are already familiar with Scheme or another dialect of LISP, you may wish to skim this chapter quickly, taking note of any unfamiliar terminology (indicated by italics), and later refer to this chapter for specific information on Scheme. We introduce only those features of Scheme that are used later in this book.

注: 1.1节论述Scheme表达式, 1.2节介绍几个基本数据类型，1.3节讨论procedure过程, 灵活的过程，还有关于表达式的形式

  LISP和Scheme有什么区别？
  答: 似乎Scheme更简洁、更小型化一些。


  运行Scheme解释器所需要的硬件?
  答: 也可以运行在MCU之上。

---

## 1.1 Simple Expressions

A **statement** is a programming language construct that is evaluated only for its effect. Examples include assignment statement, input/output statements, and control statements(while loops, if statements, etc.). Programs in most languages are composed primarily of statements; such languages are said to be **statement oriented**.

Programming language constructs that are evaluated to obtain values are called **expressions**. Arithmetic expressions are the most common example. Expressions may occur as parts of statements, as in the right-hand side of an assignment statement. The data that may be returned as the values of expressions constitute the expressed values of a programming language. Expressions that are evaluated solely for their value, and not for any other effects of the computation, are said to be **functional**.

Some programming languages, such as Scheme , are **expression oriented**: their programs are constructed of definitions and expressions; there are no statements. This section reviews basic techniques for constructing expressions in Scheme.

### 1.1.1 Literals , Procedure Calls, and Variables,
字符常量, 过程调用, 变量

The simplest form of expression is a **literal** (or constant), which always returns the indicated value. For example, the result of evaluating the **numeral** 2 is a value denoting the number two, which has the printed representation 2. Other literals we shall have occasion to use include **strings**, such as "This is a string.", the **boolean** value  #t (true) anbd #f (false), and **characters**, such as #\a and #\space. We discuss these and other Scheme data types in the next section.

注: 最简单的表达式是literal, 数字，字符串，布尔值，字符，都属于。

The next simplest form of expression is a **variable reference**. The value of a variable reference is the value currently associated with, or **bound** to , the variable. A variable is said to **denote** the value of its **binding**. The data that can be bound to variable constitute the **denoted values** of a programming language. Since all variable references in Scheme are also expressions, and the value of any expression may be bound to a variable, the denoted values and the expressed values of Scheme are the same, at least in the absence of variable assignment (section 4.5).

注: 第二简单的表达式是变量引用。一个变量引用的值就是它当前绑定，索引的值。变量表达了绑定的值。可以被绑定到变量的数据，构成了编程语言的值的表达方式。在Scheme中，所有的变量引用都是表达式，所有的表达式的值都可以绑定到一个变量，表达值和表达式是同一个东西，在没有强行进行变量赋值的情况下(4.5小节)。

Variables are represented by **identifiers**. As in most languages, sequences of letters and digits(not staring with a digit) may be used as identifiers, for example: x, x3, foo, and lognidentifer. Scheme is more premissive than most languages in the use of special characters to form identifers. For example, the following are all identifiers: +, /, two+three, zero?, long_identifer, an-even-longer-identifer. Some special characters, such as parentheses and spaces, are not allowed in identifers. Digits may generallly be used in identifers, e.g. x3,but not as the first character. A few identifers, such as **define** and **if**, are reserved for use as **keywords** and should generally not be used as variables.

注: 变量由标识符来表示，标识符即变量名。在大多数语言中，可以用字符和数字来构成标识符，数字不能作为标识符的开头处。　Scheme中,甚至可以使用特殊字符。下列的合法标识符,+,/ two-three, zero?, long_identifier, an-even-longer-identifer. define, if, 因为是编程语言的关键字，所以不能用做变量名。

Scheme provides standard bindings for a number of variables. For example, + is bound to the addition procedure and zero? is bound to a boolean procedure, or **predicate**, that tests whether its argument is zero. Other standard bindings will be introduced as they are needed. We call procedures that are the values of standard bindings standard procedures. (See appendix I.)

注: Scheme提供了一些标准的绑定变量。如加法过程被绑定给了+，加号；布尔操作绑定了zero?, 测试参数是否为零。标准绑定过程见附录I。

If a value is the binding of some variable, it is often vonvenient to refer to the value by the name of the variable. Howerver, the distinction between the name of a variable and the value of its binding is very important. In this book we observe this distinction by using different fonts. When referring to the variable named "x" as a part of a program, we use the standard typewriter-style font: \sf{x}. When referring to the value of the variable x, we use an italic font: \it{x}. Thus we use "zero?" instead of "the value of the variable zero?" when referring to the numeric zero predicate

Statement-oriented languages usually distinguish between **functions**, which return values and are used in expressions, and **procedures**, which do not return values and are invoked by procedure call statements. Though function calls and procedure calls look the same, syntactically they are distinct: function calls are expressions, while procedure calls are statements. Howerver, since Scheme does not have statements,it does not make this distinction. In fact, Scheme functions are usually called procedures, and Scheme function calls are then referred to as procedure calls. We use the term "function" to refer only to abstract mathematical functions.

The symtax of procedure calls in Scheme is not typical of other programming languages. For example, a call to the procedure p with arguments 2 and 3 is written in Scheme as (p 2 3), instead of p(2 ,3). Parentheses surround the entire procedure call, and its components are separated by spaces. We say that the procedure p is applied to the arguments 2 and 3. Procedure (or funtion) calls are sometimes referred to as applications or combinations.

The general syntax of procedure calls is
(operator operand1 ... oprandn)
The ellipsis "..." indicates possible repetition. There may in general be any number of operands, or possibly none at all (n=0). The operator and each operand are components that are themselves expressions. They are called subexpressions. The operator subexpression is evaluated to obtain a procedure , while the operand subexpressions are evaluated to obtain the arguments of the call before invoking the procedure. (Arguments are also referred to as actual parameters, or simply parameters.) In Scheme, the order in which the operator and operand subexpressions are evaluated is not specified, but in some languages it is guaranteed to be left to right and in others it is always right to left.

Any expression may be used as an operand in a procedure call. For example, the procedure call
(+ x (p 2 3))
contains the operand (p 2 3) , which is itself a procedure call. If the value of (p 2 3) is 6 and x is 3 , then (+ x (p 2 3)) is 9. (More precisely, "the value of (+ x (p 2 3)) is 9." Since a compound expression's value is not likely to be confused with the expression itself, in such cases we shall often omit the phrase "the value of.")

Operators may also be arbitraily complex, as long as they return procedures. Thus if g were a procedure that when applied to 2 returned the addition procedure, then
((g 2) 3 4)
would return 7. Procedures that return procedures are called higher-order procedures, and expressions that return procedures are called higher-order expression. They may be unfamiliar, but much will be accomplished with them later.

1.1.2 Definitions, Programs, and the Read-Eval-Print Loop
Most operations can be expressed as procedure calls. For those that cannot, a small number of **special forms** are required.

Consider the operating of binding the variable x to 3. We would like to accomplish this by saying
(define x 3)
The general definition syntax we use is
(define variable expression)
where **variable** and **expression** indicate an arbitrary variable and expression. If this were a procedure call, with the variable **define** bound to some procedure, **variable** would be evaluated as an argument and its value passed to the procedure. But **variable** may be unbound, in which case it cannot be evaluated. Even if the variable were already bound, say x was bound to 7, it still would not do to evaluate the variable. The special form define must modify the binding of x, which would not be possible if it were simply passed the value 7.

The solution is to declare that the above syntactic form is special -- distinct from a procedure call. Each special form is indicated by an identifier,in this case define, that should not be used as a variable. These special form identifers are called **keywords**. Each special form has its own sequencing rule, that is order of evaluation of subexperssoins. In this case expression is evaluated first, and then variable is bound to the value of expression.

A Scheme program consists of a sequence of definitions and expressions that are executed in order by the Scheme system. These definitions and expressions are said to be at top level. We next discuss a few fetures of typical programming environments in which the Scheme language is used. It should be borne in mind that these are not features of the language itself.

注: 程序环境并不是程序语言的一部分，上下文环境并不是语言的一部分。

p5/539







