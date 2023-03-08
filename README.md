# Getting started

Our artifact is provided as a Docker image, which can be run using any recent version of [Docker](https://docs.docker.com/get-docker/).

## Using the image

With `docker` installed and in your path, start by loading the `russol` image from the included file. From the root of the artifact, run the following command

> The `creusot` image is only required for the [Creusot subsection](#creusot), and can be ignored for now.

```bash
docker load -i russol.tar.gz
```

> If any of the commands in this section do not work for you, take a look at [alternatives](#alternatives).

### Single file

The image is used to run RusSOL; check that it is working correctly by running the following command

```bash
docker run -it -v ${PWD}/demo.rs:/demo.rs jonasalaif/russol run --release --bin ruslic /demo.rs
```

> On Windows this command must be run in PowerShell, for the `cmd` terminal replace `${PWD}` with `%cd%`.

The expected output is

```text
    Finished release [optimized] target(s) in ...s
     Running `target/release/ruslic /demo.rs`
fn foo(x: &mut std::result::Result<T, V>) -> (bool, std::result::Result<&mut V, &mut T>) {
  match x {
    Result::Ok(_0) => {
      let _1 = Result::Err(_0);
      (true, _1)
    }
    Result::Err(_0) => {
      let _1 = Result::Ok(_0);
      (false, _1)
    }
  }
} // Synth time: ...
```

To run RusSOL on an arbitrary Rust file, replace `${PWD}/demo.rs` in the above command with `/path/to/local/file.rs`.

### Crate directory

Running the tool on a crate within a directory is also possible, using the following command

> Unlike for the single file, this will **modify** the files in the directory with the synthesis results!

```bash
docker run -it -v ${PWD}/demo:/demo jonasalaif/russol run --release --bin cargo-russol -- --manifest-path=/demo/Cargo.toml
```

## Structure of the artifact

The tool is located at `/home/sbtuser/russol-alpha` within the image (the working directory when running the image). A copy of this directory is included in `russol-alpha` at the root of the artifact, to allow files to be inspected locally. We now explain the structure of this directory.

- The `ruslic` directory contains the code for the frontend translation of the Rust signature and types into and SOL task.
  - The `ruslic/tests` directory contains both the test harnesses as well as the files tested on.
- The `russol-contracts` and `russol-macros` directories define the procedural macros for annotating signatures (e.g. `#[requires(...)]`, `#[pure]` etc.)
- The `suslik` directory contains a heavily modified version of SuSLik, used as the backend for solving SOL tasks.

## Commands explained

We now explain the above command to run RusSOL in more detail. It can be broken down into

```bash
docker run -it -v ${PWD}/demo.rs:/demo.rs jonasalaif/russol run --release --bin ruslic /demo.rs
^^^^^^^^^^^^^^                            ^^^^^^^^^^^^^^^^^                                     (1)
               ^^^^^^^^^^^^^^^^^^^^^^^^^^                                                       (2)
                                                            ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ (3)
```

1. Basic command to run the image (setup to run `cargo` from within the `russol-alpha` directory).
2. Mounts a file or directory from the host into the container (**mounted files can be modified from within the container**).
3. The command to run inside the container (here equivalent to running `cargo run --release --bin ruslic /demo.rs` locally).



## Alternatives

> Alternatively, the image is on Docker Hub: `docker pull jonasalaif/russol`.

> It can also be built manually: `docker build -t jonasalaif/russol .`

There are three possible ways to load the docker image:

- Using the included image file (recommended): `docker load --input russol.tar.gz`

- Using the image on Docker Hub: `docker pull jonasalaif/russol`

- Build manually: `docker build -t jonasalaif/russol .`



# Step-by-Step Instructions

The artifact includes the tools required to verify all of our claims. We describe each in turn, and provide instructions for reproducing the results.

## Section 1

The StackOverflow example from Fig. 1 can be found at `russol-alpha/ruslic/tests/synth/paper/rust/stackoverflow/reborrow.rs`. It can be synthesized by running the tool on a single file as described above

```bash
docker run -it -v ${PWD}/russol-alpha/ruslic/tests/synth/paper/rust/stackoverflow/reborrow.rs:/reborrow.rs jonasalaif/russol run --release --bin ruslic /reborrow.rs
```

## Section 2

The running example from this section can be found at `russol-alpha/ruslic/tests/synth/paper/rust/custom/paper/list_paper.rs`. All of the functions can be synthesized in one go with

```bash
docker run -it -v ${PWD}/russol-alpha/ruslic/tests/synth/paper/rust/custom/paper/list_paper.rs:/list_paper.rs jonasalaif/russol run --release --bin ruslic /list_paper.rs
```

The order of the functions is the same as presented in the paper, though the names of some of the functions are changed to avoid clashes. There is also an additional `pop` function, which is not mentioned in the paper.

The functions can also be synthesized one by one, by annotating the one to be synthesized with `#[synth]` in the source file.

## Section 4

There are two main points to be checked for the evaluation; the results from Table 1, and that synthesis results can be verified by Creusot.

### Table 1

#### Rust, SuSLik and Verifier categories

The categories "Rust", "SuSLik" and "Verifier" are contained in respective directories inside `russol-alpha/ruslic/tests/synth/paper`. They can all be synthesized by running the test harness `russol-alpha/ruslic/tests/ci.rs` with

```bash
docker run -it -v ${PWD}/russol-alpha/ruslic/tests/synth/paper:/home/sbtuser/russol-alpha/ruslic/tests/synth/paper jonasalaif/russol test --release --test ci -- all_tests --nocapture
```

After synthesizing all of the test cases, a table similar (time results will vary) to the one found at `russol-alpha/ruslic/tests/ci-results.txt` will be printed. This table contains a summary of the results for each category, for example

```text
 # rust (rust & 50 & LOC 6.2 & AN 14.4 & SN 19.2 & USN 21.8 & RA 33.0 & ? & T 2.4)
```

means that for the "Rust" category, 50 test cases were synthesized, with an average of 6.2 LOC per function, 14.4 annotation AST nodes, 19.2 synthesized AST nodes, 21.8 unsimplified synthesized AST nodes (not reported in the paper), 33.0 rule applications and 2.4 seconds of synthesis time.

Stats per function are also reported, for example

```text
stack.rs::List::<T>::new - 0_204ms [1/3/3/5] | spec_ast: 4, pfn_ast: {"List::<T>::len": 14, "Node::<T>::len": 16}
```

means that the function `List::<T>::new` was synthesized in 0.204 seconds, with 1 LOC, 3 synthesized AST nodes, 3 unsimplified synthesized AST nodes (not reported in the paper), 5 rule applications and a specification AST of size 4. Pure functions used for the specification and their size in AST nodes are also reported.

The annotation overhead is reported as the ratio of the number of synthesized AST nodes to the number of annotation AST nodes. The pure function AST nodes are included in the annotation count only if they return a type not usable outside of specification (e.g. `Set<T>`).

Only the results under `# paper` are relevant to the paper. The results under `# other` are extra synthesis tasks which were not included in the paper and should be ignored.

The two failing cases can be found under `russol-alpha/ruslic/tests/synth/paper/rust/stackoverflow/swap_enum.rs` and at the end of `russol-alpha/ruslic/tests/synth/paper/suslik/rose-tree_multi-list/rose-tree_multi-list.rs`. They are not tested by the command above, but can be uncommented and synthesized individually (as described above).

#### 100-Crates category

The test harness `russol-alpha/ruslic/tests/top_crates.rs` downloads the top 100 crates from crates.io and runs the tool on them. It can be run with

> This command takes multiple hours to complete!

```bash
docker run -it jonasalaif/russol test --release --test top_crates -- top_crates_all --nocapture
```

At the end it will print a summary of the results, which should look similar to that included in `russol-alpha/ruslic/tests/crates-results.txt`, though there will be small differences due to changes in the crates (the test always downloads the latest version of each crate).


### Creusot

There are a few subtle differences between Creusot specifications and RusSOL ones which prevent us from directly running the former tool on the same file synthesized by the latter. Therefore, we have compiled and slightly updated the specification style of all the annotated tests into a single file at `russol-alpha/ruslic/tests/all/creusot.rs`.

> Some functions are commented out as they are either not supported by Creusot or require helper functions which the synthesis tool does not annotate with specifications.

The file can be verified by running the following three commands

```bash
docker load -i creusot.tar.gz
docker run -it -v ${PWD}/russol-alpha/ruslic/tests/all:/all jonasalaif/creusot ./mlcfg /all/creusot.rs
docker run -it -v ${PWD}/russol-alpha/ruslic/tests/all:/all jonasalaif/creusot ./prove /all/creusot.mlcfg
```

> All proof goals should be verified successfully, except the last one ("clone'_refn") which was not synthesized but rather automatically generated by `#[derive(Clone)]`.

The first command loads the Creusot image, the second command runs Creusot to generate a `.mlcfg` proof goal file, and the third command runs `why3` to try and automatically prove all goals in this file.

We include the expected proof goal file in the artifact, at `russol-alpha/ruslic/tests/all/creusot_expected.mlcfg` which can be compared to the generated one (`creusot.mlcfg`) if there are any issues.

The main changes required to adapt a file for Creusot are:

- Replace `use russol_contracts::*;` at the top of the file with `extern crate creusot_contracts; use creusot_contracts::*;`

- Replace all `#[pure]` annotations with `#[logic]`, any integer return type for these "logic" functions with `logic::Int` and `Set`s also need to be adapted (see the changes made between `tests/all/russol.rs`).

- In Creusot, "logic" functions need to be shown to terminate - to avoid having to prove this one can annotate the function with `#[trusted]` and add `#[ensures(result == (fn_body))]` where `fn_body` is the body of the function.

- Integer literals within specifications may need to be written as `5_u32` instead of `5`, if the Rust type is needed, to avoid Creusot's automatic conversion to `logic::Int`.