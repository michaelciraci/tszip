### tszip

Simple command line utility for compressing and decompressing a directory through tar and snap compression

### Documentation

See `tszip --help`.

### Installation

```
cargo install tszip
```

### Usage

`tszip` works similar to gzip, except it works on directories

This is equivalent to the following commands, except all the commands are compiled together allowing for optimizations.

This could be replaced by the following commands, although compiling the commands into a single executable allows for increase compiler optimizations.

#### Equivalence

Compression:
```
tar -c <input-dir> | szip | > <input-dir>.tszip
```

Decompression:
```
cat <input-dir>.tszip | szip -d | tar -x
```

By default, `tszip` will remove the input file (similar to gzip). The `-k` flag will keep the original input.
