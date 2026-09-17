# Regras de Negócio e Decisões de Arquitetura

## Unidade Atômica: Hexágono e Mapeamento de Regiões por Agrupamento (Region-Based Terrain)

---

## 1. O Hexágono como Menor Unidade Atômica de D&D 5e

- **1 Hexágono = 1 Célula de $5\text{ft}$ ($1.5\text{m}$)**.
- O hexágono é a menor unidade indivisível do grid de combate.

---

## 2. Estrutura de Resposta do DeepSeek: Regiões de Terreno -> Lista de Hexágonos

Em vez de listar obstáculos isolados um a um, o DeepSeek mapeia o campo agrupando hexágonos em **Regiões de Terreno** (`TerrainRegion`), descrevendo a geografia de forma orgânica e contínua.

### Schema JSON da Região:
```json
{
  "regions": [
    {
      "name": "rio_de_lava_central",
      "terrain_type": "Difficult",
      "description": "Rio de lava fervente cortando o centro da caverna",
      "hexes": [
        { "q": 3, "r": 2 },
        { "q": 4, "r": 2 },
        { "q": 5, "r": 3 },
        { "q": 6, "r": 3 }
      ]
    },
    {
      "name": "barricada_de_barris",
      "terrain_type": "CoverHalf",
      "description": "Barris de carvalho tombados que oferecem cobertura",
      "hexes": [
        { "q": 5, "r": 6 },
        { "q": 6, "r": 6 }
      ]
    },
    {
      "name": "estalagmite_maciça",
      "terrain_type": "Obstacle",
      "description": "Coluna de rocha intransponível",
      "hexes": [
        { "q": 2, "r": 4 }
      ]
    }
  ]
}
```

---

## 3. Vantagens do Mapeamento por Regiões

1. **Eficiência de Tokens e Organização**: Agrupa rios, estradas, abismos e áreas pegando fogo sob um único objeto regional.
2. **Semântica Tática**: O motor de jogo sabe exatamente quais hexágonos pertencem à mesma região geográfica (ex: se o jogador entrar em qualquer hexágono da região `rio_de_lava_central`, sofre o efeito contínuo daquela região).
3. **Fácil Renderização**: O frontend pode colorir/destacar regiões completas de terreno no mapa visual.

---

## 4. Estratégia de Sobreposição do Grid Visual (Grade Infinita)

Quando a imagem for desenhada com os hexágonos, a estratégia a ser adotada deve ser a **Grade Infinita (Overflow)**:
- O raio do hexágono (`hex_radius`) é calculado para garantir que as dimensões lógicas do combate (ex: 20x15) caibam perfeitamente na imagem gerada, sem serem cortadas.
- O código que desenha a sobreposição visual (overlay) continuará desenhando hexágonos para além dos limites `width_hexes` e `height_hexes` até preencher 100% da imagem original. Isso evita espaços em branco nas margens, caso a proporção (aspect ratio) da arte não case perfeitamente com a proporção geométrica da grade de combate.

---

## 5. Geração de Arte e Cache com Tags (Gemini)

1. **Escala e Dimensão no Prompt:** O tamanho (em unidades hex/D&D) deve SEMPRE ser passado para o Gemini no prompt visual. Isso garante que a IA geradora compreenda a escala física de objetos (árvores, barris) para que representem o hexágono como a menor unidade do D&D.
2. **Metadata e Tags:** As imagens geradas devem ser salvas com metadados (tags) e descrições textuais associadas, criando um banco de terrenos/mapas.
3. **Decisão Inteligente de Reuso:** Antes de gerar uma nova imagem, a IA tática deve procurar (buscar) no banco de mapas utilizando tags. Ela recebe a descrição textual dos mapas em cache e **decide** se um deles atende à necessidade atual ou se é estritamente necessário despachar uma requisição de geração (Gemini) para desenhar um mapa novo.

---

## 6. Fluxo de Execução e Mapeamento de Terreno (Geração -> Overlay -> Visão DeepSeek)

1. **Escopo Estrito de Criação de Mapa:**
   - O pipeline de mapa lida **exclusivamente** com a geração visual e o mapeamento físico dos terrenos nos hexágonos. **Não** insere personagens, nem lida com movimentação de unidades nesta fase.

2. **Fluxo Narrativo e Seleção de Mapa:**
   - **Gemini**: Conduz a narrativa principal da história e a descrição detalhada dos cenários.
   - **DeepSeek**: Conduz o roleplay de suporte e os diálogos/interações cotidianas (eficiência de custo).
   - **Busca por Tags (Banco Indexado)**: Ao disparar um encontro tático, o sistema busca mapas compatíveis no banco de dados indexado usando tags antes de gerar novas imagens.

3. **Mapeamento de Terrenos com Visão Computacional (DeepSeek):**
   - **Etapa A (Visual Gridless)**: O Gemini gera a imagem artística limpa, sem desenhar grades (Gridless).
   - **Etapa B (Overlay com Identificação)**: O motor aplica a grade de hexágonos sobre a imagem. Cada hexágono é rotulado visualmente com sua coordenada `(q, r)` ou identificador único.
   - **Etapa C (Análise Multimodal)**: A imagem renderizada com a grade rotulada é enviada para o **DeepSeek (Visão)**. O DeepSeek lê a imagem e mapeia precisamente qual o tipo de terreno/obstáculo de cada hexágono (ex: hex `(3, 4)` = cadeira quebrada / terreno difícil), retornando o JSON de `regions`.
   - **Etapa D (Persistência)**: O resultado (imagem + tags + mapeamento do grid) é indexado no Banco de Dados.

4. **Banco de Dados Escolhido (SurrealDB / Embutido):**
   - A persistência e indexação vetorial/tags dos mapas e metadados serão realizadas com o **SurrealDB** rodando embutido no motor Rust (RocksDB/Kv local). Ele oferece excelente suporte nativo a JSON/Documentos e consultas por tags/relacionamentos.


