# Polycarp

Competitive programming assistant.

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

![](media/example.mp4)

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

[companion-ff]: https://addons.mozilla.org/ru/firefox/addon/competitive-companion/
[companion-chrome]: https://chrome.google.com/webstore/detail/competitive-companion/cjnmckjndlpiamhfimnnjmnckgghkjbl
[history]: data/history
[example-problem]: https://codeforces.com/problemset/problem/1003/A?locale=en
[example-problem-submission]: https://codeforces.com/contest/1003/submission/109911833?locale=en
[library]: library/src
[solve.rs]: submission/src/solve.rs
[submission.rs]: submission/src/submission.rs
