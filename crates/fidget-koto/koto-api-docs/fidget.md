# fidget

Utilities for working with [fidget](https://github.com/mkeeter/fidget) data structures in Koto.

The utilities contain the [`Tree`](#tree) type, which are bindings to the equally named type in fidget.

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

## Tree.min

```kototype
|Tree, other: Tree| -> Tree
```

Returns a tree representing the minimum between itself and the other tree.

### Example

```koto
tree = x.min(y)
```

## Tree.max

```kototype
|Tree, other: Tree| -> Tree
```

Returns a tree representing the maximum between itself and the other tree.

### Example

```koto
tree = x.max(y)
```

## Tree.compare

```kototype
|Tree, other: Tree| -> Tree
```

Returns a tree representing the result of the compare function between itself and the other tree.

### Example

```koto
tree = x.compare(y)
```

## Tree.and

```kototype
|Tree, other: Tree| -> Tree
```

Returns a tree representing the result of the and function between itself and the other tree.

### Example

```koto
tree = x.and(y)
```

## Tree.or

```kototype
|Tree, other: Tree| -> Tree
```

Returns a tree representing the result of the or function between itself and the other tree.

### Example

```koto
tree = x.or(y)
```

## Tree.atan2

```kototype
|Tree, other: Tree| -> Tree
```

Returns a tree representing the result of the atan2 function between itself and the other tree.

### Example

```koto
tree = x.atan2(y)
```

## Tree.abs

```kototype
|Tree| -> Tree
```

Returns a tree representing the absolute value.

### Example

```koto
new_tree = x.abs()
```

## Tree.sqrt

```kototype
|Tree| -> Tree
```

Returns a tree representing the result of the square root function.

### Example

```koto
new_tree = x.sqrt()
```

## Tree.square

```kototype
|Tree| -> Tree
```

Returns a tree representing the result of the square (or power by 2) function.

### Example

```koto
new_tree = x.square()
```

## Tree.sin

```kototype
|Tree| -> Tree
```

Returns a tree representing the result of the sine function.

### Example

```koto
new_tree = x.sin()
```

## Tree.cos

```kototype
|Tree| -> Tree
```

Returns a tree representing the result of the cosine function.

### Example

```koto
new_tree = x.cos()
```

## Tree.tan

```kototype
|Tree| -> Tree
```

Returns a tree representing the result of the tangent function.

### Example

```koto
new_tree = x.tan()
```

## Tree.asin

```kototype
|Tree| -> Tree
```

Returns a tree representing the result of the arcsine function.

### Example

```koto
new_tree = x.asin()
```

## Tree.acos

```kototype
|Tree| -> Tree
```

Returns a tree representing the result of arccosine function.

### Example

```koto
new_tree = x.acos()
```

## Tree.atan

```kototype
|Tree| -> Tree
```

Returns a tree representing the result of the arctangent function.

### Example

```koto
new_tree = x.atan()
```

## Tree.ln

```kototype
|Tree| -> Tree
```

Returns a tree representing the result of the natural logarithm function.

### Example

```koto
new_tree = x.ln()
```

## Tree.not

```kototype
|Tree| -> Tree
```

Returns a tree representing the result of the negation function.

### Example

```koto
new_tree = x.not()
```

## Tree.ceil

```kototype
|Tree| -> Tree
```

Returns a tree representing the result of rounding up to the nearest integer.

### Example

```koto
new_tree = x.ceil()
```

## Tree.floor

```kototype
|Tree| -> Tree
```

Returns a tree representing the result of rounding down to the nearest integer.

### Example

```koto
new_tree = x.floor()
```

## Tree.round

```kototype
|Tree| -> Tree
```

Returns a tree representing the result of rounding to the nearest integer.

### Example

```koto
new_tree = x.round()
```

<!--
## Tree.min
## Tree.max
## Tree.compare
## Tree.and
## Tree.or
## Tree.atan2
## Tree.abs
## Tree.sqrt
## Tree.square
## Tree.sin
## Tree.cos
## Tree.tan
## Tree.asin
## Tree.acos
## Tree.atan
## Tree.exp
## Tree.ln
## Tree.not
## Tree.ceil
## Tree.floor
## Tree.round
-->
