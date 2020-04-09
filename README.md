# raide
raide - A ray of light in the world of IDEs

![screenshot](pictures/Raide.png)

## Introduction

raide is a simple IDE with a multi project approach in mind. It features currently:

* A save button
* Multiple Tabs of files that are closable and writable. Right click reveals the full path.
* Source highlighting of all languages understood by gtksourceview, including Rust, Java, JSON and Markdown.
* Generatable buttons with commands read from the workspace file raide.ron
* File-based completion, which completes text to previously typed words
* Project-based completion, which completes text of the common programming language
It is useful on platforms with limited resources, as the application requires only 90 MB RAM.
## Intended features

* Another feature would be completion at standard library level of the respective language, which completes text to previously typed words.
* File-based search and replace would be another feature

## Example raide.ron


```
(
    name: "raide",
    exclude_files: [],
    commands: [(name:"build",has_button:true,command:"cargo build",key_binding:None,),(name:"run",has_button:true,command:"./file",key_binding:None,),(name:"format",has_button:true,command:"rustfmt {file}",key_binding:None,),],
)


```

This config features a 

* project level build button
* project level run button
* file level format button 
