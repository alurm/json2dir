# YAML support in `json2dir` and `dir2json`, and other formats

YAML is currently not supported. But maybe it should, as it would make archives much more readable.

YAML seems to be quite a useful format for human-readable copy-pastable archives.

Currently this is possible with `yq`, but that requires an external program.

## Issues

* For JSON-accepting clients, JSON output should be supported (duplicate work?).
* It's unclear what's the key limit for pretty-printed YAML. That might cause futher issues.
* Many YAML libraries are no longer supported. Future of current YAML libraries is unclear. Using `libyaml` means linking to C.
* It's unclear if YAML is a strict JSON subset or not. [Discussion on HN](https://news.ycombinator.com/item?id=30052633).
* It's unclear how the CLI should look like.

### Cognitive complexity

To increase the chances that this project is used with regular users, complexity should be kept to minimum.

Having two formats sure does introduce a lot of complexity.

### Project naming

`jsondir` is a fine name. But then, if YAML support is added, JSON is no longer the only supported format.

## Other formats

* TOML: doesn't make sense since it's support for pretty-printed hierarchies is weak.
* KDL: is not very popular and is not a JSON subset.
* Cue: seems like this one currently requires FFI with Go.

## Potential CLI designs

### (1)

4 binaries: `json2dir`, `dir2json`, `yaml2dir`, `dir2yaml`. That's a lot.

### (2)

Have 1 binary that does everything (possibly in the style of `busybox`).

Potential names:

* jsondir
* brigadir
* commandir
* mordir
* transdir
* dirconv
* crocodir

```sh
<name> to json
<name> to yaml
<name> from yaml
<name> from json
```