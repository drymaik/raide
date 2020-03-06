# raide
Raide - A ray of light in the world of IDEs

![screenshot](pictures/Raide.png)

## Introduction

As IDEs grow big with many MenuItems, working with them gets confusing. Sadly, many IDEs are only suited to a subset of languages.

Raide is a simple IDE with a multi project approach in mind. It features currently:

* A save button
* Multiple Tabs of files that are closable and writable. Right click reveals the full path.
* Source highlighting of many languages including Rust, Java, JSON and Markdown.
* Generatable buttons with commands read from the workspace file raide.ron

## Intended features

Another useful feature would be code completion on file level, which is file and project dependent.

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
