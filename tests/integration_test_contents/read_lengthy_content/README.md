> [!WARNING]
> This project creates a large (100MB) file, operations on which cause Tinymist on VSCode to run your computer out of RAM. Turn Tinymist off when developing for this. If you run out of RAM anyways, consider lowering the size. 20MB seemed sufficient to trigger the bug below.

This is a test case project for the bug we fixed here:

```
commit 2c06197a2b030be742ab0d9eec50c582626edefa (write-file-listing)
Date:   Thu Nov 13 23:48:04 2025 -0500

    fix: files that are long no longer cause deadlock

    - this happens eg if user embeds an entire #image instead of linking with #html.a
    - https://stackoverflow.com/questions/69528338/why-commands-stdin-write-all-never-terminate
    - https://stackoverflow.com/questions/68327635/how-do-i-avoid-deadlock-when-using-subprocess-popen-to-connect-multiple-processe
    - https://doc.rust-lang.org/std/thread/struct.JoinHandle.html#method.join
```
