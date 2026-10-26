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
  onGrantExtraAction?: () => void;
  onHealSecondWind?: (amount: number) => void;
  onConsumeAction?: (costType: 'action' | 'bonus_action' | 'reaction' | 'free', label: string) => void;
  onUpdatePreparedSpells?: (spells: string[]) => void;
  onRestoreSpellSlots?: (levelsToRecover: number) => void;
  onNotify?: (message: string, type?: 'info' | 'success' | 'warning') => void;
}

export const ClassHUDDispatcher: FC<ClassHUDDispatcherProps> = ({
  character,
  onGrantExtraAction,
  onHealSecondWind,
  onConsumeAction,
  onUpdatePreparedSpells,
  onRestoreSpellSlots,
  onNotify,
}) => {
  switch (character.className) {
    // 1. Marciais & Especialistas
    case 'Barbarian':
      return (
        <BarbarianHUD
          character={character}
          onConsumeAction={onConsumeAction}
          onNotify={onNotify}
        />
      );
    case 'Fighter':
      return (
        <FighterHUD
          character={character}
          onGrantExtraAction={onGrantExtraAction}
          onHealSecondWind={onHealSecondWind}
          onConsumeAction={onConsumeAction}
          onNotify={onNotify}
        />
      );
    case 'Rogue':
      return (
        <RogueHUD
          character={character}
          onConsumeAction={onConsumeAction}
          onNotify={onNotify}
        />
      );
    case 'Monk':
      return (
        <MonkHUD
          character={character}
          onConsumeAction={onConsumeAction}
          onNotify={onNotify}
        />
      );

    // 2. Conjuradores Divinos & Naturais
    case 'Cleric':
      return (
        <ClericHUD
          character={character}
          onHeal={onHealSecondWind}
          onConsumeAction={onConsumeAction}
          onNotify={onNotify}
        />
      );
    case 'Druid':
      return (
        <DruidHUD
          character={character}
          onConsumeAction={onConsumeAction}
          onNotify={onNotify}
        />
      );
    case 'Paladin':
      return (
        <PaladinHUD
          character={character}
          onHealLayOnHands={onHealSecondWind}
          onConsumeAction={onConsumeAction}
          onNotify={onNotify}
        />
      );
    case 'Ranger':
      return (
        <RangerHUD
          character={character}
          onConsumeAction={onConsumeAction}
          onNotify={onNotify}
        />
      );

    // 3. Conjuradores Arcanos, Pacto & Artísticos
    case 'Wizard':
      return (
        <WizardHUD
          character={character}
          onUpdatePreparedSpells={onUpdatePreparedSpells}
          onRestoreSpellSlots={onRestoreSpellSlots}
          onNotify={onNotify}
        />
      );
    case 'Sorcerer':
      return (
        <SorcererHUD
          character={character}
          onRestoreSpellSlots={onRestoreSpellSlots}
          onConsumeAction={onConsumeAction}
          onNotify={onNotify}
        />
      );
    case 'Warlock':
      return (
        <WarlockHUD
          character={character}
          onHeal={onHealSecondWind}
          onConsumeAction={onConsumeAction}
          onNotify={onNotify}
        />
      );
    case 'Bard':
      return (
        <BardHUD
          character={character}
          onHeal={onHealSecondWind}
          onConsumeAction={onConsumeAction}
          onNotify={onNotify}
        />
      );

    default:
      return null;
  }
};
