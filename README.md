# minosse
An ultra fast CLI app that fixes json files in large codebase or folders

## USAGE:
    minosse [OPTIONS] <input-dir>

## FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

## OPTIONS:
        --field-license <field-license>    New value for Package.json field `license`
        --field-private <field-private>    New value for Package.json field `private`
        --suffix <suffix>                  Output file: `file` or `file<suffix>` [default: ]

## ARGS:
    <input-dir>    Input dir

## Examples

```bash
./minosse . --field-private true --field-license MIT --suffix .new
```
