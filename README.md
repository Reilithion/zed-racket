# Zed Racket Extension

This extension adds support for the [Racket](https://racket-lang.org/) to the Zed code editor.

## Requirements

This extension is designed to work with Zed version 0.162.2 or later. It makes use of the Zed extension API 0.2.0.

Many convenient features for interacting with Racket source files are provided
through a language server. The currently supported language server is
[`racket-langserver`](https://github.com/jeapostrophe/racket-langserver). In
order to take advantage of these features you will need to have Racket and
`racket-langserver` installed, and `racket` in your PATH environment variable.
This is is not a hard requirement. You can still edit Racket source files with
syntax highlighting without either of these soft dependencies.

## How to install

The extension itself can be installed from within Zed using Zed's extension manager
(`zed: extensions` in the command palette, Ctrl-Shift-X or Cmd-Shift-X by default).

To get language server support, you'll need Racket and `racket-langserver`.

Installing Racket will differ depending on your operating system and preference.
It is widely available in package managers on Linux distributions and otherwise
can be downloaded from [the Racket website](https://racket-lang.org).

Once you have Racket installed and in your PATH, you can install
`racket-langserver` using Racket's `raco` package manager:

```
raco pkg install racket-langserver
```
