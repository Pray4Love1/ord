import { useMemo, useState } from 'react';

const emptyRecord = {
  commitment: '',
  block_height: 0,
  timestamp: '',
  parent_hash: null,
  entropy: 0,
  metadata: {}
};

export default function App() {
  const [commitment, setCommitment] = useState('');
  const [record, setRecord] = useState(emptyRecord);
  const [history, setHistory] = useState([]);
  const [error, setError] = useState('');
  const [loading, setLoading] = useState(false);

  const orderedHistory = useMemo(() => history.slice().reverse(), [history]);

  async function handleSubmit(event) {
    event.preventDefault();
    if (!commitment) return;

    setLoading(true);
    setError('');

    try {
      const response = await fetch(`/mirror/${commitment}`);
      if (!response.ok) {
        throw new Error('Mirror not found');
      }

      const payload = await response.json();
      setRecord(payload);
      setHistory((current) => {
        const updated = current.filter((item) => item.commitment !== payload.commitment);
        updated.push({ ...payload, at: new Date().toISOString() });
        return updated.slice(-20);
      });
    } catch (err) {
      setError(err.message);
      setRecord(emptyRecord);
    } finally {
      setLoading(false);
    }
  }

  return (
    <div className="app">
      <header>
        <h1>Living Ordinal Viewer</h1>
        <p>Trace mirrored inscriptions as the watcher breathes life into the verifier.</p>
      </header>

      <form onSubmit={handleSubmit} style={{ display: 'flex', gap: '1rem' }}>
        <input
          type="text"
          placeholder="0x commitment hash"
          value={commitment}
          onChange={(event) => setCommitment(event.target.value)}
        />
        <button type="submit" disabled={loading}>
          {loading ? 'Querying…' : 'View'}
        </button>
      </form>

      {error && <div className="card">{error}</div>}

      {record.commitment && !error && (
        <div className="card">
          <h2>{record.commitment}</h2>
          <p>
            Block <strong>{record.block_height}</strong> ·{' '}
            {new Date(record.timestamp).toLocaleString()}
          </p>
          {record.parent_hash && (
            <p>
              Parent: <code>{record.parent_hash}</code>
            </p>
          )}
          <p>Entropy: {(record.entropy * 100).toFixed(2)}%</p>
          <pre>{JSON.stringify(record.metadata, null, 2)}</pre>
        </div>
      )}

      <section className="chart">
        <h3>Recent Lookups</h3>
        {orderedHistory.length === 0 && <p>No queries yet.</p>}
        <ul>
          {orderedHistory.map((item) => (
            <li key={item.commitment}>
              <strong>{item.commitment}</strong>
              <br />
              Block {item.block_height} · {new Date(item.at).toLocaleTimeString()}
            </li>
          ))}
        </ul>
      </section>
    </div>
  );
}
