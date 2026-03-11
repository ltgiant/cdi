function cdi
  if test (count $argv) -eq 0
    command cdi
  else if string match -q -- '-*' $argv[1]
    command cdi $argv
  else
    set -l target (command cdi $argv[1])
    and cd $target
  end
end
