# Report
This is the in-detail report of the project "Asymmetric split and merge" by Luca Sartore

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

## Implementations

### Splitter Trait Implementation

In total I have created 5 different splitter implementations. They are listed below followed by a brief description
- `BlindSplitter`

    this is a simple splitter that always split an image (up to a certain size).
    It is not a useful component by it-self however it is encapsulated into the more advanced components logic.

    It works as follow:
    - compare the height and width of an area to a pre-defined threshold
    - if they are both lower than it doesn't split
    - otherwise it split the longer direction in half
- `HueStdSplitter`

    This is a splitter that is primarily based on the color of an image 
    (infact it uses the Hue component of the HSV color space)

    It works as follow:
    - convert the image into the HSV color space
    - Calculate the standard deviation of the hue component
    - if the standard deviation is lower than a pre-defined threshold it doesn't split
    - otherwise it split using the same logic as a `BlindSplitter`
- `StdSplitter`
    
    This splitter works well with homogenous colors 
    (as it try to minimize the standard deviation inside every area)

    It works as follow:
    - Calculate the standard deviation of the 3 components of an image (R,G,B)
    - calculate the absolute value of the standard deviation: `sqrt(std_r^2 + std_g^2 + std_b^2)`
    - if the standard deviation is lower than a pre-defined threshold it doesn't split
    - otherwise it split using the same logic as a `BlindSplitter`
- `MaxDeltaSplitter`

    This splitter is also primary based on the color, however it works a bit better than `HueStdSplitter` as if the color is really close to white or black the Hue component of the HSV color space become really "usable".
    It works by ensuring that inside an area the distance that every pixels's color has to the mean color is lower than a certain threshold. A gaussian blur is also applied at the beginning to ensure that high frequency disturbs (like for example a single black pixel in a red area) don't singlehandedly force a split.

    It works as follow:
    - Apply a gaussian blur to the image
    - Calculate the average color
    - Calculate the max delta: 
    $$
    \text{MaxDelta} \coloneqq \max\left\{ \left|| \text{Image}[x, y] - \text{AverageColor} \right|| \;\big|\; x, y \in \text{Image} \right\}
    $$
    - if the max delta is lower than a pre-defined threshold it doesn't split
    - otherwise it split using the same logic as a `BlindSplitter`

- `HeuristicAsymmetricSplitter`

    This splitter try to split an image in the optimal point, and to do so it uses a simple heuristic based on the partial derivative of the image. This splitter doesn't have an internal way of deciding wether to split or not,
    it instead uses the logic from another splitter of choice (like for example `MaxDeltaSplitter`)

    It works as follow:

    - calculate the partial derivate for x or y axis
    - reduce the image into a single row or column    
    - chose to split in the point where the first derivate is maximized
    ![image](./report_images/heuristic_split.png)

### Merger Trait Implementation
In total I have created 3 different merger implementations. They are listed below followed by a brief description
- `BlindMerger`

    A simple merger that always decide to merge (used only for testing)
- `StdMerger`

    This merger works well with homogenous colors 
    (as it try to minimize the standard deviation inside every area)

    It works as follow:
     - Calculate what the resulting image would look like if the merge went through
     - Calculate the standard deviation of the area
     - merge if the standard deviation is lower than a threshold
- `ColorBasedMerger`

    A merger that take into consideration the difference in color as well as the
    differences in the standard deviation fo the two areas.

    It works as follow:
     - Calculate the average color of the image 1 and 2
     - Calculate de distance between the two colors
     - Calculate the standard deviation for every channel (R G B) of the image 1 and 2
     - Calculate the distance between the two standard deviations
     - Merge only if both the color distance and the std distance are lower than their respective thresholds.

    The standard deviation is used so that the program is (sometimes) able to distinguish between areas
    that have the same color but different textures

## Results

As mention in the beginning an in-depth evaluation of the performances of the algorithm, as well as a comparison with a "traditional" symmetric version of the split and merge is NOT one of the objectives of this project. However I will still provide some examples.

### Real world test case

