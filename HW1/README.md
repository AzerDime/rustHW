# HW1 For CS410P - Rust
### Personal Info:
Ian Guy
HW1: Modular Exponentiation
Jan 4th, 2022
    
## Project Goals: 
The goal of this project is to implement modular exponentiation using Rust.
A link to an explanation of the algorithm used can be found here: 
https://en.wikipedia.org/wiki/Modular_exponentiation#Right-to-left_binary_method

## Project Approach:
Overall, I think this project went rather well. 
Starting with taking what I could from the provided info, there wasn't too much which needed to be solved. 
The only issues along the way were with new (to me) syntax and some quirks with getting GitHub / rust-analyzer to notice this file.
My methodology for this class is to have a single repository with an individual folder for each homework assignment. 
When it comes to the methodology of actually programming, I wrote things a bit less "object oriented" as I would have liked, as it made testing more difficult later.

## Testing Approach:
For this program, I used the test module provided with the homework - but getting it to work with my program was rather difficult (in a good way)! 
Not only were some bugs found, a noticable increase in modularity was achieved (the modexp function was in main at first) allowing for the unit testing to actually be done. 
