# postcss-rs

🚀 Fast and 100% API compatible postcss replacer, built in Rust

> ⚠️ DO NOT USE. STILL WORK IN PROGRESS.

## Performance Improvement

Tokenizer (Compare with Node.js v16.13.0):

```bash
js:   0.71s user 0.09s system 152% cpu 0.619 total
rust: 0.16s user 0.03s system  93% cpu 0.199 total
```

| **file** | tailwind-components.css | bootstrap-reboot.css | bootstrap-grid.css | bootstrap.css | tailwind.css | tailwind-dark.css |
| -------: | ----------------------: | -------------------: | -----------------: | ------------: | -----------: | ----------------: |
| **size** |                    2.8K |                 7.4K |                71K |          201K |         3.5M |              5.8M |
|   **js** |                 0.046ms |              0.121ms |            1.193ms |       3.716ms |    127.647ms |         217.074ms |
| **rust** |                 0.047ms |              0.019ms |            0.139ms |       0.451ms |      6.732ms |          11.093ms |
|    **~** |                    0.9x |                   6x |               8.5x |          8.2x |          19x |               19x |

🎉 Welcome to contribute, here is a guide:

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

## License

[postcss-rs](https://github.com/justjavac/postcss-rs) is released under the
MIT License. See the [LICENSE](./LICENSE) file in the project root directory for details.

## ACKNOWLEDGEMENT

**The project is based on the wonderful work of Andrey Sitnik([@ai](https://github.com/ai)) 's [postcss](https://github.com/postcss/postcss), which is under MIT License(See [HERE](./POSTCSS.LICENSE)), Great thanks to Mr. Sitnik and other contributors' work.**
