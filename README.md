# postcss-rs

🚀 Fast and 100% API compatible postcss replacer, built in Rust

> ⚠️ DO NOT USE. STILL WORK IN PROGRESS.

## Performance Improvement 

Tokenize [bootstrap.css](./assets/bootstrap.css) (Compare with postcss.js):

```bash
rust: 0.01s user 0.00s system  79% cpu 0.009 total
js:   0.13s user 0.02s system 126% cpu 0.109 total
```

🎉 Welcome contribute, here is a guide:

```bash
git checkout main
cargo bench -- --save-baseline main
```

Create a baseline, Then do some optimization work here.

```bash
git checkout feature  ## `feature` is your branch 
```

Compare against the `main` baseline.

```bash
cargo bench -- --baseline main
```
