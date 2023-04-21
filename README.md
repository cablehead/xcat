# xcat

```bash
Usage: xcat [--tiktoken <N>] <COMMAND> [ARGS]...

Options:
  --tiktoken <N>    Divide stdin by chunks up to N tokens long

Arguments:
  <COMMAND>  
  [ARGS]...  
```

`xcat` reads from its `stdin`. For each line read, it spawns `<COMMAND>` with
`[ARGS]`, and puts that single line on the spawned processes `stdin`.

When the `--tiktoken <N>` option is present, instead of dividing stdin by
newlines, it divides it by chunks up to `N` tokens long. It currently
cl100k_base	encoding, suitable for the ChatGPT models and
text-embedding-ada-002.

## Example

```bash
$ cat Cargo.toml | xcat -- wc -c
      10
      14
      18
      17
       1
      15
      52
```

```bash
$ echo "This is an example of using the tiktoken option." | xcat --tiktoken 5 -- wc -w
       5
       4
```