The published [video](https://www.youtube.com/watch?v=ElU1I7_PCIQ) show the split and merge algorithm at work with a real world image,
we can se that the final result is overall quite good, with only some minor over-segmentation in the border of the image

It is also interesting to analyze the first few splits executed by the algorithm.
The thing wart noting here is how well the first derivative heuristic is working in the determination of the splitting coordinates 

![image](./report_images/stuff_first_few_splits.png)


### Synthetic test cases

I have also designed 4 synthetic testcases that are designed to show the potential (as well as the weaknesses) 
of the asymmetric splitter.

The 3 test cases are:
 - image with squares, symmetric splitter: [video](https://youtu.be/ydyZKXghq9k)
 - image with squares, asymmetric splitter: [video](https://youtu.be/ydyZKXghq9k?t=51)
 - image with circles, symmetric splitter: [video](https://youtu.be/ydyZKXghq9k?t=61)
 - image with circles, asymmetric splitter: [video](https://youtu.be/ydyZKXghq9k?t=106)

from the video we can see that somehow intuitively the asymmetric split heuristic works better if the underlying image is mostly made of squared objects.

### Performance evaluation

The performance of the algorithm are not amazing. with the real world test case tacking a few minutes
to be executed.

This was partially expected due to the complexity of the split and merge algorithm, and partially can be explained
by the lack of optimization due to the generic structure of the algorithm.

If we only want to implement one merge strategy and one split strategy many more optimization could be made.
For example the average color of an area that has just been merged could be calculated in O(1) time if we knew
the  average color of the two merged areas as well as the relative pixel count.

However if we want to keep a "clean" structure that allow our code to be re-used for every possible merge strategy
this optimization is not possible, and we have to re-calculate the average color every time with a cost of O(n).

Another factor that influence the performance is how often some items are tested to be merged. For example assume we have 3 areas all connected to each others named `A`, `B` and `C`. We then take the following steps:
 - Try to merge `A` and `B` but they are not compatible
 - Try to merge `B` and `C` and they are compatible, so we merge
 - now we haven't tested the compatibility between `C` and `A` should we do it?
    - Option one is to NOT do it, since now `C` is merged with `B` and we already know that `A` and `B` are not compatible
    - Option two is to do it, since in general we can't know if the fact that we have merge `B` and `C` can change the decision of the merge algorithm

Ultimately I decided for option two, in order to keep my algorithm the most re-usable.
If I were using only one merge heuristic I could know if the fact that `B` and `C` has been merged can 
change the merge decision, but since the code is designed for maximum re-usability I can't make this optimization.

## Multi threading
The multi threaded architecture can be divided into two separated items: The main thread and the Worker treads.

The main thread is the one that handle most of the logic, it contains the split tree as well as the adjacency graph.
The fact that is a single thread make it possible to not protect with mutexes all the aforementioned structures making them faster.

The worker treas are much simpler and only execute split/merge requests that are sent in by the main tread.

### Scaling

the architecture uses two channels (one to send data and one to get results from the worker treads) and only tow mutexes
(one for the worker tread's receiver and one for the sender).
This make the architecture lightweight and make it scale decently well as seen in the table below.

| Workers | Time    |
|---------|---------|
| 1       | 5m01s   |
| 2       | 3m30s   |
| 4       | 1m51s   |

The main scaling issue that this architecture may have is that the main thread could become the bottleneck if the number of worker threads become really high, however this is not a problem with consumer hardware.

### Main challenge with multithreading
I wanted to also report one particular challenge that the multithreading architecture has caused, since I found it interesting

Assume we have 3 areas: `A`, `B` and `C` all connected to each others,
and assume I execute some merges with the following order
 - `A` with `B` return `true`
 - `B` with `C` return `true`
 - `C` with `A` return `true`

You may have noted that the third merge was a wasted calculation.
In fact if `A` and `B` are merged and `B` and `C` are merged, then `A` and `C` are already part of the same area and don't need to be tested for merging.

If the program is not using multi threading I can be sure that the result of one merge will be ready before I execute the next one,
therefore I can skip the last step saving some time.

However using multi threading means that I will need to execute multiple merge request at the same time,
and if I sent the 3 requests at te same time I can't  take advantage of this specific optimization.

The solution to this problem has been to send split requests in batches, and to make sure that in each batch the graph of the
split requests is acyclic. In this way I can be sure that no wasted merge is executed.

## Personal opinions on opencv-rust