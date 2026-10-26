#!/usr/bin/env bash
# ==============================================================================
# Script para finalizar todos os processos e serviços do RPG Engine (Back + Front)
# ==============================================================================

set -e

RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

echo -e "${YELLOW}🛑 Finalizando todos os sistemas do RPG Engine...${NC}"

# Função para matar processos por porta
kill_port() {
  local port=$1
  local name=$2
  local pids=$(lsof -ti :"$port" 2>/dev/null || true)
  if [ -n "$pids" ]; then
    echo -e "${BLUE}▶ Encerrando ${name} na porta ${port} (PID: ${pids})...${NC}"
    kill $pids 2>/dev/null || true
    sleep 1
    # Se ainda estiver rodando, força com kill -9
    pids_remaining=$(lsof -ti :"$port" 2>/dev/null || true)
    if [ -n "$pids_remaining" ]; then
      kill -9 $pids_remaining 2>/dev/null || true
    fi
  else
    echo -e "  Porta ${port} (${name}) já está liberada."
  fi
}

# 0. Para serviços systemd caso tenham sido iniciados via systemd-run
systemctl --user stop rpg-backend.service >/dev/null 2>&1 || true
systemctl --user stop rpg-frontend.service >/dev/null 2>&1 || true

# 1. Finaliza por portas ouvidas (3001 backend, 5173 frontend vite)
kill_port 3001 "Backend Rust Axum"
kill_port 5173 "Frontend Vite"

# 2. Finaliza processos remanescentes por nome
kill_name() {
  local pattern=$1
  local name=$2
  if pgrep -f "$pattern" >/dev/null 2>&1; then
    echo -e "${BLUE}▶ Matando processos restantes de ${name} ($pattern)...${NC}"
    pkill -f "$pattern" 2>/dev/null || true
    sleep 1
    pkill -9 -f "$pattern" 2>/dev/null || true
  fi
}

kill_name "target/debug/server" "Servidor Rust"
kill_name "node.*vite" "Vite Dev Server"

echo -e "${GREEN}✅ Todos os sistemas foram desligados com sucesso.${NC}"
