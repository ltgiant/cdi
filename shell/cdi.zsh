cdi() {
  if [ $# -eq 0 ]; then
    command cdi
  elif [[ "$1" == -* ]]; then
    command cdi "$@"
  else
    local target
    target=$(command cdi "$1") && cd "$target"
  fi
}
