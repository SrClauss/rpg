#!/usr/bin/env bash
# ==============================================================================
# Script para reiniciar todos os sistemas do RPG Engine (Back + Front)
# ==============================================================================

set -e

DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"

GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
CYAN='\033[0;36m'
NC='\033[0m' # No Color

echo -e "${YELLOW}🔄 Reiniciando sistemas do RPG Engine...${NC}"

# 1. Executa o stop para derrubar instâncias antigas
"$DIR/scripts/stop.sh"

echo ""
echo -e "${CYAN}🚀 Iniciando serviços em background...${NC}"

# Cria diretório de logs se não existir
mkdir -p "$DIR/logs"

# 2. Inicia o Backend Rust (Axum na porta 3001)
echo -e "${BLUE}▶ Iniciando Backend Rust (port 3001)...${NC}"
cd "$DIR"

# Verifica se o binário existe; caso contrário, compila
if [ ! -f "$DIR/target/debug/server" ]; then
  echo -e "  Binário target/debug/server não encontrado. Compilando..."
  cargo build --bin server
fi

if command -v systemd-run >/dev/null 2>&1; then
  systemd-run --user --unit=rpg-backend \
    --working-directory="$DIR" \
    -p "StandardOutput=append:$DIR/logs/backend.log" \
    -p "StandardError=append:$DIR/logs/backend.log" \
    "$DIR/target/debug/server" >/dev/null 2>&1
  echo -e "  Backend iniciado via systemd user unit: rpg-backend (logs: logs/backend.log)"
else
  nohup "$DIR/target/debug/server" </dev/null >> "$DIR/logs/backend.log" 2>&1 &
  BACKEND_PID=$!
  disown $BACKEND_PID 2>/dev/null || true
  echo -e "  Backend iniciado (PID: $BACKEND_PID, logs: logs/backend.log)"
fi

# 3. Inicia o Frontend Vite (na porta 5173)
echo -e "${BLUE}▶ Iniciando Frontend Vite (port 5173)...${NC}"
cd "$DIR/frontend"

if command -v systemd-run >/dev/null 2>&1; then
  systemd-run --user --unit=rpg-frontend \
    --working-directory="$DIR/frontend" \
    -p "StandardOutput=append:$DIR/logs/frontend.log" \
    -p "StandardError=append:$DIR/logs/frontend.log" \
    /usr/bin/npm run dev >/dev/null 2>&1
  echo -e "  Frontend iniciado via systemd user unit: rpg-frontend (logs: logs/frontend.log)"
else
  nohup npm run dev </dev/null >> "$DIR/logs/frontend.log" 2>&1 &
  FRONTEND_PID=$!
  disown $FRONTEND_PID 2>/dev/null || true
  echo -e "  Frontend iniciado (PID: $FRONTEND_PID, logs: logs/frontend.log)"
fi

# 4. Aguarda inicialização das portas
echo ""
echo -e "${YELLOW}⏳ Verificando subida dos serviços...${NC}"
sleep 2

check_service() {
  local port=$1
  local name=$2
  local tries=10
  while [ $tries -gt 0 ]; do
    if lsof -ti :"$port" >/dev/null 2>&1; then
      echo -e "${GREEN}✓ ${name} está ONLINE na porta ${port}.${NC}"
      return 0
    fi
    sleep 1
    tries=$((tries - 1))
  done
  echo -e "${YELLOW}⚠️ Aviso: ${name} ainda não respondeu na porta ${port}. Verifique logs.${NC}"
}

check_service 3001 "Backend Rust"
check_service 5173 "Frontend Vite"

echo ""
echo -e "${GREEN}===================================================================${NC}"
echo -e "${GREEN}✨ Todos os sistemas foram reiniciados com sucesso!${NC}"
echo -e "  🌐 Frontend: http://localhost:5173"
echo -e "  ⚙️ Backend:  http://localhost:3001"
echo -e "  📄 Logs em:  $DIR/logs/"
echo -e "${GREEN}===================================================================${NC}"
