`cat file.json | json2dir spec`

# spec: which files do you want?

`{ a, b, c }`

`{ go.mod }`

`<(find . -iname '*.go' -0)`

```yaml
{
  a: null, b: null, c: null, d: null
}
```

`{}` -- I want everything.

dir2json:
* consume dir
* can produce a summary of changes?

json2dir:
* produce dir
* consume json from stdin
* consume spec as file?
* can produce a summary of changes

dir2json spec.json

dir2json go.mod main.go

cat archive.json | json2dir go.mod main.go foo/{bar,baz}

json2dir go.mod main.go foo { bar baz }

jsondir to-dir
jsondir to-json
