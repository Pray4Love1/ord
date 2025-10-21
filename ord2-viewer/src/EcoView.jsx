import { useEffect, useState, useRef } from "react";
import axios from "axios";

export default function EcoView() {
  const [entities, setEntities] = useState([]);
  const canvasRef = useRef(null);

  // Poll your Rust API every few seconds for new mirrors or evolutions
  useEffect(() => {
    const fetchEntities = async () => {
      try {
        const res = await axios.get("/mirror/list"); // your API can return all known mirrors
        setEntities(res.data);
      } catch (_) {}
    };
    fetchEntities();
    const id = setInterval(fetchEntities, 5000);
    return () => clearInterval(id);
  }, []);

  // Simple physics + draw loop
  useEffect(() => {
    const ctx = canvasRef.current.getContext("2d");
    let frame;
    function draw() {
      ctx.clearRect(0, 0, canvasRef.current.width, canvasRef.current.height);
      entities.forEach((e) => {
        const { traits = {}, mood = "neutral" } = e;
        const energy = traits.energy || 1;
        const x =
          parseInt(e.commitment.slice(2, 6), 16) % canvasRef.current.width;
        const y =
          parseInt(e.commitment.slice(6, 10), 16) % canvasRef.current.height;
        const size = 5 + 10 * Math.min(1, energy);
        const color =
          traits.color === "blue"
            ? "#3b82f6"
            : traits.color === "red"
            ? "#ef4444"
            : "#a3e635";
        ctx.beginPath();
        ctx.arc(x, y, size, 0, 2 * Math.PI);
        ctx.fillStyle = color;
        ctx.globalAlpha = mood === "energized" ? 1.0 : 0.6;
        ctx.fill();
      });
      frame = requestAnimationFrame(draw);
    }
    frame = requestAnimationFrame(draw);
    return () => cancelAnimationFrame(frame);
  }, [entities]);

  return (
    <div className="w-full h-screen bg-black text-white relative">
      <canvas
        ref={canvasRef}
        width={window.innerWidth}
        height={window.innerHeight}
      />
      <div className="absolute top-4 left-4 bg-gray-900 bg-opacity-50 p-4 rounded">
        <h2 className="font-semibold text-lg">Ecosystem View</h2>
        <p>{entities.length} living inscriptions</p>
      </div>
    </div>
  );
}
