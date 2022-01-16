# HW2 For CS410P - Rust
### Personal Info:
Ian Guy
HW2: Toy RSA
Jan 14th & 15th, 2022

## Project Goals:
The goal of this project is to implement modules for operating RSA encryption using Rust.
A link to an explanation of the method used can be found here:
https://en.wikipedia.org/wiki/RSA_%28cryptosystem%29

## Project Approach: 
Overall, this project went swimmingly!
Using the provided function declarations, I was able to implement the entire RSA encryption module set on the 14th.
On the following day, testing was implemented, and this is where most of the (overall few) difficulties came from.
With the experience from the 1st homework, getting GitHub and rust-analyzer working was much less difficult this time around.
As for methodology, this assignment follows the same as last time. This is just another folder in the overall "Rust Homework" repo I'll be maintaining for this term.
Now, when it comes to actual programming, this assignment doesn't actually have a main and can't be ran from the command line. It's more a setup for creating our own tests. 

## Testing Approach: 
For this program, only one test case was provided - and we were required to create our own other tests.
To do this, I (of course) start by using the provided test case and values to make sure everything is ok.
Next, with the nature of how RSA encryption works, we need to make sure the encrypted message matches the decrypted message when the decrypted message is either 0 or 1.
Lastly, a loop was created to randomly generate an arbitrary number of other cases to make sure that the key generator works properly. I chose 5000 cases and on my system running this test takes about 30-40 seconds. 