# xcat

```bash
Usage: xcat [OPTIONS] <COMMAND> [ARGS]...

Arguments:
  <COMMAND>
  [ARGS]...

Options:
      --tiktoken <TIKTOKEN>  Divide stdin by chunks up to N tokens long
  -h, --help                 Print help
  -V, --version              Print version
```

`xcat` reads from its `stdin`. For each line read, it spawns `<COMMAND>` with
`[ARGS]`, and puts that single line on the spawned processes `stdin`.

When the `--tiktoken <N>` option is present, instead of dividing stdin by
newlines, it divides it by chunks up to `N` tokens long. It currently
uses the 
[`cl100k_base`](https://github.com/openai/openai-cookbook/blob/main/examples/How_to_count_tokens_with_tiktoken.ipynb) 
encoding suitable for the ChatGPT models and text-embedding-ada-002.

## Examples

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
$ echo "This is an example of using the tiktoken option." | xcat --tiktoken 5 -- cat
This is an example of
using the tiktoken option
.
```
