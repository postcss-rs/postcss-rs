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
|   **js** |                 1.813ms |              2.380ms |           11.856ms |      33.919ms |    167.309ms |         224.577ms |
| **rust** |                 0.019ms |              0.037ms |            0.191ms |       0.721ms |   9.624272ms |       15.743094ms |
|    **~** |                     95x |                  64x |                61x |           47x |          17x |               14x |

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
