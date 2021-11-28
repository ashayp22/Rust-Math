<h1 align="center">
  Rusty Warriors
</h1>


<h4 align="center">CS 199-128 Fall 2021 Final Project</h4>

## Project ~~Introduction~~ Motivation

While Rust is known for its focus on speed, memory safety, and parallelism, it isn't known for creating stunning visuals. After watching the [Lecture on Calculating the Julia Set](https://www.youtube.com/watch?v=ifZ5Od92MXY), we decided that we wanted to expand upon the idea of using parallelism, concurrency, and shared memory to create mathematical visualizations. Our objective is to build a website that contains fractals, animations, and other cool visuals. 

## ~~System Overview~~ Roadmap

We plan on using [egui](https://github.com/emilk/egui) and [eframe](https://github.com/emilk/egui/tree/master/eframe) to build a GUI using Rust. We will compile our app into Web Assembly so it can be run in the browser. 

Week 1 (11/1)

- [x] 11/1 Group Submission Form  
- [x] 11/5 Group Idea Form  

- [x] Download an eframe template. 
- [x] Clean it up.
- [x] Add fractal from egui demo.

Week 2 (11/8)

- [x] Jimit - Write math algorithm for generating a H-Tree fractal.
- [x] Khoa - Write math algorithm for generating a Fibonacci tree.
- [x] Ashay - Write math algorithm for generating a Sierpinski Carpet.

Week 3 (11/15)

- [x] 11/15 Checkpoint I
- [x] Ashay - Write code for visualizing fractal using egui.
- [x] Khoa - Write code for visualizing fractal using egui.
- [x] Jimit - Write code for visualizing fractal using egui.

Week 4 (11/22)

- [x] Add controls for customizing fractal generation.
- [x] Combined our fractals into one website.
- [x] Documentation.

Week 5 (11/29)

- [ ] 11/29 Checkpoint II
- [ ] Add 1 more fractal.
- [ ] Add slides for customizing H-Tree, Fibonacci, and Sierpinski fractals.
- [ ] Format code using RustFMT.

Week 6 (12/6)

- [ ] Documentation
- [ ] 12/8 Final Submission

## Possible Challenges

- Speed with the animation
- Math Calculation error
- Implementing multithreading

## References

- [egui web demo](https://emilk.github.io/egui/index.html)
- [fractal trees in rust](https://github.com/redwarp/fractal-trees)

## Team

<table align="center">
  <tr>
    <td align="center"><a href=""><img src="https://stickershop.line-scdn.net/stickershop/v1/product/1014241/LINEStorePC/main.png" width="75px;" alt="Pusheen"/><br /><b>Ashay Parikh</b></a><br /><sub>ashayp2</sub></td>
    <td align="center"><a href=""><img src="https://stickershop.line-scdn.net/stickershop/v1/sticker/637244/android/sticker.png" width="75px;" alt="Unicorn Pusheen"/><br /><b>Jimit Gosar</b></a><br /><sub>jgosar2</sub></td>
    <td align="center"><a href=""><img src="https://stickershop.line-scdn.net/stickershop/v1/sticker/637275/android/sticker.png" width="75px;" alt="Pilot Pusheen"/><br /><b>Dang Khoa Pham</b></a><br /><sub>khoadp2</sub></td>
  </tr>
</table>
