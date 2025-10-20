import { BrowserRouter, Routes, Route, Link } from "react-router-dom";
import EcoView from "./EcoView";
import Viewer from "./Viewer";

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
