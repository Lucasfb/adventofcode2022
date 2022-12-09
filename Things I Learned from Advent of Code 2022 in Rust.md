## Day 1
* Implement ordering traits
* Apply function (parsing) to all elements of vector
* (Remind) Parse string to number
* (Remind) Access elements of vector from the end instead of beginning
* (fasterthanli.me) fs-err and colour-eyre crates
* (nightly) take() method for vectors
## Day 2
* A string with a single character gets converted to 'char' instead of '&str' with split()
* Removing elements from vector gives an Option that needs to be handled
* Implementing try_from() and from_str(), to parse structs we create
* The matches! macro
* Collecting a vector as a trick to avoid dealing with Result
## Day 3
* Double check if you really know the alphabet
* Filling lots of data manually is error prone
## Day 4
* Reusing all stuff from previous days
* Sometimes brute forcing still works, but the result is hard to read
## Day 5
* VecDeque might not be necessary when using pop() and push(), but it can make intention clearer
* Parsing columns instead of lines can be hard manually
* Rust does not like to use indices
* Starting to understand when to use map
    - Apply a series of procedures to each element of an iterator
## Day 6
* Working with indexes is annoying and not a good practice (again)
* Dedup on Vecs only works on sequential elements
* Chars cannot be sorted normally
* (fasterthanli.me) itertools::tuple_windows(),::counts() and ::unique()
* (fasterthanli.me) Iterator::all(), ::position()
* (fasterthanli.me) Slice::windows()