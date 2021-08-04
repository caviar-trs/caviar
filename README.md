# Caviar: An E-Graph Based Term Rewriting System for Automatic Code Optimization
## Abstract
A lot of study and efforts are being dedicated to optimizing code and making it run faster which contributed to the development of automatic code optimization tools,
optimizing compilers are an example of such a tool. Optimizing compilers focus on modifying code so that it runs faster and uses the underlying hardware more efficiently.
To do so, optimizing compilers often need to simplify or prove algebraic expressions, whether they were arithmetic or boolean.
They generally use a term rewriting system that transforms syntactically the given expressions to either prove them or simplify them.
A term rewriting system is essentially composed of a set of rewrite rules along with an algorithm that determines how these rules will be applied.
In the last few years, equality graphs have inspired a tremendous of academic interest because they hold a lot of potential for addressing equivalence 
relations with the ability to apply rewrites without altering the original terms. The objective of this work is to build an equality-graphs-based term 
rewriting system for simplifying and proving compilers’ algebraic expressions. The TRS (that we named Caviar) is designed to prove and simplify expressions generated
by the Halide compiler and contains only axiomatic rules. The implementation of this TRS is done using the state-of-the-art e-graphs library: egg [Willsey et al., 2021].
To increase the efficiency and the speed of our TRS, we adapt the equality saturation algorithm for proving expressions by designing, implementing, and integrating several
contributions to this technique including: Iteration Level Check for Equivalence (ILC): a technique which increased performances by 14x, Pulsing Caviar:
a heuristic that improved performances by 15x while also raising the number of expressions we can prove, and Non Provable Patterns technique which added the ability of detecting 
non-provable expressions to our TRS. By evaluating our solution on expressions extracted from Halide programs compilation processes, we show that the proposed TRS is 
able to prove 51% of the expressions Halide’s TRS cannot prove and that the contributions we made to the equality saturation method have made it 20 times faster for this task.
## About this project
This work is the final year project of two computer science students: KOURTA Smail & NAMANI Adel Abderahmane. To get a better knowledge about the foundations of this work 
(Term rewriting systems, E-Graphs and Equality Saturation ...), how we desinged and implemented our solutions, the tests and evaluations ... you can take a look at our 
manuscript, it's on Research Gate: [link](https://www.researchgate.net/publication/353403145_An_E-Graph_Based_Term_Rewriting_System_for_Automatic_Code_Optimization)
## Setup
This assumes you have rust installed in your system. If you don't have it, go get it here: [link](https://www.rust-lang.org/tools/install)
```
# Clone this repository
git clone https://github.com/caviar-trs/egg_halides_trs.git
# Build the project
cd egg_halides_trs
cargo build
```
## Test the expressions' prover and simplifier
### For proving a csv file run:
```   
# Specify your own parameters
~ cargo run --release prove [Path to csv file containing the expressions in prefix format]  [Iterations Limit] [Egraph Size Limit] [Time Limit]
```
### For simplifying a csv run:
```   
# Specify your own parameters
~ cargo run --release simplify [Path to csv file containing the expressions in prefix format]  [Iterations Limit] [Egraph Size Limit] [Time Limit]
```
### For simplifying a single expression:

```   
# Using default parameters
~ cargo run --release
# Specify your own parameters
~ cargo run --release [Iterations Limit] [Egraph Size Limit] [Time Limit]
```
### Other Techniques and Executions
Please check the main.rs where you will find all the execution functions available for use.
