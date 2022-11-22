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
2. If $ l $ is a list-of-numbers and n is a number, then the pair $ (n \space . \space l) $ is a list-of-numbers.

From this definition we infer the following:

