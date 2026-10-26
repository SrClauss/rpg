import React, { useState, useEffect, useRef } from 'react';
import { MessageSquare, Send, X } from 'lucide-react';
import { sendPartyChatMessage } from '../../services/api';

export interface ChatMessage {
  id: string;
  sender_id: string;
  sender_name: string;
  text: string;
  timestamp: string;
  is_action?: boolean;
}

interface PartyChatWidgetProps {
  campaignId: string;
  roomCode: string;
  currentUserId: string;
  currentUserName: string;
}

export const PartyChatWidget: React.FC<PartyChatWidgetProps> = ({
  campaignId,
  roomCode,
  currentUserId,
  currentUserName,
}) => {
  const [isOpen, setIsOpen] = useState(false);
  const [messages, setMessages] = useState<ChatMessage[]>([]);
  const [inputValue, setInputValue] = useState('');
  const [unreadCount, setUnreadCount] = useState(0);
  const messagesEndRef = useRef<HTMLDivElement>(null);

  // Escuta WebSocket ou eventos customizados de chat
  useEffect(() => {
    const wsUrl = `ws://localhost:3001/ws/${roomCode}`;
    let ws: WebSocket | null = null;
    try {
      ws = new WebSocket(wsUrl);
      ws.onmessage = (event) => {
        try {
          const data = JSON.parse(event.data);
          if (data.type === 'CHAT_MESSAGE') {
            const newMsg: ChatMessage = {
              id: `${Date.now()}_${Math.random()}`,
              sender_id: data.sender_id,
              sender_name: data.sender_name,
              text: data.text,
              timestamp: data.timestamp || new Date().toISOString(),
              is_action: data.is_action,
            };
            setMessages((prev) => [...prev, newMsg]);
            if (!isOpen) {
              setUnreadCount((c) => c + 1);
            }
          }
        } catch {}
      };
    } catch {}

    return () => {
      if (ws) ws.close();
    };
  }, [roomCode, isOpen]);

  useEffect(() => {
    if (isOpen) {
      setUnreadCount(0);
      messagesEndRef.current?.scrollIntoView({ behavior: 'smooth' });
    }
  }, [isOpen, messages]);

  const handleSend = async (e: React.FormEvent) => {
    e.preventDefault();
    if (!inputValue.trim()) return;

    const text = inputValue.trim();
    setInputValue('');

    const localMsg: ChatMessage = {
      id: `${Date.now()}_local`,
      sender_id: currentUserId,
      sender_name: currentUserName,
      text,
      timestamp: new Date().toISOString(),
    };
    setMessages((prev) => [...prev, localMsg]);

    try {
      await sendPartyChatMessage(campaignId, {
        sender_id: currentUserId,
        sender_name: currentUserName,
        text,
      });
    } catch {}
  };

  return (
    <div
      style={{
        position: 'fixed',
        bottom: '16px',
        right: '16px',
        zIndex: 9999,
        display: 'flex',
        flexDirection: 'column',
        alignItems: 'flex-end',
      }}
    >
      {/* Botão de abrir/fechar */}
      {!isOpen && (
        <button
          onClick={() => setIsOpen(true)}
          style={{
            display: 'flex',
            alignItems: 'center',
            gap: '8px',
            padding: '10px 16px',
            backgroundColor: '#0c0f17',
            border: '1px solid rgba(234, 179, 8, 0.4)',
            borderRadius: '24px',
            color: '#facc15',
            fontSize: '12px',
            fontWeight: 700,
            cursor: 'pointer',
            boxShadow: '0 4px 20px rgba(0,0,0,0.6), 0 0 15px rgba(234, 179, 8, 0.15)',
            position: 'relative',
          }}
        >
          <MessageSquare size={16} />
          <span>Chat da Party</span>
          {unreadCount > 0 && (
            <span
              style={{
                position: 'absolute',
                top: '-4px',
                right: '-4px',
                backgroundColor: '#ef4444',
                color: '#fff',
                fontSize: '10px',
                fontWeight: 800,
                padding: '2px 6px',
                borderRadius: '10px',
                border: '2px solid #000',
              }}
            >
              {unreadCount}
            </span>
          )}
        </button>
      )}

      {/* Caixa de Chat aberta */}
      {isOpen && (
        <div
          style={{
            width: '320px',
            height: '420px',
            backgroundColor: 'rgba(12, 15, 23, 0.95)',
            backdropFilter: 'blur(10px)',
            border: '1px solid rgba(234, 179, 8, 0.35)',
            borderRadius: '16px',
            display: 'flex',
            flexDirection: 'column',
            overflow: 'hidden',
            boxShadow: '0 10px 30px rgba(0,0,0,0.8), 0 0 20px rgba(234, 179, 8, 0.12)',
          }}
        >
          {/* Header */}
          <div
            style={{
              padding: '10px 14px',
              backgroundColor: '#07090e',
              borderBottom: '1px solid rgba(255, 255, 255, 0.08)',
              display: 'flex',
              alignItems: 'center',
              justifyContent: 'space-between',
            }}
          >
            <div style={{ display: 'flex', alignItems: 'center', gap: '6px' }}>
              <MessageSquare size={14} color="#facc15" />
              <span style={{ fontSize: '12px', fontWeight: 800, color: '#facc15' }}>
                Chat da Sala #{roomCode}
              </span>
            </div>
            <button
              onClick={() => setIsOpen(false)}
              style={{ background: 'transparent', border: 'none', color: '#94a3b8', cursor: 'pointer' }}
            >
              <X size={16} />
            </button>
          </div>

          {/* Mensagens */}
          <div
            style={{
              flex: 1,
              overflowY: 'auto',
              padding: '12px',
              display: 'flex',
              flexDirection: 'column',
              gap: '8px',
            }}
          >
            {messages.length === 0 ? (
              <div style={{ textAlign: 'center', color: '#64748b', fontSize: '11px', marginTop: '40px' }}>
                Nenhuma mensagem na conversa.<br />Diga algo para a party!
              </div>
            ) : (
              messages.map((m) => {
                const isMe = m.sender_id === currentUserId;
                return (
                  <div
                    key={m.id}
                    style={{
                      alignSelf: isMe ? 'flex-end' : 'flex-start',
                      maxWidth: '85%',
                      backgroundColor: isMe ? 'rgba(234, 179, 8, 0.15)' : 'rgba(255, 255, 255, 0.05)',
                      border: isMe ? '1px solid rgba(234, 179, 8, 0.3)' : '1px solid rgba(255, 255, 255, 0.08)',
                      borderRadius: '10px',
                      padding: '8px 10px',
                      fontSize: '12px',
                    }}
                  >
                    <div
                      style={{
                        fontSize: '10px',
                        fontWeight: 700,
                        color: isMe ? '#facc15' : '#38bdf8',
                        marginBottom: '2px',
                      }}
                    >
                      {isMe ? 'Você' : m.sender_name}
                    </div>
                    <div style={{ color: '#f1f5f9', wordBreak: 'break-word', lineHeight: '1.4' }}>
                      {m.text}
                    </div>
                  </div>
                );
              })
            )}
            <div ref={messagesEndRef} />
          </div>

          {/* Input */}
          <form
            onSubmit={handleSend}
            style={{
              display: 'flex',
              padding: '8px',
              backgroundColor: '#07090e',
              borderTop: '1px solid rgba(255, 255, 255, 0.08)',
              gap: '6px',
            }}
          >
            <input
              type="text"
              value={inputValue}
              onChange={(e) => setInputValue(e.target.value)}
              placeholder="Mensagem para a party..."
              style={{
                flex: 1,
                backgroundColor: '#0c0f17',
                border: '1px solid rgba(255, 255, 255, 0.12)',
                borderRadius: '8px',
                padding: '6px 10px',
                fontSize: '12px',
                color: '#fff',
                outline: 'none',
              }}
            />
            <button
              type="submit"
              disabled={!inputValue.trim()}
              style={{
                backgroundColor: 'var(--accent-gold, #facc15)',
                border: 'none',
                borderRadius: '8px',
                padding: '0 10px',
                color: '#000',
                cursor: 'pointer',
                opacity: inputValue.trim() ? 1 : 0.5,
              }}
            >
              <Send size={14} />
            </button>
          </form>
        </div>
      )}
    </div>
  );
};
