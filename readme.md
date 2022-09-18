# Random notes


There are three types of Python source input -

    1. From file
    2. From command line (ex cli -d "print('Hello World')")
    3. Interactive

## architecture choices

1. At the top of the lexer/processor object-graph needs to be a String that can then be
referenced multiple times by lower tier strs.

2. Fuck it, everything is a String, just dump everything on the heap.


## Architecture decision

The battle with &str has been seriously discouraging so I am going with the fuck it plan 
and everything being a String (heap).

