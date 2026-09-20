import { useState, useEffect, KeyboardEvent } from 'react';
import './App.css';

interface ActionResponse {
  narration: string;
}

interface PartyResponse {
  characters: string[];
}

function App() {
  const [party, setParty] = useState<string[]>([]);
  const [history, setHistory] = useState<{ role: string, text: string }[]>([]);
  const [input, setInput] = useState('');
  const [loading, setLoading] = useState(false);

  useEffect(() => {
    // Buscar membros da Party do servidor Rust (SQLite)
    fetch('http://localhost:3000/api/party')
      .then(res => res.json())
      .then((data: PartyResponse) => setParty(data.characters))
      .catch(console.error);
      
    // Mensagem de Boas-Vindas
    setHistory([
      { role: 'SYSTEM', text: 'Módulo de Narração Conectado ao Motor Rust.' },
      { role: 'GEMINI', text: 'Você se encontra diante de infinitas possibilidades. O que vocês fazem?' }
    ]);
  }, []);

  const sendAction = async () => {
    if (!input.trim() || loading) return;
    
    const actionText = input;
    setInput('');
    setHistory(prev => [...prev, { role: 'USER', text: `${actionText}` }]);
    setLoading(true);

    try {
      // Chamada HTTP para a API Rust
      const res = await fetch('http://localhost:3000/api/action', {
        method: 'POST',
        headers: { 'Content-Type': 'application/json' },
        body: JSON.stringify({ prompt: actionText })
      });
      const data: ActionResponse = await res.json();
      
      setHistory(prev => [...prev, { role: 'GEMINI', text: data.narration }]);
    } catch (err) {
      setHistory(prev => [...prev, { role: 'SYSTEM', text: 'Erro ao conectar com o backend. O Orquestrador está offline.' }]);
    } finally {
      setLoading(false);
    }
  };

  const handleKeyDown = (e: KeyboardEvent<HTMLInputElement>) => {
    if (e.key === 'Enter') {
      sendAction();
    }
  };

  return (
    <div className="app-container">
      {/* Janela de Narração (Esquerda/Centro) */}
      <div className="narration-panel">
        <div className="narration-history">
          {history.map((entry, idx) => (
            <div key={idx} className="narration-entry">
              {entry.role === 'USER' && <div className="narration-user">AÇÃO DA PARTY:</div>}
              {entry.text}
            </div>
          ))}
          {loading && <div className="loading-text">O Mestre (Gemini) está processando sua ação e invocando o Motor Tático...</div>}
        </div>
        
        {/* Barra de Input (Bottom) */}
        <div className="input-area">
          <input 
            type="text" 
            className="action-input" 
            placeholder="Descreva a ação... (ex: Atacar, Conjurar Magia, Mover)"
            value={input}
            onChange={e => setInput(e.target.value)}
            onKeyDown={handleKeyDown}
            disabled={loading}
            autoFocus
          />
          <button className="send-btn" onClick={sendAction} disabled={loading}>
            AGIR
          </button>
        </div>
      </div>

      {/* Painel da Party (Direita) */}
      <div className="party-panel">
        <div className="party-title">Painel da Party</div>
        
        {party.length === 0 ? (
          <div style={{ color: '#666' }}>Carregando heróis do SQLite...</div>
        ) : (
          party.map((charName, idx) => (
            <div key={idx} className="character-card">
              {/* Box reservado para arte futura */}
              <div className="char-avatar-placeholder">Art</div>
              <div className="char-info">
                <div className="char-name">{charName}</div>
                <div className="hp-bar-bg">
                  <div className="hp-bar-fill"></div>
                </div>
              </div>
            </div>
          ))
        )}
      </div>
    </div>
  );
}

export default App;
