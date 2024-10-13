default: install

alias i := install
install:
  cargo install --path .

alias t := test
test *args:
  cargo test {{args}} || cargo insta review

alias c := clean
clean *args:
  cargo clean
