function cdi {
  if ($args.Count -eq 0) {
    & cdi.exe
  } elseif ($args[0] -like '-*') {
    & cdi.exe @args
  } else {
    $target = & cdi.exe $args[0]
    if ($LASTEXITCODE -eq 0) { Set-Location $target }
  }
}
