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
$ sesstype-cli parse examples/simple_choice.mpst
μT.A → B:{ l().end, l2().T }
```

Projecting the global type for role A

```
$ sesstype-cli project examples/simple_choice.mpst --role A
μT.B⊕{ !l().end, !l2().T }
```

For more options, use the `-h` flag

```
$ sesstype-cli -h
```

## License

sesstype-cli is licensed under the [Apache License](http://www.apache.org/licenses/LICENSE-2.0).
