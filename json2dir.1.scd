json2dir(1)

# NAME

json2dir — convert JSON objects to directory trees

# SYNOPSIS

*json2dir* < _file.json_

# INPUT SCHEMA

- Objects represent directories.
- Strings represent contents of files.
- Arrays are used to represent symlinks and executable files.
- Arrays of the form `["link", target]` represent symlinks, second element representing the target of the symlink.
- Arrays of the form `["script", content]` represent executable files, second representing the content of the script.

# BUGS

Regular JSON constraints apply. In particular, the input must be UTF-8. Currently, there's no way to represent files containing non-UTF-8 content.

When using this utility to create files for other users, care must be taken in order to prevent TOCTOU (time of check, time of use) attacks (e.g. with symlinks).

# EXAMPLES

```
printf '%s' '{
  "file": "Hello, world!",
  "dir": {
    "subfile": "Content.\\n",
    "subdir": {}
  },
  "symlink": ["link", "target path"],
  "script": ["script", "#!/bin/sh\\necho Howdy!"]
}' | json2dir
```

Here, four files will be added to the current directory: `file`, `dir`, `symlink`, and `script`.

# SEE ALSO

*jq*(1)

# AUTHORS

Maintained by Alan Urmancheev <alan.urman@gmail.com>
