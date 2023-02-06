# Challenges

- In the previous version (2-multi_threading), there is no clean-up code for the threads to safely exit when their process is pending in case of shutdown (Ctrl + C). This part is to make sure that any haning process would finish up before shutting down the server when requested so.

# new impl

```
imple Drop for ThreadPool {
    fn drop(&mut self) {
        // cleanup codes...
    }
}
```

# How to test

```
cargo run
```

- Open a browser and connect to `http://127.0.0.1:7878` as usual.

- The server will listen to 5 streams and then clean-up threads before shutting down.

 ![Image](/archives/basics/2-multi_threading/imgsrc/log.png?raw=true "Clean-up log")