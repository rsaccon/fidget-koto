# Koto fidget prelude

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

Returns a tuple with the predefined variables `x`, `y`, and `z`.

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
sphere = (x^2 + y^2 + z^2).sqrt() - 1
draw sphere

# draw a red sphere shape
sphere = (x^2 + y^2 + z^2).sqrt() - 0.5
draw sphere, 1, 0, 0
```

## Tree

The `Tree` type represents the basic type for math expressions which can be built to express any shape.

### Example

```koto
# Create a Tree from an expression
my_tree = x.square() + y.square()

# Use Tree methods
result = my_tree.sqrt()
```

## Tree.min

```kototype
|Tree, other: Tree| -> Tree
```

Returns a tree representing the minimum of two values.

## Tree.max

```kototype
|Tree, other: Tree| -> Tree
```

Returns a tree representing the maximum of two values.

## Tree.compare

```kototype
|Tree, other: Tree| -> Tree
```

Returns a tree comparing two values.

## Tree.atan2

```kototype
|Tree, other: Tree| -> Tree
```

Returns a tree representing atan2(self, other).

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

## Tree.sqrt

```kototype
|Tree| -> Tree
```

Returns the square root of the tree.

## Tree.square

```kototype
|Tree| -> Tree
```

Returns the square of the tree.

## Tree.sin

```kototype
|Tree| -> Tree
```

Returns the sine of the tree.

## Tree.cos

```kototype
|Tree| -> Tree
```

Returns the cosine of the tree.

## Tree.tan

```kototype
|Tree| -> Tree
```

Returns the tangent of the tree.

## Tree.asin

```kototype
|Tree| -> Tree
```

Returns the arcsine of the tree.

## Tree.acos

```kototype
|Tree| -> Tree
```

Returns the arccosine of the tree.

## Tree.atan

```kototype
|Tree| -> Tree
```

Returns the arctangent of the tree.

## Tree.exp

```kototype
|Tree| -> Tree
```

Returns e raised to the power of the tree.

## Tree.ln

```kototype
|Tree| -> Tree
```

Returns the natural logarithm of the tree.

## Tree.not

```kototype
|Tree| -> Tree
```

Returns the logical negation of the tree.

## Tree.ceil

```kototype
|Tree| -> Tree
```

Returns the ceiling of the tree.

## Tree.floor

```kototype
|Tree| -> Tree
```

Returns the floor of the tree.

## Tree.round

```kototype
|Tree| -> Tree
```

Returns the rounded value of the tree.
