# icfpc2019
Team Unagi's repository for ICFPC 2019

## Brief directions for judges to build/run the solution

### Requirements

* Rust 1.35.0

### Solving tasks

```bash
cd wata
cargo run --release task.desc "" > task.sol    # Buying nothing
cargo run --release task.desc "CC" > task.sol  # Buying CC
```

### Solving puzzles

```bash
cd puzzle
cargo run task.cond task.desc
```

### Deciding what to buy

```bash
cd knapsack
cargo run --bin main knapsack-in.txt 748091 > knapsack-out.txt
```

Each line of `knapsack-in.txt` should 
contain problem ID, buy string, solution ID, and time,
which are separated by commas.
For example:

```text
prob-002,,29259,381
prob-002,B,50721,356
prob-002,C,59200,224
prob-002,CB,51232,226
prob-003,,34881,210
prob-003,B,52255,210
prob-003,C,47656,128
prob-003,CB,52766,128
...
```

It outputs selected solutions to be submitted to stdout.


## Description of the solution approach

### Solving tasks

* A single-worker solution
 is constructed by the left-hand rule
(https://en.wikipedia.org/wiki/Maze_solving_algorithm#Wall_follower).
See `chokudai/src/lib.rs`. 
* An initial sequence is prepared, which collects boosters
and replicates workers.
It uses greedy algorithms and dynamic programming.
See `common/src/{bootstrap.rs, bootstrap_clone.rs}`.
* Given them, multi-worker solution is
created by splitting the single-worker solution
and concatenating to the initial sequence. See `wata/src/main.rs`.
* Local refinement is applied to the solution.
It tries to improve the solution
by removing some parts of the solution
and reconstructing actions to perform the same things with shorter steps.
It uses depth-first and breadth-first searches.
See `common/src/local_optimization.rs`.

### Solving puzzles

* Our puzzle solvers heavily relies
on narrow paths for satisfying point constraints.
* It then adjusts the area and number of vertices 
to satisfy the area and vertices constraings.
* See `puzzle/src/*.rs`.


### Deciding what to buy

* We first prepare solutions
for different buy settings for each task.
Then, we need to select what to buy for all tasks.
We formalized this problem as a variant of knapsack problems,
and solved it by using dynamic programming
(https://en.wikipedia.org/wiki/Knapsack_problem#Dynamic_programming_in-advance_algorithm).
See `knapsack/src/*.rs`.


## Feedback about the contest

* The conditions of mining puzzles seem too loose,
even at last rounds.
Top teams could solve all mining puzzles
and performance of puzzle solvers did not matter.
In additoin, there was little incentive to create 
interesting and/or hard tasks for mining puzzles.



## Self-nomination for judges’ prize

Our mining puzzle solver creates beautiful tasks.
The following is our task,
which is officially selected for block 83.

![Our puzzle task for block 83](83.png)
