## Final Report
## Team members
Hanyang (Eric) Xu 1006153092 hyeric.xu@mail.utoronto.ca 
Steven Hill 101811301 sd.hill@mail.utoronto.ca 
James Yen 1005788355 james.yen@mail.utoronto.ca

## Motivation
As Rust quickly rises in popularity, a concerted effort has been made to explore and improve its parallel performance.  Through the detection of concurrency errors at compile time, Rust promises to enable fearless parallelism with zero costs[4]. This promise has attracted groups to research parallelism in Rust and try to increase its performance. Currently, there is an effort to integrate Cilk keywords into Rust via TAPIR[1], which would help close the performance gap between parallel Rust and Parallel C. Cilk adds in keywords to Rust to represent parallelism but also allow the compiler to perform serial optimizations. This is accomplished by extending Rust’s IR to represent the parallel flow[2]. However, due to the relatively young age of the language, many of the tools required for this work have not yet been developed. Tools such as benchmarks can be useful to compare how different changes to the Rust compiler can impact the rust’s performance. The development of these benchmarks can also provide insight into the shortcomings and issues regarding fearless parallelism.

The Problem-Based Benchmark Suite (PBBS)[3] is unique as it defines problems in terms of the desired relationship between inputs and outputs. The user then develops the algorithms required to produce the outputs from the inputs. This is desirable as it gives a framework to compare different algorithms, implementations and programming languages. A substantial effort has already been made by Professor Mark C. Jeffrey’s research group to port some of PBBS’s benchmarks into Rust, known as the Rust Parallel Benchmarks suite project (RPB) [5]. These ported benchmarks have already been successfully used to explore the extent of Rust’s fearless and zero-cost parallelism. There is a need for more benchmarks to continue this work and help with the ongoing effort to integrate tapir into the Rust compiler. In this project we will continue this work by porting more of the benchmarks into Rust.  

## Objectives 
Based on prior research and the limitations discussed in [3], **our project's objective is to extend the exploration of Rust's concurrency capabilities in relation to safe parallelism by porting three additional benchmarks from the Problem-Based Benchmark Suite (PBBS) to parallel Rust**. Specifically, the study by Abdi et al. observes that, while Rust is designed to provide "fearless parallelism," it encounters significant obstacles, such as limited compiler support and structural restrictions, which hinder its performance compared to advanced C++ constructs [1], particularly in irregular parallelism scenarios​ [3]. Our project builds on these findings by examining these constraints through the porting of three additional PBBS benchmarks to Rust. This process will allow us to evaluate the benchmarks' performance under Rust’s parallelization model and pinpoint key performance gaps relative to C++. By empirically assessing Rust's parallelism capabilities in a high-performance context, and comparing said performance to the original C++ PBBS implementation, our project aims to provide evidence supporting the paper's claims and to contribute deeper insights into the practical limitations of Rust’s current concurrency model, especially regarding its promise of "fearless" and safe parallelism.

## Features
### Benchmark Selection
We identified three benchmarks from the PBBS suite that are challenging yet achievable for this month-long project [^1]. The selected benchmarks are:  
- **BFS**: A graph traversal benchmark that measures the time required to traverse large-scale graphs using a breadth-first, parallel approach.
- **KNN**: A geometry-based problem where the k-nearest neighbouring points are identified in 2D space. Two implementations were completed for KNN:
  - *Naive*: The standard O(n^2) approach, where each point is compared to all other points. Parallelism is introduced by spawning threads to process multiple points concurrently.
  - *CKtree*: A tree-based algorithm that uses bounding boxes to search for neighbouring points. The algorithm prunes paths by estimating lower bounds for neighbor search space, reducing the number of traversals.
- **WC**: A text-processing problem that breaks strings into words and reports an aggregate count for each word.

[^1]: For each benchmark, PBBS uses a variety of parallel implementations to compare performance. We select 1-2 approaches for each benchmark.

