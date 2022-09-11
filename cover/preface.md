# Preface

---

The goal of this book is to give students a deep, hands-on understanding of the essential oncepts of programming languages, using Scheme as an executable metalanguage. Because Scheme is a wide-spectrum language, it enables us to write both at the very hight level needed to produce a concise, comprehensible interpreter and at the much lower level needed to understand how that interpreter might be coded in assembly language, or tansformed into a compiler. Because of Scheme's excellent abstraction facilities, we can write substantial language-processing systems that are nevertheless compact enough for students to understand and manipulate with a reasonable amount of effort. This is a hands-on book: everything disussed in the book may be programmed by students.

Many texts are descriptive in nature and may be of use to the casual reader. Not this one. Our approach is analytic. Though we make little use of mathematical notation, our Scheme programs often play the same role. As with analytic texts in any discipline, this book requires careful reading with attention to detail. Before the material is mastered, it will frequently require rereading and reflection. Deep concepts are only absorbed with active participation. Their power must be experienced, not passively viewed.

Though we believe Scheme is an excellent metalanguage, this book is about the eseentials of programming languages in general, not Scheme in particular. To emphasize this, we have given the interpreted language developed in this book a syntax very different from that of Scheme. This interpreted language is designed only for our pedagogic purpose and is not intended for other use. It is composed of a number of pieces, introduced throughout the text, which are not necessarily designed to form a coherent whole. Indeed, as we build new interpreters we often use the same concrete syntax with different semantics.

Beyond the use of Scheme, we use four major strategies:

1. The first strategy is the use of interpreters to explain the run-time behavior of programs in a given language. Interpreters experss language design decisions in a manner that is both formal (unambigous and compete) and executable. Furthermore, our interpretters are generally expressed in a fashion consistent with the principles of denotational semantics; they express key ideas of denotational semantics in a concrete way.

2. Instead of relying on mere descriptions, using English, diagrams, or some abstract notation, we present each principle using Scheme programs that implement it. The use of Scheme enables the student to understand these programs without drowning in a sea of irrelevant detail. The exercises allow the student to experiment with alternatives in design and implementation.

3. We emphasize the systematic derivation of low-level implementations from their high-level abstractions. We show how simple algebraic manipulation can be used to derive program properties and correctness-preserving program trnasformations. In particular, we use these techniques to derive stack-management protocols and to transform an interpreter into a compiler.

4. Finally, we use data abstraction, expressed as a modular coding style, to separate algorithms from the representation of the underlying quantities. In this way, we can change data representation without changing programs. In the case of interpreters, we use this technique to investigate different implementation strategies.

Through the use of these strategies, we are able to give students several working models, ranging from very high-level (almost formal semantics) to very low-level (almost assembly language), and to demonstrate a clear connection between these models.

Such depth must come at the expense of breadth. We make no attempt to survery existing languages, though we occasionally point out the design choices used in common languages. Although our approach is largely motivated by the developments in programming language semantics over the last 20 years, we do not address a number of important research areas, such as type checking and inference, logic programming, parallelism , and verification. We believe, howerver, that a command of the essentials will allow the student to study these topics. For example, an understanding of th emechanics of logic programming certainly requires understanding of continuations, dynamic binding, and the distinction between a variable's name, its binding, and the value of its binding. 

## Organization
The char below represents the organizational structure of the book. The squares on the organizational chart represent chapters where syntax is introduced.

The first four chapters provide the foundations for a careful study of programming languages. Chapter 1 introduces most of the features of Scheme that are required in later chapters, along with some basic terminology. Readers who have prior experience with Scheme or other Lisp dialects may wish to skim this chapter and refer to it as necessary. Chapter 2 emphasizes the connection between inductive data specification and recursive programming, and introduces several notions related to the scope of variables. A set of exercises is provided to develop facility with recursive programming. Chapter 3 introduces some commonly used syntactic abstractions, including a variant record facility. This leads to a discussion of data abstraction and examples of representational transformations of the sort used in subsequent chapters. Chapter 4 introduces, in the context of the lambda calcus, several rewrite rules that are basic program transformations and privides a brief review of imperative programming.






