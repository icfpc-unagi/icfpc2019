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
contain problem ID, buy string, solution ID, and time.
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


## Feedback about the contest


## Self-nomination for judges’ prize


