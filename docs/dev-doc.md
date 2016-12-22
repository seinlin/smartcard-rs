<!--
@Author: ronan
@Date:   22-12-2016
@Email:  ronan.lashermes@inria.fr
@Last modified by:   ronan
@Last modified time: 22-12-2016
-->



# Dev doc

This doc describes briefly the rough structure of the code.

## The wrapper

The c bindings have been created with [bindgen](https://github.com/Yamakaky/rust-bindgen) on the files in the *others* folder:
```
bindgen winscard.h --convert-macros
```
The result have been edited a bit manually.

The resulting file is *scard/winscard.rs*.

## The safe API

To hide all unsafe calls, a safe API is built on top of the bindings.

### Parameters

In this folder are stored all enums used as parameters in the logic.
Enums prevent forbidden values to be used by the API user.

### Logic

Where things actually happen.
- The context grabs a handle on the driver.
- The reader represents a reader plugged on the PC.
- The card represents a smartcard inserted in a reader.

Some utility functions are gathered in *utils.rs*.
