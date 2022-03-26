# Caviar: An E-Graph Based Term Rewriting System for Automatic Code Optimization

The **caviar** paper was accepted at [CC 2022](https://conf.researchr.org/track/CC-2022/CC-2022-research-papers#event-overview)! Check it out: [link](https://dl.acm.org/doi/10.1145/3497776.3517781)

## Abstract
Term Rewriting Systems (TRSs) are used in compilers to simplify and prove expressions.
State-of-the-art TRSs in compilers use a greedy algorithm that applies a set of rewriting rules in a predefined order (where some of the rules are not axiomatic). This leads to a loss of the ability to simplify certain expressions.
E-graphs and equality saturation sidestep this issue by representing the different equivalent expressions in a compact manner from which the optimal expression can be extracted. While an e-graph-based TRS can be more powerful than a TRS that uses a greedy algorithm, it is slower because expressions may have a large or sometimes infinite number of equivalent expressions. Accelerating e-graph construction is crucial for making the use of e-graphs practical in compilers. In this paper, we present Caviar, an e-graph-based TRS for proving expressions within compilers. 
The main advantage of Caviar is its speed. It can prove expressions much faster than base e-graph TRSs.
It relies on three techniques: 1) a technique that stops e-graphs from growing when the goal is reached, called Iteration Level Check; 2) a mechanism that balances exploration and exploitation in the equality saturation algorithm, called Pulsing Caviar; 3) a technique to stop e-graph construction before reaching saturation when a non-provable pattern is detected, called Non-Provable Patterns Detection (NPPD). We evaluate caviar on Halide, an optimizing compiler that relies on a greedy-algorithm-based TRS to simplify and prove its expressions. The proposed techniques allow Caviar to accelerate e-graph expansion for the task of proving expressions. They also allow Caviar to prove expressions that Halide’s TRS cannot prove while being only 0.68x slower. Caviar is publicly available at: <https://github.com/caviar-trs/caviar>.

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
