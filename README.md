# json2dir: convert JSON with objects and strings into a directory

Example:

```sh
printf '%s' '{
  "file": "Hello, world!\n",
  "dir": {
    "subfile": "Content.\n",
    "subdir": {}
  }
}' | json2dir
```
