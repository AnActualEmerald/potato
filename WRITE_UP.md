Computers are complicated. Looking from the outside they can appear to
be magic boxes that simply perform whatever task is asked of them. Of
course, the reality is quite a bit more complicated. There are many
components to the modern computer, but the beating heart at the center
of most computers is the CPU, or central processing unit. The CPU itself
is an extremely complex piece of technology, with modern CPUs having
transistor counts easily in the 10s of *billions* (Porter 2020).
Fortunately, the basic principle of a CPU’s operation does not change,
no matter how big or small it gets. To explore how a processor
functions, I wrote a small CHIP-8 virtual machine using the Rust
programming language that I call
[Potato](https://github.com/AnActualEmerald/potato) (as in potato chip).
Here I will explain to the best of my ability what CHIP-8 is and how it
works, and how it relates to the operation of a physical CPU.

CHIP-8 is an interpreted programming system that runs in it’s own
virtual machine (VM), rather than directly on a computer’s CPU
(Weisbecker 1978, 108). It was originally described in the mid 1970’s by
Joseph Weisbecker, as an alternative to more resource intensive
programming languages like BASIC while still allowing the programmer to
write code that is more easy to read than pure machine language
(Weisbecker 1978, 108). The CHIP-8 VM is a program that reads CHIP-8
instructions and performs a corresponding action or set of actions,
including things like reading and writing memory or drawing things to
the display (Weisbecker 1978, 110). The details of how the VM
accomplishes each action is left up to the person writing the VM
program, and the system that they are writing it for. The important part
is that the VM is able to read and decode each instruction, and know
what to do for each one.

Before the VM can start decoding things however, it needs to be able to
store some information about the program as it executes it. To do this,
CHIP-8 includes 16 one byte registers, one 16-bit address register, as
well as at least 2048 bytes (2 KiB) of memory, which is what the
original COSMAC VIP version had access to (Weisbecker 1978, 108). The
registers in a real CPU are small pieces of memory that is built
directly onto the CPU chip, making it extremely fast to access and a
good fit for data that the CPU will need to execute an instruction. In
CHIP-8, the registers serve the same purpose and allow a program to
provide the VM with the extra information it needs that wouldn’t fit in
the small size each instruction is limited to. For example, rather than
telling the processor to solve `1 + 2`, you would have to store the
value `1` in register `A`, and `2` in register `B`. Then, you would ask
the processor to add registers `A` and `B`, which would perform the
calculation and then store the result back into another register.

The way instruction decoding works is pretty easy to understand, and
mirrors the same process that happens in a real CPU. Each valid CHIP-8
instruction is made up of two bytes, or four hexadecimal digits
(hexadecimals are base-16 numbers, usually represented by the characters
0 - 9 and A - F). The first four bits of the first byte (sometimes
referred to as a ‘nibble’) is the opcode. This is first part of the
instruction the VM will check, and it tells the VM what kind of
instruction it is based on what character it is. For example, an
instruction that starts with the hexadecimal `6` will always be a
command to store a certain value in one of CHIP-8’s 16 registers
(Weisbecker 1978, 110). What value corresponds to what command is
arbitrary and was chosen by Joseph Weisbecker when he was writing the
first CHIP-8 VM.

Once the VM has the opcode, it knows how to interpret the rest of the
bits in the instruction. Any instruction that starts with `6` will
always be interpreted as `6XNN`, where `X` refers to one of the 16
1-byte registers, and `NN` is the value to store in that register
(Langhoff 2020). A real instruction might look like `6A42`, which sets
the register at `A` to the value `42` (`A` isn’t a variable here, but
rather a hexadecimal digit). There are 31 valid CHIP-8 instructions in
the original definition (Weisbecker 1978, 110), and each are decoded and
interpreted in a similar manner. For comparison, x86 which is the
architecture (or machine language) used by most laptop and desktop CPUs
has around 1,000 instructions (the exact number varies a lot depending
on how you count it and who you ask) (Heule 2016). That said, an Intel
or AMD CPU decodes x86 instructions in much the same was as a CHIP-8 VM
does, though much faster and using billions of transistors rather than a
few lines of code.

Writing this VM helped me to better understand how a processor works at
a low level. While it did not involve any circuitry like a real
processor would, Potato performs all the same tasks it would have to if
it were made out of silicon. It is hard to reckon with the fact that
everything that happens on even a modern computer is eventually rendered
down the same basic concepts that are demonstrated by something like
CHIP-8, no matter how complex the machinery involved gets.




References:

<div id="refs" class="references csl-bib-body hanging-indent">

<div id="ref-heule_2016" class="csl-entry">

Heule, Stefan. 2016. “How Many X86-64 Instructions Are There Anyway?”
*How Many X86-64 Instructions Are There Anyway?*
<https://stefanheule.com/blog/how-many-x86-64-instructions-are-there-anyway/>.

</div>

<div id="ref-langhoff_2020" class="csl-entry">

Langhoff, Tobias V. 2020. “Guide to Making a CHIP-8 Emulator.” *Tobias
V. Langhoff*. <https://tobiasvl.github.io/blog/write-a-chip-8-emulator>.

</div>

<div id="ref-porter_2020" class="csl-entry">

Porter, Jon. 2020. “Apple Says New Arm-Based M1 Chip Offers the ’Longest
Battery Life Ever in a Mac’.” *The Verge*. The Verge.
<https://www.theverge.com/2020/11/10/21558095/apple-silicon-m1-chip-arm-macs-soc-charge-power-efficiency-mobile-processor>.

</div>

<div id="ref-weisbecker_1978" class="csl-entry">

Weisbecker, Joseph. 1978. “An Easy Programming System.” *BYTES Magazine*
3 (12): 108–22. <https://archive.org/details/byte-magazine-1978-12>.

</div>

</div>