### Leveraging Safe Rust and Obtaining Comparable Results
The benchmarks ported from the PBBS are implemented in **safe** Rust to enable fearless parallelism while preserving the semantics of the original algorithms. We avoid using Rust's `unsafe` blocks at all costs even if they could result in better performance. This approach ensures that Rust’s safety guarantees are maintained, allowing us to examine Rust’s native concurrency capabilities in a controlled and comparable context. Significant effort was made to ensure that the ported benchmarks remain faithful to the original algorithms and behave identically to their counterparts in PBBS. However, performance trade-offs resulting from Rust's design principles are discussed in the [**Lessons Learned Section**](#lessons-learned).

### Fearless Parallelism using Rayon Library
Adhering to the previous efforts by the RPB team, we adapted PBBS' parallel constructs using the Rayon library in our Rust implementation too. Rayon offers higher-level abstractions and “better safety guarantees” as highlighted in the RPB paper [5]. These parallel constructs not only make the benchmarks functional but also generalize well, providing reusable building blocks for parallelism of other applications in Rust. This helps expand Rust’s parallel programming utility beyond PBBS, enabling safe and efficient parallelization across other domains.

### Performance and Verification
Performance characterization of the ported benchmarks include comprehensive runtime analysis, comparing the Rust implementations against C++ in parallel contexts. Major effort was put in to create an easy-to-use testing interface that matches with RPB. Our project is forked from RPB and testing is entirely identical to the rest of the RPB repository and can be compiled with ease. Each benchmark includes an optional flag for correctness check and can be tested with static testing data stored in the repo. These data were generated using RPB’s data generation or using example data from the RPB repo.

### Performance Results and Analysis (rpb is root of project)
| Benchmark + Algorithm Used | Input File Path  | Rust   	 | PBBS |
|-----------------------------|------------------------ |------------|-----------|
| BFS                    	| rpb/input/small_graph | 0.788497s  | 0.0059s   |
| KNN, CKtree              	| rpb/input/50kpoints     | 1.871796s  | 0.0984s   |
| KNN, naive             	| rpb/input/50kpoints     | 0.256826s  | 0.2558s   |
| WC                     	| rpb/input/pluto.txt       | 0.830522s  | 0.2859s   |

Performance data was run on Intel(R) Xeon(R) w5-2465X, 64GB RAM, 16 cores, 32 threads, with default 32 threads and hyperthreading enabled. Overall, the benchmarks implemented in Rust are slower than the PBBS (C++) implementations. While Rust's fearless parallelism ensures safety by enforcing strict rules at compile time, this likely introduced additional overhead compared to C++’s more permissive concurrency model.

For `KNN-naive` and `WC`, the runtime increase can be indirectly attributed to Rust's design philosophy prioritizing safety. We identify these potential overheads as:
- **Bounds checking:** Rust uses array bounds checking by default for memory safety, compared to unchecked operations in C++.
- **Borrow checking:** Rust’s borrow-checking enforces strict ownership and access rules at compile time, which may lead to less efficient patterns for memory/thread access compared to C++’s raw pointers.  

These overheads are minimal but the performance tradeoffs are likely due to the structure of the code written to abide by these rules. Our "safe" implementations may have led to less optimal patterns for memory/thread access.

The performance gap widens further in more complex benchmarks that rely heavily on shared mutable data structure. `BFS` and `KNN-CKtree` use synchronization primitives like `Arc` and `Mutex`, where `BFS` has a shared frontier structure that frequently updates across multiple threads. For `KNN-CKtree`, the impact is even more pronounced as the tree data structure needs `Mutex` on every parent to link nodes, as well as on each node's neighbour array that is repeatedly modified by different threads. This design leads to high contention, as multiple threads compete for access to the same locks.

Likewise, we acknowledge that further optimizations can likely be made to our Rust implementation, such as using traits to utilize binary heap structures or optimizing data iteration to improve cache locality. 

## Installation
from https://github.com/mcj-group/rusty-pbbs and https://cmuparlay.github.io/pbbsbench/
### Install Rust (cargo, rustc, ...)

```bash
# download and install the version management tool: Rustup
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# configure the current shell
source "$HOME/.cargo/env"
```

The `default` features are enough for benchmarking.
If you want to do more hacking, the `complete` profile could help.

### Download RPB and PBBS

```bash
git clone https://github.com/ericxu233/rpb.git
git clone https://github.com/cmuparlay/pbbsbench.git
git submodule update --init
```

#### install gcc if you don't have it on your system.

### Compile benchmarks

```bash
cargo build --release                # compile all benchmarks
cargo build --release --bin="wc"  # compile an specific benchmark (wordcount)
```
### Run
Cargo can run an individual benchmark (e.g. wc):
```bash
cargo run --release --bin=wc-- <input_file>
```
or the binary can be run itself:
```bash
/path/to/build/directory/pbbs/release/wc <input_file>
```
To get the full list of flags and arguments use `--help`:
```bash
/path/to/build/directory/pbbs/release/wc --help
```
### Inputs

All the benchmarks expect their data to be in the same format that PBBS uses.
Please check [PBBS's website](https://cmuparlay.github.io/pbbsbench/) for instruction on how to generate input data.
We’ve also included some small sample inputs in this repo (rpb) for the benchmarks just for the sake of functionality verification.
./input/50kpoints is for the knn benchmark  
./input/pluto.txt is for the wc benchmark  
./input/small_graph is for the bfs benchmark


## Reproducibility Guide:
Need to run installation steps before
### Generating input data (can ignore if using the included data above):
Word count data is included in the pbbs bench and needs to be decompressed
```bash
bzip2 -dk /.../pbbsbench/testData/data/<data>.bz2
```
KNN Data is generated using the following command 
```bash
cd /…/pbbsbench/testData/geometryData
make
./randPoints -d 2 50000 <filename>
```
BFS data was generated in a similar way:
```bash
$ cd /…/pbbsbench/testData/graphData
$ make
$ ./randLocalGraph -j -d 3 -m 20000000 2000000 <filename>
```

### Running the benchmarks
#### Word count:
```bash
/.../pbbs/release/wc -o <outfile> <inputfile>
# a serial version can be run using the following
/.../pbbs/release/wc -a sequential -o <outfile> <inputfile>
```
#### Run C bench for comparison
```bash
/.../pbbsbench/runall -only wordCounts/histogram
```
Include -nonuma if your system does not utilize NUMA
#### KNN:
```bash
/.../target/release/knn -a naive <inputfile>
/.../target/release/knn -a cktree <inputfile>
```
#### Run C bench for comparison
```bash
cd /…/benchmarks/nearestNeighbors/CKNN/
make
./neighbors -d 2 <inputfile>
```

#### BFS:
```bash
/.../target/release/bfs <inputfile>
```
#### Run C bench for comparison
```bash
/.../pbbsbench/runall -only 
```
## Video Demo
Please refer to the video .mp4 file included in the repo. It is named ece1724-final-project-recording_compressed.mp4.

## Contributions by each team member
### James
- Code contribution tasks include:
  - Callahan-Kosaraju Tree (CKtree) data structure. Implemented functions necessary to partition points into hierarchical bounding boxes that make spatial queries (KNN).
  - Algorithm to keep the tree balanced by ensuring that the number of points in each subtree is approximately equal. This reduces the tree depth and speeds up KNN search.
  - A recursive tree building function that divides points into smaller subsets at each level, and a search algorithm that prunes if a bounding box is too far from the queried point.
  - Conducted testing for k >= 1 number of nearest neighbours and updated verification checker to correctly test KNN implementation.
  - Optimized parallel search through fine-tuning to improve runtime.
- Report contribution
  - Wrote *Features* and *Lessons Learned and Concluding Remarks* . Updated *Objectives* to reflect project design changes since the project proposal.

### Eric
- Code contribution tasks include:
  - Implemented general setup and common benchmark user interface for KNN and BFS. Conform ed 
  - Setup results checking for KNN to verify that our implementation is correct and functional.
  - Implemented the naive parallel version of KNN with a complexity of O(n^2), utilized the rust package Rayon for auto-managed parallelization.
  - Implemented non-deterministic parallel bfs in Rust with parallel wavefront/frontier computation.
  - Performed empirical testing of all benchmarks compared to their C implementation in PBBS with various input sizes and configurations.
- Report contribution
  - Recorded the demo video. Updated minor parts of the *Features*, *User Guide* and *Reproducibility* section.
  
### Steven
- Code contribution:
  - Wrote and verified the correctness of the serial and parallel WC benchmark.
  - Wrote and modified the necessary code in RPB to include the WC benchmark.
- Report contribution:
  - Wrote the *Motivation* section.
  - Wrote the majority of the *Installation* and *Reproducibility* Guide.


## Lessons Learned and Concluding Remarks

Porting the BFS and KNN algorithms to Rust highlighted significant challenges with performance due to the use of synchronization primitives like `Arc` and `Mutex`. BFS’ shared frontier required frequent updates from multiple threads. There is high contention as threads often have to wait for locks to be released.

KNN-CKtree creates a tree data structure that is fast to traverse and can be parallelized for each point’s neighbour search. However, the majority of runtime is spent constructing the data structure. KNN-CKtree has a vector for each node that tracks neighbours of its point. This vector is heavily accessed and modified as we build out the tree. Bounding boxes are used to calculate distance bounds and identify spatially “well separated” pairs of points. Once this is identified, nodes acquire each other’s locks to add their neighbours to each other. Since threads frequently accessed and modified overlapping regions of the tree, the cost of synchronization slowed down the overall execution of the algorithm.

Building on top of the existing RPB project was challenging due to a lack of documentation and context. Differences in implementation between RPB and PBBS created confusion regarding whether certain functions were incomplete, unnecessary, or simply implemented differently. This was evident in translating basic data structures for the project.

Overall, this project extends the exploration of Rust's concurrency capabilities by porting additional benchmarks from PBBS to parallel Rust, providing an in-depth evaluation of Rust's performance in parallel scenarios. Each benchmark is implemented using Rayon to achieve data and functional parallelism, while adhering to the original algorithm implementations. Our analysis provides additional insight into the practical limitations of Rust’s parallel programming capabilities. We reinforce the notion that Rust’s unmatched safety requires additional advancements in code and infrastructure optimization to bridge performance gap with mature frameworks like C++.

## References

[1] Schardl, Tao & Moses, William & Leiserson, Charles. (2017). Tapir: Embedding Fork-Join Parallelism into LLVM's Intermediate Representation. 249-265. 10.1145/3018743.3018758.
[2] T. B. Schardl, “Enabling the Rust Compiler to Reason about Fork/Join Parallelism via Tapir,” May 01, 2024. https://dspace.mit.edu/handle/1721.1/156790  
[3] D. Anderson, G. E. Blelloch, L. Dhulipala, M. Dobson, and Y. Sun, “The problem-based benchmark suite (PBBS), V2,” Association for Computing Machinery, Mar. 2022, doi: 10.1145/3503221.3508422.  
[4] J. Abdi, G. Posluns, G. Zhang, B. Wang, and M. C. Jeffrey, “When Is Parallelism Fearless and Zero-Cost with Rust?,” Proceedings of the 36th ACM Symposium on Parallelism in Algorithms and Architectures (SPAA ’24), p. 14, 2024, [Online]. Available: https://www.eecg.utoronto.ca/~mcj/papers/2024.rpb.spaa.pdf  
[5] J. Abdi, G. Zhang, and M. C. Jeffrey, “Brief Announcement: Is the Problem-Based Benchmark Suite Fearless with Rust?,” Association for Computing Machinery, vol. 202, pp. 303–305, May 2023, doi: 10.1145/3558481.3591313. 

