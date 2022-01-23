# HW3 For CS410P - Rust
### Personal Info:
Ian Guy
HW3: Keyword Index
Jan 22nd & 23rd, 2022

## Project Goals:
The goal of this project is to implement a struct which enables us to have a Keyword Index.
A Keyword Index takes a string in and slices it up into keywords, allowing for less overall memory to be used in the storage of "owned words". 
What this means is when you pass the index "the quick brown fox" it will store "the","quick","brown","fox".

## Project Approach:
Overall this project had a fwe hiccups along the way but it was nothing which couldn't be sorted out.
Using the provided function declarations and struct at first was confusing, as I wasn't super comfortable using a tuple for these things, but after some reading I was able to get it to work. 
The main two bugs which were an issue for me came from nothing being pushed (printing would give a vector of []), and the nth all caps function being off by one.
These two problems took some time to fix but it was nothing which couldn't be tackled in an afternoon. 
Now, when it comes to formatting of the assignment, the same concept for the first two homeworks applies - this should be another folder in the "Rust Homework" repo I'll be maintaining. 
Lastly, like homework 2, there is no main for this program. This can't be ran from the command line, as it's more a library implementation for use with other programs. 

## Testing Approach:
Like program one, a healthy number of tests were provided. In this case though, they were doctests instead of being a separate testing module. 
Essentially copying a part of the doctests over into a testing folder, some of the same tests were ran. 
The importance of moving these tests out of the library directory is that there will be different methods of access being used. 
For example, self cannot be used in tests.rs as we're not in the right scope. 
To make sure there are no issues with access or ownership, we recreate some of the tests in tests.rs.
The methodology of the tests (using assert_eq!) remains consistent so far. 