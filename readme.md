# Split and Merge

This repository contain a Rust-based multithreaded implementation of the [split and merge](https://en.wikipedia.org/wiki/Split_and_merge_segmentation) segmentation algorithm.

The algorithm is designed using a generic structure, therefore all the multithreading logic can be easily reused with different Merging/Splitting factions

## Demo video
[![IMAGE ALT TEXT HERE](https://img.youtube.com/vi/ElU1I7_PCIQ/0.jpg)](https://www.youtube.com/watch?v=ElU1I7_PCIQ)

## Asymmetric split heuristic

This repository has also tested an idea where the splitting of the images can be "asymmetric" and where the splitting longitude/latitude is calculated using a greedy procedure involving the partial derivative of x and y axis.
The final results are that even if this heuristic can improve performance on some specific cases (especially the one where the image we are splitting contains a lot of squared objects) in most cases the heuristic sow the program down, and therefore is not really worth implementing

## Installation
This project require opencv to be installed and correctly linked to the rust library. To do so I recommend following the [instructions](https://github.com/twistedfall/opencv-rust/blob/master/INSTALL.md) in the official [opencv-rust](https://github.com/twistedfall/opencv-rust) repository

All the other dependencies can be install simply trough cargo