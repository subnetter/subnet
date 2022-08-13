# Commands

Commands are actions that require atomic read or write access to shared state. 
They are implemented as CryptoMailService system actor service calls. 
Commands use `features` which are non-thread-safe functions which are designed to be used from thread safe context such as actors or system services.

