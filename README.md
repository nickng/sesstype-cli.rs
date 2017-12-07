# sesstype-cli

This is a command-line interface to the `sesstype` crate, an implementation of
Multiparty Session Types.

## Build

To build the `sesstype-cli` binary from source

```
cargo build --release
```

## Using the tool

Parsing the global type

```
$ sesstype-cli examples/simple_choice.mpst
μT.A → B:{ l().end, l2().T }
```

Projecting the global type for role A

```
$ sesstype-cli examples/simple_choice.mpst -p A
μT.A⊕{ !l().end, !l2().T }
```

For more options, use the `-h` flag

```
$ sesstype-cli -h
```

## License

sesstype-cli is licensed under the [Apache License](http://www.apache.org/licenses/LICENSE-2.0).
