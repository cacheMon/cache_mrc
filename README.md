## Build Cache MRC

Cache Miss Ratio Curves (MRC) is a tool to generate MRCs for a given cache configuration. The tool is implemented in Rust and can be built using Cargo.

To enhance simulation performance, we implemented the [SHARDS](https://www.usenix.org/conference/atc17/technical-sessions/presentation/waldspurger) idea, which applies sampling techniques to both the data stream and cache size.

Make sure you have installed Rust and Cargo. If not, please follow the instructions [here](https://www.rust-lang.org/tools/install).

Install `gnuplot`, which is used to generate the MRC graph.
```bash
# Ubuntu
sudo apt install gnuplot
# MacOS
brew install gnuplot
```
## Running the MRC Generator
You can run the MRC Generator using either a configuration file or command-line arguments.
### Using a Configuration File
```bash
cargo run -r -- --config-file config.toml
```
### Using Command Line Arguments
```bash
cargo run -r -- --trace "./data/twitter_cluster52.csv" --output "./twitter_52_mrc.png" --policies lru,fifo --sample-rate 0.01 --cache-size=10mb --timestamp 0 --key 1 --size 2
```
## Options
* --trace: Path to the input trace file
* --output: Path for the output MRC graph
* --policies: Comma-separated list of cache policies to evaluate (Currently only LRU and FIFO are supported)
* --sample-rate: Sampling rate
* --cache-size: Maximum cache size to simulate

Followings are the column indices in the trace file.
* -1: means use the default value, the column may be missing in the trace file
* other: Column index for the column to use

## Roadmap
- [x] Implement the basic MRC generation tool
- [x] Incorporate sampling method based on SHARDS
- [ ] Add compatibility with more trace file formats (e.g., bin, vscsi)
- [ ] Add support for large trace files (e.g., 100GB+), chunk the trace file and process it in parallel
- [ ] Expand support for additional cache policies (e.g., LFU, TwoQueue)
- [ ] Optimize the performance of simulating large cache sizes