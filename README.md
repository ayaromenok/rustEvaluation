# rustEvaluation
Evaluation of [rust](https://www.rust-lang.org) language.

# Results

 - result on 2019.09.30. 
 - Rust itself is OK. 
 - Usage with C++ libs, like Qt - hardly unusable due to complex code for safety (up to 3 time longer and ureadable, than corresponding C++ code). Can be a good idea to make someting similar to Qt\NodeGraph on Vulkan basis, but it'a just a idea 
 - Mobile usage limited to libraries on android.

## Goals
 - evaluate rust language (OK)
 - evaluate usability of rust sub-sustem (build, doc, etc) (sometimes OK)
 - evaluate usage of existing tools with ruust (IDEs, profilers) (sometimes OK)
 - evaluate usage of huge C\C++ libs with rust (Qt, OpenCV) (NOK - code became too compilcated)
 - evaluate platform suppport(Win\Lin\OSX, Androis\iOS) of rust with all above (OK for Desktop, NOK for Mobile, except of libraries)
