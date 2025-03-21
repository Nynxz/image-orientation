# imageorientation

Quickly get orientations of images in a directory!

```usage
Usage: imageorientation [OPTIONS] <PATH>
Arguments:
  <PATH>
Options:
  -m, --mode <MODE>      [default: orientation] [possible values: orientation, resolution]
  -f, --format <FORMAT>  [default: {path}/{entry},{result}]
```

## Example

```example
> imageorientation tests/test
tests/test/600x400.jpg,landscape
tests/test/600x400.png,landscape
tests/test/400x600.png,portrait
tests/test/400x600.jpg,portrait
```

```example
./target/release/imageorientation --mode resolution --format "{entry}/{result}" tests/test
600x400.jpg/600x400
600x400.png/600x400
400x600.png/400x600
400x600.jpg/400x600
```
