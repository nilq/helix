![praise lord helix](http://assets.pokemon.com/assets/cms2/img/pokedex/full/139.png 128x128)

# helix
A programming language transpiler

```
helix language

usage:
    helix repl
    helix translate <source>
    helix (-h | --help)
    helix --version

options:
    -h --help   display this message
    --version   display version
```

## Examples

**if-statements**
```helix
# sets foo to the integral 10
foo = 10

# does things if foo is 10
if foo == 10
    bar = "foo is 10"

    if bar == "hey"
        # the `print` function dumps to console
        print("hey hey")
    else
        print("bar is not 'hey'")
```
Output AST
```
abstract syntax tree =>
[
    Assignment(
        "foo",
        Integer(
            10
        )
    ),
    If(
        Operation(
            Integer(
                10
            ),
            Equal,
            Ident(
                "foo"
            )
        ),
        Block(
            [
                Assignment(
                    "bar",
                    Text(
                        "foo is 10"
                    )
                ),
                IfElse(
                    Operation(
                        Text(
                            "hey"
                        ),
                        Equal,
                        Ident(
                            "bar"
                        )
                    ),
                    Block(
                        [
                            Expression(
                                Call(
                                    Ident(
                                        "print"
                                    ),
                                    [
                                        Text(
                                            "hey hey"
                                        )
                                    ]
                                )
                            )
                        ]
                    ),
                    Block(
                        [
                            Expression(
                                Call(
                                    Ident(
                                        "print"
                                    ),
                                    [
                                        Text(
                                            "bar is not \'hey\'"
                                        )
                                    ]
                                )
                            )
                        ]
                    )
                )
            ]
        )
    )
]
```

## Draft

Ideas for non-parallel structure.

**laws and events**
```helix
# events can be spawned onto the event-stack
event foo(a, b)
    print("foo spawned with: ", a, b)

# rules are absolute elements on the event-stack
rule
    if something == true
        foo(1, 2) # spawn foo
```
