# POS (Parts of Speech)

A very ambitious and _very_ bad programming language.

## Vision

TODO: I think it should be dynamically typed for my own sanity.
JS-style objects which are HashMaps and repurpose `noun` for var-decls?

Object/typedef is a `noun`, instances of the object are `properNoun`s. This
means capitalization conventions are switched? Instead of dot notation to
access field use `'s ` notation. Much better
```
noun city has
    string name,
    int population

city LosAngeles.
LosAngeles's name = "Los Angeles".
LosAngeles's population = 100.
```
Functions are `verbs`. Comments start with `--`
```
-- TODO: can't decide about function headers
verb fibbonacci turns int n into int
by
    if n == 0 || n == 1 then
        n
    else
        fibbonacci (n-1) + fibbonacci (n-2)
    end
end

-- Parens around args are optional for 1, required for 0 or >1
verb add turns (int a, int b) into int by
    a + b
end
```
Rust-style traits are `adjectives`. Periods instead of semicolons.
If dynamically typed, non of these?
```
adjective cloneable can
    -- `Self` implied like Java
    verb clone turns () into self

city is cloneable because
    verb clone turns () into self
    by
        city NewCity.
        -- `my` is syntactic sugar for `Self's`
        NewCity's name = my name.
        NewCity's population = my population.
        NewCity
    end
```
TODO: adverbs

`it` register? Most recently modified variable?
```
city Atlantis.
-- `its` syntactic sugar for `Atlantis's`
-- If you try to use `it's` here, the compiler will smite you
its name = "Atlantis".
-- `it` syntactic sugar for `Atlantis`
print (its name). -- -> Atlantis

int X = 5.
print it. -- -> 5
```
Lists can be made by pluralizing. :( can't do this if dynamically typed.
```
verb max turns cities List into city
by
    -- Functions called without parentheses
    if length List == 1 then
        -- indexing
        List's 0
    else
        int TailMax = max (List's 1..).
        if List's 0 > it then
            List's 0
        else
            -- TailMax
            it
        end
    end
end
```
