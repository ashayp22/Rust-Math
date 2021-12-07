## Requirements

* Working installation of Rust `>=1.54.0`.
* A machine that supports the following crates and their dependencies:
```
getrandom = { version = "0.2", features = ["js"] }
eframe = "0.15.0"
rand = "0.8.4"
num = "0.4.0"
```


## How to run:

If you have Rust installed in an environment that supports GUIs:

1. `git clone https://github.com/ashayp22/Rust-Math.git`
2. Change directory into the project folder on a command prompt that has [Cargo](https://doc.rust-lang.org/cargo/). Alternatively, you can open the folder in VS code and open the built in terminal.
3. `cargo run`
4. Enjoy playing around with the awesome fractals by changing the sliders in the top left.
 
If you do not have Rust installed in an environment that supports GUI *or* want to see the application in the browser:
 
1. `git clone https://github.com/ashayp22/Rust-Math.git`
2. Change directory into the project folder on a command prompt that has [Cargo](https://doc.rust-lang.org/cargo/). Alternatively, you can open the folder in VS code and open the built in terminal.
3. Run `./setup_web.sh`
4. Run `./build-web.sh`
5. Navigate to http://localhost:8080 in the Google Chrome browser.
