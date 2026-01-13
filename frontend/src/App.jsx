import React, { useState } from 'react'

export default function App() {
  const [seed, setSeed] = useState('example')
  const [format, setFormat] = useState('svg')
  const [preset, setPreset] = useState('monogram-badge')
  const [size, setSize] = useState(512)
  const [padding, setPadding] = useState(0.12)
  const [variant, setVariant] = useState('')
  const [transparent, setTransparent] = useState(false)
  const [imgUrl, setImgUrl] = useState(null)
  const [loading, setLoading] = useState(false)

  async function generate() {
    setLoading(true)
    setImgUrl(null)
    const body = {
      input: seed,
      preset,
      format,
      size_px: Number(size),
      padding_frac: Number(padding),
      variant: variant === '' ? null : Number(variant),
      transparent_background: transparent,
    }

    const res = await fetch('http://127.0.0.1:3000/generate', {
      method: 'POST',
      headers: { 'Content-Type': 'application/json' },
      body: JSON.stringify(body),
    })
    setLoading(false)
    if (!res.ok) {
      const txt = await res.text()
      alert('Generate failed: ' + txt)
      return
    }
    if (format === 'svg') {
      const text = await res.text()
      const blob = new Blob([text], { type: 'image/svg+xml' })
      setImgUrl(URL.createObjectURL(blob))
    } else {
      const blob = await res.blob()
      setImgUrl(URL.createObjectURL(blob))
    }
  }

  return (
    <div className="app-root">
      <header>
        <h1>LoGen</h1>
      </header>
      <main className="two-col">
        <section className="params">
          <h2>Parameters</h2>

          <label>Preset</label>
          <select value={preset} onChange={e => setPreset(e.target.value)}>
            <option value="monogram-badge">Monogram Badge</option>
          </select>

          <label>Seed</label>
          <input value={seed} onChange={e => setSeed(e.target.value)} placeholder="Enter seed" />

          <label>Format</label>
          <select value={format} onChange={e => setFormat(e.target.value)}>
            <option value="png">PNG</option>
            <option value="svg">SVG</option>
          </select>

          <label>Size: {size}px</label>
          <input type="range" min="64" max="1024" value={size} onChange={e => setSize(e.target.value)} />

          <label>Padding: {padding}</label>
          <input type="range" min="0" max="0.5" step="0.01" value={padding} onChange={e => setPadding(e.target.value)} />

          <label>Variant (optional)</label>
          <input type="number" value={variant} onChange={e => setVariant(e.target.value)} placeholder="Leave empty for none" />

          <label className="checkbox">
            <input type="checkbox" checked={transparent} onChange={e => setTransparent(e.target.checked)} /> Transparent background
          </label>

          <div style={{ marginTop: 12 }}>
            <button onClick={generate} disabled={loading}>{loading ? 'Running…' : 'Run'}</button>
          </div>
        </section>

        <section className="result">
          <h2>Result</h2>
          <div className="result-box">
            {imgUrl ? (
              <img src={imgUrl} alt="generated" />
            ) : (
              <div className="placeholder">No image yet — click Run</div>
            )}
          </div>
        </section>
      </main>
    </div>
  )
}
