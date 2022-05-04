# ErgoMap

**ErgoMap** is simple data structure library for wrapping the std `HashMap` in a way that makes code easier to write and restricts key creation to reduce the amount of invalid `get` calls. All `ErgoMap` objects use the `Id` type as a key. `Id` contains the corresponding value type as a generic parameter and has no public constructor. THis means that `Id` must be obtained through `ErgoMap` method calls like `insert`.

For more flexibility, values can be inserted into `ErgoMap` using a specified `Vec<u8>` as a key. The provided `Vec` is converted into and returned as an `Id`. User types which implement the `BuildId` trait can instead provide a `Vec<u8>` for `Id` creation themselves.

 It also implements some methods not found on the std `HashMap` for functional programming and chaining method calls.

 # License
 ErgoMap is licensed under the [MIT license.](https://choosealicense.com/licenses/mit/)
 
