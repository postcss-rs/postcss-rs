# postcss-rs benchmark

Run in the root directory:

```bash
cargo build --release --locked --bin "benchmark*"
cargo run --release --bin benchmark
```

Output:

```plain
js  : tokenizer/tailwind-components.css(2.8K):   1.73,   1.58,   1.24
rust: tokenizer/tailwind-components.css(2.8K):   0.02,   0.02,   0.01
                                                           97x
js  : tokenizer/bootstrap-reboot.css(7.4K):      3.03,   2.73,   2.20
rust: tokenizer/bootstrap-reboot.css(7.4K):      0.04,   0.04,   0.03
                                                           70x
js  : tokenizer/bootstrap-grid.css(71K):        12.05,  10.99,   9.29
rust: tokenizer/bootstrap-grid.css(71K):         0.19,   0.20,   0.14
                                                           56x
js  : tokenizer/bootstrap.css(201K):            22.86,  20.75,  16.78
rust: tokenizer/bootstrap.css(201K):             0.52,   0.57,   0.50
                                                           37x
js  : tokenizer/tailwind.css(3.5M):            174.74, 183.61, 156.43
rust: tokenizer/tailwind.css(3.5M):              8.50,   9.15,   7.93
                                                           20x
js  : tokenizer/tailwind-dark.css(5.8M):       255.43, 266.87, 223.31
rust: tokenizer/tailwind-dark.css(5.8M):        12.96,  13.28,  11.40
                                                           20x
js  : parser/tailwind-components.css(2.8K):      4.40,   3.92,   3.23
rust: parser/tailwind-components.css(2.8K):      0.07,   0.08,   0.06
                                                           51x
js  : parser/bootstrap-reboot.css(7.4K):         7.09,   6.77,   5.02
rust: parser/bootstrap-reboot.css(7.4K):         0.12,   0.13,   0.11
                                                           53x
js  : parser/bootstrap-grid.css(71K):           34.84,  31.36,  25.93
rust: parser/bootstrap-grid.css(71K):            0.84,   0.89,   0.72
                                                           35x
js  : parser/bootstrap.css(201K):               63.61,  58.67,  47.94
rust: parser/bootstrap.css(201K):                2.53,   2.66,   2.21
                                                           22x
js  : parser/tailwind.css(3.5M):               437.17, 443.01, 371.15
rust: parser/tailwind.css(3.5M):                38.55,  39.56,  33.88
                                                           11x
js  : parser/tailwind-dark.css(5.8M):          608.93, 632.51, 548.09
rust: parser/tailwind-dark.css(5.8M):           62.42,  63.21,  52.15
                                                           10x
```
