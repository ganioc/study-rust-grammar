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

注: 变量由标识符来表示，标识符即变量名。在大多数语言中，可以用字符和数字来构成标识符，数字不能作为标识符的开头处。　Scheme中,甚至可以使用特殊字符。下列的合法标识符,+,/ two-three, zero?, long_identifier, an-even-longer-identifer. define, if, 因为是编程语言的关键字，所b不能用做变量名。






