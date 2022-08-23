{
  const res = await fetch('http://localhost:9999');
  console.log(res)
}

{
  const res = await window.fetch('http://localhost:9999');
  console.log(res)
}

{
  const res = await globalThis.fetch('http://localhost:9999');
  console.log(res)
}

{
  const dummy = {
    fetch: () => false,
  }
  const res = await dummy.fetch('http://localhost:9999');
  console.log(res)
}
