json2dir(1)

# NAME


json2dir — a tool to convert JSON to a directory

# SYNOPSIS

*json2dir* < file.json_

# DESCRIPTION

TODO: dedup?
The json2dir tool recursively converts a JSON object given to stdin to a directory structure.

Files and directories will be made in the directory where the tool is launched.

Object keys map to directory names. String values map to file contents.

# BUGS

Currently, symlinks can't be represented and files' contents must be valid JSON strings.

# EXAMPLES

```
printf '%s' '{
  "file": "Hello, world!\n",
  "dir": {
    "subfile": "Content.\n",
    "subdir": {}
  }
}' | json2dir
```

# SEE ALSO

*jq*(1)

# AUTHORS

Maintained by Alan Urmancheev <alan.urman@gmail.com>
