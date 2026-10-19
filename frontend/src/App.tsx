import { useState, useEffect } from 'react';
import type { CharacterSheet } from './types/character';
import { CharacterCreationWizard } from './components/character-creation/CharacterCreationWizard';
import { BaseGameHUD } from './components/game/BaseGameHUD';
import { ClassHUDDispatcher } from './components/game/classes/ClassHUDDispatcher';
import { checkBackendHealth, syncCharacterWithBackend } from './services/api';
import './App.css';

export function App() {
  const [character, setCharacter] = useState<CharacterSheet | null>(null);
  const [backendOnline, setBackendOnline] = useState<boolean>(false);

  useEffect(() => {
    checkBackendHealth().then((health) => {
      setBackendOnline(Boolean(health));
    });
  }, []);

  const handleCharacterCreated = (sheet: CharacterSheet) => {
    setCharacter(sheet);
    syncCharacterWithBackend(sheet).then((res) => {
      console.log('Sincronização com backend:', res.message);
    });
  };

  const handleResetCharacter = () => {
    setCharacter(null);
  };

  return (
    <div className="app-container">
      {/* Backend connection pill */}
      <div
        style={{
          display: 'flex',
          justifyContent: 'flex-end',
          padding: '8px 16px 0',
          fontSize: '11px',
        }}
      >
        <span
          style={{
            display: 'inline-flex',
            alignItems: 'center',
            gap: '6px',
            padding: '3px 8px',
            borderRadius: '12px',
            backgroundColor: backendOnline ? 'rgba(16, 185, 129, 0.15)' : 'rgba(234, 179, 8, 0.15)',
            color: backendOnline ? 'var(--accent-emerald)' : 'var(--accent-gold)',
            border: backendOnline ? '1px solid rgba(16, 185, 129, 0.3)' : '1px solid rgba(234, 179, 8, 0.3)',
          }}
        >
          <span
            style={{
              width: '6px',
              height: '6px',
              borderRadius: '50%',
              backgroundColor: backendOnline ? 'var(--accent-emerald)' : 'var(--accent-gold)',
            }}
          />
          {backendOnline ? 'Servidor Rust Axum Conectado (Porta 3001)' : 'Motor Determinístico Ativo (Cliente / Local)'}
        </span>
      </div>

      {!character ? (
        <CharacterCreationWizard onCharacterCreated={handleCharacterCreated} />
      ) : (
        <BaseGameHUD
          character={character}
          onResetCharacter={handleResetCharacter}
          classSpecificPanel={<ClassHUDDispatcher character={character} />}
        />
      )}
    </div>
  );
}

export default App;
