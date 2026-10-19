import type { FC } from 'react';
import type { CharacterSheet } from '../../../types/character';
import { BarbarianHUD } from './martial/BarbarianHUD';
import { FighterHUD } from './martial/FighterHUD';
import { RogueHUD } from './martial/RogueHUD';
import { MonkHUD } from './martial/MonkHUD';
import { ClericHUD } from './divine/ClericHUD';
import { DruidHUD } from './divine/DruidHUD';
import { PaladinHUD } from './divine/PaladinHUD';
import { RangerHUD } from './divine/RangerHUD';
import { WizardHUD } from './arcane/WizardHUD';
import { SorcererHUD } from './arcane/SorcererHUD';
import { WarlockHUD } from './arcane/WarlockHUD';
import { BardHUD } from './arcane/BardHUD';

interface ClassHUDDispatcherProps {
  character: CharacterSheet;
}

export const ClassHUDDispatcher: FC<ClassHUDDispatcherProps> = ({ character }) => {
  switch (character.className) {
    // 1. Marciais & Especialistas
    case 'Barbarian':
      return <BarbarianHUD character={character} />;
    case 'Fighter':
      return <FighterHUD character={character} />;
    case 'Rogue':
      return <RogueHUD character={character} />;
    case 'Monk':
      return <MonkHUD character={character} />;

    // 2. Conjuradores Divinos & Naturais
    case 'Cleric':
      return <ClericHUD character={character} />;
    case 'Druid':
      return <DruidHUD character={character} />;
    case 'Paladin':
      return <PaladinHUD character={character} />;
    case 'Ranger':
      return <RangerHUD character={character} />;

    // 3. Conjuradores Arcanos, Pacto & Artísticos
    case 'Wizard':
      return <WizardHUD character={character} />;
    case 'Sorcerer':
      return <SorcererHUD character={character} />;
    case 'Warlock':
      return <WarlockHUD character={character} />;
    case 'Bard':
      return <BardHUD character={character} />;

    default:
      return null;
  }
};
