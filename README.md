# cxon -- C++ builder configured with JSON

> [!IMPORTANT]
>
> - cxon is still in development, and the configrantion field is not stable yet.

## Installation

- ### Build and insall from cargo
```sh
cargo install cxon
```

- ### [Install directly from github release](https://github.com/CoraBlack/cxon/releases)

## Features(Order by priority)

- [x] Build project with compiler and linker by cxon.json immediately.(unsupport custom)
- [x] Build cache
- [x] Mult-thread build.
- [x] Muti-build-target.
- [ ] cxon.json schema support 
- [ ] Debug field support(Only debug currently).
- [ ] Submodule support.
- [ ] Multiple compile targets.
- [ ] Platform-specific configuration.

## What is the goal for cxon?

We want to provide a build system that is easy to use and config for C++ beginners or who are not the programming prefessors, which can help them focus on their project and code instrendad of dealing with the build system configration.  
We only require a small number of essential fields for cxon.json and make the build system configration as simple as possible.

## cxon.json Example
```json5
{
    "project": "HelloWorld",    // (Required) project name
    "target_name": "hello",     // the final compiled product name, the default value is the project field
    "target_type": "execuable", // (Required) build type (execuable, static_lib, shared_lib, object_lib)
    "build_dir": "build",       // the directory storing intermediate compiled product
    "output_dir": "bin",        // the directory storing final compiled product

    "toolchain": "gnu",         // gnu, llvm, msvc only currently
    "cc": "",                   // (unsupport) custom c compiler
    "cxx": "",                  // (unsupport) custom c++ compiler

    "threads": 4,               // count of build threads, the default value is number of your cpu - 1

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

    "sources": [                // (Required) source files which will be compiled
        "./main.cpp",
        "./func.cpp"
    ],

    "link": [                   // directories storing required libraries

    ],

    "libs": [                   // required libaries

    ]
}
```
