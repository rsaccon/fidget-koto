# fidget-koto
[Koto](https:://koto.dev) scripting for fidget (as alternative to Rhai)

## Demo
fidget-viewer https://github.com/mkeeter/fidget/tree/main/demos/viewer modified to accept Koto scripts.
```Shell
cargo run --release -p fidget-viewer PATH_TO_YOUR_KOTO_SCRIPT
```

## Differences to fidget rhai scripting:

* no `draw()`: just return shape or list of shapes (last statement in script)
* no `draw_rgb()` yet
* core library currently only provides: `move`, `sphere`, `union`, `intersection`, `inverse` and `differnce`

## Sphere Example
```Koto
# sphere
(x.square() + y.square() + z.square()).sqrt() - 1
```
or
```Koto
from fidget import square, sqrt

# sphere
sqrt(square(x) + square(y) + square(z)) - 1
```
see `models` folder for more examples (all the fidget models have been ported tp Koto)
