
# Rust Interpreter

Rust Interpreter following the [Crafting Interpreters Series](https://craftinginterpreters.com/parsing-expressions.html).

# Language Overview

````haskell
-- iam a comment

set x = 10;
set myVariable = 100;

echo "Hello, World!";
echo x + 20 * myVariable / 2; -- 1010

-- mutability and type inferring
set inferredImmutable = 10;
set mut inferredMutable = 10;
i32 typedImmutable = 10;
i32 mut typedMutable = 10;
````
