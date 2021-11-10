# minosse
An ultra fast CLI app that will help you to manage (add/update/remove field and dependencies) on large codebases containing multiple package.json (ie: monorepo).

## USAGE:
    minosse [OPTIONS] <input-dir>

## FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

## OPTIONS:
        --remove-dependencies <remove-dependencies>             Remove value from package.json's field `dependencies`
        --remove-dev-dependencies <remove-dev-dependencies>     Remove value from package.json's field `dependencies`
        --remove-peer-dependencies <remove-peer-dependencies>   Remove value from package.json's field `peerDependencies`

        --suffix <suffix>                                       Avoid inplace replace by adding suffix ie: `file<suffix>`
        --update-dependencies <update-dependencies>             Update value for package.json's field `dependencies`
        --update-dev-dependencies <update-dev-dependencies>     Update value for package.json's field `devDependencies`
        --update-license <update-license>                       Update value for package.json's field `license`
        --update-peer-dependencies <update-peer-dependencies>   Update value for package.json's field `peerDependencies`
        --update-private <update-private>                       Update value for package.json's field `private`

## ARGS:
    <input-dir>    Input dir

## Examples

```bash
./minosse . --update-private true --update-license MIT --suffix .new
```

```bash
./minosse . \
    --update-peer-dependencies "@storybook/addon-a11y=^6.3.12" \
    --update-dev-dependencies "@storybook/addon-a11y=^6.3.12" \
    --update-dependencies "@storybook/addon-a11y=^6.3.12"
```

```bash
./minosse . \
    --update-peer-dependencies "backbone=^1.4.0" \
    --remove-dependencies "react"
```
