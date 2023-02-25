# xcat

```bash
Usage: xcat <COMMAND> [ARGS]...

Arguments:
  <COMMAND>  
  [ARGS]...  
```

`xcat` reads from its `stdin`. For each line read, it spawns `<COMMAND>` with
`[ARGS]`, and puts that single line on the spawned processes `stdin`.

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

