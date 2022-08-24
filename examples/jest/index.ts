export const localFetcher = async () => {
  const res = await fetch('http://localhost:9999')
  return res.json()
}
