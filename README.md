# Polycarp

Competitive Programming Assistant for Rust.

I drew inspiration from the JHelper plugin for CLion, but I made Polycarp a regular Rust repository that is not bound to
any IDE.

With Polycarp, you can listen to parsed tasks from Competitive Assistant browser extension and generate tests with ease.
All you need is to implement a single function, and you're good to go! Additionally, I included a library with commonly
used algorithms, so you don't have to write the same code repeatedly.

If you encounter any issues or have feedback, feel free to report them on the GitHub page.

[Discussion at Codeforces Blogs][codeforces-post].

## Quick start

1. Install Competitive Companion: [firefox][companion-ff], [chrome][companion-chrome].
2. Start Polycarp service in a separate terminal: `cargo run --bin polycarp`.
3. Navigate to the problem web page ([example][example-problem]).
4. Click `+` icon to parse the problem.
5. Now unimplemented solution and tests are generated in [solve.rs][solve.rs].
6. Run tests using `cargo test --bin submission`.
7. Implement `fn solve` in [solve.rs][solve.rs].
8. Generate [submission.rs][submission.rs] using `cargo run --bin finalizer`.
9. Upload [submission.rs][submission.rs] to e-judge ([example][example-problem-submission]).

https://user-images.githubusercontent.com/56959852/229355124-3670f984-fc7e-4450-be64-16d81e231873.mp4

## Features

### Extendable library

There is a [library][library] module that provides most frequently used primitives for competitive programming.

### Library modules are merged into `submission.rs`

For example if you add

```rust
use library::ordering;
```

then you can use it in [solve.rs][solve.rs]:

```rust
fn solve() {
    // ...
    vec.sort_by(ordering::reverse);
}
```

and when generating a [submission.rs][submission.rs], the above import will be expanded to

```rust
mod ordering {
    pub fn reverse<T: core::cmp::Ord>(a: &T, b: &T) -> core::cmp::Ordering {
        b.cmp(a)
    }
}
```

### All the user data is added to `.gitignore`

This way user can just clone this repo and start competing without any additional setup.

### Previous solutions are saved to `data/history`

When a new problem is parsed, then previous [solve.rs][solve.rs] is copied to [history][history] directory (which is
added to `.gitignore`).

[codeforces-post]: https://codeforces.com/blog/entry/114637

[companion-ff]: https://addons.mozilla.org/ru/firefox/addon/competitive-companion/

[companion-chrome]: https://chrome.google.com/webstore/detail/competitive-companion/cjnmckjndlpiamhfimnnjmnckgghkjbl

[history]: data/history

[example-problem]: https://codeforces.com/problemset/problem/1003/A?locale=en

[example-problem-submission]: https://codeforces.com/contest/1003/submission/109911833?locale=en

[library]: library/src

[solve.rs]: submission/src/solve.rs

[submission.rs]: submission/src/submission.rs
