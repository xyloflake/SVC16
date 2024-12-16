#import "@preview/colorful-boxes:1.4.0":slanted-colorbox
#set page(
paper:"a4",
background: rotate(24deg,
  text(48pt, fill: rgb("FFCBC4"))[
    *WORK IN PROGRESS*
  ]
))

#let not-specified(txt) = slanted-colorbox(
  title: "Not specified",
  color: "gray",
)[#txt]


= SVC16: A Simple Virtual Computer

== Motivation and Goals

We want to fully specify a very simple virtual computer that can be emulated.
The goal is to recreate the feeling of writing games for a system with very tight hardware constraints without having to deal with the complicated reality of real retro systems. 
It should be simple to understand every instruction, to write machine code that runs on it, and to write a compiler for it.
The instruction set and the design in general are in no way meant to resemble something that would make sense in real hardware.
It is also not intended to be as simple and elegant as it could possibly be. This might make it easier to explain but harder to develop for.
Since learning about assemblers and compilers is the point, we provide no guidelines on how to build the programs.

=== Reproducibility

The biggest secondary goal is to design a system that behaves the same everywhere.
The question of how the emulation is run should never matter for the person writing the program or game.
This means there can be no features that might only available in one implementation. 
It also means, that the performance characteristics must be the same.
An emulator can either run the system at the intended speed, or it can not. 


== General Principles

Every value is represented as a (little-endian) unsigned 16-bit integer.
That includes numbers, addresses, colors, the instruction pointer and the input.
Booleans are represented as `u16` values as well: 0 for `false` and >0 for `true`. 
Whenever an instruction writes out a boolean explicitly, it is guaranteed to represented as the number 1.

All numerical operations that will appear in the instructions are wrapping operations. 
This includes manipulations of the instruction pointer.

Division by zero crashes the program.

== The Simulated System
#figure(
  image("sketch.svg", width: 65%),
  caption: [A sketch of all components of the virtual computer.],
) <sketch>

As seen in @sketch, the addressable memory contains one value for each address.
There is a separate screen buffer of the same size as the main memory.
The screen itself has a fixed resolution ($256 times 256$). 
The instruction pointer is stored separately. 
It always starts at zero. 

=== Screen and Colors 
The color of each pixel is represented with 16-bits using `RGB565`.
The coordinate $(x,y)$ of the screen maps to the index $256 y + x$ in the screen buffer.
The coordinate $(0,0)$ is in the upper left-hand corner. Changes to the screen-buffer are not reflected on the screen until the system is synchronized.

#not-specified[
- Colors do not have to be represented accurately (accessability options).
- There is no rule for how scaling and aspect ratio might be handled.
- It is not fixed what the screen shows before it is first synchronized.
- A cursor can be shown on the window, as long as its position matches the mouse position passed to the system]

=== Input
The only supported inputs are the mouse position and the left and right mouse keys.
This is because it makes it much easier to emulate the system consistently.

The input the system sees is only allowed to change at synchronization.


The input is represented with two values: the *position code* and the *key code*.
The *position code* is the index of the pixel, the mouse is currently on. The *key code* is given by left_mouse+2*right_mouse. So it can have the values 0 1 2 or 3.

Before the first synchronization, both input codes are zero.


=== Synchronization
When the console executes the *Sync* instruction, the screen buffer is drawn to the screen.
It is not cleared. The system will be put to sleep until the beginning of the next frame. 
The targeted timing is 30fps. There is a hard limit of 3000000 instructions per frame. 
This means that if the Sync command has not been called for 3000000 instructions, it will be performed automatically.
This can mean that an event (like a mouse click) is never handled.

== Instruction Set
