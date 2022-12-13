
chap 2
========

# 2 Induction , Recursion, and scope
This chapter introduces the fundamental technique of recursive programming, along with its relation to the mathematical technique of induction. The notion of scope, which plays a primary role in programming language, is also presented. Finally, the material in this chapter will improve your facility with the tools introduced in chapter 1. Section 2.1 and section 2.2 introduce techniques for inductively specifying data structures and show how such specifications may be used to guide the construction of recursive programs. Section 2.3 then introduces the notions of variable binding and scope.

The programming exercises are the heart of this chapter. They provide experience that is essential for mastering the technique of recursive programming upon which the rest of this book is based.

[按]
- induction : 归纳法
- recursion : 递归
- scope : 作用域

本章介绍了递归编程方法，虽然这种方法因为早期计算机硬件容量的限制，而不被提倡。

## 2.1 Inductively Specified Data
 We have already seen a number of data types in Scheme. What is a data type in general? For our purposes, it is enough to say that a data type consists of a set of values along with a set of operations on those values. Every time we decide to represent a certain set of quantities in a particular way, we are defining a new data type: the data type whose values are those representations and whose operations are the procedures that manipulate those entities.

 [按]
 - 数据类型，数据，对数据的操作

 When writing code for the operations on a new data type, it is important to know precisely what values may occur as members of the type. In this section we introduce formal techniques for specifying the set of  values that belong to a data type. 

 [按]
 - 定义属于某种数据类型的一组数值

 ### 2.1.1 Inductive Specification
 **Inductive specification** is a powerful method of specifying a set of values. To illustrate this method, we use it to describe a certain subset of the natural numbers:

 归纳的规则来定义数据类型，产生新的数据类型

 Define the set S to be the smallest set of natural numbers satisfying the following two properties:

 1. $ 0 \in  S$ and
 2. Whenever $ x \in S $, then  $ x+3 \in S $

 [按]
 使用逻辑符号, 数学的方法来对数据类型的定义进行严格的证明。

 "The smallest set" is the one that is a subset of all the sets satisfying properties 1 and 2.

 Let us see if we can describe some partial information about S to arrive at a noninductive specification. We know that 0 is in S, by property 1. Since $ 0 \in S $, by property 2 it must be that $ 3 \in S $. Then since $ 3 \in S $, by property 2 we conclude that $ 6 \in S $, and so on. So we see that ll the multiples of 3 are in S. If we let M denote the set of all multiples of 3, we can restate this conclusion as $ M \subseteq S $. But the set M itself satisfies properties 1 and 2. Since S is a subset of every set that satisfies properties 1 and 2, it must be that $ S \subseteq M $. So we deduce that $ S = M $, the set of multiples of 3. This is plausible: we know all the multiples of 3 must be in S, and anything else is extraneous.

 [按]
 extraneous, 无关，没有直接联系

This is a typical inductive definition. To specify a set $ S $ by induction, we define it to be the smallest set satisfying two properties of the following form:

1. Some specific values must be in $ S $.
2. If certain values are in $ S $, then certain other values are also in $ S $.

Sticking to this recipe guarantees that $ S $ consists precisely of those values inserted by property 1 and those values included by repeated application of property 2. As stated, this recipe is rather vague. It can be stated more precisely, but that would take us too far afield. Instead, let us see how this proces works on some more examples.

The data type list-of-numbers is the smallest set of values satisfying the two properties:

1. The empty list is a list-of-numbers, and
2. If $ l $ is a list-of-numbers and $ n $ is a number, then the pair $ (n \space . \space l) $ is a list-of-numbers.

From this definition we infer the following:

1. () is a list-of-numbers, because of property 1.
2. $ (14 \space . \space ()\space ) $ is a list-of-numbers, because 14 is a number and () is a list-of-numbers.
3. $ (3\space.\space(14\space. \space ())) $ is a list-of-numbers, because 3 is a number and $ (14\space.\space())$ is a list-of-numbers.
4. $(-7\space.\space(3\space.\space(14\space.\space())))  $ is a list-of-numbers, because $-7$ is a number and $(3\space.\space(14\space.\space()))$ is a list-of-numbers.
5. Nothing is a list-of-numbers unless it is built in this fashion.

Converting from dot notation to list notation, we see that $()$, $(14)$, $(3 \space 14)$,and $(-7\space3\space14)$ are all members of list-of-numbers.

### Backus-Naur Form and Syntactic Derivations
The previous example was fairly straightforward, but it is easy to imagine how the process of describing more complex data types might become quite cumbersome. To remedy this, a notation has been developed to express the same ideas more concisely: **Backus-Naur Form** or **BNF**. We frequently use it to describe the structure of a data type. BNF is also widely used in specifying the syntax of programming languages.

