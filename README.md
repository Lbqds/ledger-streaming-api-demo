# Streaming Api Demo

This demo is used to demonstrate the issue of `Speculos` crashing when using the streaming api from the ledger rust sdk.

This demo will call `NbglStreamingReview::start` when receiving the first data frame, call `NbglStreamingReview::continue_review` upon receiving subsequent data frames, and call `NbglStreamingReview::finish` when receiving the last data frame. You can refer to the code in the `src/handler.rs` file.

You can run the following commands to reproduce the issue:

1. `make release`
2. `make run-speculos`
3. `cd js && npm install && npm run test`

The crash occurs when calling `NbglStreamingReview::finish`. If you run `Speculos` with the `--trace` parameter, you can see the log output as follows:

```
[*] syscall: nbgl_screen_reinit() = 0 (0x0)
[*] syscall: nbgl_front_draw_rect0x5000ac90 = 0 (0x0)
[*] syscall: nbgl_front_draw_img_file0x50002cce, 0x5000, 0, 0x5000224a
[-] The app crashed with signal 11
```
