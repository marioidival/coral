# CORAL

## Knowledge

### Scanning

A scanner (or “lexer”) takes in the linear stream of characters and chunks them together into a series of something more akin to “words”.

In programming languages, each of these words is called a token. Some tokens are single characters, like `(` and `,`. Others may be several characters long, like numbers (`123`), string literals (`"hi!"`), and identifiers (`min`).

Some characters in a source file don’t actually mean anything. Whitespace is often insignificant and comments, by definition, are ignored by the language.

The next step is parsing.

### Parsing

This is where our syntax gets a grammar—the ability to compose larger expressions and statements out of smaller parts.

A parser takes the flat sequence of tokens and builds a tree structure that mirrors the nested nature of the grammar. These trees have a couple of different names — `parse tree` or `abstract syntax tree` — depending on how close to the bare syntactic structure of the source language they are. In practice, language hackers usually call them “syntax trees”, “ASTs”, or often just “trees”.

### Static Analysis

The first two stages are pretty similar across all implementations. Now, the individual characteristics of each language start coming into play. At this point, we know the syntactic structure of the code—things like operator precedence and expression nesting—but we don’t know much more than that.

In an expression like `a + b`, we know we are adding a and b, but we don’t know what those names refer to. Are they local variables? Global? Where are they defined?

The first bit of analysis that most languages do is called `binding` or `resolution`. For each `identifier` we find out where that name is defined and wire the two together. This is where `scope` comes into play—the region of source code where a certain name can be used to refer to a certain declaration.

If the language is statically typed, this is when we type check. Once we know where `a` and `b` are declared, we can also figure out their types. Then if those types don’t support being added to each other, we report a `type error`.

All this semantic insight that is visible to us from analysis needs to be stored somewhere. There are a few places we can squirrel it away:

* Often, it gets stored right back as `attributes` on the syntax tree itself—extra fields in the nodes that aren’t initialized during parsing but get filled in later.

* Other times, we may store data in a look-up table off to the side. Typically, the keys to this table are identifiers—names of variables and declarations. In that case, we call it a `symbol table` and the values it associates with each key tell us what that identifier refers to.

* The most powerful bookkeeping tool is to transform the tree into an entirely new data structure that more directly expresses the semantics of the code. That’s the next section.

Everything up to this point is considered the `front end` of the implementation. You might guess everything after this is the `back end`, but no. 
Back in the days of yore when `“front end” and “back end”` were coined, compilers were much simpler. Later researchers invented new phases to stuff between the two halves. Rather than discard the old terms, William Wulf and company lumped them into the charming but spatially paradoxical name `“middle end”`.

### Intermediate Representations

You can think of the compiler as a pipeline where each stage’s job is to organize the code in a way that makes the next stage simpler to implement. The front end of the pipeline is specific to the source language the user is programming in. The back end is concerned with the final architecture that the code will run on.

In the middle, the code may be stored in some intermediate representation (or “IR”) that isn’t tightly tied to either the source or destination forms (hence “intermediate”). Instead, the IR acts as an interface between these two languages.

There are a few well-established styles of IRs out there. Hit your search engine of choice and look for “control flow graph”, “static single-assignment”, “continuation-passing style”, and “three-address code”.
This lets you support multiple source languages and target platforms with less effort. Say you want to implement Pascal, C and Fortran compilers and you want to target x86, ARM, and, I dunno, SPARC. Normally, that means you’re signing up to write nine full compilers: Pascal→x86, C→ARM, and every other combination.

A shared intermediate representation reduces that dramatically. You write one front end for each source language that produces the IR. Then one back end for each target architecture. Now you can mix and match those to get every combination.

If you’ve ever wondered how GCC supports so many crazy languages and architectures, like Modula-3 on Motorola 68k, now you know. Language front ends target one of a handful of IRs, mainly GIMPLE and RTL. Target backends like the one for 68k then take those IRs and produce native code.
There’s another big reason we might want to transform the code into a form that makes the semantics more apparent

### Optimization

Once we understand what the user’s program means, we are free to swap it out with a different program that has the same semantics but implements them more efficiently—we can optimize it.

A simple example is constant folding: if some expression always evaluates to the exact same value, we can do the evaluation at compile time and replace the code for the expression with its result. 

### Code Generation

We have applied all of the optimizations we can think of to the user’s program. The last step is converting it to a form the machine can actually run. In other words generating code, where “code” refers to the kind of primitive assembly-like instructions a CPU runs and not the kind of “source code” a human might want to read.

### Virtual Machine

