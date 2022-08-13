// Copyright (c) 2021, Subnet Authors. cmdev2@proton.me.
// This work is licensed under the Subnet v0.1.0 license published in the LICENSE file of this repo.
//

# Commands

- Commands are actions that require atomic read or write access to shared state. 
- They are implemented as SimpleBlockchainService system actor service calls. 
- Commands use `features` which are non-thread-safe functions which are designed to be used from thread safe context such as actors or system services.

