# fidget-koto
[Koto](https:://koto.dev) scripting for fidget (as alternative to Rhai)

## Demo
fidget-viewer https://github.com/mkeeter/fidget/tree/main/demos/viewer modified to accept Koto scripts.
```Shell
cargo run --release -p fidget-viewer PATH_TO_YOUR_KOTO_SCRIPT_MODEL
```
The `models` folder has some examples (all Rhai fidget models have been ported to Koto and some new ones added). The fidget-viewer can currently only watch one Koto file, therefore any attempt to import from a differnt module represented by a differnt Koto file in the `models` folder will fail.

## Differences to fidget Rhai scripting:
* core library currently only provides: `move`, `sphere`, `union`, `intersection`, `inverse` and `differnce`
* no `draw_rgb`, just use `draw` with optionally adding the color arguments `r`, `g` and `b`.
* Engine initialization
  * Default: `Engine::default()`
  * Custom: `Engine::new(settings: EngineSettings)`. The following options are available:
    * `add_fidget_fns: bool` for making all `fidget` helper functions avaliable at top level
    * `execution_limit: Duration` for scripting execution time limit

## Sphere Example
Either use the built-in implementation from the core library:
```koto
# sphere with radius=1.0, x=0.0 (default), y=0.0 (default), z=0.0 (default)
draw sphere 1
```
or build it from scratch by using fidget `Tree` operations as with Rhai:
```koto
draw (x.square() + y.square() + z.square()).sqrt() - 1
```
or with operators where possible:
```koto
draw (x^2 + y^2 + z^2)).sqrt() - 1
```
or from scratch, but with importing fidget `Tree` operations into global namespace:
```koto
from fidget import square, sqrt

draw sqrt(square(x) + square(y) + square(z)) - 1
```