If your compiler produces bytecode, your work isn’t over once that’s done. Since there is no chip that speaks that bytecode, it’s your job to translate. Again, you have two options. You can write a little mini-compiler for each target architecture that converts the bytecode to native code for that machine. You still have to do work for each chip you support, but this last stage is pretty simple and you get to reuse the rest of the compiler pipeline across all of the machines you support. You’re basically using your bytecode as an intermediate representation.

Or you can write a virtual machine (VM), a program that emulates a hypothetical chip supporting your virtual architecture at runtime. Running bytecode in a VM is slower than translating it to native code ahead of time because every instruction must be simulated at runtime each time it executes. In return, you get simplicity and portability. Implement your VM in, say, C, and you can run your language on any platform that has a C compiler. This is what our second interpreter does.

### Runtime

We have finally hammered the user’s program into a form that we can execute. The last step is running it. If we compiled it to machine code, we simply tell the operating system to load the executable and off it goes. If we compiled it to bytecode, we need to start up the VM and load the program into that.

In both cases, for all but the basest of low-level languages, we usually need some services that our language provides while the program is running. For example, if the language automatically manages memory, we need a garbage collector going in order to reclaim unused bits. If our language supports “instance of” tests so you can see what kind of object you have, then we need some representation to keep track of the type of each object during execution.

All of this stuff is going at runtime, so it’s called, well, the “runtime”. In a fully compiled language, the code implementing the runtime gets inserted directly into the resulting executable. In, say, Go, each compiled application has its own copy of Go’s runtime directly embedded in it. If the language is run inside an interpreter or VM, then the runtime lives there. This is how most implementations of languages like Java, Python, and JavaScript work.

## Define data types

### Booleans

* True
* False

### Numbers

* 10
* 10.00

### Strings

* "This is a string"

### None (no value)

* None

## Expressions

If built-in data types and their literals are atoms, then `expressions` must be the molecules. Most of these will be familiar.

### Arithmetic

* add + me
* sub - me
* multiply * me
* divide / me

The subexpressions on either side of the operator are `operands`. Because there are two of them, these are called `binary` operators. (It has nothing to do with the ones-and-zeroes use of “binary”.) Because the operator is fixed in the middle of the operands, these are also called `infix` operators as opposed to `prefix` operators where the operator comes before and `postfix` where it follows the operand.

One arithmetic operator is actually both an infix and a prefix one. The `-` operator can also be used to negate a number:

* -negate

All of these operators work on numbers, and it’s an error to pass any other types to them. The exception is the `+` operator — you can also pass it two strings to concatenate them.

### Comparison and Equality

* less < than
* lessThan <= orEqual
* greater > than
* greaterThan >= orEqual
* 1 == 2 # True
* False != True # True
* "hello" != "hello" # False
* 123 == "123" # False

### Logical Operators

* not True
* not False

The other two logical operators really are control flow constructs in the guise of expressions. An and expression determines if two values are both true. It returns the left operand if it’s false, or the right operand otherwise:

* True and False
* True and True

And an or expression determines if either of two values (or both) are true. It returns the left operand if it is true and the right operand otherwise:

* False or False
* True or False

The reason `and` and `or` are like control flow structures is because they `short-circuit`. Not only does and return the left operand if it is false, it doesn’t even evaluate the right one in that case.

### Precedence and Gruping

All of these operators have the same precedence and associativity that you’d expect coming from C. (When we get to parsing, we’ll get way more precise about that.) In cases where the precedence isn’t what you want, you can use () to group stuff:

* average = (min + max) / 2

## Statements

Those are the expression forms (except for a couple related to specific features that we’ll get to later), so let’s move up a level. Now we’re at statements. Where an expression’s main job is to produce a value, a statement’s job is to produce an effect.
Since, by definition, statements don’t evaluate to a value, to be useful they have to otherwise change the world in some way — usually modifying some state, reading input, or producing output.

* print("hello world")

## Variables

* sum = 1 + 9
* nome = "Mario"

## Control Flow

It’s hard to write useful programs if you can’t skip some code, or execute some more than once. We need some control flow.

### ifs

```python
if condition:
    print("hello")
```

### while

```python
while True:
    print("loop")
```

### for C-Style

```python
a = 10
for i = 0; i < 10; i = i + 1:
    print(i)
```

#### for-in

```python
for i in iterator:
    print(i)
```

## Functions

Functions are first class

```python
def function(arg1, arg2):
    return arg1 + arg2


def function():
    return 1 + 2
```

### Closures

```python
def function():
    return 1 + 1

def other_function(f):
    return f

print(other_function(function)()) # prints 2
```

## Classes

```python
# Like Python Classes
class Class:
    def __init__(self):
        self.name = "auto"

c = Class() # maybe c = init Class?
```