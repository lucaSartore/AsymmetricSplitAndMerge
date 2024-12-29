# Report
This is the in-detail report of the project "Asymmetric split and merge"

## Objectives

The main objective of this project is to create a fully functional split and merge implementation,
with a small change to the original algorithm. I want to allow for "Asymmetric" splitting,
this means that when an area is split in half the two resulting pieces can be of any size relative to each other
(instead of been forced to be of the same size).

I have also posed to myself a few secondary objective for this project:
 - Make the implementation multithreaded
 - Make the code as reusable as possible 
 - evaluating the [opencv-rust](https://github.com/twistedfall/opencv-rust) crate, that provide rust bindings for open-cv

The choice of rust was made for the multithreading-first approach of the language,
as well as the fact that I had already used opencv a lot in python and c++ and I wanted to try something new
(See [Puzzle solver](https://github.com/lucaSartore/PuzzleSolver) and [RoboCup junior robot](https://github.com/lucaSartore/Robocup-Rescue-Line-simulation) if interested)

It is important to note that an in depth comparison between my version of split and merge (with the asymmetric split)
and the traditional one, is NOT one of the objective of this project. This is because I splitted the work with another
student that will implement the traditional version of split and merge and will provide a comparison.
However I have still provided 2 small examples where the difference between the various algorithms can be observed.

## High level code overview

To maximize cote re-usability I have split the logic into some traits (rust's version of an interface).
In particular I have defined a `Splitter` trait a `Merger` trait for the execution of the project, as well
as a `Logger` trait that is useful to generate animations, debug or to be used inside integration testing.

### Splitter trait

The splitter trait is defined as one single function that take as input a non mutable reference to an image, and perform some calculation to defined if it need to split.

If the result is negative (no splitting is needed) the function shall return `None`

Otherwise the function can return a split direction (x or y axis) as well as in `i32` (the relative coordinate of the split)
```rust
pub trait SplitterTrait: Sync + 'static{
    fn split(&self, image: &Mat) -> Option<(CutDirection, i32)>;
}
```
### Merger trait
The merger trait is even simpler, it takes as input 2 binary masks as well as an image and returns a boolean.

The image must be the original colored image, and the two masks identify the two areas that we want to merge.
The return value is True if the two areas can be merged false otherwise.

```rust
pub trait MergerTrait: Sync + 'static{
    fn merge(&self, mask_a: &Mat, mask_b: &Mat, image: &Mat) -> bool;
}
```


### Logger trait
As the name imply this trait's objective is to log every action. this is what has been used to generate the videos that
will be shown later, but it has also been used for debugging and for automated integration tests
```rust
pub trait LoggerTrait{
    fn log_split(&mut self, area_to_split_id: usize, splits: [Area;2]) -> Result<()>;
    fn log_merge(&mut self, new_item_id: usize, to_merge: [usize;2]) -> Result<()>;
    fn finalize_log(&mut self) -> Result<()>;
}
```

### Main logic
All the logic that is not inside the various trait is encapsulated inside the `MainLogic` struct, this is a generic struct
with 4 generic parameters: `S`, `M`, `L` and `ST`.

Of these the first 3 are implementation of the trait seen above, this allow the main logic to be re-usable with 
different implementations of the splitter trait, fulfilling the objective of code re-utilization.

The fourth generic parameter `ST` is the state the splitter is in (can be Splitting, Merging or Finished)
and is used to implement a `Typestate Pattern`.
```rust
pub struct MainLogic<'a, S: SplitterTrait, M: MergerTrait, L: LoggerTrait, ST: SplitMergeState> {
    splitter: S,
    merger: M,
    logger: L,
    state: ST,
    image: &'a ImageContainer,
    split_tree: Vec<SplitTree<'a>>,
}
```

### Image container split
The last important structure I want to explain is the `ImageContainerSplit` struct, this is used 
to represent a rectangular portion of the original image.
Every time the algorithm perform a split it creates 2 new `ImageContainerSplit`. The underlying
implementation is made in a way that the split operation can be a `O(1)` operation 
(as it reference the original image instead of copying it)
```rust
pub struct ImageContainerSplit<'a> {
    pub image: BoxedRef<'a, Mat>,
    pub x_start: i32,
    pub y_start: i32,
    pub height: i32,
    pub width: i32,
}
```