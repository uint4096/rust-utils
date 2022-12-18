## Rust Utils

Rust utils is an attempt at rewriting GNU core utils in Rust.
<br/>
The modules in this repository currently only cover the most common usage patterns. 

### Programs

The following list covers the implemented programs and the available options:

| Name      | Description                            | Usage                                         | Supported Options                               |
|-----------|----------------------------------------|-----------------------------------------------|-------------------------------------------------|
| ls        | List files in a directory.             | ls [options] [\<file\>]                       | -l: Detailed list <br/> -a: include hidden files|
| cat       | Write files to stdout                  | cat \<file\>                                  | -                                               |
| less      | Read files partially                   | less \<file\>                                 | -                                               |
| grep      | Match patterns in a file or from stdin | grep \<pattern\> [\<file\>] [options]         | -a: lines after <br/> -b: lines before          |
