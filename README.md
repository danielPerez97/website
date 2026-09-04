# danperez.dev
This is a CLI tool to generate a static website. It uses Liquid templates to re-use
HTML files. 

## Building
To build, you will need to [install Rust][1]. Then, run `cargo build` to build an executable in the `target` folder.

## Usage
Make sure your CLI is inside the `site` folder. Then, run
```
../target/debug/rusty-website .
```

If you don't need skip syntax highlighting, this will run significantly faster using the `skip-syntax-highlighting` 
switch:
```
../target/debug/rusty-website . --skip-syntax-highlighting
```

Once the command completes, `cd` into `site/out` and spin up an HTTP server:
```
python3 -m http.server 8000
```

You can now use the website on your local machine at http://127.0.0.1:8000

[1]: https://rust-lang.org/tools/install/