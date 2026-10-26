import React, { useState, useEffect } from 'react';
import type { CampaignSession } from '../../types/session';
import type { CharacterSheet } from '../../types/character';
import {
  createCampaign,
  joinCampaign,
  fetchSavedCampaigns,
  importCampaignBackup,
  analyzePremise,
  fetchSavedPremises,
  adminSavePremise,
  adminDeletePremise,
  deleteCampaign,
  type PremiseAnalysis,
  type PrebuiltStory,
  type PartyRequirement,
} from '../../services/api';
import { createRequiredHero } from '../../utils/quickParty';
import { CharacterCreationWizard } from '../character-creation/CharacterCreationWizard';
import {
  Sparkles,
  Users,
  BookOpen,
  Sword,
  Shield,
  Copy,
  Check,
  Flame,
  Dice5,
  Globe2,
  Download,
  Upload,
  FolderOpen,
  Monitor,
  Play,
  ArrowRight,
  ArrowLeft,
  UserPlus,
  Trash2,
  CheckCircle2,
  Settings,
  Plus,
  Edit2,
  Radio,
  Search,
  Tag,
  Eye,
  X,
  AlertTriangle,
} from 'lucide-react';

import type { AuthUser } from '../../services/auth';
import { normalizeTags } from '../../utils/tags';

interface SessionLobbyProps {
  onSessionReady: (session: CampaignSession, initialParty?: CharacterSheet[]) => void;
  authUser?: AuthUser | null;
  onRequestLogin?: () => void;
}

export type LobbyTab = 'create_premise' | 'create_rules' | 'create_party' | 'waiting_room' | 'join' | 'resume';

