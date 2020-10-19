# PNGme

A little project for practicing Rust. [Here](https://picklenerd.github.io/pngme_book/) is the tutorial.

## usage

1. Encode a message into a PNG file.
```
cargo run encode <file-path> <chunk-type> <message> [output-file]
```

2. Decode a message stored in a PNG file.
```
cargo run decode <file-path> <chunk-type>
```

3. Remove a message from a PNG file.
```
cargo run remove <file-path> <chunk-type>
```

4. Print a list of PNG chunks that can be searched for messages.
```
cargo run print <file-path>
```

PS: `<chunk-type>` consists of four letters. The third letter is uppercase and the other three letters are lowercase. For example, `<chunk-type>` can be `"ruSt"`.