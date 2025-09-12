# Design considerations of `json2dir`

## "Time of check, time of use" attacks

TOCTOUs are not handled at the moment.

## Deletion of files

`json2dir` is not strictly additive — it tries to delete a file or directory before overwriting it. This is an implementation detail.

## UTF-8

At the moment, file names and their contents must be valid UTF-8. In the future, arbitrary file names and contents will be supported by base64-encoding them.

## String paths with multiple path segments

Strings with multiple path segments are currently rejected. This restriction might be relaxed in the future.

## String paths with special path segments

String paths with special path segments (e.g. `..`, `.`) are rejected. This is a measure to localize effects of `json2dir` to its working directory.