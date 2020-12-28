# Powerset generator
A simple rust program that prints to STDOUT the powerset (sets of all subsets, including the empty set) of the given set like follows:

Input: ```123,456,789```
Output:
```

123
456
789
123,456
123,789
456,789
123,456,789
```
#### Building

```sh
$ cargo build
```

#### Run
Run with a raw string input:
```sh
$ cargo run -- -i <raw_input>
```
Run with a file input:
```sh
$ cargo run -- -f <path_to_file>
```

#### Testing

```sh
$ cargo test
```