# `dir2json`: directory-to-JSON converter producing human-readable copy-pastable archives

`dir2json` allows the user to create human-readable JSON representations of directories. It's an archiver. But what's unusual about such archives is that they are both human-readable and machine-readable. The archives can be easily embedded in plain text documents. So, you can add such archives in your:

* blogs posts,
* examples,
* docs,
* text messages (on Slack, Discord, Email, etc.)

Usage: `dir2json <paths to be added in the archive...>`.

Such archives can later be passed to [`json2dir`](https://github.com/alurm/json2dir) to unpack them.

## Integrations, uses

### Even more human-readable archives

`dir2json` can be used in conjunction with [`yq`](https://github.com/mikefarah/jq) (or another JSON-to-YAML converter) to create YAML archives.

Here's an example:

```sh
dir2json . | yq -Poy
```

> Hint: pipe the output to `pbcopy` on MacOS or `xclip -selection clipboard` on Linux/X11 to copy the resulting archive to your clipboard.

Such YAML archives, when sent to stdin, can later be unpacked to the working directory using `yq` and `json2dir` in the following manner:

```sh
yq -oj | json2dir
```

> Hint: use `pbpaste` on MacOS or `xclip -selection clipboard` to paste the archive you copied to the input of the command above.

### Compression

The generated archives can be compressed with JSON using `gzip` or another compressor.

Here's an example:

```sh
dir2json . | gzip
```

### Archiving

You can archive directories using `dir2json`:

```sh
dir2json . > archive.json
```

Note that only essential metadata is currently supported. In particular, executable bit is kept and symbolic links are preserved as is.
