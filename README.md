# fidget-koto
[Koto](https:://koto.dev) scripting for fidget (as alternative to Rhai)

## Demo
fidget-viewer https://github.com/mkeeter/fidget/tree/main/demos/viewer modified to accept Koto scripts.
```Shell
cargo run --release -p fidget-viewer PATH_TO_YOUR_KOTO_SCRIPT_MODEL
```
The `models` folder has some examples (all Rhai fidget models have been ported to Koto and some new ones added).

## Differences to fidget Rhai scripting:
* `draw_rgb()` has not been implemented yet
* core library currently only provides: `move`, `sphere`, `union`, `intersection`, `inverse` and `differnce`
* Default engine initialization is same as with Rhai: `Engine::default()`, but if we want to set
a scripting executuion time limit, the initialisation is: `Engine::new(execution_limit: Duration)`

## Sphere Example
Either use the built-in implementation from the core library:
```Koto
# sphere with radius=1.0, x=0.0 (default), y=0.0 (default), z=0.0 (default)
draw sphere 1
```
or build it from scratch by using fidget `Tree` operations in similar as with Rhai:
```Koto
# sphere
draw (x.square() + y.square() + z.square()).sqrt() - 1
```
or from scratch, but with importing fidget `Tree` operations into global namespace:
```Koto
from fidget import square, sqrt

# sphere
draw sqrt(square(x) + square(y) + square(z)) - 1
```
