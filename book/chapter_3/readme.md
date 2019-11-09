# [Book Chaper 3](https://doc.rust-lang.org/book/ch03-05-control-flow.html#summary)

-   [Convert temperatures between Fahrenheit and Celsius.](./src/temperature.rs)

-   [Generate the nth Fibonacci number](./src/fibonacci.rs)

| fib  | iterative  | recursive |
| ---- | ---------- | --------- |
| 30th | 0.000547ms | 6.5ms     |
| 92th | 0.000877ms | -         |

-   [Print the lyrics to the Christmas carol “The Twelve Days of Christmas,” taking advantage of the repetition in the song.](./src/christmas.rs)

### Running Tests

```bash
cargo test -p book_chapter_3
```

### Running Benchmarks

```bash
cargo bench -p book_chapter_3
```
