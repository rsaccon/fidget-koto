# builtin

Utilities made available at global namespace for working with fidget objects in Koto.

The utilities contain the [`Tree`](#x-1), [`Tree`](#y-1), [`Tree`](#z-1), [`Tree`](#axes-1), [`Null`](#draw-1) and [`Null`](#draw_rgb-1) types.

## x

```kototype
|| -> Tree
```

## y

```kototype
|| -> Tree
```

## z

```kototype
|| -> Tree
```

## axes

```kototype
|| -> (Tree, Tree, Tree)
```

## draw

```kototype
|shape: Tree| -> Null
```

Initializes a default `Rect` with each component set to `0`.

```kototype
|x: Number, y: Number, width: Number, height: Number| -> Rect
|xy: Vec2, size: Vec2| -> Rect
```

Initializes a `Rect` with corresponding position and size.

### Example

```koto
# draw a sphare shape
draw (x^2 + y^2 + z^2)).sqrt() - 1
```

## draw_rgb

```kototype
|shape: Tree, r: Number, g: Number, b: Number| -> Null
```
