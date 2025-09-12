# Addressing limitations of `json2dir`

This documents some known issues of `json2dir` and potential ways to solve them.

## TOCTOU

`json2dir` should be rewritten not to be affected by TOCTOU attacks.

## Non-UTF-8 file contents

`["base64", string]` builtin should be added to support arbitrary file contents.

## Directory metadata

An builtin for directories should be added. This would allow to represent file metadata and non-UTF-8 file names.

Potential syntax:

```text
[
    "directory",

    # The short form, allowing arbitrary file names
    
    ## Binary file name. 
    ["base64", string],
    
    ## File contents.
    "Hello"

    # The full form. Not clear how useful it is.

    {
        "name": ...,
        "content": ...,

        # Unclear what the permissions should be exactly, but could be useful.
        "permissions": ...,

        # Not sure if this will make the cut.
        "user": ...,
        "group": ...,
    }
]
```

# Executable files which aren't scripts

Currently, all executable files are tagged `script`. This is misleading.

Potential solution: rename the `script` tag to something better. For example, `executable`.
