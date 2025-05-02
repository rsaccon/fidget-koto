# builtin

Utilities made available at global namespace for working with fidget objects in Koto.

The utilities contain the [`Tree`](#tree) type.

## x

```kototype
Tree
```

 `x` variable.

## y

```kototype
Tree
```

`y` variable.

## z

```kototype
Tree
```

 `z` variable.

## axes

```kototype
|| -> (Tree, Tree, Tree)
```

TODO

### Example

```koto
# TODO
my_x, my_y, my_z = axes
```

## draw

```kototype
|shape: Tree| -> Null
```

TODO

### Example

```koto
# draw a sphere shape
draw (x^2 + y^2 + z^2)).sqrt() - 1
```

## draw_rgb

```kototype
|shape: Tree, r: Number, g: Number, b: Number| -> Null
```

TODO

### Example

```koto
# TODO
```

## Tree

The `Tree` type represents ...

TODO

Comparison operations are available, and the tree's components are iterable. ???

### Example

```koto
# TODO
```

## Tree.abs

```kototype
|Tree| -> ???
```

Returns the ...???

### Example

```koto
# TODO
```
