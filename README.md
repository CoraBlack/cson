# cson -- C++ builder configured with JSON

> [!IMPORTANT]
>
> - cson is still in development, and the configrantion field is not stable yet.
> - cson is not only a build system, but also a configuration tool for build systems.

## Features(Order by priority)

- [x] Build project with compiler and linker by cson.json immediately.(only GNU)
- [ ] Build cache and mult-thread.
- [ ] Submodule support.
- [ ] Multiple compile targets.
- [ ] Platform-specific configuration.
- [ ] Build for cmake.

## What is the goal for cson?

We want to provide a build system that is easy to use and config for C++ beginners or who are not the programming prefessors, which can help them focus on their project and code instrendad of dealing with the build system configration.  
We only require a small number of essential fields for cson.json and make the build system configration as simple as possible.

## cson.json Example
```jsonc
{
    "project": "HelloWorld",    // project name
    "target_name": "HelloWorld",// the final compiled product name
    "build_dir": "build",       // the directory storing intermediate compiled product
    "output_dir": "bin",        // the directory storing final compiled product

    "toolchain": "gnu",         // unsuport currently but required

    "flags": [                  // parameters for c and c++ compiler
        "-Wall",
        "-Wextra"
    ],

    "cflags": [                 // parameters for c compiler

    ],

    "cxxflags": [               // parameters for c++ compiler

    ],

    "include": [                // directories where the header files are
        
    ],

    "defines": [                // defination for compiler

    ],

    "sources": [                // source files which will be compiled
        "./main.cpp",
        "./func.cpp"
    ],

    "link": [                   // directories storing required libraries

    ],

    "libs": [                   // required libaries

    ]
}
```