export const SessionLobby: React.FC<SessionLobbyProps> = ({
  onSessionReady,
  authUser,
  onRequestLogin: _onRequestLogin,
}) => {
  const [tab, setTab] = useState<LobbyTab>('create_premise');
  const [loading, setLoading] = useState(false);
  const [error, setError] = useState<string | null>(null);
  const [successMessage, setSuccessMessage] = useState<string | null>(null);
  const [copiedCode, setCopiedCode] = useState(false);

  // Form de Criação do Mundo / Campanha
  const [premiseTitle, setPremiseTitle] = useState('');
  const [premise, setPremise] = useState('');
  const [literaryStyle, setLiteraryStyle] = useState('Fantasia Sombria, narrativa cinematográfica com foco tático');

  // Modo Um Terminal (Presencial / Hot-Seat) vs Modo Online
  const [isOfflineMode, setIsOfflineMode] = useState(true);
  const [partySize, setPartySize] = useState<number>(4);

  // Foco Dramático Contínuo D20 (1 a 20)
  const [dramaticSlider, setDramaticSlider] = useState<number>(10);

  // Composição de Heróis Pré-Campanha (1 a 4 heróis - começa zerada conforme especificação)
  const [partyHeroes, setPartyHeroes] = useState<CharacterSheet[]>([]);
  const [premiseAnalysis, setPremiseAnalysis] = useState<PremiseAnalysis | null>(null);
  const [isAnalyzing, setIsAnalyzing] = useState<boolean>(false);
  const [showInlineWizard, setShowInlineWizard] = useState<boolean>(false);
  const [editingHeroIndex, setEditingHeroIndex] = useState<number | null>(null);

  // Form de Entrada (Convidado & Modo Um Terminal)
  const [roomCode, setRoomCode] = useState('');
  const [campaignUuid, setCampaignUuid] = useState('');
  const [playerCodeInput, setPlayerCodeInput] = useState('');
  const [isSingleTerminalMode, setIsSingleTerminalMode] = useState(false);
  const [copiedUuid, setCopiedUuid] = useState(false);
  const [copiedPlayerCode, setCopiedPlayerCode] = useState(false);

  // Limites Criativos da IA
  const [aiCreativeFreedom, setAiCreativeFreedom] = useState<'strict' | 'balanced' | 'free'>('balanced');
  const [storyFocus, setStoryFocus] = useState<'combat' | 'roleplay' | 'd20'>('d20');

  // Sessão criada aguardando e Lobby WebSocket em Tempo Real
  const [createdSession, setCreatedSession] = useState<CampaignSession | null>(null);
  const [liveSlots, setLiveSlots] = useState<any[]>([]);
  const [wsConnected, setWsConnected] = useState<boolean>(false);

  // Campanhas salvas para retomar
  const [savedCampaigns, setSavedCampaigns] = useState<any[]>([]);

  // Catálogo e Painel de Histórias Pré-Construídas
  const [prebuiltStories, setPrebuiltStories] = useState<PrebuiltStory[]>([]);
  const [lobbyStorySearch, setLobbyStorySearch] = useState('');
  const [lobbyStoryTag, setLobbyStoryTag] = useState<string | null>(null);
  const [lobbyStoryPage, setLobbyStoryPage] = useState(1);
  const [lobbyPageSize, setLobbyPageSize] = useState<number>(5);
  const [premiseMode, setPremiseMode] = useState<'catalog' | 'custom'>('catalog');
  const [showStoryPanel, setShowStoryPanel] = useState<boolean>(false);
  const [newStoryTitle, setNewStoryTitle] = useState('');
  const [newStoryPremise, setNewStoryPremise] = useState('');
  const [newStoryStyle, setNewStoryStyle] = useState('Fantasia Sombria, narrativa cinematográfica com foco tático');
  const [newStorySlider, setNewStorySlider] = useState<number>(10);
  const [isSavingStory, setIsSavingStory] = useState(false);

  const loadPrebuiltStories = () => {
    fetchSavedPremises().then((stories) => {
      if (stories && stories.length > 0) {
        const normalized = stories.map((s) => ({
          ...s,
          tags: normalizeTags(s.tags),
        }));
        setPrebuiltStories(normalized);
      }
    });
  };

  useEffect(() => {
    fetchSavedCampaigns().then(setSavedCampaigns);
    loadPrebuiltStories();
  }, []);

  // Modal Amigável de Leitura & Edição de Contexto de Premissas
  const [inspectingStory, setInspectingStory] = useState<PrebuiltStory | null>(null);
  const [modalTitle, setModalTitle] = useState('');
  const [modalPremise, setModalPremise] = useState('');
  const [modalStyle, setModalStyle] = useState('');
  const [modalSlider, setModalSlider] = useState<number>(10);
  const [modalIsEditing, setModalIsEditing] = useState(false);

  const handleOpenStoryModal = (story: PrebuiltStory) => {
    setInspectingStory(story);
    setModalTitle(story.title);
    setModalPremise(story.premise);
    setModalStyle(story.literary_style || 'Fantasia Sombria, narrativa cinematográfica com foco tático');
    setModalSlider(story.dramatic_focus_slider || 10);
    setModalIsEditing(false);
  };

  const handleApplyStoryFromModal = () => {
    if (!inspectingStory) return;
    setPremiseTitle(modalTitle.trim() || inspectingStory.title);
    setPremise(modalPremise.trim() || inspectingStory.premise);
    setLiteraryStyle(modalStyle.trim() || inspectingStory.literary_style || 'Fantasia Sombria');
    setDramaticSlider(modalSlider || inspectingStory.dramatic_focus_slider || 10);

    if ((inspectingStory.custom_monsters && inspectingStory.custom_monsters.length > 0) ||
        (inspectingStory.subclass_overrides && inspectingStory.subclass_overrides.length > 0) ||
        (inspectingStory.custom_spells && inspectingStory.custom_spells.length > 0)) {
      setPremiseAnalysis({
        analysis_summary: `Premissa "${modalTitle}" configurada com sucesso.`,
        adventure_tone: modalStyle,
        recommended_party_size: "1 a 4 jogadores",
        has_thematic_requirement: false,
        subclass_overrides: inspectingStory.subclass_overrides || [],
        custom_spells: inspectingStory.custom_spells || [],
        custom_monsters: inspectingStory.custom_monsters || [],
        party_requirements: [],
        party_hooks: [
          `A comitiva inicia sua jornada inspirada nos acontecimentos de ${modalTitle}.`,
          "O destino da região depende da coragem e das escolhas imediatas do grupo."
        ]
      });
    }

    setInspectingStory(null);
  };

  const handleSelectPrebuiltStory = (story: PrebuiltStory) => {
    handleOpenStoryModal(story);
  };

  const handleSaveNewStory = async (e: React.FormEvent) => {
    e.preventDefault();
    if (!newStoryTitle.trim() || !newStoryPremise.trim()) {
      setError('Informe o título e a premissa da nova história.');
      return;
    }
    setIsSavingStory(true);
    try {
      await adminSavePremise({
        title: newStoryTitle.trim(),
        premise: newStoryPremise.trim(),
        literary_style: newStoryStyle.trim(),
        dramatic_focus_slider: newStorySlider,
      });
      setSuccessMessage(`Nova história "${newStoryTitle}" adicionada ao catálogo!`);
      setTimeout(() => setSuccessMessage(null), 3000);
      setNewStoryTitle('');
      setNewStoryPremise('');
      loadPrebuiltStories();
    } catch (err: any) {
      setError(err.message || 'Erro ao gravar história');
    } finally {
      setIsSavingStory(false);
    }
  };

  const handleDeleteStory = async (id: string, e?: React.MouseEvent) => {
    if (e) e.stopPropagation();
    try {
      await adminDeletePremise(id);
      loadPrebuiltStories();
      setSuccessMessage('História excluída do catálogo.');
      setTimeout(() => setSuccessMessage(null), 2000);
    } catch (err: any) {
      setError(err.message || 'Erro ao excluir história');
    }
  };

  // Avaliação estrita dos requisitos de roleplay exigidos pela IA
  const isRequirementSatisfied = (req: PartyRequirement, heroes: CharacterSheet[]): boolean => {
    if (req.requirement_type === 'free' || !req.is_mandatory) return true;

    return heroes.some((h) => {
      // 1. Nome específico exigido (SOMENTE se requirement_type for 'character_name' e for nome próprio real)
      if (
        req.requirement_type === 'character_name' &&
        req.required_name &&
        !req.required_class &&
        !h.name.toLowerCase().includes(req.required_name.toLowerCase())
      ) {
        return false;
      }

      // 2. Classe exigida (ex: Ranger / Druid / Fighter / Rogue / Cleric)
      if (req.required_class) {
        const rc = req.required_class.toLowerCase();
        const hc = h.className.toLowerCase();
        const isPal = rc.includes('paladin');
        const classMatch =
          hc === rc ||
          hc.includes(rc) ||
          rc.includes(hc) ||
          ((rc.includes('guerreir') || rc.includes('vanguarda') || rc.includes('atleta')) && hc === 'fighter') ||
          ((rc.includes('ladin') || rc.includes('espião') || rc.includes('acrobata')) && !isPal && hc === 'rogue') ||
          ((rc.includes('clérig') || rc.includes('clerig') || rc.includes('sacerdot')) && hc === 'cleric') ||
          ((rc.includes('mag') || rc.includes('mago') || rc.includes('artífice') || rc.includes('artifice') || rc.includes('inventor')) && hc === 'wizard') ||
          ((rc.includes('barbar') || rc.includes('bárbar')) && hc === 'barbarian') ||
          (isPal && hc === 'paladin') ||
          (rc.includes('bard') && hc === 'bard') ||
          ((rc.includes('druid') || rc.includes('xamã') || rc.includes('xama')) && hc === 'druid') ||
          ((rc.includes('monk') || rc.includes('monge')) && hc === 'monk') ||
          ((rc.includes('ranger') || rc.includes('patrulheir') || rc.includes('arqueir') || rc.includes('caçador') || rc.includes('cacador')) && hc === 'ranger');
        if (!classMatch) return false;
      }

      // 3. Raça exigida (ex: Elfo)
      if (req.required_race) {
        const rr = req.required_race.toLowerCase();
        const hr = h.race.toLowerCase();
        const raceMatch =
          hr === rr ||
          hr.includes(rr) ||
          (rr.includes('elf') && hr.includes('elf')) ||
          ((rr.includes('anão') || rr.includes('anao') || rr.includes('dwarf')) && (hr.includes('anão') || hr.includes('anao'))) ||
          ((rr.includes('lagarto') || rr.includes('draconat')) && (hr.includes('draconat') || hr.includes('lagarto')));
        if (!raceMatch) return false;
      }

      // 4. Se houver suggested_options (suporta formato narrativo com parênteses ex: "A Arqueira Elfa (Ranger)")
      if (!req.required_name && !req.required_class && req.suggested_options && req.suggested_options.length > 0) {
        const matchAny = req.suggested_options.some((opt) => {
          const o = opt.toLowerCase();
          const hc = h.className.toLowerCase();
          const hr = h.race.toLowerCase();
          const hn = h.name.toLowerCase();

          // Match direto no nome limpo sem parênteses
          const cleanOpt = o.replace(/\s*\(.*?\)/g, '').trim();
          if (hn.includes(cleanOpt) || cleanOpt.includes(hn)) return true;

          // Match em classe ou classe entre parênteses
          if (
            (o.includes('ranger') || o.includes('patrulheir') || o.includes('arqueir') || o.includes('caçador')) && hc === 'ranger' ||
            (o.includes('druid') || o.includes('xamã') || o.includes('xama')) && hc === 'druid' ||
            (o.includes('cleric') || o.includes('clérig') || o.includes('clerig') || o.includes('sacerdot')) && hc === 'cleric' ||
            (o.includes('paladin')) && hc === 'paladin' ||
            (o.includes('rogue') || o.includes('ladin')) && hc === 'rogue' ||
            (o.includes('wizard') || o.includes('mag') || o.includes('artífice') || o.includes('inventor')) && hc === 'wizard' ||
            (o.includes('barbar') || o.includes('bárbar')) && hc === 'barbarian' ||
            (o.includes('bard')) && hc === 'bard' ||
            (o.includes('monk') || o.includes('monge')) && hc === 'monk' ||
            (o.includes('fighter') || o.includes('guerreir')) && hc === 'fighter'
          ) {
            return true;
          }

          return hr.includes(cleanOpt) || hc.includes(cleanOpt);
        });
        if (!matchAny) return false;
      }

      return true;
    });
  };

  const unmetMandatoryReqs = (premiseAnalysis?.party_requirements || []).filter(
    (req) => req.is_mandatory && !isRequirementSatisfied(req, partyHeroes)
  );

  const maxPartySize = Math.min(
    6,
    Math.max(
      partySize || 4,
      partyHeroes.length,
      premiseAnalysis?.recommended_party_size?.includes('6') ? 6 :
      premiseAnalysis?.recommended_party_size?.includes('5') ? 5 :
      premiseAnalysis?.recommended_party_size?.toLowerCase().includes('quinteto') ? 5 :
      premiseAnalysis?.recommended_party_size?.toLowerCase().includes('sexteto') ? 6 : 4
    )
  );



  const handleProceedToParty = async () => {
    setError(null);
    setTab('create_party');

    if (premise.trim()) {
      // Se não tem análise OU se a análise atual veio de prebuilt e não tem party_requirements
      const needsFullAnalysis = !premiseAnalysis || (!premiseAnalysis.party_requirements || premiseAnalysis.party_requirements.length === 0);
      if (needsFullAnalysis) {
        setIsAnalyzing(true);
        try {
          const analysis = await analyzePremise({
            premise: premise.trim(),
            literary_style: literaryStyle.trim(),
            ai_creative_freedom: aiCreativeFreedom,
            dramatic_focus_slider: dramaticSlider,
          });
          setPremiseAnalysis((prev) => {
            const mergedMonsters = [...(prev?.custom_monsters || [])];
            for (const m of (analysis.custom_monsters || [])) {
              if (!mergedMonsters.some((existing) => existing.name === m.name)) {
                mergedMonsters.push(m);
              }
            }
            const mergedOverrides = [...(prev?.subclass_overrides || [])];
            for (const o of (analysis.subclass_overrides || [])) {
              if (!mergedOverrides.some((existing) => existing.name === o.name)) {
                mergedOverrides.push(o);
              }
            }
            const mergedSpells = [...(prev?.custom_spells || [])];
            for (const s of (analysis.custom_spells || [])) {
              if (!mergedSpells.some((existing) => existing.name === s.name)) {
                mergedSpells.push(s);
              }
            }
            return {
              ...analysis,
              custom_monsters: mergedMonsters,
              subclass_overrides: mergedOverrides,
              custom_spells: mergedSpells,
            };
          });
        } catch (err) {
          console.warn('Erro ao analisar premissa:', err);
        } finally {
          setIsAnalyzing(false);
        }
      }
    }
  };

  const handleCreate = async (e?: React.FormEvent) => {
    if (e) e.preventDefault();

    if (!premise.trim()) {
      setError('Por favor, informe a premissa da aventura na aba "1. Forjar" antes de criar o mundo.');
      setTab('create_premise');
      return;
    }
    if (partyHeroes.length === 0) {
      setError('O grupo de heróis precisa ter pelo menos 1 aventureiro antes de avançar.');
      setTab('create_party');
      return;
    }

    setLoading(true);
    setError(null);
    setSuccessMessage(null);
    try {
      const custom_monsters: any[] = premiseAnalysis?.custom_monsters || [];

      const partyPayload = partyHeroes.map((hero) => ({
        id: hero.id,
        name: hero.name,
        class: hero.className,
        race: hero.race,
        level: hero.level,
        hp: hero.vitals.hpMax,
        max_hp: hero.vitals.hpMax,
        ac: hero.vitals.armorClass,
        armor_class: hero.vitals.armorClass,
        ability_scores: {
          STR: hero.abilities.strength,
          DEX: hero.abilities.dexterity,
          CON: hero.abilities.constitution,
          INT: hero.abilities.intelligence,
          WIS: hero.abilities.wisdom,
          CHA: hero.abilities.charisma,
        },
        inventory: hero.inventory,
      }));

      const session = await createCampaign({
        premise: premise.trim(),
        literary_style: literaryStyle.trim() || 'Fantasia Sombria, narrativa cinematográfica com foco tático',
        custom_monsters: custom_monsters.length > 0 ? custom_monsters : undefined,
        ai_creative_freedom: aiCreativeFreedom,
        story_focus: storyFocus,
        dramatic_focus_slider: dramaticSlider,
        party: partyPayload,
      });

      const enrichedSession = {
        ...session,
        is_offline: isOfflineMode,
        is_single_terminal: isOfflineMode,
        party_size: partyHeroes.length,
        is_imported_backup: false,
      };

      setCreatedSession(enrichedSession);
      setLiveSlots(session.player_slots || []);

      // Inicia WebSocket para a Sala de Espera da Party
      try {
        const protocol = window.location.protocol === 'https:' ? 'wss:' : 'ws:';
        const wsUrl = `${protocol}//${window.location.host}/ws/${session.room_code}`;
        const ws = new WebSocket(wsUrl);

        ws.onopen = () => {
          setWsConnected(true);
        };

        ws.onmessage = (event) => {
          try {
            const data = JSON.parse(event.data);
            if (data.type === 'INITIAL_SYNC' && data.player_slots) {
              setLiveSlots(data.player_slots);
            } else if (data.type === 'PLAYER_JOINED' && data.player_slots) {
              setLiveSlots(data.player_slots);
            }
          } catch (e) {
            console.warn('WS message parse error:', e);
          }
        };

        ws.onclose = () => {
          setWsConnected(false);
        };
      } catch (wsErr) {
        console.warn('Erro ao conectar WebSocket do Lobby:', wsErr);
      }

      // Abre o Lobby de Espera (WebSocket)
      setTab('waiting_room');
    } catch (err: unknown) {
      setError(err instanceof Error ? err.message : 'Falha ao forjar campanha');
    } finally {
      setLoading(false);
    }
  };

  const handleJoin = async (e: React.FormEvent) => {
    e.preventDefault();
    if (!roomCode.trim()) {
      setError('Informe o código de 4 letras da sala.');
      return;
    }
    setLoading(true);
    setError(null);
    try {
      const session = await joinCampaign({
        campaign_id: campaignUuid.trim() || undefined,
        room_code: roomCode.trim().toUpperCase(),
        player_code: playerCodeInput.trim().toUpperCase() || undefined,
        is_single_terminal: isSingleTerminalMode,
      });
      onSessionReady(session);
    } catch (err: unknown) {
      setError(err instanceof Error ? err.message : 'Falha ao ingressar na sala');
    } finally {
      setLoading(false);
    }
  };

  // Exportar Premissa para arquivo .json local
  const handleExportPremiseFile = () => {
    const custom_monsters: any[] = [];
    const data = {
      title: premiseTitle,
      premise,
      literary_style: literaryStyle,
      ai_creative_freedom: aiCreativeFreedom,
      story_focus: storyFocus,
      custom_monsters,
      exported_at: new Date().toISOString(),
    };
    const blob = new Blob([JSON.stringify(data, null, 2)], { type: 'application/json' });
    const url = URL.createObjectURL(blob);
    const a = document.createElement('a');
    a.href = url;
    a.download = `premissa_${premiseTitle.toLowerCase().replace(/\s+/g, '_')}.json`;
    a.click();
    URL.revokeObjectURL(url);
    setSuccessMessage('Premissa exportada para arquivo JSON com sucesso!');
  };

  // Processador Polimórfico Inteligente para qualquer JSON (Campanha, Manifesto, Auditoria, Premissa ou Boss)
  const processUniversalJson = async (obj: any) => {
    setError(null);
    setSuccessMessage(null);

    // CASO 1: É uma Campanha Completa (Backup Bundle, Auditoria ou Manifesto direto)
    if (obj.manifest || obj.campaign_id || (obj.room_code && obj.premise)) {
      setLoading(true);
      try {
        const res = await importCampaignBackup(obj);
        const joined = await joinCampaign({ room_code: res.room_code });
        setSuccessMessage(`Campanha restaurada com sucesso! Código da sala: #${joined.room_code}`);
        onSessionReady({ ...joined, is_imported_backup: true });
        return;
      } catch (err: any) {
        setError(err.message || 'Falha ao restaurar backup da campanha.');
      } finally {
        setLoading(false);
      }
      return;
    }

    // CASO 2: É uma Ficha de Boss / Monstro (como branca_de_neve_princesa_corrompida.json)
    if (obj.entity_type === 'Npc' || obj.custom_flags?.is_homebrew_boss || obj.dnd_stats || (obj.name && (obj.hp || obj.dnd_stats?.max_hp))) {
      const bossName = obj.name || 'Chefe Customizado';
      const hp = obj.dnd_stats?.max_hp || obj.hp || 135;
      const ac = obj.dnd_stats?.armor_class || obj.ac || 16;
      const str = obj.dnd_stats?.ability_scores?.STR ?? 10;
      const dex = obj.dnd_stats?.ability_scores?.DEX ?? 16;
      const con = obj.dnd_stats?.ability_scores?.CON ?? 14;
      const int = obj.dnd_stats?.ability_scores?.INT ?? 14;
      const wis = obj.dnd_stats?.ability_scores?.WIS ?? 12;
      const cha = obj.dnd_stats?.ability_scores?.CHA ?? 20;

      const rawText = `\n\n[ENTIDADE / BOSS HOMEBREW]\nNome: ${bossName} | PV: ${hp} | CA: ${ac} | FOR ${str} DES ${dex} CON ${con} INT ${int} SAB ${wis} CAR ${cha}\nDescrição: ${obj.description || 'Antagonista principal'}`;

      setPremise((prev) => (prev ? prev + rawText : rawText.trim()));
      setTab('create_premise');
      setSuccessMessage(`Boss "${bossName}" (PV: ${hp}, CA: ${ac}) anexado diretamente à Premissa Unificada!`);
      return;
    }

    // CASO 3: É uma Premissa (arquivo premissa_*.json)
    if (obj.premise || obj.title) {
      let combinedPremise = obj.premise || '';
      if (obj.custom_monsters && Array.isArray(obj.custom_monsters) && obj.custom_monsters.length > 0) {
        const bossesSummary = obj.custom_monsters.map((m: any) => 
          `\n[MONSTRO / BOSS HOMEBREW]: ${m.name} | PV: ${m.hp || 100} | CA: ${m.ac || 15} | ND: ${m.challenge_rating || '?'}\nFOR ${m.str || 10} DES ${m.dex || 10} CON ${m.con || 10} INT ${m.int || 10} SAB ${m.wis || 10} CAR ${m.cha || 10}\n${m.description || ''}`
        ).join('\n');
        combinedPremise = combinedPremise ? `${combinedPremise}\n${bossesSummary}` : bossesSummary;
      }
      setPremise(combinedPremise);
      if (obj.literary_style) setLiteraryStyle(obj.literary_style);
      if (obj.title) setPremiseTitle(obj.title);
      if (obj.ai_creative_freedom) setAiCreativeFreedom(obj.ai_creative_freedom);
      if (obj.story_focus) setStoryFocus(obj.story_focus);
      setSuccessMessage('Premissa e entidades integradas com sucesso à Entrada Unificada!');
      setTab('create_premise');
      return;
    }

    setError('Formato JSON não reconhecido como Campanha, Premissa ou Ficha de Monstro.');
  };

  // Importar Premissa de arquivo .json
  const handleImportPremiseFile = (e: React.ChangeEvent<HTMLInputElement>) => {
    const file = e.target.files?.[0];
    if (!file) return;
    const reader = new FileReader();
    reader.onload = (event) => {
      try {
        const obj = JSON.parse(event.target?.result as string);
        processUniversalJson(obj);
      } catch {
        setError('Arquivo JSON corrompido ou inválido.');
      }
    };
    reader.readAsText(file);
    e.target.value = '';
  };

  // Importar Campanha externa (JSON Backup)
  const handleImportCampaignFile = (e: React.ChangeEvent<HTMLInputElement>) => {
    const file = e.target.files?.[0];
    if (!file) return;
    const reader = new FileReader();
    reader.onload = async (event) => {
      try {
        const obj = JSON.parse(event.target?.result as string);
        processUniversalJson(obj);
      } catch (err: any) {
        setError(err.message || 'Falha ao restaurar backup da campanha.');
      }
    };
    reader.readAsText(file);
    e.target.value = '';
  };

  const copyRoomCode = () => {
    if (!createdSession?.room_code) return;
    navigator.clipboard.writeText(createdSession.room_code);
    setCopiedCode(true);
    setTimeout(() => setCopiedCode(false), 2500);
  };

  const copyUuid = () => {
    if (!createdSession?.campaign_id) return;
    navigator.clipboard.writeText(createdSession.campaign_id);
    setCopiedUuid(true);
    setTimeout(() => setCopiedUuid(false), 2500);
  };

  const copyPlayerCode = () => {
    if (!createdSession?.player_code) return;
    navigator.clipboard.writeText(createdSession.player_code);
    setCopiedPlayerCode(true);
    setTimeout(() => setCopiedPlayerCode(false), 2500);
  };

  return (
    <div className="rpg-lobby-viewport">
      <style>{`
        .rpg-lobby-viewport {
          min-height: calc(100vh - 48px);
          display: flex;
          align-items: center;
          justify-content: center;
          padding: 24px 20px;
          background:
            linear-gradient(180deg, rgba(7, 9, 13, 0.72) 0%, rgba(7, 9, 13, 0.88) 100%),
            url('/assets/lobby_background.jpg') center center / cover no-repeat fixed;
          color: #f8fafc;
          position: relative;
        }
        .rpg-grimoire-shell {
          width: 100%;
          max-width: 1040px;
          background: linear-gradient(180deg, rgba(17, 21, 31, 0.88) 0%, rgba(11, 14, 20, 0.94) 100%);
          backdrop-filter: blur(14px);
          -webkit-backdrop-filter: blur(14px);
          border: 1px solid rgba(168, 85, 247, 0.28);
          border-radius: 20px;
          padding: 28px 32px;
          box-shadow:
            0 30px 70px rgba(0, 0, 0, 0.9),
            0 0 35px rgba(168, 85, 247, 0.12),
            0 0 0 1px rgba(255, 255, 255, 0.05) inset;
          display: flex;
          flex-direction: column;
          gap: 20px;
          position: relative;
        }
        .rpg-panel-box {
          background: rgba(9, 11, 16, 0.82);
          backdrop-filter: blur(10px);
          -webkit-backdrop-filter: blur(10px);
          border: 1px solid rgba(255, 255, 255, 0.08);
          border-radius: 14px;
          padding: 22px;
          display: flex;
          flex-direction: column;
          gap: 16px;
        }
        .rpg-input-field {
          width: 100%;
          background-color: #06080c;
          border: 1px solid rgba(255, 255, 255, 0.12);
          border-radius: 10px;
          padding: 11px 14px;
          font-size: 13px;
          color: #f1f5f9;
          outline: none;
          transition: all 0.18s ease;
          box-sizing: border-box;
        }
        .rpg-input-field:focus {
          border-color: #eab308;
          box-shadow: 0 0 0 3px rgba(234, 179, 8, 0.14);
        }
        .rpg-label {
          font-size: 11px;
          font-weight: 800;
          color: #94a3b8;
          text-transform: uppercase;
          letter-spacing: 0.08em;
          display: flex;
          align-items: center;
          justify-content: space-between;
          gap: 6px;
        }
        .rpg-freedom-grid {
          display: grid;
          grid-template-columns: repeat(3, 1fr);
          gap: 10px;
        }
        .rpg-freedom-card {
          background: #06080c;
          border: 1px solid rgba(255, 255, 255, 0.08);
          border-radius: 10px;
          padding: 11px 13px;
          cursor: pointer;
          transition: all 0.16s ease;
          display: flex;
          flex-direction: column;
          gap: 4px;
        }
        .rpg-freedom-card:hover {
          border-color: rgba(168, 85, 247, 0.45);
        }
        .rpg-freedom-card.active {
          background: linear-gradient(135deg, rgba(168, 85, 247, 0.18) 0%, rgba(168, 85, 247, 0.04) 100%);
          border-color: #a855f7;
          box-shadow: 0 0 15px rgba(168, 85, 247, 0.15);
        }
        .rpg-preset-pill {
          background: rgba(255, 255, 255, 0.04);
          border: 1px solid rgba(255, 255, 255, 0.1);
          color: #cbd5e1;
          font-size: 11px;
          font-weight: 600;
          padding: 4px 10px;
          border-radius: 999px;
          cursor: pointer;
          transition: all 0.15s ease;
        }
        .rpg-preset-pill:hover {
          border-color: #eab308;
          color: #eab308;
          background: rgba(234, 179, 8, 0.08);
        }
        .rpg-cta-gold {
          width: 100%;
          padding: 15px 22px;
          background: linear-gradient(135deg, #facc15 0%, #ca8a04 100%);
          color: #090b0e;
          font-size: 14px;
          font-weight: 900;
          text-transform: uppercase;
          letter-spacing: 0.08em;
          border: none;
          border-radius: 12px;
          box-shadow: 0 6px 24px rgba(234, 179, 8, 0.32);
          cursor: pointer;
          display: flex;
          align-items: center;
          justify-content: center;
          gap: 10px;
          transition: all 0.2s ease;
        }
        .rpg-cta-gold:hover:not(:disabled) {
          transform: translateY(-2px);
          box-shadow: 0 10px 30px rgba(234, 179, 8, 0.45);
          filter: brightness(1.05);
        }
        .rpg-cta-gold:disabled {
          opacity: 0.55;
          cursor: not-allowed;
        }
        @media (max-width: 820px) {
          .rpg-grimoire-shell {
            padding: 20px 16px;
          }
          .rpg-freedom-grid {
            grid-template-columns: 1fr;
          }
        }
      `}</style>

      <div className="rpg-grimoire-shell">
        {/* HEADER */}
        <div
          style={{
            display: 'flex',
            justifyContent: 'space-between',
            alignItems: 'center',
            flexWrap: 'wrap',
            gap: '16px',
            paddingBottom: '16px',
            borderBottom: '1px solid rgba(255, 255, 255, 0.08)',
          }}
        >
          <div>
            <div
              style={{
                display: 'inline-flex',
                alignItems: 'center',
                gap: '6px',
                padding: '4px 12px',
                backgroundColor: 'rgba(234, 179, 8, 0.12)',
                border: '1px solid rgba(234, 179, 8, 0.35)',
                borderRadius: '999px',
                color: '#facc15',
                fontSize: '11px',
                fontWeight: 800,
                textTransform: 'uppercase',
                letterSpacing: '0.08em',
                marginBottom: '8px',
              }}
            >
              <Sparkles size={13} /> Motor Procedural D&amp;D 5e • Forja &amp; Crônicas
            </div>
            <h1
              className="rpg-title"
              style={{
                fontSize: 'clamp(22px, 3vw, 28px)',
                color: '#ffffff',
                margin: '0 0 4px 0',
                fontWeight: 900,
                letterSpacing: '0.02em',
                textShadow: '0 0 24px rgba(234, 179, 8, 0.22)',
              }}
            >
              LOBBY DA CAMPANHA &amp; ARQUITETURA DO MUNDO
            </h1>
            <p style={{ color: '#94a3b8', fontSize: '13px', margin: 0 }}>
              Jogue online via WebSockets ou em Modo Offline (Hot-Seat local para 1 a 4 jogadores).
            </p>
          </div>

          {/* Seletor de Abas */}
          {!createdSession && (
            <div
              style={{
                display: 'flex',
                gap: '6px',
                backgroundColor: '#06080c',
                padding: '5px',
                borderRadius: '12px',
                border: '1px solid rgba(255, 255, 255, 0.1)',
                flexWrap: 'wrap',
              }}
            >
              <button
                type="button"
                onClick={() => {
                  setTab('create_premise');
                  setError(null);
                }}
                style={{
                  padding: '8px 14px',
                  borderRadius: '8px',
                  fontSize: '12px',
                  fontWeight: 700,
                  display: 'flex',
                  alignItems: 'center',
                  gap: '6px',
                  backgroundColor: tab === 'create_premise' ? 'rgba(234, 179, 8, 0.16)' : 'transparent',
                  color: tab === 'create_premise' ? '#facc15' : '#94a3b8',
                  border: tab === 'create_premise' ? '1px solid rgba(234, 179, 8, 0.45)' : '1px solid transparent',
                  cursor: 'pointer',
                }}
              >
                <BookOpen size={14} /> 1. Forjar
              </button>

              <button
                type="button"
                onClick={() => {
                  setTab('create_rules');
                  setError(null);
                }}
                style={{
                  padding: '8px 14px',
                  borderRadius: '8px',
                  fontSize: '12px',
                  fontWeight: 700,
                  display: 'flex',
                  alignItems: 'center',
                  gap: '6px',
                  backgroundColor: tab === 'create_rules' ? 'rgba(234, 179, 8, 0.16)' : 'transparent',
                  color: tab === 'create_rules' ? '#facc15' : '#94a3b8',
                  border: tab === 'create_rules' ? '1px solid rgba(234, 179, 8, 0.45)' : '1px solid transparent',
                  cursor: 'pointer',
                }}
              >
                <Dice5 size={14} /> 2. Regras &amp; Foco D20
              </button>

              <button
                type="button"
                data-testid="tab-lobby-party"
                onClick={() => {
                  handleProceedToParty();
                }}
                style={{
                  padding: '8px 14px',
                  borderRadius: '8px',
                  fontSize: '12px',
                  fontWeight: 700,
                  display: 'flex',
                  alignItems: 'center',
                  gap: '6px',
                  backgroundColor: tab === 'create_party' ? 'rgba(56, 189, 248, 0.16)' : 'transparent',
                  color: tab === 'create_party' ? '#38bdf8' : '#94a3b8',
                  border: tab === 'create_party' ? '1px solid rgba(56, 189, 248, 0.45)' : '1px solid transparent',
                  cursor: 'pointer',
                }}
              >
                <Users size={14} /> 3. Grupo de Heróis ({partyHeroes.length})
              </button>

              <button
                type="button"
                onClick={() => {
                  setTab('join');
                  setError(null);
                }}
                style={{
                  padding: '8px 14px',
                  borderRadius: '8px',
                  fontSize: '12px',
                  fontWeight: 700,
                  display: 'flex',
                  alignItems: 'center',
                  gap: '6px',
                  backgroundColor: tab === 'join' ? 'rgba(16, 185, 129, 0.16)' : 'transparent',
                  color: tab === 'join' ? '#34d399' : '#94a3b8',
                  border: tab === 'join' ? '1px solid rgba(16, 185, 129, 0.45)' : '1px solid transparent',
                  cursor: 'pointer',
                }}
              >
                <Shield size={14} /> 4. Ingressar
              </button>

              <button
                type="button"
                onClick={() => {
                  setTab('resume');
                  setError(null);
                }}
                style={{
                  padding: '8px 14px',
                  borderRadius: '8px',
                  fontSize: '12px',
                  fontWeight: 700,
                  display: 'flex',
                  alignItems: 'center',
                  gap: '6px',
                  backgroundColor: tab === 'resume' ? 'rgba(168, 85, 247, 0.16)' : 'transparent',
                  color: tab === 'resume' ? '#c084fc' : '#94a3b8',
                  border: tab === 'resume' ? '1px solid rgba(168, 85, 247, 0.45)' : '1px solid transparent',
                  cursor: 'pointer',
                }}
              >
                <FolderOpen size={14} /> 5. Campanhas Salvas &amp; JSON
              </button>
            </div>
          )}
        </div>

        {/* FEEDBACK DE MENSAGENS */}
        {error && (
          <div
            style={{
              padding: '12px 16px',
              backgroundColor: 'rgba(239, 68, 68, 0.12)',
              border: '1px solid rgba(239, 68, 68, 0.4)',
              borderRadius: '10px',
              color: '#fca5a5',
              fontSize: '13px',
              display: 'flex',
              alignItems: 'center',
              gap: '8px',
            }}
          >
            <strong style={{ color: '#ef4444' }}>⚠️ Falha:</strong> {error}
          </div>
        )}
        {successMessage && (
          <div
            style={{
              padding: '12px 16px',
              backgroundColor: 'rgba(16, 185, 129, 0.15)',
              border: '1px solid rgba(16, 185, 129, 0.4)',
              borderRadius: '10px',
              color: '#6ee7b7',
              fontSize: '13px',
            }}
          >
            ✓ {successMessage}
          </div>
        )}

        {/* ESTADO 1: CAMPANHA CRIADA (SALA PRONTA) - Usado apenas se explicitamente em modo resumo legado */}
        {createdSession && tab !== 'waiting_room' && tab !== 'create_premise' && tab !== 'create_rules' && tab !== 'create_party' ? (
          <div style={{ display: 'flex', flexDirection: 'column', gap: '22px', maxWidth: '680px', margin: '12px auto', width: '100%' }}>
            <div
              style={{
                padding: '28px 24px',
                background: 'linear-gradient(135deg, rgba(16, 185, 129, 0.12) 0%, rgba(6, 78, 59, 0.18) 100%)',
                border: '1px solid rgba(16, 185, 129, 0.4)',
                borderRadius: '16px',
                display: 'flex',
                flexDirection: 'column',
                alignItems: 'center',
                textAlign: 'center',
                gap: '14px',
              }}
            >
              <span
                style={{
                  color: '#34d399',
                  fontSize: '11px',
                  fontWeight: 800,
                  textTransform: 'uppercase',
                  letterSpacing: '0.12em',
                  backgroundColor: 'rgba(16, 185, 129, 0.15)',
                  padding: '4px 12px',
                  borderRadius: '999px',
                }}
              >
                {createdSession.is_offline ? '🖥️ Modo Offline Local (Mesmo PC)' : '✓ Mundo Gerado com Sucesso'}
              </span>

              <h2 style={{ fontSize: '16px', color: '#e2e8f0', margin: 0, fontWeight: 700 }}>
                Códigos de Acesso da Campanha:
              </h2>

              {/* Código de 4 Letras da Sala */}
              <div style={{ display: 'flex', flexDirection: 'column', alignItems: 'center', gap: '6px' }}>
                <span style={{ fontSize: '11px', color: '#94a3b8', textTransform: 'uppercase', letterSpacing: '0.08em', fontWeight: 600 }}>
                  Código da Sala (4 Letras)
                </span>
                <div style={{ display: 'flex', alignItems: 'center', gap: '10px', flexWrap: 'wrap', justifyContent: 'center' }}>
                  <div
                    style={{
                      fontSize: '36px',
                      fontFamily: 'var(--font-mono, monospace)',
                      fontWeight: 900,
                      color: '#facc15',
                      letterSpacing: '0.25em',
                      backgroundColor: '#06080c',
                      padding: '8px 24px 8px 32px',
                      borderRadius: '12px',
                      border: '1.5px solid rgba(234, 179, 8, 0.5)',
                      boxShadow: '0 0 25px rgba(234, 179, 8, 0.15)',
                      userSelect: 'all',
                    }}
                  >
                    {createdSession.room_code}
                  </div>

                  <button
                    type="button"
                    onClick={copyRoomCode}
                    style={{
                      padding: '12px 16px',
                      borderRadius: '10px',
                      backgroundColor: copiedCode ? 'rgba(16, 185, 129, 0.2)' : 'rgba(255, 255, 255, 0.06)',
                      border: copiedCode ? '1px solid #10b981' : '1px solid rgba(255, 255, 255, 0.15)',
                      color: copiedCode ? '#34d399' : '#f8fafc',
                      fontWeight: 700,
                      fontSize: '12px',
                      cursor: 'pointer',
                      display: 'flex',
                      alignItems: 'center',
                      gap: '6px',
                    }}
                  >
                    {copiedCode ? <Check size={15} /> : <Copy size={15} />}
                    {copiedCode ? 'Copiado!' : 'Copiar'}
                  </button>
                </div>
              </div>

              {/* UUID da Campanha */}
              <div style={{ display: 'flex', flexDirection: 'column', alignItems: 'center', gap: '4px', width: '100%', maxWidth: '480px' }}>
                <span style={{ fontSize: '11px', color: '#94a3b8', textTransform: 'uppercase', letterSpacing: '0.08em', fontWeight: 600 }}>
                  UUID da Campanha (Para Reconexão &amp; Modo Híbrido)
                </span>
                <div style={{ display: 'flex', alignItems: 'center', gap: '8px', width: '100%' }}>
                  <div
                    style={{
                      flex: 1,
                      fontSize: '12px',
                      fontFamily: 'var(--font-mono, monospace)',
                      color: '#93c5fd',
                      backgroundColor: '#080c14',
                      padding: '8px 12px',
                      borderRadius: '8px',
                      border: '1px solid rgba(56, 189, 248, 0.25)',
                      overflow: 'hidden',
                      textOverflow: 'ellipsis',
                      whiteSpace: 'nowrap',
                    }}
                  >
                    {createdSession.campaign_id}
                  </div>
                  <button
                    type="button"
                    onClick={copyUuid}
                    style={{
                      padding: '8px 12px',
                      borderRadius: '8px',
                      backgroundColor: copiedUuid ? 'rgba(16, 185, 129, 0.2)' : 'rgba(255, 255, 255, 0.06)',
                      border: copiedUuid ? '1px solid #10b981' : '1px solid rgba(255, 255, 255, 0.15)',
                      color: copiedUuid ? '#34d399' : '#f8fafc',
                      fontWeight: 700,
                      fontSize: '11px',
                      cursor: 'pointer',
                      display: 'flex',
                      alignItems: 'center',
                      gap: '4px',
                      whiteSpace: 'nowrap',
                    }}
                  >
                    {copiedUuid ? <Check size={13} /> : <Copy size={13} />}
                    {copiedUuid ? 'Copiado!' : 'Copiar UUID'}
                  </button>
                </div>
              </div>

              {/* Slot do Host: Jogador #1 */}
              <div
                style={{
                  display: 'flex',
                  alignItems: 'center',
                  justifyContent: 'space-between',
                  gap: '12px',
                  backgroundColor: 'rgba(234, 179, 8, 0.08)',
                  border: '1px solid rgba(234, 179, 8, 0.25)',
                  borderRadius: '8px',
                  padding: '8px 14px',
                  width: '100%',
                  maxWidth: '480px',
                  fontSize: '12px',
                }}
              >
                <div style={{ display: 'flex', alignItems: 'center', gap: '6px' }}>
                  <span style={{ fontWeight: 800, color: 'var(--accent-gold)' }}>Jogador #1 (Host):</span>
                  <span style={{ color: '#cbd5e1' }}>Código Pessoal:</span>
                  <strong style={{ fontFamily: 'monospace', color: '#facc15', fontSize: '13px' }}>
                    {createdSession.player_code || 'HOST'}
                  </strong>
                </div>

                {createdSession.player_code && (
                  <button
                    type="button"
                    onClick={copyPlayerCode}
                    style={{
                      padding: '4px 8px',
                      borderRadius: '6px',
                      backgroundColor: copiedPlayerCode ? 'rgba(16, 185, 129, 0.2)' : 'transparent',
                      border: '1px solid rgba(255, 255, 255, 0.15)',
                      color: copiedPlayerCode ? '#34d399' : '#cbd5e1',
                      fontSize: '10px',
                      cursor: 'pointer',
                      display: 'inline-flex',
                      alignItems: 'center',
                      gap: '4px',
                    }}
                  >
                    {copiedPlayerCode ? <Check size={11} /> : <Copy size={11} />}
                    {copiedPlayerCode ? 'Copiado' : 'Copiar'}
                  </button>
                )}
              </div>

              {/* Checkbox Modo Um Terminal */}
              <label
                style={{
                  display: 'flex',
                  alignItems: 'center',
                  gap: '8px',
                  fontSize: '13px',
                  color: '#e2e8f0',
                  cursor: 'pointer',
                  backgroundColor: createdSession.is_single_terminal ? 'rgba(56, 189, 248, 0.15)' : 'rgba(255, 255, 255, 0.03)',
                  border: createdSession.is_single_terminal ? '1px solid #38bdf8' : '1px solid rgba(255, 255, 255, 0.1)',
                  borderRadius: '8px',
                  padding: '8px 14px',
                  width: '100%',
                  maxWidth: '480px',
                  transition: 'all 0.2s',
                }}
              >
                <input
                  type="checkbox"
                  checked={Boolean(createdSession.is_single_terminal)}
                  onChange={(e) => {
                    setCreatedSession((prev) => (prev ? { ...prev, is_single_terminal: e.target.checked } : null));
                  }}
                  style={{ cursor: 'pointer' }}
                />
                <span>🖥️ Ativar <strong>Modo Um Terminal</strong> (Presencial / Sem WebSockets)</span>
              </label>

              <p style={{ color: '#94a3b8', fontSize: '12px', margin: 0, textAlign: 'center', maxWidth: '480px' }}>
                Você pode começar presencialmente no mesmo terminal e depois os jogadores podem entrar pelo celular/notebook usando o UUID + Código da sala!
              </p>
            </div>

            <div className="rpg-panel-box" style={{ gap: '12px', fontSize: '13px' }}>
              <div style={{ display: 'flex', justifyContent: 'space-between', gap: '16px', borderBottom: '1px solid rgba(255,255,255,0.06)', paddingBottom: '10px' }}>
                <span style={{ color: '#94a3b8', fontWeight: 600, flexShrink: 0 }}>Premissa Central:</span>
                <span style={{ color: '#e2e8f0', textAlign: 'right' }}>{createdSession.premise}</span>
              </div>
              <div style={{ display: 'flex', justifyContent: 'space-between', gap: '16px' }}>
                <span style={{ color: '#94a3b8', fontWeight: 600, flexShrink: 0 }}>Direção Narrativa:</span>
                <span style={{ color: '#cbd5e1', textAlign: 'right' }}>{createdSession.literary_style}</span>
              </div>
            </div>

            <button onClick={() => onSessionReady(createdSession)} className="rpg-cta-gold">
              <Sword size={18} /> Avançar para a Forja de Personagens &gt;&gt;
            </button>
          </div>
        ) : tab === 'create_premise' ? (
          /* TAB 1: FORJA DE MUNDO (PREMISSA & NARRATIVA) */
          <div className="rpg-panel-box" style={{ gap: '18px' }}>
            <div style={{ display: 'flex', justifyContent: 'space-between', alignItems: 'center', flexWrap: 'wrap', gap: '10px' }}>
              <div style={{ display: 'flex', alignItems: 'center', gap: '8px' }}>
                <BookOpen size={18} color="#facc15" />
                <div>
                  <h2 style={{ fontSize: '15px', fontWeight: 800, color: '#ffffff', margin: 0, textTransform: 'uppercase', letterSpacing: '0.04em' }}>
                    1. Forjar (Narrativa &amp; Premissa)
                  </h2>
                  <span style={{ fontSize: '12px', color: '#94a3b8' }}>
                    Defina o título, o conflito principal e o tom literário da sua aventura.
                  </span>
                </div>
              </div>

              {/* Botão para Painel de Histórias (Somente Admin) ou Status do Catálogo */}
              {authUser?.role === 'admin' && (
                <button
                  type="button"
                  onClick={() => setShowStoryPanel(true)}
                  className="rpg-preset-pill"
                  data-testid="open-story-panel-btn"
                  style={{
                    display: 'flex',
                    alignItems: 'center',
                    gap: '6px',
                    backgroundColor: 'rgba(234, 179, 8, 0.15)',
                    border: '1px solid rgba(234, 179, 8, 0.45)',
                    color: '#facc15',
                    padding: '7px 14px',
                    fontSize: '12px',
                    fontWeight: 800,
                    cursor: 'pointer',
                    borderRadius: '8px',
                  }}
                >
                  <Settings size={14} /> ⚙️ Catálogo Administrativo ({prebuiltStories.length})
                </button>
              )}
            </div>

            {/* SUB-ABAS: CATÁLOGO OFICIAL VS FORJA DO ZERO */}
            <div style={{ display: 'flex', gap: '8px', borderBottom: '1px solid rgba(255, 255, 255, 0.08)', paddingBottom: '12px' }}>
              <button
                type="button"
                onClick={() => setPremiseMode('catalog')}
                data-testid="tab-premise-catalog"
                style={{
                  display: 'flex',
                  alignItems: 'center',
                  gap: '6px',
                  padding: '8px 16px',
                  borderRadius: '8px',
                  fontSize: '12.5px',
                  fontWeight: 800,
                  cursor: 'pointer',
                  border: premiseMode === 'catalog' ? '1px solid #facc15' : '1px solid rgba(255, 255, 255, 0.08)',
                  backgroundColor: premiseMode === 'catalog' ? 'rgba(234, 179, 8, 0.16)' : 'rgba(255, 255, 255, 0.02)',
                  color: premiseMode === 'catalog' ? '#facc15' : '#94a3b8',
                  transition: 'all 0.15s ease',
                }}
              >
                <Sparkles size={14} /> 📚 Usar História Pré-Construída ({prebuiltStories.length})
              </button>

              <button
                type="button"
                onClick={() => setPremiseMode('custom')}
                data-testid="tab-premise-custom"
                style={{
                  display: 'flex',
                  alignItems: 'center',
                  gap: '6px',
                  padding: '8px 16px',
                  borderRadius: '8px',
                  fontSize: '12.5px',
                  fontWeight: 800,
                  cursor: 'pointer',
                  border: premiseMode === 'custom' ? '1px solid #38bdf8' : '1px solid rgba(255, 255, 255, 0.08)',
                  backgroundColor: premiseMode === 'custom' ? 'rgba(56, 189, 248, 0.16)' : 'rgba(255, 255, 255, 0.02)',
                  color: premiseMode === 'custom' ? '#38bdf8' : '#94a3b8',
                  transition: 'all 0.15s ease',
                }}
              >
                <Edit2 size={14} /> ✍️ Forjar Nova Narrativa do Zero
              </button>
            </div>

            {/* CONTEÚDO DA SUB-ABA 1: CATÁLOGO OFICIAL */}
            {premiseMode === 'catalog' ? (
              <div style={{ display: 'flex', flexDirection: 'column', gap: '14px' }}>
                {prebuiltStories.length > 0 && (() => {
                  const allTags = Array.from(
                    new Set(
                      prebuiltStories
                        .flatMap((s) => normalizeTags(s.tags))
                        .map((t) => t.toLowerCase().trim())
                        .filter(Boolean)
                    )
                  ).sort();

                  const filtered = prebuiltStories.filter((s) => {
                    const q = lobbyStorySearch.toLowerCase().trim();
                    const sTags = normalizeTags(s.tags);
                    const matchesQ =
                      !q ||
                      s.title.toLowerCase().includes(q) ||
                      s.premise.toLowerCase().includes(q) ||
                      (s.literary_style || '').toLowerCase().includes(q) ||
                      sTags.some((t) => t.toLowerCase().includes(q));

                    const matchesT =
                      !lobbyStoryTag ||
                      sTags.some((t) => t.toLowerCase() === lobbyStoryTag.toLowerCase().trim());
                    return matchesQ && matchesT;
                  });

                  const totalLobbyPages = Math.max(1, Math.ceil(filtered.length / lobbyPageSize));
                  const currentLobbyPage = Math.min(lobbyStoryPage, totalLobbyPages);
                  const startIdx = filtered.length === 0 ? 0 : (currentLobbyPage - 1) * lobbyPageSize + 1;
                  const endIdx = Math.min(filtered.length, currentLobbyPage * lobbyPageSize);
                  const pagedStories = filtered.slice(
                    (currentLobbyPage - 1) * lobbyPageSize,
                    currentLobbyPage * lobbyPageSize
                  );

                  return (
                    <div
                      style={{
                        padding: '12px 14px',
                        backgroundColor: 'rgba(234, 179, 8, 0.05)',
                        border: '1px solid rgba(234, 179, 8, 0.25)',
                        borderRadius: '10px',
                        display: 'flex',
                        flexDirection: 'column',
                        gap: '10px',
                      }}
                    >
                      <div style={{ display: 'flex', justifyContent: 'space-between', alignItems: 'center', flexWrap: 'wrap', gap: '8px' }}>
                        <span style={{ fontSize: '11px', fontWeight: 800, color: '#facc15', textTransform: 'uppercase', letterSpacing: '0.08em', display: 'flex', alignItems: 'center', gap: '5px' }}>
                          <Sparkles size={12} /> Catálogo Oficial de Histórias ({prebuiltStories.length}):
                        </span>
                        <span style={{ fontSize: '11px', color: '#94a3b8' }}>
                          {filtered.length} encontrada{filtered.length !== 1 ? 's' : ''} (Clique em qualquer linha para abrir os detalhes)
                        </span>
                      </div>

                      {/* Barra de Pesquisa Rápida */}
                      <div style={{ position: 'relative' }}>
                        <Search size={14} color="#64748b" style={{ position: 'absolute', left: '10px', top: '50%', transform: 'translateY(-50%)' }} />
                        <input
                          type="text"
                          placeholder="Filtrar por título, tags (#fantasia, #sombria, #scifi) ou estilo..."
                          value={lobbyStorySearch}
                          onChange={(e) => {
                            setLobbyStorySearch(e.target.value);
                            setLobbyStoryPage(1);
                          }}
                          data-testid="lobby-story-search-input"
                          style={{
                            width: '100%',
                            padding: '7px 10px 7px 32px',
                            backgroundColor: '#0c1017',
                            border: '1px solid rgba(234, 179, 8, 0.25)',
                            borderRadius: '6px',
                            color: '#f8fafc',
                            fontSize: '11.5px',
                            boxSizing: 'border-box',
                          }}
                        />
                        {lobbyStorySearch && (
                          <button
                            type="button"
                            onClick={() => {
                              setLobbyStorySearch('');
                              setLobbyStoryPage(1);
                            }}
                            style={{
                              position: 'absolute',
                              right: '8px',
                              top: '50%',
                              transform: 'translateY(-50%)',
                              background: 'none',
                              border: 'none',
                              color: '#94a3b8',
                              cursor: 'pointer',
                              fontSize: '11px',
                            }}
                          >
                            ✕
                          </button>
                        )}
                      </div>

                      {/* Filtros por Tags */}
                      {allTags.length > 0 && (
                        <div style={{ display: 'flex', alignItems: 'center', gap: '5px', flexWrap: 'wrap' }}>
                          <span style={{ fontSize: '10.5px', color: '#64748b', display: 'flex', alignItems: 'center', gap: '3px' }}>
                            <Tag size={11} /> Tags:
                          </span>
                          <button
                            type="button"
                            onClick={() => {
                              setLobbyStoryTag(null);
                              setLobbyStoryPage(1);
                            }}
                            data-testid="lobby-story-tag-all"
                            style={{
                              padding: '2px 7px',
                              borderRadius: '4px',
                              fontSize: '10.5px',
                              fontWeight: 700,
                              cursor: 'pointer',
                              border: lobbyStoryTag === null ? '1px solid #facc15' : '1px solid rgba(255,255,255,0.08)',
                              backgroundColor: lobbyStoryTag === null ? 'rgba(234, 179, 8, 0.2)' : 'rgba(255,255,255,0.02)',
                              color: lobbyStoryTag === null ? '#facc15' : '#94a3b8',
                            }}
                          >
                            Todas ({prebuiltStories.length})
                          </button>
                          {allTags.map((tag) => {
                            const count = prebuiltStories.filter((s) =>
                              normalizeTags(s.tags).some((t) => t.toLowerCase() === tag.toLowerCase())
                            ).length;
                            const isSelected = lobbyStoryTag?.toLowerCase() === tag.toLowerCase();
                            return (
                              <button
                                key={tag}
                                type="button"
                                onClick={() => {
                                  setLobbyStoryTag(isSelected ? null : tag.toLowerCase());
                                  setLobbyStoryPage(1);
                                }}
                                data-testid={`lobby-story-tag-${tag.toLowerCase()}`}
                                style={{
                                  padding: '2px 7px',
                                  borderRadius: '4px',
                                  fontSize: '10.5px',
                                  fontWeight: 700,
                                  cursor: 'pointer',
                                  border: isSelected ? '1px solid #facc15' : '1px solid rgba(255,255,255,0.08)',
                                  backgroundColor: isSelected ? 'rgba(234, 179, 8, 0.2)' : 'rgba(255,255,255,0.02)',
                                  color: isSelected ? '#facc15' : '#cbd5e1',
                                }}
                              >
                                #{tag.toLowerCase()} <span style={{ fontSize: '9.5px', color: isSelected ? '#fef08a' : '#64748b' }}>({count})</span>
                              </button>
                            );
                          })}
                        </div>
                      )}

                      {/* TABELA PAGINADA DE HISTÓRIAS (UX Amigável para 1000+ Premissas) */}
                      {filtered.length === 0 ? (
                        <div style={{ padding: '20px', textAlign: 'center', color: '#94a3b8', fontSize: '12px', backgroundColor: '#070a0e', borderRadius: '8px', border: '1px dashed rgba(255,255,255,0.08)' }}>
                          Nenhuma história encontrada para os filtros aplicados.
                        </div>
                      ) : (
                        <div style={{ backgroundColor: '#070a0f', border: '1px solid rgba(255, 255, 255, 0.08)', borderRadius: '8px', overflow: 'hidden' }}>
                          <table style={{ width: '100%', borderCollapse: 'collapse', textAlign: 'left', fontSize: '12px' }}>
                            <thead>
                              <tr style={{ backgroundColor: 'rgba(255, 255, 255, 0.02)', borderBottom: '1px solid rgba(255, 255, 255, 0.08)', color: '#94a3b8', fontSize: '10.5px', textTransform: 'uppercase', letterSpacing: '0.05em' }}>
                                <th style={{ padding: '10px 12px', width: '45%' }}>História &amp; Sinopse</th>
                                <th style={{ padding: '10px 12px', width: '27%' }}>Tags &amp; Categorias</th>
                                <th style={{ padding: '10px 12px', width: '16%' }}>Estilo &amp; Foco</th>
                                <th style={{ padding: '10px 12px', width: '12%', textAlign: 'right' }}>Ação</th>
                              </tr>
                            </thead>
                            <tbody>
                              {pagedStories.map((story) => {
                                const isSelected = premiseTitle === story.title;
                                return (
                                  <tr
                                    key={story.id}
                                    onClick={() => handleOpenStoryModal(story)}
                                    data-testid={`prebuilt-story-row-${story.id}`}
                                    style={{
                                      borderBottom: '1px solid rgba(255, 255, 255, 0.04)',
                                      backgroundColor: isSelected ? 'rgba(234, 179, 8, 0.08)' : 'transparent',
                                      cursor: 'pointer',
                                      transition: 'background-color 0.15s ease',
                                    }}
                                    onMouseEnter={(e) => {
                                      if (!isSelected) e.currentTarget.style.backgroundColor = 'rgba(255, 255, 255, 0.03)';
                                    }}
                                    onMouseLeave={(e) => {
                                      if (!isSelected) e.currentTarget.style.backgroundColor = 'transparent';
                                    }}
                                  >
                                    <td style={{ padding: '10px 12px' }}>
                                      <div style={{ display: 'flex', alignItems: 'center', gap: '6px', marginBottom: '2px' }}>
                                        <strong style={{ fontSize: '12.5px', color: isSelected ? '#fef08a' : '#f8fafc' }}>
                                          {story.title}
                                        </strong>
                                        {isSelected && (
                                          <span style={{ fontSize: '9px', fontWeight: 800, color: '#facc15', backgroundColor: 'rgba(234, 179, 8, 0.2)', padding: '1px 5px', borderRadius: '4px' }}>
                                            ✓ Em Uso
                                          </span>
                                        )}
                                      </div>
                                      <p style={{ color: '#94a3b8', fontSize: '11px', margin: 0, overflow: 'hidden', textOverflow: 'ellipsis', display: '-webkit-box', WebkitLineClamp: 2, WebkitBoxOrient: 'vertical', lineHeight: 1.35 }}>
                                        {story.premise}
                                      </p>
                                    </td>

                                    <td style={{ padding: '10px 12px' }}>
                                      <div style={{ display: 'flex', flexWrap: 'wrap', gap: '3px' }}>
                                        {normalizeTags(story.tags).length > 0 ? (
                                          normalizeTags(story.tags).map((t) => (
                                            <span
                                              key={t}
                                              style={{
                                                fontSize: '9.5px',
                                                padding: '1px 6px',
                                                borderRadius: '3px',
                                                backgroundColor: 'rgba(56, 189, 248, 0.12)',
                                                border: '1px solid rgba(56, 189, 248, 0.25)',
                                                color: '#38bdf8',
                                                fontWeight: 600,
                                              }}
                                            >
                                              #{t.toLowerCase()}
                                            </span>
                                          ))
                                        ) : (
                                          <span style={{ fontSize: '10.5px', color: '#64748b' }}>Sem tags</span>
                                        )}
                                      </div>
                                    </td>

                                    <td style={{ padding: '10px 12px' }}>
                                      <div style={{ fontSize: '10.5px', color: '#cbd5e1', marginBottom: '3px' }}>
                                        {story.literary_style || 'Fantasia'}
                                      </div>
                                      <div style={{ display: 'flex', alignItems: 'center', gap: '4px' }}>
                                        <span style={{ fontSize: '9.5px', color: '#facc15', backgroundColor: 'rgba(234, 179, 8, 0.15)', border: '1px solid rgba(234, 179, 8, 0.25)', padding: '1px 5px', borderRadius: '4px', fontWeight: 800 }}>
                                          D20: {story.dramatic_focus_slider || 10}
                                        </span>
                                        <span style={{ fontSize: '9.5px', color: '#38bdf8', backgroundColor: 'rgba(56, 189, 248, 0.12)', border: '1px solid rgba(56, 189, 248, 0.25)', padding: '1px 5px', borderRadius: '4px', fontWeight: 800 }}>
                                          Nv {story.recommended_starting_level || 1}
                                        </span>
                                      </div>
                                    </td>

                                    <td style={{ padding: '10px 12px', textAlign: 'right' }}>
                                      <button
                                        type="button"
                                        onClick={(e) => {
                                          e.stopPropagation();
                                          handleOpenStoryModal(story);
                                        }}
                                        data-testid={`prebuilt-story-btn-${story.id}`}
                                        style={{
                                          padding: '5px 9px',
                                          borderRadius: '6px',
                                          backgroundColor: 'rgba(234, 179, 8, 0.12)',
                                          border: '1px solid rgba(234, 179, 8, 0.35)',
                                          color: '#facc15',
                                          fontSize: '11px',
                                          fontWeight: 700,
                                          cursor: 'pointer',
                                          display: 'inline-flex',
                                          alignItems: 'center',
                                          gap: '4px',
                                        }}
                                      >
                                        <Eye size={12} /> Ver / Ajustar
                                      </button>
                                    </td>
                                  </tr>
                                );
                              })}
                            </tbody>
                          </table>
                        </div>
                      )}

                      {/* Barra de Paginação Sempre Visível (com Seletor de Itens por Página) */}
                      <div
                        style={{
                          display: 'flex',
                          justifyContent: 'space-between',
                          alignItems: 'center',
                          flexWrap: 'wrap',
                          gap: '8px',
                          paddingTop: '6px',
                          borderTop: '1px solid rgba(255, 255, 255, 0.06)',
                        }}
                      >
                        <div style={{ display: 'flex', alignItems: 'center', gap: '10px', flexWrap: 'wrap' }}>
                          <span style={{ fontSize: '11px', color: '#94a3b8' }}>
                            Mostrando <strong>{startIdx}–{endIdx}</strong> de <strong>{filtered.length}</strong> histórias (Página <strong>{currentLobbyPage}</strong> de <strong>{totalLobbyPages}</strong>)
                          </span>
                          <div style={{ display: 'flex', alignItems: 'center', gap: '4px' }}>
                            <span style={{ fontSize: '10px', color: '#64748b' }}>Por página:</span>
                            {[3, 5, 10, 20].map((sz) => (
                              <button
                                key={sz}
                                type="button"
                                onClick={() => {
                                  setLobbyPageSize(sz);
                                  setLobbyStoryPage(1);
                                }}
                                data-testid={`lobby-page-size-${sz}`}
                                style={{
                                  padding: '2px 6px',
                                  borderRadius: '4px',
                                  fontSize: '10px',
                                  fontWeight: 700,
                                  cursor: 'pointer',
                                  border: lobbyPageSize === sz ? '1px solid #facc15' : '1px solid rgba(255,255,255,0.08)',
                                  backgroundColor: lobbyPageSize === sz ? 'rgba(234, 179, 8, 0.2)' : 'transparent',
                                  color: lobbyPageSize === sz ? '#facc15' : '#64748b',
                                }}
                              >
                                {sz}
                              </button>
                            ))}
                          </div>
                        </div>

                        <div style={{ display: 'flex', gap: '6px' }}>
                          <button
                            type="button"
                            disabled={currentLobbyPage <= 1}
                            onClick={() => setLobbyStoryPage((p) => Math.max(1, p - 1))}
                            data-testid="lobby-story-prev-page"
                            style={{
                              padding: '4px 10px',
                              borderRadius: '5px',
                              backgroundColor: currentLobbyPage <= 1 ? 'rgba(255,255,255,0.02)' : 'rgba(234, 179, 8, 0.15)',
                              border: '1px solid rgba(255,255,255,0.08)',
                              color: currentLobbyPage <= 1 ? '#475569' : '#facc15',
                              fontSize: '11px',
                              cursor: currentLobbyPage <= 1 ? 'not-allowed' : 'pointer',
                            }}
                          >
                            « Anterior
                          </button>
                          <button
                            type="button"
                            disabled={currentLobbyPage >= totalLobbyPages}
                            onClick={() => setLobbyStoryPage((p) => Math.min(totalLobbyPages, p + 1))}
                            data-testid="lobby-story-next-page"
                            style={{
                              padding: '4px 10px',
                              borderRadius: '5px',
                              backgroundColor: currentLobbyPage >= totalLobbyPages ? 'rgba(255,255,255,0.02)' : 'rgba(234, 179, 8, 0.15)',
                              border: '1px solid rgba(255,255,255,0.08)',
                              color: currentLobbyPage >= totalLobbyPages ? '#475569' : '#facc15',
                              fontSize: '11px',
                              cursor: currentLobbyPage >= totalLobbyPages ? 'not-allowed' : 'pointer',
                            }}
                          >
                            Próxima »
                          </button>
                        </div>
                      </div>

                      {/* MODAL DE LEITURA AMIGÁVEL & EDIÇÃO DE CONTEXTO */}
                      {inspectingStory && (
                        <div
                          className="modal-backdrop-fade"
                          style={{
                            position: 'fixed',
                            top: 0,
                            left: 0,
                            right: 0,
                            bottom: 0,
                            backgroundColor: 'rgba(0, 0, 0, 0.85)',
                            backdropFilter: 'blur(8px)',
                            display: 'flex',
                            alignItems: 'center',
                            justifyContent: 'center',
                            zIndex: 9999,
                            padding: '20px',
                          }}
                        >
                          <div
                            className="modal-content-fade"
                            style={{
                              backgroundColor: '#0c1017',
                              border: '1px solid rgba(234, 179, 8, 0.4)',
                              borderRadius: '14px',
                              width: '100%',
                              maxWidth: '780px',
                              maxHeight: '90vh',
                              display: 'flex',
                              flexDirection: 'column',
                              overflow: 'hidden',
                              boxShadow: '0 20px 45px rgba(0,0,0,0.85)',
                            }}
                          >
                            {/* Header do Modal */}
                            <div
                              style={{
                                padding: '16px 20px',
                                borderBottom: '1px solid rgba(255,255,255,0.1)',
                                display: 'flex',
                                justifyContent: 'space-between',
                                alignItems: 'flex-start',
                                backgroundColor: '#070a0e',
                              }}
                            >
                              <div>
                                <div style={{ display: 'flex', alignItems: 'center', gap: '8px' }}>
                                  <Sparkles size={16} color="#facc15" />
                                  <h3 style={{ fontSize: '16px', fontWeight: 900, color: '#f8fafc', margin: 0 }}>
                                    {modalTitle || inspectingStory.title}
                                  </h3>
                                </div>

                                {/* Tags & Nível Recomendado */}
                                <div style={{ display: 'flex', flexWrap: 'wrap', gap: '6px', alignItems: 'center', marginTop: '6px' }}>
                                  <span
                                    style={{
                                      padding: '2px 8px',
                                      borderRadius: '4px',
                                      backgroundColor: 'rgba(250, 204, 21, 0.15)',
                                      border: '1px solid rgba(250, 204, 21, 0.35)',
                                      color: '#facc15',
                                      fontSize: '11px',
                                      fontWeight: 800,
                                    }}
                                  >
                                    ⭐ Nível Recomendado: Nv {inspectingStory.recommended_starting_level || 1} (Tier {Math.min(4, Math.ceil((inspectingStory.recommended_starting_level || 1) / 4))})
                                  </span>
                                  {normalizeTags(inspectingStory.tags).map((t) => (
                                    <span
                                      key={t}
                                      style={{
                                        padding: '2px 7px',
                                        borderRadius: '4px',
                                        backgroundColor: 'rgba(56, 189, 248, 0.15)',
                                        border: '1px solid rgba(56, 189, 248, 0.3)',
                                        color: '#38bdf8',
                                        fontSize: '10.5px',
                                        fontWeight: 700,
                                      }}
                                    >
                                      #{t.toLowerCase()}
                                    </span>
                                  ))}
                                </div>
                              </div>

                              <button
                                type="button"
                                onClick={() => setInspectingStory(null)}
                                data-testid="lobby-close-story-modal"
                                style={{
                                  background: 'none',
                                  border: 'none',
                                  color: '#94a3b8',
                                  cursor: 'pointer',
                                  padding: '6px',
                                }}
                              >
                                <X size={20} />
                              </button>
                            </div>

                            {/* Barra de Alternância: Leitura vs Edição de Contexto */}
                            <div style={{ display: 'flex', borderBottom: '1px solid rgba(255,255,255,0.08)', backgroundColor: '#090d14' }}>
                              <button
                                type="button"
                                onClick={() => setModalIsEditing(false)}
                                data-testid="lobby-story-tab-view"
                                style={{
                                  flex: 1,
                                  padding: '10px 14px',
                                  backgroundColor: !modalIsEditing ? 'rgba(234, 179, 8, 0.12)' : 'transparent',
                                  border: 'none',
                                  borderBottom: !modalIsEditing ? '2px solid #facc15' : '2px solid transparent',
                                  color: !modalIsEditing ? '#facc15' : '#94a3b8',
                                  fontWeight: 800,
                                  fontSize: '12px',
                                  cursor: 'pointer',
                                  display: 'flex',
                                  alignItems: 'center',
                                  justifyContent: 'center',
                                  gap: '6px',
                                }}
                              >
                                <Eye size={14} /> 📖 Leitura Amigável &amp; Detalhes
                              </button>
                              <button
                                type="button"
                                onClick={() => setModalIsEditing(true)}
                                data-testid="lobby-story-tab-edit"
                                style={{
                                  flex: 1,
                                  padding: '10px 14px',
                                  backgroundColor: modalIsEditing ? 'rgba(56, 189, 248, 0.12)' : 'transparent',
                                  border: 'none',
                                  borderBottom: modalIsEditing ? '2px solid #38bdf8' : '2px solid transparent',
                                  color: modalIsEditing ? '#38bdf8' : '#94a3b8',
                                  fontWeight: 800,
                                  fontSize: '12px',
                                  cursor: 'pointer',
                                  display: 'flex',
                                  alignItems: 'center',
                                  justifyContent: 'center',
                                  gap: '6px',
                                }}
                              >
                                <Edit2 size={14} /> ✏️ Ajustar / Personalizar Contexto
                              </button>
                            </div>

                            {/* Conteúdo do Modal (Scrollável) */}
                            <div style={{ padding: '20px', overflowY: 'auto', display: 'flex', flexDirection: 'column', gap: '14px' }}>
                              {!modalIsEditing ? (
                                /* MODO LEITURA AMIGÁVEL */
                                <>
                                  {/* Faixa de Metadados */}
                                  <div style={{ display: 'grid', gridTemplateColumns: '1fr 1fr', gap: '10px', backgroundColor: '#070a0e', padding: '12px 14px', borderRadius: '8px', border: '1px solid rgba(255,255,255,0.06)' }}>
                                    <div>
                                      <span style={{ fontSize: '11px', color: '#94a3b8', display: 'block', marginBottom: '2px' }}>Estilo Literário:</span>
                                      <strong style={{ fontSize: '12.5px', color: '#f8fafc' }}>{modalStyle}</strong>
                                    </div>
                                    <div>
                                      <span style={{ fontSize: '11px', color: '#94a3b8', display: 'block', marginBottom: '2px' }}>Foco Dramático D20:</span>
                                      <strong style={{ fontSize: '12.5px', color: '#facc15' }}>D20: {modalSlider}/20</strong>
                                    </div>
                                  </div>

                                  {/* Narrativa da Premissa com Tipografia Confortável */}
                                  <div>
                                    <span style={{ fontSize: '11px', fontWeight: 800, color: '#facc15', textTransform: 'uppercase', letterSpacing: '0.05em' }}>
                                      📜 Narrativa &amp; Premissa da Campanha:
                                    </span>
                                    <div
                                      style={{
                                        backgroundColor: '#070a0e',
                                        padding: '16px',
                                        borderRadius: '8px',
                                        border: '1px solid rgba(255,255,255,0.08)',
                                        color: '#e2e8f0',
                                        fontSize: '13px',
                                        lineHeight: 1.65,
                                        whiteSpace: 'pre-wrap',
                                        maxHeight: '300px',
                                        overflowY: 'auto',
                                        marginTop: '8px',
                                      }}
                                    >
                                      {modalPremise}
                                    </div>
                                  </div>

                                  {/* Monstros Homebrew se houver */}
                                  {inspectingStory.custom_monsters && inspectingStory.custom_monsters.length > 0 && (
                                    <div>
                                      <span style={{ fontSize: '11px', fontWeight: 800, color: '#f87171', textTransform: 'uppercase' }}>
                                        👹 Ameaças e Monstros Customizados ({inspectingStory.custom_monsters.length}):
                                      </span>
                                      <div style={{ display: 'flex', flexDirection: 'column', gap: '6px', marginTop: '6px' }}>
                                        {inspectingStory.custom_monsters.map((m: any, idx: number) => (
                                          <div key={idx} style={{ backgroundColor: 'rgba(239, 68, 68, 0.08)', padding: '8px 12px', borderRadius: '6px', border: '1px solid rgba(239, 68, 68, 0.25)', fontSize: '12px' }}>
                                            <strong style={{ color: '#fca5a5' }}>{m.name}</strong> ({m.type}) — CA: {m.ac} | HP: {m.hp} | ND: {m.cr}
                                          </div>
                                        ))}
                                      </div>
                                    </div>
                                  )}

                                  {/* Overrides de Subclasse se houver */}
                                  {inspectingStory.subclass_overrides && inspectingStory.subclass_overrides.length > 0 && (
                                    <div>
                                      <span style={{ fontSize: '11px', fontWeight: 800, color: '#d8b4fe', textTransform: 'uppercase' }}>
                                        👑 Subclasses Homebrew Integradas ({inspectingStory.subclass_overrides.length}):
                                      </span>
                                      <div style={{ display: 'flex', flexDirection: 'column', gap: '6px', marginTop: '6px' }}>
                                        {inspectingStory.subclass_overrides.map((sub: any, idx: number) => (
                                          <div key={idx} style={{ backgroundColor: 'rgba(168, 85, 247, 0.08)', padding: '8px 12px', borderRadius: '6px', border: '1px solid rgba(168, 85, 247, 0.25)', fontSize: '12px' }}>
                                            <strong style={{ color: '#e9d5ff' }}>{sub.name}</strong> ({sub.parent_class}) — {sub.description}
                                          </div>
                                        ))}
                                      </div>
                                    </div>
                                  )}
                                </>
                              ) : (
                                /* MODO EDIÇÃO E PERSONALIZAÇÃO DE CONTEXTO */
                                <>
                                  <div style={{ backgroundColor: 'rgba(56, 189, 248, 0.08)', padding: '10px 14px', borderRadius: '8px', border: '1px solid rgba(56, 189, 248, 0.25)', color: '#bae6fd', fontSize: '12px' }}>
                                    💡 <strong>Ajuste Livre de Contexto:</strong> Você pode alterar o título, premissa, estilo ou foco dramático à vontade para a sua campanha sem alterar o arquivo original do catálogo.
                                  </div>

                                  <div>
                                    <label style={{ fontSize: '11px', color: '#94a3b8', fontWeight: 700, textTransform: 'uppercase', marginBottom: '4px', display: 'block' }}>
                                      Título da Campanha
                                    </label>
                                    <input
                                      type="text"
                                      value={modalTitle}
                                      onChange={(e) => setModalTitle(e.target.value)}
                                      data-testid="lobby-modal-edit-title"
                                      style={{
                                        width: '100%',
                                        padding: '8px 12px',
                                        backgroundColor: '#070a0e',
                                        border: '1px solid rgba(255,255,255,0.15)',
                                        borderRadius: '6px',
                                        color: '#ffffff',
                                        fontSize: '13px',
                                        boxSizing: 'border-box',
                                      }}
                                    />
                                  </div>

                                  <div style={{ display: 'grid', gridTemplateColumns: '1fr 140px', gap: '12px' }}>
                                    <div>
                                      <label style={{ fontSize: '11px', color: '#94a3b8', fontWeight: 700, textTransform: 'uppercase', marginBottom: '4px', display: 'block' }}>
                                        Estilo / Tom Literário
                                      </label>
                                      <input
                                        type="text"
                                        value={modalStyle}
                                        onChange={(e) => setModalStyle(e.target.value)}
                                        data-testid="lobby-modal-edit-style"
                                        style={{
                                          width: '100%',
                                          padding: '8px 12px',
                                          backgroundColor: '#070a0e',
                                          border: '1px solid rgba(255,255,255,0.15)',
                                          borderRadius: '6px',
                                          color: '#ffffff',
                                          fontSize: '12.5px',
                                          boxSizing: 'border-box',
                                        }}
                                      />
                                    </div>

                                    <div>
                                      <label style={{ fontSize: '11px', color: '#94a3b8', fontWeight: 700, textTransform: 'uppercase', marginBottom: '4px', display: 'block' }}>
                                        Foco D20 (1-20)
                                      </label>
                                      <input
                                        type="number"
                                        min={1}
                                        max={20}
                                        value={modalSlider}
                                        onChange={(e) => setModalSlider(Math.min(20, Math.max(1, parseInt(e.target.value) || 10)))}
                                        data-testid="lobby-modal-edit-slider"
                                        style={{
                                          width: '100%',
                                          padding: '8px 12px',
                                          backgroundColor: '#070a0e',
                                          border: '1px solid rgba(255,255,255,0.15)',
                                          borderRadius: '6px',
                                          color: '#facc15',
                                          fontWeight: 800,
                                          fontSize: '13px',
                                          textAlign: 'center',
                                          boxSizing: 'border-box',
                                        }}
                                      />
                                    </div>
                                  </div>

                                  <div>
                                    <label style={{ fontSize: '11px', color: '#94a3b8', fontWeight: 700, textTransform: 'uppercase', marginBottom: '4px', display: 'block' }}>
                                      Narrativa da Premissa &amp; Entidades Homebrew (Texto Integral)
                                    </label>
                                    <textarea
                                      rows={8}
                                      value={modalPremise}
                                      onChange={(e) => setModalPremise(e.target.value)}
                                      data-testid="lobby-modal-edit-premise"
                                      style={{
                                        width: '100%',
                                        padding: '12px',
                                        backgroundColor: '#070a0e',
                                        border: '1px solid rgba(56, 189, 248, 0.3)',
                                        borderRadius: '8px',
                                        color: '#ffffff',
                                        fontSize: '12.5px',
                                        lineHeight: 1.5,
                                        boxSizing: 'border-box',
                                      }}
                                    />
                                  </div>
                                </>
                              )}
                            </div>

                            {/* Rodapé do Modal com Ações */}
                            <div
                              style={{
                                padding: '14px 20px',
                                borderTop: '1px solid rgba(255,255,255,0.1)',
                                display: 'flex',
                                justifyContent: 'space-between',
                                alignItems: 'center',
                                backgroundColor: '#070a0e',
                              }}
                            >
                              <button
                                type="button"
                                onClick={() => setInspectingStory(null)}
                                style={{
                                  padding: '8px 16px',
                                  borderRadius: '6px',
                                  backgroundColor: 'rgba(255,255,255,0.06)',
                                  border: '1px solid rgba(255,255,255,0.12)',
                                  color: '#94a3b8',
                                  fontSize: '12px',
                                  cursor: 'pointer',
                                }}
                              >
                                Cancelar
                              </button>

                              <button
                                type="button"
                                onClick={handleApplyStoryFromModal}
                                data-testid="lobby-apply-story-btn"
                                style={{
                                  padding: '9px 20px',
                                  borderRadius: '8px',
                                  background: 'linear-gradient(135deg, #facc15 0%, #ca8a04 100%)',
                                  border: 'none',
                                  color: '#000000',
                                  fontWeight: 900,
                                  fontSize: '12.5px',
                                  cursor: 'pointer',
                                  display: 'flex',
                                  alignItems: 'center',
                                  gap: '6px',
                                  boxShadow: '0 4px 15px rgba(234, 179, 8, 0.3)',
                                }}
                              >
                                <Sparkles size={14} /> Usar esta História na Campanha
                              </button>
                            </div>
                          </div>
                        </div>
                      )}
                    </div>
                  );
                })()}

                {/* Barra de Ação da Sub-Aba Catálogo */}
                <div
                  style={{
                    display: 'flex',
                    justifyContent: 'space-between',
                    alignItems: 'center',
                    flexWrap: 'wrap',
                    gap: '12px',
                    paddingTop: '12px',
                    borderTop: '1px solid rgba(255, 255, 255, 0.08)',
                  }}
                >
                  <div>
                    {premise.trim() ? (
                      <span style={{ fontSize: '12px', color: '#fef08a', display: 'flex', alignItems: 'center', gap: '6px' }}>
                        <span style={{ color: '#22c55e', fontWeight: 900 }}>✓</span> História Selecionada:{' '}
                        <strong>{premiseTitle || 'História Oficial'}</strong>
                      </span>
                    ) : (
                      <span style={{ fontSize: '11.5px', color: '#94a3b8' }}>
                        💡 Selecione uma história na tabela acima (clicando na linha ou em <em>Ver / Ajustar</em>) para avançar.
                      </span>
                    )}
                  </div>

                  <button
                    type="button"
                    disabled={!premise.trim()}
                    onClick={() => {
                      setError(null);
                      setTab('create_rules');
                    }}
                    data-testid="lobby-advance-from-catalog-btn"
                    className="rpg-cta-gold"
                    style={{
                      width: 'auto',
                      padding: '10px 24px',
                      fontSize: '13px',
                      opacity: premise.trim() ? 1 : 0.45,
                      cursor: premise.trim() ? 'pointer' : 'not-allowed',
                    }}
                  >
                    <span>Avançar para: 2. Regras</span>
                    <ArrowRight size={16} />
                  </button>
                </div>
              </div>
            ) : (
              /* CONTEÚDO DA SUB-ABA 2: FORJA MANUAL DO ZERO */
              <div style={{ display: 'flex', flexDirection: 'column', gap: '16px' }}>
                {/* Título da Premissa */}
                <div style={{ display: 'flex', flexDirection: 'column', gap: '6px' }}>
                  <label className="rpg-label">
                    <span>Título da Campanha / Aventura</span>
                    <span style={{ fontSize: '10px', color: '#64748b', textTransform: 'none' }}>Opcional</span>
                  </label>
                  <input
                    type="text"
                    className="rpg-input-field"
                    placeholder="Ex: A Maldição da Torre Cinzenta, Os Mistérios de Valfenda..."
                    value={premiseTitle}
                    onChange={(e) => setPremiseTitle(e.target.value)}
                    data-testid="lobby-manual-story-title"
                  />
                </div>

                {/* Premissa Central da História */}
                <div style={{ display: 'flex', flexDirection: 'column', gap: '6px' }}>
                  <label className="rpg-label">
                    <span>Premissa do Mundo, Enredo &amp; Entidades Homebrew (Entrada Unificada) *</span>
                  </label>
                  <textarea
                    className="rpg-input-field"
                    value={premise}
                    onChange={(e) => setPremise(e.target.value)}
                    rows={8}
                    required
                    placeholder="Insira a premissa completa da sua campanha. Você pode descrever o conflito central, reviravoltas de lore, ou incluir diretamente seus Monstros e Chefes Homebrew com estatísticas (ex: Boss: Malakor, o Flagelo | PV: 180 | CA: 17 | Habilidades de Fogo e Maldições). O Mestre IA assimilará tudo organicamente!"
                    style={{ lineHeight: '1.6', resize: 'vertical', fontSize: '13.5px' }}
                    data-testid="lobby-manual-story-premise"
                  />
                </div>

                {/* Estilo e Tom Narrativo */}
                <div style={{ display: 'flex', flexDirection: 'column', gap: '6px' }}>
                  <label className="rpg-label">
                    <span>Estilo &amp; Tom Literário da Narrativa</span>
                  </label>
                  <input
                    type="text"
                    className="rpg-input-field"
                    value={literaryStyle}
                    onChange={(e) => setLiteraryStyle(e.target.value)}
                    placeholder="Ex: Fantasia Sombria visceral, horror gótico tenso, mistério investigativo, épico heróico..."
                    data-testid="lobby-manual-story-style"
                  />
                </div>

                {/* Ações da Aba 1: Importar/Exportar e Avançar */}
                <div
                  style={{
                    display: 'flex',
                    justifyContent: 'space-between',
                    alignItems: 'center',
                    flexWrap: 'wrap',
                    gap: '12px',
                    paddingTop: '14px',
                    borderTop: '1px solid rgba(255, 255, 255, 0.08)',
                  }}
                >
                  <div style={{ display: 'flex', gap: '8px', flexWrap: 'wrap' }}>
                    {premise.trim() && (
                      <button
                        type="button"
                        onClick={handleExportPremiseFile}
                        className="rpg-preset-pill"
                        title="Baixar esta premissa em arquivo JSON"
                        style={{ borderColor: 'rgba(234, 179, 8, 0.35)', color: '#facc15' }}
                      >
                        <Download size={13} style={{ marginRight: '4px' }} /> Salvar Premissa (.JSON)
                      </button>
                    )}
                    <label
                      className="rpg-preset-pill"
                      style={{ cursor: 'pointer', borderColor: 'rgba(255, 255, 255, 0.15)', color: '#cbd5e1' }}
                      title="Carregar arquivo de premissa (.json)"
                    >
                      <Upload size={13} style={{ marginRight: '4px' }} /> Importar Arquivo de Premissa
                      <input type="file" accept=".json" onChange={handleImportPremiseFile} style={{ display: 'none' }} />
                    </label>
                  </div>

                  <button
                    type="button"
                    disabled={!premise.trim()}
                    onClick={() => {
                      if (!premise.trim()) {
                        setError('Informe a narrativa ou selecione uma história antes de avançar.');
                        return;
                      }
                      setError(null);
                      setTab('create_rules');
                    }}
                    className="rpg-cta-gold"
                    data-testid="lobby-manual-advance-btn"
                    style={{
                      width: 'auto',
                      padding: '10px 24px',
                      fontSize: '13px',
                      opacity: premise.trim() ? 1 : 0.45,
                      cursor: premise.trim() ? 'pointer' : 'not-allowed',
                    }}
                  >
                    <span>Avançar para: 2. Regras</span>
                    <ArrowRight size={16} />
                  </button>
                </div>
              </div>
            )}
          </div>
        ) : tab === 'create_rules' ? (
          /* TAB 2: FORJA DE MUNDO (REGRAS, MODO DE JOGO & MESTRE IA) */
          <div className="rpg-panel-box" style={{ gap: '20px' }}>
            <div style={{ display: 'flex', justifyContent: 'space-between', alignItems: 'center', flexWrap: 'wrap', gap: '10px' }}>
              <div style={{ display: 'flex', alignItems: 'center', gap: '8px' }}>
                <Dice5 size={18} color="#facc15" />
                <div>
                  <h2 style={{ fontSize: '15px', fontWeight: 800, color: '#ffffff', margin: 0, textTransform: 'uppercase', letterSpacing: '0.04em' }}>
                    2. Regras (Modo de Jogo &amp; Mestre IA)
                  </h2>
                  <span style={{ fontSize: '12px', color: '#94a3b8' }}>
                    Configure como os jogadores vão participar e a postura da IA como narrador.
                  </span>
                </div>
              </div>
            </div>

            {/* 1. SELETOR DE MODO DE JOGO */}
            <div style={{ display: 'flex', flexDirection: 'column', gap: '8px' }}>
              <label className="rpg-label" style={{ color: '#38bdf8' }}>
                <span style={{ display: 'flex', alignItems: 'center', gap: '6px' }}>
                  <Monitor size={14} /> Modo de Jogo da Sessão
                </span>
              </label>

              <div style={{ display: 'grid', gridTemplateColumns: 'repeat(auto-fit, minmax(260px, 1fr))', gap: '12px' }}>
                {/* Opção 1: Modo Um Terminal (Presencial) */}
                <div
                  onClick={() => setIsOfflineMode(true)}
                  style={{
                    padding: '14px',
                    borderRadius: '12px',
                    cursor: 'pointer',
                    backgroundColor: isOfflineMode ? 'rgba(56, 189, 248, 0.14)' : '#06080c',
                    border: isOfflineMode ? '1.5px solid #38bdf8' : '1px solid rgba(255, 255, 255, 0.08)',
                    boxShadow: isOfflineMode ? '0 0 15px rgba(56, 189, 248, 0.15)' : 'none',
                    transition: 'all 0.18s ease',
                    display: 'flex',
                    flexDirection: 'column',
                    gap: '6px',
                  }}
                >
                  <div style={{ display: 'flex', alignItems: 'center', justifyContent: 'space-between' }}>
                    <span style={{ fontWeight: 800, fontSize: '13px', color: isOfflineMode ? '#38bdf8' : '#e2e8f0', display: 'flex', alignItems: 'center', gap: '6px' }}>
                      <Monitor size={16} /> 🖥️ Modo Um Terminal (Presencial)
                    </span>
                    {isOfflineMode && <span style={{ width: '8px', height: '8px', borderRadius: '50%', backgroundColor: '#38bdf8' }} />}
                  </div>
                  <span style={{ fontSize: '11.5px', color: '#94a3b8', lineHeight: '1.4' }}>
                    Mesa local compartilhada na mesma tela sem WebSockets. Cada jogador recebe código de 4 letras para migrar online depois se quiser.
                  </span>

                  {/* Seletor de Heróis na Mesa */}
                  {isOfflineMode && (
                    <div style={{ display: 'flex', alignItems: 'center', gap: '8px', marginTop: '6px', paddingTop: '6px', borderTop: '1px solid rgba(56, 189, 248, 0.2)' }}>
                      <span style={{ fontSize: '11px', color: '#7dd3fc', fontWeight: 700 }}>Jogadores no Terminal:</span>
                      {[1, 2, 3, 4, 5, 6].map((num) => (
                        <button
                          key={num}
                          type="button"
                          onClick={(e) => {
                            e.stopPropagation();
                            setPartySize(num);
                          }}
                          style={{
                            padding: '3px 9px',
                            borderRadius: '6px',
                            fontSize: '11.5px',
                            fontWeight: 700,
                            backgroundColor: partySize === num ? '#38bdf8' : 'rgba(255,255,255,0.06)',
                            color: partySize === num ? '#000' : '#cbd5e1',
                            border: 'none',
                            cursor: 'pointer',
                          }}
                        >
                          {num}
                        </button>
                      ))}
                    </div>
                  )}
                </div>

                {/* Opção 2: Modo Online */}
                <div
                  onClick={() => setIsOfflineMode(false)}
                  style={{
                    padding: '14px',
                    borderRadius: '12px',
                    cursor: 'pointer',
                    backgroundColor: !isOfflineMode ? 'rgba(16, 185, 129, 0.14)' : '#06080c',
                    border: !isOfflineMode ? '1.5px solid #10b981' : '1px solid rgba(255, 255, 255, 0.08)',
                    boxShadow: !isOfflineMode ? '0 0 15px rgba(16, 185, 129, 0.15)' : 'none',
                    transition: 'all 0.18s ease',
                    display: 'flex',
                    flexDirection: 'column',
                    gap: '6px',
                  }}
                >
                  <div style={{ display: 'flex', alignItems: 'center', justifyContent: 'space-between' }}>
                    <span style={{ fontWeight: 800, fontSize: '13px', color: !isOfflineMode ? '#34d399' : '#e2e8f0', display: 'flex', alignItems: 'center', gap: '6px' }}>
                      <Globe2 size={16} /> 🌐 Modo Online (Multijogador)
                    </span>
                    {!isOfflineMode && <span style={{ width: '8px', height: '8px', borderRadius: '50%', backgroundColor: '#10b981' }} />}
                  </div>
                  <span style={{ fontSize: '11.5px', color: '#94a3b8', lineHeight: '1.4' }}>
                    Conexão em tempo real via WebSockets. Cada jogador conecta de seu próprio dispositivo inserindo o UUID e o código da sala.
                  </span>
                </div>
              </div>
            </div>

            {/* 2. FOCO DRAMÁTICO DA CAMPANHA (SLIDER CONTÍNUO 1 A 20) */}
            <div style={{ display: 'flex', flexDirection: 'column', gap: '10px' }}>
              <div style={{ display: 'flex', justifyContent: 'space-between', alignItems: 'center', flexWrap: 'wrap', gap: '6px' }}>
                <label className="rpg-label" style={{ color: '#facc15', margin: 0 }}>
                  <span style={{ display: 'flex', alignItems: 'center', gap: '6px' }}>
                    <Sword size={14} /> Foco Dramático &amp; Tático: Slider D20 ({dramaticSlider}/20)
                  </span>
                </label>
                <button
                  type="button"
                  onClick={() => setDramaticSlider(Math.floor(Math.random() * 20) + 1)}
                  className="rpg-preset-pill"
                  style={{ fontSize: '11px', borderColor: 'rgba(234, 179, 8, 0.4)', color: '#fef08a' }}
                >
                  🎲 Rolar D20 do Destino
                </button>
              </div>

              {/* Slider Contínuo de 1 a 20 */}
              <input
                type="range"
                min={1}
                max={20}
                value={dramaticSlider}
                onChange={(e) => setDramaticSlider(parseInt(e.target.value, 10))}
                style={{
                  width: '100%',
                  accentColor: dramaticSlider <= 6 ? '#818cf8' : dramaticSlider >= 14 ? '#ef4444' : '#facc15',
                  cursor: 'pointer',
                }}
              />

              <div
                style={{
                  display: 'flex',
                  justifyContent: 'space-between',
                  fontSize: '11px',
                  color: '#94a3b8',
                  padding: '0 4px',
                }}
              >
                <span style={{ color: dramaticSlider <= 6 ? '#a5b4fc' : '#64748b', fontWeight: dramaticSlider <= 6 ? 800 : 400 }}>
                  1–6: Roleplay &amp; Intriga
                </span>
                <span style={{ color: dramaticSlider > 6 && dramaticSlider < 14 ? '#fef08a' : '#64748b', fontWeight: dramaticSlider > 6 && dramaticSlider < 14 ? 800 : 400 }}>
                  7–13: Equilíbrio Clássico (50/50)
                </span>
                <span style={{ color: dramaticSlider >= 14 ? '#fca5a5' : '#64748b', fontWeight: dramaticSlider >= 14 ? 800 : 400 }}>
                  14–20: Guerra Tática &amp; Letais
                </span>
              </div>

              <div
                style={{
                  padding: '8px 12px',
                  borderRadius: '8px',
                  backgroundColor:
                    dramaticSlider <= 6
                      ? 'rgba(99, 102, 241, 0.12)'
                      : dramaticSlider >= 14
                      ? 'rgba(239, 68, 68, 0.12)'
                      : 'rgba(234, 179, 8, 0.12)',
                  border: `1px solid ${
                    dramaticSlider <= 6
                      ? 'rgba(99, 102, 241, 0.3)'
                      : dramaticSlider >= 14
                      ? 'rgba(239, 68, 68, 0.3)'
                      : 'rgba(234, 179, 8, 0.3)'
                  }`,
                  fontSize: '12px',
                  color: dramaticSlider <= 6 ? '#c7d2fe' : dramaticSlider >= 14 ? '#fecaca' : '#fef9c3',
                }}
              >
                {dramaticSlider <= 6 &&
                  '🎭 Foco em Roleplay, Intriga & Diálogo: A trama prioriza conspirações, enigmas e escolhas morais. Os combates serão raros e diplomáticos.'}
                {dramaticSlider > 6 &&
                  dramaticSlider < 14 &&
                  '⚖️ Equilíbrio Clássico D&D 5e: Exploração de masmorras com armadilhas, enigmas e encontros táticos com frequência balanceada (50/50).'}
                {dramaticSlider >= 14 &&
                  '⚔️ Alta Densidade Tática: Masmorras brutais, patrulhas agressivas, emboscadas e chefes impiedosos. Alta letalidade tática.'}
              </div>
            </div>

            {/* 3. LIMITES CRIATIVOS DA IA */}
            <div style={{ display: 'flex', flexDirection: 'column', gap: '8px' }}>
              <label className="rpg-label" style={{ color: '#c084fc' }}>
                <span style={{ display: 'flex', alignItems: 'center', gap: '6px' }}>
                  <Dice5 size={14} /> Autonomia Criativa do Mestre IA
                </span>
              </label>

              <div className="rpg-freedom-grid">
                {[
                  {
                    id: 'strict',
                    title: '📜 Cânone Puro',
                    desc: 'Usa estritamente regras SRD 5e oficiais sem improvisações fora da premissa.',
                  },
                  {
                    id: 'balanced',
                    title: '⚖️ Equilibrado',
                    desc: 'Fiel à premissa, podendo criar 1 chefe autoral inédito para o clímax.',
                  },
                  {
                    id: 'free',
                    title: '🔥 Caos Criativo',
                    desc: 'Liberdade total para surpresas narrativas, reviravoltas e chefes cósmicos.',
                  },
                ].map((mode) => {
                  const isActive = aiCreativeFreedom === mode.id;
                  return (
                    <div
                      key={mode.id}
                      onClick={() => setAiCreativeFreedom(mode.id as any)}
                      className={`rpg-freedom-card ${isActive ? 'active' : ''}`}
                    >
                      <div style={{ display: 'flex', alignItems: 'center', justifyContent: 'space-between' }}>
                        <strong style={{ color: isActive ? '#e9d5ff' : '#f1f5f9', fontSize: '12px' }}>
                          {mode.title}
                        </strong>
                        {isActive && (
                          <span style={{ width: '7px', height: '7px', borderRadius: '50%', backgroundColor: '#c084fc' }} />
                        )}
                      </div>
                      <span style={{ color: '#94a3b8', fontSize: '11px', lineHeight: 1.35 }}>
                        {mode.desc}
                      </span>
                    </div>
                  );
                })}
              </div>
            </div>

            {/* BOTÕES DE NAVEGAÇÃO DA PÁGINA 2 */}
            <div
              style={{
                display: 'flex',
                justifyContent: 'space-between',
                alignItems: 'center',
                gap: '12px',
                paddingTop: '12px',
                borderTop: '1px solid rgba(255, 255, 255, 0.08)',
                flexWrap: 'wrap',
              }}
            >
              <button
                type="button"
                onClick={() => {
                  setError(null);
                  setTab('create_premise');
                }}
                style={{
                  display: 'inline-flex',
                  alignItems: 'center',
                  gap: '6px',
                  padding: '10px 18px',
                  borderRadius: '8px',
                  backgroundColor: 'rgba(255, 255, 255, 0.05)',
                  border: '1px solid rgba(255, 255, 255, 0.15)',
                  color: '#cbd5e1',
                  fontSize: '13px',
                  fontWeight: 600,
                  cursor: 'pointer',
                }}
              >
                <ArrowLeft size={16} /> Voltar para: 1. Forjar
              </button>

              <button
                type="button"
                onClick={handleProceedToParty}
                disabled={isAnalyzing}
                className="rpg-cta-gold"
                style={{ width: 'auto', padding: '12px 28px', fontSize: '13.5px' }}
                data-testid="advance-to-party-btn"
              >
                <Users size={18} />
                <span>{isAnalyzing ? 'Consultando Mestre IA...' : 'Avançar para: 3. Grupo de Heróis'}</span>
                <ArrowRight size={16} />
              </button>
            </div>
          </div>
        ) : tab === 'create_party' ? (
          /* TAB 3: COMPOSIÇÃO DE HERÓIS PRÉ-CAMPANHA & AVALIAÇÃO DA IA */
          <div className="rpg-panel-box" style={{ gap: '20px' }}>
            <div style={{ display: 'flex', justifyContent: 'space-between', alignItems: 'center', flexWrap: 'wrap', gap: '10px' }}>
              <div style={{ display: 'flex', alignItems: 'center', gap: '8px' }}>
                <Users size={18} color="#38bdf8" />
                <div>
                  <h2 style={{ fontSize: '15px', fontWeight: 800, color: '#ffffff', margin: 0, textTransform: 'uppercase', letterSpacing: '0.04em' }}>
                    3. Grupo de Heróis (Composição &amp; Avaliação da IA)
                  </h2>
                  <span style={{ fontSize: '12px', color: '#94a3b8' }}>
                    Defina os aventureiros (1 a 4 heróis) que entrarão juntos na crônica antes do início da aventura.
                  </span>
                </div>
              </div>
            </div>

            {/* FEEDBACK DO MESTRE IA / CONSULTA DE PREMISSA */}
            {isAnalyzing ? (
              <div
                style={{
                  padding: '16px',
                  borderRadius: '10px',
                  backgroundColor: 'rgba(56, 189, 248, 0.08)',
                  border: '1px dashed rgba(56, 189, 248, 0.4)',
                  display: 'flex',
                  alignItems: 'center',
                  gap: '12px',
                  color: '#93c5fd',
                  fontSize: '13px',
                }}
              >
                <Sparkles size={20} className="animate-spin" color="#38bdf8" />
                <span>O Mestre IA está avaliando a premissa, o tom dramático e as necessidades temáticas do grupo...</span>
              </div>
            ) : premiseAnalysis ? (
              <div
                style={{
                  padding: '14px 16px',
                  borderRadius: '10px',
                  backgroundColor: 'rgba(168, 85, 247, 0.08)',
                  border: '1px solid rgba(168, 85, 247, 0.35)',
                  display: 'flex',
                  flexDirection: 'column',
                  gap: '8px',
                }}
              >
                <div style={{ display: 'flex', justifyContent: 'space-between', alignItems: 'center', flexWrap: 'wrap', gap: '8px' }}>
                  <strong data-testid="adventure-tone-badge" style={{ fontSize: '13px', color: '#e9d5ff', display: 'flex', alignItems: 'center', gap: '6px' }}>
                    <Sparkles size={15} color="#c084fc" /> Tom da Aventura: {premiseAnalysis.adventure_tone}
                  </strong>
                  <div style={{ display: 'flex', alignItems: 'center', gap: '6px', flexWrap: 'wrap' }}>
                    <span
                      style={{
                        fontSize: '11px',
                        color: '#facc15',
                        backgroundColor: 'rgba(234, 179, 8, 0.15)',
                        border: '1px solid rgba(234, 179, 8, 0.3)',
                        padding: '2px 8px',
                        borderRadius: '4px',
                        fontWeight: 700,
                      }}
                    >
                      ⭐ Nível Recomendado: Nv {premiseAnalysis.recommended_starting_level || 1}
                    </span>
                    <span
                      style={{
                        fontSize: '11px',
                        color: '#cbd5e1',
                        backgroundColor: 'rgba(0,0,0,0.3)',
                        padding: '2px 8px',
                        borderRadius: '4px',
                      }}
                    >
                      Tamanho sugerido: {premiseAnalysis.recommended_party_size}
                    </span>
                  </div>
                </div>

                <p style={{ fontSize: '12px', color: '#cbd5e1', margin: 0, lineHeight: 1.45 }}>
                  {premiseAnalysis.analysis_summary}
                </p>

                {(premiseAnalysis.recommended_starting_level || 1) > 1 &&
                  partyHeroes.some((h) => (h.level || 1) < (premiseAnalysis.recommended_starting_level || 1)) && (
                    <div
                      style={{
                        padding: '8px 12px',
                        borderRadius: '6px',
                        backgroundColor: 'rgba(239, 68, 68, 0.15)',
                        border: '1px solid rgba(239, 68, 68, 0.4)',
                        display: 'flex',
                        alignItems: 'center',
                        gap: '8px',
                        fontSize: '11.5px',
                        color: '#fca5a5',
                      }}
                    >
                      <AlertTriangle size={16} color="#f87171" style={{ flexShrink: 0 }} />
                      <span>
                        <strong>⚠️ Alerta de Sobrevivência (D&D 5e):</strong> Esta narrativa foi calibrada para heróis de <strong>Nível {premiseAnalysis.recommended_starting_level}+</strong>. Heróis de nível inferior correm sério risco de derrota fatal (TPK) contra os monstros da crônica!
                      </span>
                    </div>
                  )}

                {/* Parecer Avaliativo de Requisitos */}
                <div style={{ marginTop: '4px', paddingTop: '8px', borderTop: '1px solid rgba(255, 255, 255, 0.06)' }}>
                  {premiseAnalysis.has_thematic_requirement ? (
                    <div style={{ display: 'flex', flexDirection: 'column', gap: '6px' }}>
                      {premiseAnalysis.party_requirements.map((req, rIdx) => {
                        const isMet = isRequirementSatisfied(req, partyHeroes);
                        const isMandatory = req.is_mandatory;

                        return (
                          <div
                            key={rIdx}
                            style={{
                              fontSize: '12px',
                              padding: '10px 14px',
                              borderRadius: '8px',
                              backgroundColor: isMet
                                ? 'rgba(16, 185, 129, 0.12)'
                                : isMandatory
                                ? 'rgba(239, 68, 68, 0.14)'
                                : 'rgba(234, 179, 8, 0.1)',
                              border: `1px solid ${
                                isMet
                                  ? 'rgba(16, 185, 129, 0.35)'
                                  : isMandatory
                                  ? 'rgba(239, 68, 68, 0.45)'
                                  : 'rgba(234, 179, 8, 0.3)'
                              }`,
                              display: 'flex',
                              alignItems: 'center',
                              justifyContent: 'space-between',
                              flexWrap: 'wrap',
                              gap: '10px',
                            }}
                          >
                            <div style={{ display: 'flex', flexDirection: 'column', gap: '2px', flex: 1, minWidth: '220px' }}>
                              <span style={{ color: isMet ? '#6ee7b7' : isMandatory ? '#fca5a5' : '#fef08a' }}>
                                <strong>
                                  {isMandatory ? '🔒 Requisito Obrigatório do Mestre IA:' : '💡 Sugestão Temática:'}
                                </strong>{' '}
                                {req.description}
                              </span>
                              {req.suggested_options && req.suggested_options.length > 0 && (
                                <span style={{ fontSize: '11px', color: '#94a3b8' }}>
                                  Opções válidas: {req.suggested_options.join(', ')}
                                </span>
                              )}
                            </div>

                            <div style={{ display: 'flex', alignItems: 'center', gap: '8px' }}>
                              <span
                                style={{
                                  fontSize: '11px',
                                  fontWeight: 800,
                                  color: isMet ? '#10b981' : isMandatory ? '#ef4444' : '#facc15',
                                  whiteSpace: 'nowrap',
                                  backgroundColor: 'rgba(0,0,0,0.3)',
                                  padding: '3px 8px',
                                  borderRadius: '6px',
                                }}
                              >
                                {isMet ? '✅ Atendido no Grupo' : isMandatory ? '⛔ Obrigatório Pendente' : '💡 Sugestão Opcional'}
                              </span>

                              {/* Botão de Auto-Criação do Herói Requerido caso não esteja atendido e seja obrigatório */}
                              {!isMet && isMandatory && (
                                <button
                                  type="button"
                                  onClick={() => {
                                    const requiredHero = createRequiredHero(req);
                                    setPartyHeroes((prev) => {
                                      if (prev.length < maxPartySize) {
                                        return [...prev, requiredHero];
                                      } else {
                                        // Substitui o último herói do grupo para dar lugar ao herói obrigatório
                                        const nextParty = [...prev];
                                        nextParty[prev.length - 1] = requiredHero;
                                        return nextParty;
                                      }
                                    });
                                    setSuccessMessage(`Herói obrigatório "${requiredHero.name}" (${requiredHero.className}) forjado e adicionado ao grupo!`);
                                  }}
                                  data-testid={`auto-forge-req-${rIdx}`}
                                  style={{
                                    padding: '5px 10px',
                                    borderRadius: '6px',
                                    backgroundColor: 'rgba(234, 179, 8, 0.2)',
                                    border: '1px solid #facc15',
                                    color: '#facc15',
                                    fontSize: '11px',
                                    fontWeight: 700,
                                    cursor: 'pointer',
                                    display: 'flex',
                                    alignItems: 'center',
                                    gap: '4px',
                                    whiteSpace: 'nowrap',
                                  }}
                                >
                                  <Sparkles size={12} /> {partyHeroes.length >= maxPartySize ? `Substituir por ${req.required_name || req.required_class || (req.suggested_options && req.suggested_options[0]) || 'Herói Requerido'}` : `Forjar ${req.required_name || req.required_class || (req.suggested_options && req.suggested_options[0]) || 'Herói Requerido'}`}
                                </button>
                              )}
                            </div>
                          </div>
                        );
                      })}
                    </div>
                  ) : (
                    <div style={{ fontSize: '12px', color: '#6ee7b7', display: 'flex', alignItems: 'center', gap: '6px' }}>
                      <CheckCircle2 size={14} color="#10b981" />
                      <span>
                        <strong>Formação Livre:</strong> O Mestre IA avaliou que qualquer composição de heróis se adapta perfeitamente a esta saga.
                      </span>
                    </div>
                  )}
                </div>

                {/* Ameaças Centrais Homebrew da Premissa (Sem Spoilers: Ficha Técnica Oculta como Segredo do Mestre) */}
                {premiseAnalysis.custom_monsters && premiseAnalysis.custom_monsters.length > 0 && (
                  <div style={{ marginTop: '8px', paddingTop: '8px', borderTop: '1px solid rgba(255, 255, 255, 0.08)' }}>
                    <div style={{ fontSize: '11.5px', fontWeight: 800, color: '#f87171', display: 'flex', alignItems: 'center', gap: '6px', marginBottom: '6px' }}>
                      <Shield size={13} color="#f87171" /> Ameaças &amp; Antagonistas Registrados na Crônica ({premiseAnalysis.custom_monsters.length}):
                    </div>
                    <div style={{ display: 'flex', flexWrap: 'wrap', gap: '8px' }}>
                      {premiseAnalysis.custom_monsters.map((m: any, mIdx: number) => (
                        <div
                          key={mIdx}
                          style={{
                            backgroundColor: 'rgba(0, 0, 0, 0.4)',
                            border: '1px solid rgba(239, 68, 68, 0.3)',
                            borderRadius: '6px',
                            padding: '6px 10px',
                            fontSize: '11px',
                            display: 'flex',
                            alignItems: 'center',
                            gap: '8px',
                          }}
                        >
                          <strong style={{ color: '#fca5a5' }}>{m.name}</strong>
                          <span style={{ fontSize: '10px', color: '#94a3b8', fontStyle: 'italic' }}>
                            🔒 Estatísticas e Táticas Ocultas (Segredo do Mestre)
                          </span>
                        </div>
                      ))}
                    </div>
                  </div>
                )}

                {/* Overrides de Subclasse e Magias Homebrew */}
                {((premiseAnalysis.subclass_overrides && premiseAnalysis.subclass_overrides.length > 0) ||
                  (premiseAnalysis.custom_spells && premiseAnalysis.custom_spells.length > 0)) && (
                  <div style={{ marginTop: '8px', paddingTop: '8px', borderTop: '1px solid rgba(255, 255, 255, 0.08)', display: 'flex', flexDirection: 'column', gap: '6px' }}>
                    {premiseAnalysis.subclass_overrides && premiseAnalysis.subclass_overrides.length > 0 && (
                      <div style={{ fontSize: '11px', color: '#e9d5ff' }}>
                        <strong style={{ color: '#c084fc' }}>👑 Subclasses & Patronos Homebrew:</strong>{' '}
                        {premiseAnalysis.subclass_overrides.map((ov, i) => (
                          <span key={i} style={{ marginRight: '8px' }}>
                            {ov.name} ({ov.parent_class}){i < (premiseAnalysis.subclass_overrides?.length || 0) - 1 ? ',' : ''}
                          </span>
                        ))}
                      </div>
                    )}
                    {premiseAnalysis.custom_spells && premiseAnalysis.custom_spells.length > 0 && (
                      <div style={{ fontSize: '11px', color: '#93c5fd' }}>
                        <strong style={{ color: '#38bdf8' }}>✨ Magias Homebrew Inclusas:</strong>{' '}
                        {premiseAnalysis.custom_spells.map((sp, i) => (
                          <span key={i} style={{ marginRight: '8px' }}>
                            {sp.name} ({sp.level}º Círc. - {sp.classes.join('/')}){i < (premiseAnalysis.custom_spells?.length || 0) - 1 ? ',' : ''}
                          </span>
                        ))}
                      </div>
                    )}
                  </div>
                )}

                {/* Ganchos Iniciais da Party */}
                {premiseAnalysis.party_hooks && premiseAnalysis.party_hooks.length > 0 && (
                  <div style={{ marginTop: '6px', fontSize: '11.5px', color: '#94a3b8' }}>
                    <strong style={{ color: '#cbd5e1' }}>Ganchos Narrativos da Party:</strong>
                    <ul style={{ margin: '4px 0 0 16px', padding: 0 }}>
                      {premiseAnalysis.party_hooks.map((hook, hIdx) => (
                        <li key={hIdx}>{hook}</li>
                      ))}
                    </ul>
                  </div>
                )}
              </div>
            ) : null}

            {/* ROSTER DOS HERÓIS NA MESA */}
            <div style={{ display: 'flex', flexDirection: 'column', gap: '10px' }}>
              <div style={{ display: 'flex', justifyContent: 'space-between', alignItems: 'center', flexWrap: 'wrap', gap: '8px' }}>
                <label className="rpg-label" style={{ color: '#38bdf8', margin: 0 }}>
                  <span>Integrantes do Grupo ({partyHeroes.length}/{maxPartySize} Heróis):</span>
                </label>
                <div style={{ display: 'flex', gap: '8px', flexWrap: 'wrap' }}>
                  {partyHeroes.length < maxPartySize && (
                    <button
                      type="button"
                      onClick={() => {
                        setEditingHeroIndex(null);
                        setShowInlineWizard(true);
                      }}
                      className="rpg-preset-pill"
                      style={{ fontSize: '11.5px', borderColor: 'rgba(56, 189, 248, 0.4)', color: '#38bdf8' }}
                      data-testid="forge-new-hero-btn"
                    >
                      <UserPlus size={13} style={{ marginRight: '4px' }} /> Forjar Novo Herói (Personalizado)
                    </button>
                  )}
                </div>
              </div>

              {/* Cards dos Heróis */}
              <div style={{ display: 'grid', gridTemplateColumns: 'repeat(auto-fill, minmax(260px, 1fr))', gap: '12px' }}>
                {partyHeroes.map((hero, idx) => (
                  <div
                    key={hero.id || idx}
                    data-testid={`hero-card-${idx}`}
                    style={{
                      padding: '14px',
                      borderRadius: '10px',
                      backgroundColor: '#0c1017',
                      border: idx === 0 ? '1.5px solid #facc15' : '1px solid rgba(255, 255, 255, 0.1)',
                      display: 'flex',
                      flexDirection: 'column',
                      justifyContent: 'space-between',
                      gap: '10px',
                    }}
                  >
                    <div>
                      <div style={{ display: 'flex', justifyContent: 'space-between', alignItems: 'center', marginBottom: '6px' }}>
                        <span style={{ fontWeight: 800, fontSize: '14px', color: '#ffffff' }}>
                          {hero.name}
                        </span>
                        {idx === 0 && (
                          <span
                            style={{
                              fontSize: '10px',
                              padding: '2px 6px',
                              borderRadius: '4px',
                              backgroundColor: 'rgba(234, 179, 8, 0.2)',
                              color: '#facc15',
                              fontWeight: 800,
                            }}
                          >
                            HOST (P1)
                          </span>
                        )}
                      </div>

                      <div style={{ fontSize: '12px', color: '#93c5fd', fontWeight: 600 }}>
                        {hero.race} • {hero.className} Nv.{hero.level}
                      </div>

                      <div style={{ display: 'flex', gap: '10px', marginTop: '8px', fontSize: '11.5px', color: '#cbd5e1' }}>
                        <span style={{ color: '#4ade80', fontWeight: 700 }}>PV {hero.vitals.hpMax}/{hero.vitals.hpMax}</span>
                        <span style={{ color: '#38bdf8', fontWeight: 700 }}>CA {hero.vitals.armorClass}</span>
                        <span style={{ color: '#94a3b8' }}>Desl. {hero.vitals.speedFeet}ft</span>
                      </div>

                      <div style={{ display: 'flex', gap: '6px', marginTop: '6px', fontSize: '10.5px', color: '#64748b', fontFamily: 'monospace' }}>
                        <span>FOR {hero.abilities.strength}</span>
                        <span>DES {hero.abilities.dexterity}</span>
                        <span>CON {hero.abilities.constitution}</span>
                        <span>INT {hero.abilities.intelligence}</span>
                        <span>SAB {hero.abilities.wisdom}</span>
                        <span>CAR {hero.abilities.charisma}</span>
                      </div>
                    </div>

                    <div style={{ display: 'flex', justifyContent: 'flex-end', gap: '8px', paddingTop: '6px', borderTop: '1px solid rgba(255, 255, 255, 0.06)' }}>
                      <button
                        type="button"
                        onClick={() => {
                          setEditingHeroIndex(idx);
                          setShowInlineWizard(true);
                        }}
                        data-testid={`edit-hero-${idx}`}
                        style={{
                          padding: '4px 10px',
                          borderRadius: '4px',
                          backgroundColor: 'rgba(56, 189, 248, 0.1)',
                          border: '1px solid rgba(56, 189, 248, 0.3)',
                          color: '#38bdf8',
                          fontSize: '11px',
                          cursor: 'pointer',
                          display: 'flex',
                          alignItems: 'center',
                          gap: '4px',
                          fontWeight: 600,
                        }}
                      >
                        <Edit2 size={12} /> Editar
                      </button>

                      {partyHeroes.length > 1 && (
                        <button
                          type="button"
                          onClick={() => setPartyHeroes((prev) => prev.filter((_, i) => i !== idx))}
                          data-testid={`remove-hero-${idx}`}
                          style={{
                            padding: '4px 8px',
                            borderRadius: '4px',
                            backgroundColor: 'rgba(239, 68, 68, 0.1)',
                            border: '1px solid rgba(239, 68, 68, 0.3)',
                            color: '#f87171',
                            fontSize: '11px',
                            cursor: 'pointer',
                            display: 'flex',
                            alignItems: 'center',
                            gap: '4px',
                          }}
                        >
                          <Trash2 size={12} /> Remover
                        </button>
                      )}
                    </div>
                  </div>
                ))}

                {partyHeroes.length === 0 && (
                  <div
                    style={{
                      gridColumn: '1 / -1',
                      padding: '24px 20px',
                      borderRadius: '10px',
                      backgroundColor: 'rgba(15, 23, 42, 0.6)',
                      border: '1.5px dashed rgba(56, 189, 248, 0.35)',
                      textAlign: 'center',
                      display: 'flex',
                      flexDirection: 'column',
                      alignItems: 'center',
                      gap: '10px',
                    }}
                  >
                    <div style={{ padding: '10px', borderRadius: '50%', backgroundColor: 'rgba(56, 189, 248, 0.1)' }}>
                      <Users size={28} color="#38bdf8" />
                    </div>
                    <div style={{ color: '#f8fafc', fontWeight: 700, fontSize: '14px' }}>
                      Nenhum herói forjado para esta campanha ainda
                    </div>
                    <div style={{ color: '#94a3b8', fontSize: '12px', maxWidth: '460px', lineHeight: 1.4 }}>
                      Adicione pelo menos 1 personagem para formar a party inicial ou utilize os botões de requisitos temáticos recomendados pela IA acima. O botão de avançar permanecerá desabilitado até haver ao menos um herói.
                    </div>
                    <button
                      type="button"
                      onClick={() => {
                        setEditingHeroIndex(null);
                        setShowInlineWizard(true);
                      }}
                      className="rpg-cta-gold"
                      style={{ marginTop: '6px', fontSize: '12.5px', padding: '8px 20px', width: 'auto' }}
                      data-testid="forge-first-hero-btn"
                    >
                      <UserPlus size={15} /> Forjar Primeiro Herói
                    </button>
                  </div>
                )}
              </div>
            </div>

            {/* BOTÕES DE NAVEGAÇÃO DA PÁGINA 3 */}
            <div
              style={{
                display: 'flex',
                justifyContent: 'space-between',
                alignItems: 'center',
                gap: '12px',
                paddingTop: '14px',
                borderTop: '1px solid rgba(255, 255, 255, 0.08)',
                flexWrap: 'wrap',
              }}
            >
              <button
                type="button"
                onClick={() => {
                  setError(null);
                  setTab('create_rules');
                }}
                style={{
                  display: 'inline-flex',
                  alignItems: 'center',
                  gap: '6px',
                  padding: '10px 18px',
                  borderRadius: '8px',
                  backgroundColor: 'rgba(255, 255, 255, 0.05)',
                  border: '1px solid rgba(255, 255, 255, 0.15)',
                  color: '#cbd5e1',
                  fontSize: '13px',
                  fontWeight: 600,
                  cursor: 'pointer',
                }}
              >
                <ArrowLeft size={16} /> Voltar para: 2. Regras &amp; Foco
              </button>

              <button
                type="button"
                onClick={() => handleCreate()}
                disabled={loading || partyHeroes.length === 0}
                className="rpg-cta-gold"
                data-testid="start-campaign-btn"
                style={{
                  width: 'auto',
                  padding: '12px 28px',
                  fontSize: '13.5px',
                  backgroundColor: partyHeroes.length === 0 ? '#475569' : undefined,
                  boxShadow: partyHeroes.length === 0 ? 'none' : undefined,
                  cursor: partyHeroes.length === 0 ? 'not-allowed' : undefined,
                }}
              >
                <Flame size={18} />
                {loading
                  ? 'Forjando Esqueleto Procedural & Prólogo Nominal...'
                  : partyHeroes.length === 0
                  ? '🔒 Bloqueado: Forje pelo menos 1 Herói para Iniciar'
                  : unmetMandatoryReqs.length > 0
                  ? `Avançar para a Sala de Espera (${unmetMandatoryReqs.length} req. pendente${unmetMandatoryReqs.length > 1 ? 's' : ''})`
                  : `⚔️ Iniciar Campanha com Este Grupo (${partyHeroes.length} Heróis)`}
              </button>
            </div>
          </div>
        ) : tab === 'waiting_room' ? (
          /* ESTADO LOBBY DE ESPERA WEBSOCKET DA SALA */
          <div className="rpg-panel-box" style={{ padding: '28px', gap: '20px', maxWidth: '850px', margin: '0 auto', width: '100%' }}>
            <div style={{ display: 'flex', justifyContent: 'space-between', alignItems: 'center', borderBottom: '1px solid rgba(255, 255, 255, 0.1)', paddingBottom: '16px', flexWrap: 'wrap', gap: '12px' }}>
              <div style={{ display: 'flex', alignItems: 'center', gap: '12px' }}>
                <div style={{ padding: '10px', borderRadius: '50%', backgroundColor: 'rgba(234, 179, 8, 0.15)' }}>
                  <Radio size={24} color="#facc15" />
                </div>
                <div>
                  <h2 style={{ fontSize: '18px', fontWeight: 800, color: '#ffffff', margin: 0, display: 'flex', alignItems: 'center', gap: '8px' }}>
                    Lobby da Sala de Espera (Ao Vivo)
                  </h2>
                  <div style={{ fontSize: '12px', color: '#94a3b8', display: 'flex', alignItems: 'center', gap: '6px', marginTop: '2px' }}>
                    <span style={{ display: 'inline-block', width: '8px', height: '8px', borderRadius: '50%', backgroundColor: wsConnected ? '#10b981' : '#f59e0b' }} />
                    <span>{wsConnected ? 'Conectado ao WebSocket da Sala' : 'Sincronizando via WebSocket...'}</span>
                  </div>
                </div>
              </div>

              {/* Informações de Convite (Código & UUID) */}
              <div style={{ display: 'flex', gap: '10px', alignItems: 'center', flexWrap: 'wrap' }}>
                {createdSession?.room_code && (
                  <div
                    style={{
                      display: 'flex',
                      alignItems: 'center',
                      gap: '8px',
                      padding: '6px 12px',
                      borderRadius: '8px',
                      backgroundColor: 'rgba(56, 189, 248, 0.12)',
                      border: '1px solid rgba(56, 189, 248, 0.3)',
                    }}
                  >
                    <span style={{ fontSize: '11px', color: '#94a3b8' }}>Código:</span>
                    <strong style={{ fontSize: '15px', color: '#38bdf8', fontFamily: 'monospace', letterSpacing: '0.1em' }}>
                      {createdSession.room_code}
                    </strong>
                    <button
                      type="button"
                      onClick={() => {
                        navigator.clipboard.writeText(createdSession.room_code);
                        setCopiedCode(true);
                        setTimeout(() => setCopiedCode(false), 2000);
                      }}
                      title="Copiar Código"
                      style={{ background: 'transparent', border: 'none', color: '#38bdf8', cursor: 'pointer', padding: '2px' }}
                    >
                      {copiedCode ? <Check size={14} color="#10b981" /> : <Copy size={14} />}
                    </button>
                  </div>
                )}

                {createdSession?.campaign_id && (
                  <div
                    style={{
                      display: 'flex',
                      alignItems: 'center',
                      gap: '8px',
                      padding: '6px 12px',
                      borderRadius: '8px',
                      backgroundColor: 'rgba(234, 179, 8, 0.12)',
                      border: '1px solid rgba(234, 179, 8, 0.3)',
                    }}
                  >
                    <span style={{ fontSize: '11px', color: '#94a3b8' }}>UUID de Convite:</span>
                    <strong style={{ fontSize: '12px', color: '#facc15', fontFamily: 'monospace' }}>
                      {createdSession.campaign_id.slice(0, 8)}...
                    </strong>
                    <button
                      type="button"
                      onClick={() => {
                        navigator.clipboard.writeText(createdSession.campaign_id);
                        setCopiedUuid(true);
                        setTimeout(() => setCopiedUuid(false), 2000);
                      }}
                      title="Copiar UUID de Convite"
                      style={{ background: 'transparent', border: 'none', color: '#facc15', cursor: 'pointer', padding: '2px' }}
                    >
                      {copiedUuid ? <Check size={14} color="#10b981" /> : <Copy size={14} />}
                    </button>
                  </div>
                )}
              </div>
            </div>

            {/* Checklist de Formação da Party: O que falta preencher */}
            <div style={{ backgroundColor: 'rgba(0, 0, 0, 0.35)', border: '1px solid rgba(255, 255, 255, 0.08)', borderRadius: '10px', padding: '14px 16px' }}>
              <div style={{ fontSize: '13px', fontWeight: 700, color: '#f8fafc', marginBottom: '8px', display: 'flex', alignItems: 'center', gap: '6px' }}>
                <Users size={16} color="#38bdf8" />
                <span>Status de Preenchimento da Comitiva:</span>
              </div>

              {/* Pendências da Party */}
              <div style={{ display: 'flex', flexDirection: 'column', gap: '6px' }}>
                {(() => {
                  const slots = liveSlots.length > 0 ? liveSlots : (createdSession?.player_slots || []);
                  const totalOccupied = Math.max(
                    partyHeroes.length,
                    slots.filter((s: any) => s.player_name && s.player_name.trim().length > 0).length
                  );
                  const effectivePartyMax = Math.max(maxPartySize, totalOccupied, 4);
                  const missingSlots = Math.max(0, effectivePartyMax - totalOccupied);

                  // Avalia se há requisitos de roleplay que ainda faltam ser preenchidos pelos slots conectados ou partyHeroes
                  const connectedHeroes = slots.map((s: any) => ({
                    id: s.player_id,
                    name: s.player_name,
                    className: s.class_name,
                    race: 'Desconhecida',
                    level: 1,
                    background: '',
                    alignment: '',
                    proficiencyBonus: 2,
                    abilities: { strength: 10, dexterity: 10, constitution: 10, intelligence: 10, wisdom: 10, charisma: 10 },
                    savingThrowProficiencies: [],
                    skillProficiencies: [],
                    vitals: { hpCurrent: 10, hpMax: 10, hpTemp: 0, armorClass: 10, initiativeModifier: 0, speedFeet: 30, hitDiceTotal: 1, hitDiceCurrent: 1, hitDiceType: 'd8', deathSaves: { successes: 0, failures: 0 }, conditions: [] },
                    spellSlots: { levels: {} },
                    spellsKnownOrPrepared: [],
                    classResources: {},
                    inventory: [],
                  }));

                  const allPresentHeroes = partyHeroes.length > 0 ? partyHeroes : connectedHeroes;

                  const unmetNow = (premiseAnalysis?.party_requirements || []).filter(
                    (req) => req.is_mandatory && !isRequirementSatisfied(req, allPresentHeroes as any)
                  );

                  return (
                    <>
                      {unmetNow.length > 0 ? (
                        unmetNow.map((req, uIdx) => (
                          <div
                            key={uIdx}
                            style={{
                              padding: '8px 12px',
                              borderRadius: '6px',
                              backgroundColor: 'rgba(239, 68, 68, 0.12)',
                              border: '1px solid rgba(239, 68, 68, 0.3)',
                              fontSize: '12px',
                              color: '#fca5a5',
                              display: 'flex',
                              alignItems: 'center',
                              justifyContent: 'space-between',
                            }}
                          >
                            <span>
                              <strong>Falta preencher na Party:</strong> {req.required_name ? `Herói '${req.required_name}' (${req.required_class || 'Classe Livre'})` : req.required_class ? `Classe '${req.required_class}'` : req.description}
                            </span>
                            <span style={{ fontSize: '11px', color: '#f87171', fontWeight: 700 }}>Obrigatório</span>
                          </div>
                        ))
                      ) : (
                        <div
                          style={{
                            padding: '8px 12px',
                            borderRadius: '6px',
                            backgroundColor: 'rgba(16, 185, 129, 0.12)',
                            border: '1px solid rgba(16, 185, 129, 0.3)',
                            fontSize: '12px',
                            color: '#6ee7b7',
                            display: 'flex',
                            alignItems: 'center',
                            gap: '6px',
                          }}
                        >
                          <CheckCircle2 size={15} color="#10b981" />
                          <span>Todos os requisitos mandatórios da narrativa foram satisfeitos!</span>
                        </div>
                      )}

                      {missingSlots > 0 && (
                        <div style={{ fontSize: '12px', color: '#94a3b8', marginTop: '4px' }}>
                          💡 Há <strong>{missingSlots} vaga{missingSlots > 1 ? 's' : ''} aberta{missingSlots > 1 ? 's' : ''}</strong> no grupo (total de {effectivePartyMax} heróis suportados). Outros jogadores podem se conectar usando o código de 4 letras da sala.
                        </div>
                      )}
                    </>
                  );
                })()}
              </div>
            </div>

            {/* Visualização dos Slots do Grupo via WebSocket (1 a 6) */}
            <div style={{ display: 'grid', gridTemplateColumns: 'repeat(auto-fit, minmax(200px, 1fr))', gap: '12px' }}>
              {Array.from({ length: Math.min(6, Math.max(liveSlots.length, (createdSession?.player_slots || []).length, partyHeroes.length, maxPartySize, 4)) }, (_, i) => i + 1).map((slotNumber) => {
                const slots = liveSlots.length > 0 ? liveSlots : (createdSession?.player_slots || []);
                const slot = slots.find((s: any) => s.player_number === slotNumber);
                const isOccupied = Boolean(slot && slot.player_name && slot.player_name.trim().length > 0);

                return (
                  <div
                    key={slotNumber}
                    style={{
                      padding: '16px',
                      borderRadius: '10px',
                      backgroundColor: isOccupied ? '#0c1017' : 'rgba(15, 23, 42, 0.4)',
                      border: isOccupied
                        ? slotNumber === 1
                          ? '1.5px solid #facc15'
                          : '1px solid rgba(56, 189, 248, 0.4)'
                        : '1.5px dashed rgba(255, 255, 255, 0.15)',
                      display: 'flex',
                      flexDirection: 'column',
                      justifyContent: 'space-between',
                      gap: '10px',
                      minHeight: '130px',
                    }}
                  >
                    <div>
                      <div style={{ display: 'flex', justifyContent: 'space-between', alignItems: 'center' }}>
                        <span style={{ fontSize: '11px', fontWeight: 800, color: slotNumber === 1 ? '#facc15' : '#38bdf8' }}>
                          SLOT #{slotNumber} {slotNumber === 1 ? '(HOST)' : ''}
                        </span>
                        <span
                          style={{
                            fontSize: '10px',
                            padding: '2px 6px',
                            borderRadius: '4px',
                            backgroundColor: isOccupied ? 'rgba(16, 185, 129, 0.2)' : 'rgba(255, 255, 255, 0.05)',
                            color: isOccupied ? '#34d399' : '#64748b',
                            fontWeight: 700,
                          }}
                        >
                          {isOccupied ? 'PRONTO' : 'AGUARDANDO'}
                        </span>
                      </div>

                      {isOccupied ? (
                        <div style={{ marginTop: '8px' }}>
                          <div style={{ fontWeight: 800, fontSize: '14px', color: '#ffffff' }}>
                            {slot.player_name}
                          </div>
                          <div style={{ fontSize: '12px', color: '#93c5fd', marginTop: '2px' }}>
                            {slot.class_name || 'Aventureiro'}
                          </div>
                          {slot.player_code && (
                            <div style={{ fontSize: '10px', color: '#64748b', fontFamily: 'monospace', marginTop: '4px' }}>
                              Cód: {slot.player_code}
                            </div>
                          )}
                        </div>
                      ) : (
                        <div style={{ marginTop: '12px', color: '#64748b', fontSize: '12px', textAlign: 'center' }}>
                          Aguardando entrada de aventureiro...
                        </div>
                      )}
                    </div>
                  </div>
                );
              })}
            </div>

            {/* Ações do Lobby: Iniciar Aventura no HUD */}
            <div style={{ display: 'flex', justifyContent: 'space-between', alignItems: 'center', paddingTop: '14px', borderTop: '1px solid rgba(255, 255, 255, 0.08)', flexWrap: 'wrap', gap: '12px' }}>
              <button
                type="button"
                onClick={() => setTab('create_party')}
                style={{
                  display: 'inline-flex',
                  alignItems: 'center',
                  gap: '6px',
                  padding: '10px 18px',
                  borderRadius: '8px',
                  backgroundColor: 'rgba(255, 255, 255, 0.05)',
                  border: '1px solid rgba(255, 255, 255, 0.15)',
                  color: '#cbd5e1',
                  fontSize: '13px',
                  fontWeight: 600,
                  cursor: 'pointer',
                }}
              >
                <ArrowLeft size={16} /> Voltar para Composição
              </button>

              <button
                type="button"
                onClick={() => {
                  if (createdSession) {
                    onSessionReady(createdSession, partyHeroes);
                  }
                }}
                className="rpg-cta-gold"
                style={{ width: 'auto', padding: '12px 30px', fontSize: '14px' }}
                data-testid="enter-hud-btn"
              >
                <Flame size={18} />
                <span>Entrar na Cena Viva (Iniciar Saga no HUD)</span>
                <ArrowRight size={16} />
              </button>
            </div>
          </div>
        ) : tab === 'join' ? (
          /* ESTADO 3: INGRESSAR COM UUID E CÓDIGO DE 4 LETRAS */
          <form onSubmit={handleJoin} className="rpg-panel-box" style={{ padding: '30px 24px', gap: '16px', maxWidth: '580px', margin: '0 auto', width: '100%' }}>
            <div style={{ textAlign: 'center' }}>
              <div style={{ display: 'inline-flex', padding: '10px', borderRadius: '50%', backgroundColor: 'rgba(16, 185, 129, 0.12)', marginBottom: '8px' }}>
                <Shield size={28} color="#10b981" />
              </div>
              <h2 style={{ fontSize: '18px', fontWeight: 800, color: '#ffffff', margin: '0 0 4px 0' }}>
                Entrar em Campanha Existente
              </h2>
              <p style={{ fontSize: '13px', color: '#94a3b8', margin: 0 }}>
                Informe o UUID da Campanha e o Código de 4 Letras gerados pelo criador do mundo.
              </p>
            </div>

            {/* Campo 1: UUID da Campanha */}
            <div style={{ display: 'flex', flexDirection: 'column', gap: '6px', textAlign: 'left', width: '100%' }}>
              <label style={{ fontSize: '12px', fontWeight: 700, color: '#cbd5e1' }}>
                UUID da Campanha (Identificador Único)
              </label>
              <input
                type="text"
                value={campaignUuid}
                onChange={(e) => setCampaignUuid(e.target.value.trim())}
                placeholder="Ex: c7595300-349f-4da3-aa5d-7a54a72d73be"
                className="rpg-input-field"
                style={{
                  fontFamily: 'monospace',
                  fontSize: '12px',
                  padding: '10px 12px',
                  backgroundColor: '#0c1017',
                  border: '1px solid rgba(56, 189, 248, 0.3)',
                  color: '#93c5fd',
                }}
              />
            </div>

            {/* Campo 2: Código de 4 Letras da Sala */}
            <div style={{ display: 'flex', flexDirection: 'column', gap: '6px', textAlign: 'left', width: '100%' }}>
              <label style={{ fontSize: '12px', fontWeight: 700, color: '#cbd5e1' }}>
                Código da Sala (4 Letras) *
              </label>
              <input
                type="text"
                maxLength={4}
                value={roomCode}
                onChange={(e) => setRoomCode(e.target.value.toUpperCase())}
                required
                placeholder="ABCD"
                style={{
                  width: '100%',
                  maxWidth: '180px',
                  backgroundColor: '#06080c',
                  border: '2px solid rgba(16, 185, 129, 0.5)',
                  borderRadius: '10px',
                  padding: '8px 14px',
                  fontSize: '26px',
                  fontFamily: 'var(--font-mono, monospace)',
                  fontWeight: 900,
                  color: '#34d399',
                  textAlign: 'center',
                  letterSpacing: '0.25em',
                  outline: 'none',
                }}
              />
            </div>

            {/* Campo 3: Código Pessoal do Jogador (Opcional - Retomada de Slot) */}
            <div style={{ display: 'flex', flexDirection: 'column', gap: '6px', textAlign: 'left', width: '100%' }}>
              <div style={{ display: 'flex', justifyContent: 'space-between', alignItems: 'center' }}>
                <label style={{ fontSize: '12px', fontWeight: 700, color: '#cbd5e1' }}>
                  Código Pessoal do Jogador (Opcional)
                </label>
                <span style={{ fontSize: '11px', color: '#94a3b8' }}>Para reassumir seu personagem existente</span>
              </div>
              <input
                type="text"
                maxLength={6}
                value={playerCodeInput}
                onChange={(e) => setPlayerCodeInput(e.target.value.toUpperCase())}
                placeholder="Ex: H1AB ou P2XY"
                className="rpg-input-field"
                style={{
                  width: '100%',
                  maxWidth: '180px',
                  fontFamily: 'monospace',
                  fontSize: '13px',
                  letterSpacing: '0.1em',
                  textTransform: 'uppercase',
                }}
              />
            </div>

            {/* Aviso sobre criação de personagem na Tela 2 */}
            <div
              style={{
                padding: '10px 14px',
                borderRadius: '8px',
                backgroundColor: 'rgba(234, 179, 8, 0.08)',
                border: '1px solid rgba(234, 179, 8, 0.25)',
                color: '#fde047',
                fontSize: '12px',
                textAlign: 'left',
                lineHeight: 1.45,
              }}
            >
              ⚔️ <strong>Nota:</strong> A criação e configuração do seu herói (nome, raça, classe e atributos) será feita na próxima etapa (<strong>2. Personagem</strong>).
            </div>

            {/* Checkbox Modo Um Terminal */}
            <label
              style={{
                display: 'flex',
                alignItems: 'center',
                gap: '8px',
                fontSize: '12px',
                color: '#e2e8f0',
                cursor: 'pointer',
                backgroundColor: isSingleTerminalMode ? 'rgba(56, 189, 248, 0.15)' : 'rgba(255, 255, 255, 0.03)',
                border: isSingleTerminalMode ? '1px solid #38bdf8' : '1px solid rgba(255, 255, 255, 0.1)',
                borderRadius: '8px',
                padding: '8px 12px',
                width: '100%',
                transition: 'all 0.2s',
              }}
            >
              <input
                type="checkbox"
                checked={isSingleTerminalMode}
                onChange={(e) => setIsSingleTerminalMode(e.target.checked)}
                style={{ cursor: 'pointer' }}
              />
              <span>🖥️ Jogar no <strong>Modo Um Terminal</strong> (Presencial / Sem necessidade de WebSockets)</span>
            </label>

            <button
              type="submit"
              disabled={loading}
              className="rpg-cta-gold"
              style={{
                width: '100%',
                background: 'linear-gradient(135deg, #34d399 0%, #059669 100%)',
                marginTop: '4px',
              }}
            >
              <Users size={18} />
              {loading ? 'Conectando...' : 'Entrar na Campanha'}
            </button>
          </form>
        ) : (
          /* ESTADO 4: RETOMAR CAMPANHA SALVA OU IMPORTAR ARQUIVO EXTERNO */
          <div className="rpg-panel-box">
            <div style={{ display: 'flex', justifyContent: 'space-between', alignItems: 'center', flexWrap: 'wrap', gap: '8px' }}>
              <div>
                <h2 style={{ margin: 0, fontSize: '16px', fontWeight: 800, color: '#fff' }}>
                  Restaurar Arquivo de Campanha (.JSON)
                </h2>
                <span style={{ fontSize: '12px', color: '#94a3b8' }}>
                  Restaura o estado salvo completo do mundo, cenas ativas, histórico da crônica e fichas de personagens.
                </span>
              </div>

              <label className="rpg-preset-pill" style={{ cursor: 'pointer', borderColor: '#a855f7', color: '#c084fc', padding: '8px 14px', fontWeight: 700 }}>
                <Upload size={14} style={{ marginRight: '6px' }} /> Importar Arquivo de Campanha (.JSON)
                <input type="file" accept=".json" onChange={handleImportCampaignFile} style={{ display: 'none' }} />
              </label>
            </div>

            {savedCampaigns.length === 0 ? (
              <div style={{ textAlign: 'center', padding: '40px', color: '#94a3b8' }}>
                Nenhuma campanha gravada no servidor encontrada.
              </div>
            ) : (
              <div style={{ display: 'flex', flexDirection: 'column', gap: '10px' }}>
                {savedCampaigns.map((camp) => (
                  <div
                    key={camp.campaign_id}
                    style={{
                      backgroundColor: '#07090e',
                      border: '1px solid rgba(255, 255, 255, 0.08)',
                      borderRadius: '12px',
                      padding: '14px 18px',
                      display: 'flex',
                      justifyContent: 'space-between',
                      alignItems: 'center',
                      flexWrap: 'wrap',
                      gap: '12px',
                    }}
                  >
                    <div style={{ display: 'flex', flexDirection: 'column', gap: '4px' }}>
                      <div style={{ display: 'flex', alignItems: 'center', gap: '8px', flexWrap: 'wrap' }}>
                        <strong style={{ fontSize: '14px', color: '#facc15' }}>
                          Sala #{camp.room_code} • {camp.current_act_title}
                        </strong>
                        <span style={{ fontSize: '10px', backgroundColor: 'rgba(234, 179, 8, 0.15)', color: '#facc15', padding: '2px 6px', borderRadius: '4px' }}>
                          Ato {camp.current_act_number}
                        </span>
                        <span style={{ fontSize: '10px', fontFamily: 'monospace', color: '#93c5fd', backgroundColor: 'rgba(56, 189, 248, 0.1)', padding: '2px 6px', borderRadius: '4px' }}>
                          UUID: {camp.campaign_id.slice(0, 8)}...
                        </span>
                      </div>
                      <span style={{ fontSize: '12px', color: '#cbd5e1', maxWidth: '580px' }}>
                        {camp.premise.slice(0, 110)}...
                      </span>
                    </div>

                    <div style={{ display: 'flex', gap: '8px', flexWrap: 'wrap' }}>
                      <button
                        type="button"
                        onClick={async () => {
                          setLoading(true);
                          try {
                            const joined = await joinCampaign({
                              campaign_id: camp.campaign_id,
                              room_code: camp.room_code,
                              is_single_terminal: false,
                            });
                            onSessionReady(joined);
                          } catch (err: any) {
                            setError(err.message || 'Erro ao carregar campanha');
                          } finally {
                            setLoading(false);
                          }
                        }}
                        style={{
                          display: 'flex',
                          alignItems: 'center',
                          gap: '6px',
                          padding: '8px 14px',
                          backgroundColor: 'rgba(16, 185, 129, 0.15)',
                          border: '1px solid #10b981',
                          borderRadius: '8px',
                          color: '#6ee7b7',
                          fontWeight: 700,
                          fontSize: '12px',
                          cursor: 'pointer',
                        }}
                      >
                        <Play size={14} /> Retomar Online
                      </button>

                      <button
                        type="button"
                        onClick={async () => {
                          setLoading(true);
                          try {
                            const joined = await joinCampaign({
                              campaign_id: camp.campaign_id,
                              room_code: camp.room_code,
                              is_single_terminal: true,
                            });
                            onSessionReady(joined);
                          } catch (err: any) {
                            setError(err.message || 'Erro ao carregar campanha');
                          } finally {
                            setLoading(false);
                          }
                        }}
                        style={{
                          display: 'flex',
                          alignItems: 'center',
                          gap: '6px',
                          padding: '8px 14px',
                          backgroundColor: 'rgba(56, 189, 248, 0.18)',
                          border: '1px solid #38bdf8',
                          borderRadius: '8px',
                          color: '#7dd3fc',
                          fontWeight: 700,
                          fontSize: '12px',
                          cursor: 'pointer',
                        }}
                      >
                        <Monitor size={14} /> Modo Um Terminal
                      </button>

                      <button
                        type="button"
                        onClick={async () => {
                          const confirmDelete = window.confirm(
                            `Tem certeza de que deseja excluir permanentemente a campanha da Sala #${camp.room_code}?\n\nEsta ação apagará todos os dados, fichas e histórico da crônica do servidor.`
                          );
                          if (!confirmDelete) return;

                          setLoading(true);
                          try {
                            await deleteCampaign(camp.campaign_id);
                            setSavedCampaigns((prev) => prev.filter((c) => c.campaign_id !== camp.campaign_id));
                            setSuccessMessage(`Campanha da Sala #${camp.room_code} excluída permanentemente com sucesso!`);
                          } catch (err: any) {
                            setError(err.message || 'Erro ao excluir campanha');
                          } finally {
                            setLoading(false);
                          }
                        }}
                        data-testid={`delete-campaign-${camp.room_code}`}
                        style={{
                          display: 'flex',
                          alignItems: 'center',
                          gap: '6px',
                          padding: '8px 14px',
                          backgroundColor: 'rgba(239, 68, 68, 0.15)',
                          border: '1px solid #ef4444',
                          borderRadius: '8px',
                          color: '#f87171',
                          fontWeight: 700,
                          fontSize: '12px',
                          cursor: 'pointer',
                        }}
                        title="Excluir campanha salva permanentemente"
                      >
                        <Trash2 size={14} /> Excluir
                      </button>
                    </div>
                  </div>
                ))}
              </div>
            )}
          </div>
        )}
      </div>

      {showInlineWizard && (
        <div
          className="modal-backdrop-fade"
          style={{
            position: 'fixed',
            inset: 0,
            zIndex: 9999,
            backgroundColor: 'rgba(5, 5, 10, 0.85)',
            backdropFilter: 'blur(8px)',
            display: 'flex',
            alignItems: 'center',
            justifyContent: 'center',
            padding: '20px',
          }}
        >
          <div
            className="modal-content-fade"
            style={{
              width: '100%',
              maxWidth: '1100px',
              maxHeight: '90vh',
              overflowY: 'auto',
              backgroundColor: '#0f172a',
              borderRadius: '16px',
              border: '1px solid rgba(234, 179, 8, 0.3)',
              boxShadow: '0 25px 50px -12px rgba(0, 0, 0, 0.7)',
              position: 'relative',
              padding: '24px',
            }}
          >
            <button
              onClick={() => {
                setShowInlineWizard(false);
                setEditingHeroIndex(null);
              }}
              style={{
                position: 'absolute',
                top: '16px',
                right: '16px',
                background: 'rgba(255, 255, 255, 0.1)',
                border: 'none',
                color: '#cbd5e1',
                width: '32px',
                height: '32px',
                borderRadius: '50%',
                cursor: 'pointer',
                fontWeight: 800,
                zIndex: 10,
              }}
              title="Fechar Forja de Herói"
              data-testid="close-wizard-modal-btn"
            >
              ✕
            </button>
            <CharacterCreationWizard
              isAddingToParty={editingHeroIndex === null}
              nextPlayerNumber={partyHeroes.length + 1}
              subclassOverrides={premiseAnalysis?.subclass_overrides || []}
              customSpells={premiseAnalysis?.custom_spells || []}
              initialSheet={editingHeroIndex !== null ? partyHeroes[editingHeroIndex] : undefined}
              onCharacterCreated={(sheet) => {
                if (editingHeroIndex !== null) {
                  setPartyHeroes((prev) => {
                    const next = [...prev];
                    next[editingHeroIndex] = sheet;
                    return next;
                  });
                  setSuccessMessage(`Herói "${sheet.name}" atualizado com sucesso!`);
                } else {
                  setPartyHeroes((prev) => [...prev, sheet]);
                  setSuccessMessage(`Novo herói "${sheet.name}" forjado e integrado ao grupo!`);
                }
                setShowInlineWizard(false);
                setEditingHeroIndex(null);
              }}
              onCancel={() => {
                setShowInlineWizard(false);
                setEditingHeroIndex(null);
              }}
            />
          </div>
        </div>
      )}

      {/* MODAL: PAINEL DE CONTROLE DE HISTÓRIAS PRÉ-CONSTRUÍDAS */}
      {showStoryPanel && (
        <div
          className="modal-backdrop-fade"
          style={{
            position: 'fixed',
            top: 0,
            left: 0,
            width: '100vw',
            height: '100vh',
            backgroundColor: 'rgba(0, 0, 0, 0.85)',
            backdropFilter: 'blur(6px)',
            zIndex: 99999,
            display: 'flex',
            alignItems: 'center',
            justifyContent: 'center',
            padding: '20px',
          }}
        >
          <div
            className="modal-content-fade"
            style={{
              width: '100%',
              maxWidth: '920px',
              maxHeight: '90vh',
              overflowY: 'auto',
              backgroundColor: '#0c1017',
              borderRadius: '16px',
              border: '1px solid rgba(234, 179, 8, 0.4)',
              boxShadow: '0 25px 50px -12px rgba(0, 0, 0, 0.85)',
              position: 'relative',
              padding: '28px',
              display: 'flex',
              flexDirection: 'column',
              gap: '20px',
            }}
          >
            {/* Modal Header */}
            <div style={{ display: 'flex', justifyContent: 'space-between', alignItems: 'center', borderBottom: '1px solid rgba(255, 255, 255, 0.08)', paddingBottom: '16px' }}>
              <div>
                <div style={{ display: 'inline-flex', alignItems: 'center', gap: '6px', color: '#facc15', fontSize: '11px', fontWeight: 800, textTransform: 'uppercase', letterSpacing: '0.08em', marginBottom: '4px' }}>
                  <Sparkles size={14} /> Biblioteca &amp; Repositório de Aventuras
                </div>
                <h2 style={{ fontSize: '20px', color: '#ffffff', margin: 0, fontWeight: 900 }}>
                  PAINEL DE CONTROLE DE HISTÓRIAS PRÉ-CONSTRUÍDAS
                </h2>
                <span style={{ fontSize: '12px', color: '#94a3b8' }}>
                  Gerencie o catálogo de premissas salvas no servidor, crie novas aventuras prontas ou carregue-as diretamente.
                </span>
              </div>
              <button
                type="button"
                onClick={() => setShowStoryPanel(false)}
                data-testid="close-story-panel"
                style={{
                  background: 'rgba(255, 255, 255, 0.08)',
                  border: '1px solid rgba(255, 255, 255, 0.15)',
                  color: '#cbd5e1',
                  width: '32px',
                  height: '32px',
                  borderRadius: '50%',
                  cursor: 'pointer',
                  display: 'flex',
                  alignItems: 'center',
                  justifyContent: 'center',
                  fontWeight: 800,
                }}
              >
                ✕
              </button>
            </div>

            {/* Grid com Histórias Cadastradas */}
            <div style={{ display: 'flex', flexDirection: 'column', gap: '10px' }}>
              <div style={{ display: 'flex', justifyContent: 'space-between', alignItems: 'center' }}>
                <span style={{ fontSize: '12px', fontWeight: 800, color: '#38bdf8', textTransform: 'uppercase', letterSpacing: '0.08em' }}>
                  Histórias no Servidor ({prebuiltStories.length}):
                </span>
                <span style={{ fontSize: '11px', color: '#64748b' }}>Armazenadas em data/premises/</span>
              </div>

              <div style={{ display: 'grid', gridTemplateColumns: 'repeat(auto-fill, minmax(280px, 1fr))', gap: '12px' }}>
                {prebuiltStories.map((story) => (
                  <div
                    key={story.id}
                    style={{
                      padding: '14px',
                      borderRadius: '10px',
                      backgroundColor: 'rgba(255, 255, 255, 0.03)',
                      border: '1px solid rgba(255, 255, 255, 0.08)',
                      display: 'flex',
                      flexDirection: 'column',
                      justifyContent: 'space-between',
                      gap: '12px',
                    }}
                  >
                    <div>
                      <div style={{ display: 'flex', justifyContent: 'space-between', alignItems: 'flex-start', gap: '8px', marginBottom: '6px' }}>
                        <strong style={{ fontSize: '13px', color: '#f8fafc', lineHeight: 1.3 }}>
                          {story.title}
                        </strong>
                        <span style={{ fontSize: '10px', color: '#facc15', backgroundColor: 'rgba(234, 179, 8, 0.15)', padding: '2px 6px', borderRadius: '4px', whiteSpace: 'nowrap' }}>
                          D20: {story.dramatic_focus_slider}
                        </span>
                      </div>
                      <p style={{ fontSize: '11.5px', color: '#94a3b8', margin: '0 0 8px 0', lineHeight: 1.4, maxHeight: '60px', overflowY: 'auto' }}>
                        {story.premise}
                      </p>
                      <div style={{ fontSize: '10.5px', color: '#64748b' }}>
                        <em>{story.literary_style}</em>
                      </div>
                    </div>

                    <div style={{ display: 'flex', gap: '8px', borderTop: '1px solid rgba(255, 255, 255, 0.06)', paddingTop: '10px' }}>
                      <button
                        type="button"
                        onClick={() => {
                          handleSelectPrebuiltStory(story);
                          setShowStoryPanel(false);
                        }}
                        data-testid={`use-story-btn-${story.id}`}
                        style={{
                          flex: 1,
                          padding: '6px 12px',
                          borderRadius: '6px',
                          backgroundColor: 'rgba(16, 185, 129, 0.15)',
                          border: '1px solid #10b981',
                          color: '#34d399',
                          fontSize: '11.5px',
                          fontWeight: 700,
                          cursor: 'pointer',
                        }}
                      >
                        Carregar na Mesa
                      </button>

                      <button
                        type="button"
                        onClick={() => handleDeleteStory(story.id)}
                        data-testid={`delete-story-btn-${story.id}`}
                        style={{
                          padding: '6px 10px',
                          borderRadius: '6px',
                          backgroundColor: 'rgba(239, 68, 68, 0.1)',
                          border: '1px solid rgba(239, 68, 68, 0.3)',
                          color: '#f87171',
                          fontSize: '11px',
                          cursor: 'pointer',
                        }}
                        title="Excluir história"
                      >
                        <Trash2 size={13} />
                      </button>
                    </div>
                  </div>
                ))}
              </div>
            </div>

            {/* Formulário: Adicionar Nova História Pré-Construída */}
            <div
              style={{
                marginTop: '10px',
                padding: '16px',
                borderRadius: '12px',
                backgroundColor: 'rgba(234, 179, 8, 0.04)',
                border: '1px solid rgba(234, 179, 8, 0.25)',
                display: 'flex',
                flexDirection: 'column',
                gap: '12px',
              }}
            >
              <div style={{ display: 'flex', alignItems: 'center', gap: '8px' }}>
                <Plus size={16} color="#facc15" />
                <strong style={{ fontSize: '13px', color: '#facc15', textTransform: 'uppercase', letterSpacing: '0.06em' }}>
                  Adicionar Nova História Pronta ao Catálogo
                </strong>
              </div>

              <div style={{ display: 'grid', gridTemplateColumns: 'repeat(auto-fit, minmax(280px, 1fr))', gap: '10px' }}>
                <div>
                  <label className="rpg-label" style={{ fontSize: '11px', marginBottom: '4px' }}>Título da História *</label>
                  <input
                    type="text"
                    className="rpg-input-field"
                    placeholder="Ex: O Culto do Verme das Sombras"
                    value={newStoryTitle}
                    onChange={(e) => setNewStoryTitle(e.target.value)}
                    data-testid="input-new-story-title"
                  />
                </div>
                <div>
                  <label className="rpg-label" style={{ fontSize: '11px', marginBottom: '4px' }}>Estilo Literário</label>
                  <input
                    type="text"
                    className="rpg-input-field"
                    placeholder="Ex: Fantasia Sombria, Horror Cósmico"
                    value={newStoryStyle}
                    onChange={(e) => setNewStoryStyle(e.target.value)}
                    data-testid="input-new-story-style"
                  />
                </div>
              </div>

              <div>
                <label className="rpg-label" style={{ fontSize: '11px', marginBottom: '4px' }}>
                  Premissa Central (Conflito, cenário e requisitos de roleplay) *
                </label>
                <textarea
                  className="rpg-input-field"
                  rows={3}
                  placeholder="Ex: Os aventureiros precisam resgatar o paladino Cole das garras de um culto profano nas profundezas das catacumbas..."
                  value={newStoryPremise}
                  onChange={(e) => setNewStoryPremise(e.target.value)}
                  data-testid="input-new-story-premise"
                />
              </div>

              <div style={{ display: 'flex', justifyContent: 'space-between', alignItems: 'center', flexWrap: 'wrap', gap: '12px' }}>
                <div style={{ display: 'flex', alignItems: 'center', gap: '10px' }}>
                  <label className="rpg-label" style={{ fontSize: '11px', margin: 0 }}>Foco D20 (1 Roleplay - 20 Tático):</label>
                  <input
                    type="number"
                    min={1}
                    max={20}
                    value={newStorySlider}
                    onChange={(e) => setNewStorySlider(Math.min(20, Math.max(1, parseInt(e.target.value) || 10)))}
                    style={{
                      width: '60px',
                      padding: '4px 8px',
                      backgroundColor: '#000',
                      border: '1px solid rgba(255,255,255,0.2)',
                      borderRadius: '6px',
                      color: '#facc15',
                      fontWeight: 800,
                      textAlign: 'center',
                    }}
                  />
                </div>

                <button
                  type="button"
                  onClick={handleSaveNewStory}
                  disabled={isSavingStory || !newStoryTitle.trim() || !newStoryPremise.trim()}
                  className="rpg-cta-gold"
                  data-testid="save-new-story-btn"
                  style={{ width: 'auto', padding: '10px 22px', fontSize: '12px' }}
                >
                  <Plus size={14} />
                  {isSavingStory ? 'Salvando História...' : 'Salvar no Catálogo'}
                </button>
              </div>
            </div>
          </div>
        </div>
      )}
    </div>
  );
};
