# builtin

Utilities made available at global namespace for working with [fidget](https://github.com/mkeeter/fidget) data structures in Koto.

The utilities contain the [`Tree`](#tree) type, which are binding to the equally named type in fidget.

## x

```kototype
|| -> Tree
```

Returns the predefined variable `x`.

## y

```kototype
|| -> Tree
```

Returns the predefined variable  `y`.

## z

```kototype
|| -> Tree
```

Returns the predefined variable `z`.

## axes

```kototype
|| -> (Tree, Tree, Tree)
```

Returns a tuple with the predefined variable `z`, 'y' and 'z'.

### Example

```koto
ax, ay, az = axes()
```

## draw

```kototype
|shape: Tree| -> Null
|shape: Tree, r: Number, g: Number, b: Number| -> Null
```

Inserts a shape into the evaluation and rendering pipeline. Optionally a color can be  set by defining values in the range from `0.0` to `1.0` for the `r`, `g` and `b` arguments.

### Example

```koto
# draw a sphere shape
sphere = (x^2 + y^2 + z^2)).sqrt() - 1
draw sphere

# draw a red sphere shape
sphere = (x^2 + y^2 + z^2)).sqrt() - 0.5
draw sphere, 1, 0, 0
```

## Tree

The `Tree` type represents the basic type for math expressions which can be built to express any shape.

### Example

```koto
# TODO
```

## Tree.abs

```kototype
|Tree| -> Tree
```

Returns a tree representing the absolute value.

### Example

```koto
new_tree = x.abs()

# or use the abs() helper function

from fidget import abs
new_tree = abs x
```
