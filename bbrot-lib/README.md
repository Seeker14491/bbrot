### Todo

* Be able to run indefinitely; stop when a signal is sent
* Resumable rendering
* Support custom coloring
* Use symmetry option
* Proper benchmarking setup

### Arbitrary notes related to this library

Good Mandelbrot set bounds for an overview of the whole set:

```
x: -2.0 to 0.6
y: -1.3 to 1.3
```

#### Optimization for massive decrease in allocations and bounded memory use

##### Naive iteration method

A random point is selected, and then iterated through the complex function. At each step in the iteration, the current location of the point is recorded into a vec. If a cycle is found during the iteration, the point is discarded. If the point escapes to infinity, all the positions stored in the vec are written over to the bucket field.

There are several problems with this approach. For maximum performance, allocating memory should be kept to a minimum, yet a new vec is allocated for each point. Additionally, since it is not known how large the vec will need to be, the vec will usually reallocate several times as it grows. Worst of all, if there is no iteration limit, the size of a vec can grow without bound, exhausting all RAM.

##### Much better method

We instead use an alternative implementation. In this implementation the visited locations are not stored. Instead, if the point escapes, then the initial point is reiterated, and each location written directly to the bucket field.

This implementation uses a constant, tiny amount of memory, and performs zero allocations.

#### Thoughts on handling overflowing buckets

Currently, each bucket is represented as a u32. There is no overflow checking. For "normal" workloads, overflowing isn't thought to be likely, but it becomes more and more likely to happen as more points are iterated, and/or a smaller bucket field is used. I first consider the obvious change: use u64s instead, but this would double the size of the bucket field. This seems like a large waste of memory, protecting against something that's unlikely to happen under normal circumstances.
