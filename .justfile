default: musl

alias i := install
install:
  cargo install --path .

musl:
  cross install --path . --target x86_64-unknown-linux-musl

alias t := test
test *args:
  cargo test {{args}} || cargo insta review

alias c := clean
clean *args:
  cargo clean
