## Install

```bash
cargo install --path .
```

## Usage

```bash
╰─λ hardpass --help                                                                                                                                                                                                                                                                                0 (0.007s)
Usage: hardpass [OPTIONS]

Options:
  -u, --upper-case
  -l, --lower-case
  -d, --digits
  -s, --special
      --min-len <MIN_LEN>  [default: 8]
      --max-len <MAX_LEN>  [default: 10]
  -c, --count <COUNT>      [default: 1]
  -h, --help               Print help information
```

## Output

```bash
╰─λ hardpass -uld -c 10 --max-len 15 --min-len 10                                                                                                                                                                                                                                                  0 (0.010s)
1) iWewsaTC77r
2) RyCYGxAh6vFI
3) EBNW70nkUs7
4) 78Xiv11VhON
5) SOkCMpTOauyQj
6) qcLz7kdJiWN
7) u8NdJVBii4l6
8) D762JddsqN
9) utnkCKOoQ77
10) vWoHA5NwKzZ938
```
