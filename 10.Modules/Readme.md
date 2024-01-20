# Modules
Rust provides a powerful module system that can be used to hierarchically split code in logical units (modules), and manage visibility (public/private) between them.

A module is a collection of items: functions, structs, traits, impl blocks, and even other modules.

## Visibility
By default, the items in a module have private visibility, but this can be overridden with the pub modifier. Only the public items of a module can be accessed from outside the module scope.