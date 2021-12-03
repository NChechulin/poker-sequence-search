# Poker Sequence Search
A task from Algorithms and Data Structures course in HSE

## Task statement

Available [here](assets/task_statement.pdf)

## Approach

The main idea was to modify Z-String algorithm in such a way that it could be used with custom object. It turns out that the simplest solution was to use bytes like `0x00_SS_RRRR`, where `SS` is suit (from 0 to 3) and `RRRR` is rank (from 1 to 13).
Each of 52 playing cards can be encoded that way.

So, basically, the whole modification was to replace `strings` with vectors of bytes.

## Running the code

1. Install rust (and `cargo`)
2. Clone the project: `git clone https://github.com/NChechulin/poker-sequence-search.git`
3. `cd poker-sequence-search/`
4. Run tests: `cargo test`
5. Compile and run code in debug mode: `cargo run`
6. (Optional) Build a release version: `cargo build --release && cd target/release/` and launch `poker-sequence-search`
