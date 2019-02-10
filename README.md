# jvm-class-file-parser [![Travis CI Status](https://api.travis-ci.org/ExcaliburZero/jvm-class-file-parser.svg)](https://travis-ci.org/ExcaliburZero/jvm-class-file-parser) [![Coverage Status](https://coveralls.io/repos/github/ExcaliburZero/jvm-class-file-parser/badge.svg?branch=master)](https://coveralls.io/github/ExcaliburZero/jvm-class-file-parser?branch=master) [![Library documentation](https://img.shields.io/readthedocs/pip.svg)](https://excaliburzero.github.io/jvm-class-file-parser/master/jvm_class_file_parser/index.html)
This is a (partially implemented) Rust library and program for parsing JVM class files.

```
$ cargo +nightly run classes/Dummy.class
Classfile /home/chris/Code/jvm-class-file-parser/classes/Dummy.class
  Compiled from: "Dummy.java"
class Dummy
  minor version: 0
  major version: 52
  flags: ACC_PUBLIC, ACC_SUPER
Constant pool:
   #1 = Methodref           #3.#10          // java/lang/Object."<init>":()V
   #2 = Class               #11             // Dummy
   #3 = Class               #12             // java/lang/Object
   #4 = Utf8                <init>
   #5 = Utf8                ()V
   #6 = Utf8                Code
   #7 = Utf8                LineNumberTable
   #8 = Utf8                SourceFile
   #9 = Utf8                Dummy.java
  #10 = NameAndType         #4:#5           // "<init>":()V
  #11 = Utf8                Dummy
  #12 = Utf8                java/lang/Object
{
  Dummy();
    descriptor: ()V
    flags: TODO
    Code:
      stack=1, locals=1, args_size=TODO
          0: aload_0                            
          1: invokespecial #1                   
          4: return                             
}
SourceFile: "Dummy.java"
```

```
extern crate jvm_class_file_parser;

use std::fs::File;
use jvm_class_file_parser::ClassFile;

let mut file = File::open("classes/Dummy.class").unwrap();
let class_file = ClassFile::from_file(&mut file).unwrap();

assert_eq!("Dummy", class_file.get_class_name());
```