BNF can be used to inductively define a number of sets at once. These sets are called **syntactic categories**, or sometimes **nonterminals**, and are customarily written with angle brackets around the name of the set: <list-of-numbers>. (When no ambiguity results, we shall sometimes refer informally to syntactic categories without using angle brackets or dashes: "list of numbers.") Each syntactic category is defined by a finite set of **rules**, or **productions**. Each rule asserts that certain values must be in the syntactic category.

The BNF definition of list-of-numbers has two rules that correspond to the two properties.

$$
\begin{aligned}
\langle list\text{--}of\text{--}numbers \rangle &::=()  \\

    \langle list\text{--}of\text{--}numbers \rangle &::=(\langle number\rangle \space . \space \langle list\text{--}of\text{--}numbers \rangle )  \\
\end{aligned}

$$

The first rule says that the empty list is in $ \langle list\text{--}of\text{--}numbers \rangle $ , and the second says that if $n$ is in $ \langle number \rangle $ and $l$ is in $ \langle list\text{--}of\text{--}numbers \rangle$, then $(n\space . \space l)$ is in $ \langle list\text{--}of\text{--}numbers \rangle$.

Each rule begins by naming the syntactic category being defined, followed by ::= (read $is$ ). The right-hand side of each rule specifies a method for constructing members of the syntactic category in terms of other syntactic categories and $ terminal\space symbols$, such as the left and right parentheses, and the period in the previous example.

Often some syntactic categories mentioned in a BNF rule are left undefined, such as $\langle number \rangle$. Defining all categories would needlessly complicate the rule if it is safe to assume the reader knows what some of the categories are.

BNF is often extended with a few notational shortcuts. One can write a set of rules for a single syntactic category by writing the left-hand side and ::= just once, followed by all the right-hand sides separated by the special symbol $|$ (vertical bar, read $or$). A $\langle list\text{--}of\text{--}numbers \rangle$ can then be defined by

$$
\begin{aligned}
\langle list\text{--}of\text{--}numbers \rangle &::= () | (\langle number\rangle \space . \space \langle list\text{--}of\text{--}numbers \rangle )

\end{aligned}

$$

$\quad$ Another shortcut is the $Kleene \space star$, expressed by the notation $\{...\}^*$. When this appears in a right-hand side, it indicates a sequence of any number of instances of whatever appears between the braces. This includes the possibility of no instances at all. Using the Kleene star, the definition of $\langle list\text{--}of\text{--}numbers \rangle$ in list notation is simply

$$
\begin{aligned}
\langle list\text{--}of\text{--}numbers \rangle &::=  (\{\langle number\rangle\}^* )

\end{aligned}

$$

$\quad$ If there are zero instances, we get the empty list. A variant of the star notation is $Kleene plus \{... \}^+$, which indicates a sequence of $one$ or more instances. Substituting $^+$ for $^*$ in the above example would define the syntactic category of nonempty lists of numbers. These notational shortcuts are just that $\text{--}$ it is always possible to do without them by using additional BNF rules.

$\quad$ If a type is specified use BNF rules , a $syntactic \space derivation$ may be used to prove that a given data value is a member of the type. Such a derivation starts with the nonterminal corresponding to the type. At each step, indicated by an arrow $\Rightarrow$, a nonterminal is replace by the right-hand side of a corresponding rule, or with a known member of its syntactic class if the class was left undefined. For example, the previous demonstration that $(14\space.\space())$ is a list of numbers may be formalized with the following syntactic derivation:

$$
\begin{aligned}
&\langle list\text{--}of\text{--}numbers\rangle \\
\Rightarrow &(\langle number \rangle \space . \space \langle list\text{--}of\text{--}numbers\rangle ) \\
\Rightarrow &(14 \space . \space \langle list\text{--}of\text{--}numbers\rangle) \\
\Rightarrow &(14 \space . \space ()) \\
\end{aligned}
$$

$\quad$ The order in which nonterminals are replaced is not significant. Thus another possible derivation of $(14 \space . \space ())$ is 

$$
\begin{aligned}
&\langle list\text{--}of\text{--}numbers\rangle \\
\Rightarrow &(\langle number \rangle \space . \space \langle list\text{--}of\text{--}numbers\rangle ) \\
\Rightarrow &(\langle number \rangle \space . ()) \\
\Rightarrow &(14 \space . \space ()) \\
\end{aligned}
$$

