## Rust Utils

Rust utils is an attempt at rewriting GNU core utils in Rust.
<br/>
The modules in this repository currently only cover the most common usage patterns. 

### Programs

The following list covers the implemented programs and the available options:

| Name      | Description                             | Usage                                  | Supported Options                                          |
|-----------|-----------------------------------------|----------------------------------------|------------------------------------------------------------|
| ls        | List files in a directory.              | ls [options] [\<dir\>]                 | --list: Detailed list <br/> --all: include hidden files    |
| cat       | Write files to stdout.                  | cat \<file\>                           | -                                                          |
| less      | Read files partially.                   | less \<file\>                          | -                                                          |
| grep      | Match patterns in a file or from stdin. | grep \<pattern\> [\<file\>] [options]  | --after: lines after <br/> --before: lines before          |
| tail      | Print last n lines for a file.          | tail \<file\> [options]                | --lines: number of lines to print                          |
