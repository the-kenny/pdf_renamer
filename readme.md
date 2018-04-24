# PDF Renamer

One-off tool to rename a PDF file to its `CreationDate` + `.pdf`.
Useful if you have a folder of unordered email attachments from a
scanner and want to sort them.

Note that this only works for the specific date format of
`YYYYMMDDHHMMSS`. The scanner I was confronted with seems to be a
secial snowflake in this regard.

# Usage

`cargo run -- <path-to-pdf>`


