# riow - Ray Tracing in One Weekend
For the 5th time, I will be starting the [Ray Tracing in One Weekend](https://raytracing.github.io/books/RayTracingInOneWeekend.html) book in Rust
<br>
## Postmortem
The biggest problem I encountered was with **memory management**: in particular with Rust's borrow checker. I totally ignored the concept of **moved values**, and thus, I needed to redo entire chapters because of my ignorance. I am beginning to understand the power of Rust's memory management model, but I still lack the fundementals; I got lots of reading to do.
<br><br>
I also encountered problems with porting C++ **abstract classes** and **shared pointers** to Rust. I used reference-counters (Rc) for multiple ownership, but I don't think I fully grasped what exactly I was doing. This shines through quite obviously with my inconsistent use of Rc and references. Plainly, I need to read more about memory management and references. Additionally, I used enums in place of abstract classes to group together similar structs which seemed like the best option at the time.
<br><br>
I learned a lot and got the ray tracer to work (a rust tracer if you will), so this would be one of my more successful projects to date.
<br>
![image of finished render](https://github.com/manorajesh/riow/blob/master/final.png)
