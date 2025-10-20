import { BrowserRouter, Routes, Route, Link } from "react-router-dom";
import { useState } from "react";
import axios from "axios";
import { LineChart, Line, XAxis, YAxis, Tooltip, ResponsiveContainer } from "recharts";

// Viewer Component (from master)
function Viewer() {
  const [commitment, setCommitment] = useState("");
  const [data, setData] = useState(null);
  const [history, setHistory] = useState([]);

  async function fetchMirror() {
    try {
      const res = await axios.get(`/mirror/${commitment}`);
      setData(res.data);
      setHistory((h) => [res.data, ...h]);
    } catch (e) {
      alert("No record found or RPC error");
    }
  }

  return (
    <div className="min-h-screen bg-gray-950 text-gray-100 p-8">
      <h1 className="text-3xl mb-6 font-bold">Living Ordinal Explorer</h1>
      <input
        className="p-2 bg-gray-800 rounded mr-2"
        placeholder="Commitment (0x...)"
        value={commitment}
        onChange={(e) => setCommitment(e.target.value)}
      />
      <button onClick={fetchMirror} className="px-3 py-2 bg-indigo-600 rounded">
        View
      </button>

      {data && (
        <div className="mt-8 border-t border-gray-700 pt-6">
          <h2 className="text-xl font-semibold mb-2">Mirror Record</h2>
          <pre className="bg-gray-900 p-4 rounded text-sm">
            {JSON.stringify(data, null, 2)}
          </pre>
        </div>
      )}

      {history.length > 0 && (
        <div className="mt-10">
          <h3 className="font-semibold mb-2">Recent Queries</h3>
          <ul className="space-y-1">
            {history.map((h, i) => (
              <li key={i} className="text-sm text-gray-400">
                {h.commitment?.slice(0, 14)}… block {h.block_height}
              </li>
            ))}
          </ul>
        </div>
      )}

      {history.length > 1 && (
        <div className="mt-10">
          <h3 className="font-semibold mb-2">Block Heights Timeline</h3>
          <ResponsiveContainer width="100%" height={200}>
            <LineChart data={history}>
              <XAxis dataKey="timestamp" hide />
              <YAxis />
              <Tooltip />
              <Line type="monotone" dataKey="block_height" stroke="#82ca9d" />
            </LineChart>
          </ResponsiveContainer>
        </div>
      )}
    </div>
  );
}

// Placeholder EcoView Page
function EcoView() {
  return (
    <div className="min-h-screen bg-gray-950 text-gray-100 p-8">
      <h1 className="text-3xl mb-6 font-bold">Ecosystem Overview</h1>
      <p>Future analytics or network visualization will go here.</p>
    </div>
  );
}

// App Router
export default function App() {
  return (
    <BrowserRouter>
      <nav className="p-2 bg-gray-800 flex gap-4">
        <Link to="/">Viewer</Link>
        <Link to="/eco">Ecosystem</Link>
      </nav>
      <Routes>
        <Route path="/" element={<Viewer />} />
        <Route path="/eco" element={<EcoView />} />
      </Routes>
    </BrowserRouter>
  );
}