### $\circ$ Exercise 2.1.1
Write a syntactic derivation that proves $(-7 \space . \space (3 \space . \space (14 \space . \space ())))$ is a list of numbers. $\Box$

### $\bold{2.1.3 \space using \space  BNF\space to  \space Specify \space Data}$

The term $datum$ refers to any literal data representation. BNF may be used to specify concisely the syntactic category of data in Scheme. We have seen that numbers, symbols, booleans, and strings all have literal representations, which we associate with the syntactic categories $\langle number \rangle$, $\langle symbol \rangle$, $\langle boolean \rangle$, and $\langle string \rangle$, respectively. Section 1.2 informally introduced representations for lists, improper lists (which end with dotted pairs), and vectors. These compound data types contain components that may be numbers, symbols, booleans , strings , or other lists, improper lists or vectors. This is formally specified by the following BNF rules

---

$$
\begin{aligned}
\langle list \rangle &::= (\{\langle datum \rangle\}^*) \\
\langle dotted\text{--}datum \rangle &::= (\{ \langle datum \rangle\}^+ \space . \space \langle datum \rangle ) \\
\langle vector \rangle &::=  \# (\{ \langle datum \rangle \}^*) \\
\langle datum \rangle &::= \langle number \rangle | \langle symbol \rangle | \langle boolean \rangle  \\
&| \langle string \rangle | \langle list \rangle | \langle dotted\text{--}datum \rangle | \langle vector \rangle

\end{aligned}
$$

---

These four syntactic categories are all defined in terms of each other. This is legitimate because there are some simple possibilities for data that are not defined in terms of the other categories.

$\quad$ To illustrate the use of this grammar, consider the following syntactic derivation proving that $(\#t \space (foo \space . \space ()) \space 3)$ is a datum.

$$
\begin{aligned}
&\langle list \rangle \\
\Rightarrow &(\langle datum \rangle \space \langle datum \rangle \space \langle datum \rangle) \\
\Rightarrow &(\langle boolean \rangle \space \langle datum \rangle \space \langle datum \rangle) \\
\Rightarrow &(\#t \space \langle datum \rangle \space \langle datum \rangle) \\
\Rightarrow &(\#t \space \langle dotted\text{--}datum \rangle \space \langle datum \rangle) \\
\Rightarrow &(\#t \space ( \{ \langle datum \rangle  \}^+ \space . \space \langle datum \rangle ) \space \langle datum \rangle) \\
\Rightarrow &(\#t \space ( \langle symbol \rangle \space . \space \langle datum \rangle ) \space \langle datum \rangle) \\
\Rightarrow &(\#t \space ( foo \space . \space \langle datum \rangle ) \space \langle datum \rangle) \\
\Rightarrow &(\#t \space ( foo \space . \space \langle list \rangle ) \space \langle datum \rangle) \\
\Rightarrow &(\#t \space ( foo \space . \space ())  \space \langle datum \rangle) \\
\Rightarrow &(\#t \space ( foo \space . \space ())  \space \langle number \rangle) \\
\Rightarrow &(\#t \space ( foo \space . \space ())  \space \langle 3 \rangle) \\
\end{aligned}
$$

$\quad$ All three elements of the outer list were introduced at once. This shortcut was possible because the grammar uses a Kleene star. Of course, the Kleene star and plus notation could be eliminated by introducing new nonterminals and productions, and the three list elements would then be introduced with three derivation steps instead of one.

### $\circ$ Exercise 2.1.2
Rewrite the $\langle datum \rangle$ grammar without using the **Kleene star** or **plus**. Then indicate the changes to the above derivation that are required by your grammar. $\square$

### $\circ$ Exercise 2.1.3
Write a syntactic derivation that proves $(a \space \text{\textquotedblleft}mixed\text{\textquotedblright} \#(bag \space (of \space . \space data)))$ is a datum , using either the grammar in the book or your grammar from the last exercise. What can you say about $(a\space . b \space . \space c)$ ? $\square$

[答]

$$
\begin{aligned}
&(a \space \text{\textquotedblleft}mixed\text{\textquotedblright} \space \#(bag \space (of \space . \space data))) \\
\Rightarrow &(\langle datum \rangle \space \langle datum \rangle \space \langle datum \rangle) \\
\Rightarrow &(\langle symbol \rangle \space \langle string \rangle \space \langle vector \rangle) \\
\Rightarrow &(\langle list \rangle) \\
\end{aligned}
$$


$\quad$ Let us consider the BNF definitions of some other useful data types. Many symbol manipulation procedures are designed to operate on lists that contain only symbols and other similarly restricted lists. We formalize this notion with their rules:

