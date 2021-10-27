# postcss-rs

🚀 Fast and 100% API compatible postcss replacer, built in Rust

> ⚠️ DO NOT USE. STILL WORK IN PROGRESS.

## Performance Improvement 

Benchmark:

```bash
tokenizer/small(7K)     time:   [418.15 us 422.79 us 428.63 us]
tokenizer/large(201K)   time:   [12.828 ms 13.231 ms 13.660 ms]
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
