Exploring if app-services can replace `log` with `tracing`.

`app` is a pretend app, much like gecko, firefox-android or firefox-ios.

`second_lib` is a utility crate, called by `app`.

`first_lib` is another utility crate, called by `second_lib`

`log_lib` is another utility crate, but one which still used the `log` crate.

All crates other than `log_lib` use `tracing` rather than `log`. Further, `app`
has a "tracing subscriber" which is used to ferry logs across an FFI.
