**Rust Ownership**
- Rust's ownership system is unique and sets it apart from other programming languages.
- Set of rules that govern memory management
- Rules are enfored at the compile time.
- If any of the rules are violated, the program won't compile.
* Three Rules of Ownership *
- 1. Each value in Rust has an owner
- 2. There can only be one owner at a time
- 3. When the owner goes out of scope, the value will be dropped
**Owner:** The owner of a value is the variable or data structure that holds it and is responsible for allocating and freeing the memory used to store that data
**Scope:** Range withing a program for which an item is valid.
**Global scope:** Accessible throughout the entire program.
**Local scope:**
- Accessible only within particular function or block of code.
- Not accessible outside of that function or block.
**General rule:** Scope ends where block of code ends(curly brackets)

**Memory**
- Components in the computer to store data and instructions for the processor to execute.
- Random Access Memory(RAM) is volatile, when power turned off all content are lost.
- Two types of regions in RAM used by a program at runtime: Stack memory(LIFO) and Heap memory.