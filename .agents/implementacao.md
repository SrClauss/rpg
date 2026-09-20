# Plano de Implementação - Sistema de Monstros & Criaturas (SRD 5.1)

Este documento estabelece o plano detalhado de implementação e a checklist completa de mecanização para todos os monstros, criaturas e NPCs do **SRD 5.1 (Páginas 254 a 359)** na RPG Engine em Rust.

---

## 🏗️ 1. Infraestrutura Base & Suporte a Monstros

- [x] **Mecânicas Globais de Monstros**
  - [x] Estrutura `MonsterStatBlock` / `Monster` & Enums `CreatureType` e `CreatureSize`
  - [x] Mapeamento de Dados de Vida por Tamanho (`Tiny d4` até `Gargantuan d20`)
  - [x] Sistema de Resistências, Imunidades e Vulnerabilidades a Danos e Condições
  - [x] Sistema de Sentidos Especiais (`Blindsight`, `Darkvision`, `Tremorsense`, `Truesight`)
  - [x] Suporte a Conjuração Inata (`Innate Spellcasting`) e Psionismo (`Psionics`)
  - [x] Mecânica de Recarga em Combate (`Recharge X-Y` via rola d6 no início do turno)
  - [x] Suporte a Ações Lendárias (`Legendary Actions`), Ações de Guarida (`Lair Actions`) e Efeitos Regionais

---

## 👹 2. Checklist por Categoria de Criaturas

### 👁️ Aberrações (Aberrations)
- [x] Aboleth
- [x] Cloaker
- [x] Gibbering Mouther
- [x] Otyugh
- [x] Roper
- [x] Will-o'-Wisp

### 🐻 Bestas & Animais (Beasts)
- [x] Ape
- [x] Baboon
- [x] Badger
- [x] Black Bear
- [x] Boar
- [x] Brown Bear
- [x] Constrictor Snake
- [x] Crocodile
- [x] Dire Wolf
- [x] Draft Horse
- [x] Eagle
- [x] Elephant
- [x] Giant Ape
- [x] Giant Badger
- [x] Giant Bat
- [x] Giant Boar
- [x] Giant Centipede
- [x] Giant Constrictor Snake
- [x] Giant Crab
- [x] Giant Crocodile
- [x] Giant Eagle
- [x] Giant Elk
- [x] Giant Fire Beetle
- [x] Giant Frog
- [x] Giant Goat
- [x] Giant Hyena
- [x] Giant Lizard
- [x] Giant Octopus
- [x] Giant Owl
- [x] Giant Poisonous Snake
- [x] Giant Rat
- [x] Giant Raven
- [x] Giant Scorpion
- [x] Giant Shark
- [x] Giant Spider
- [x] Giant Toad
- [x] Giant Vulture
- [x] Giant Wasp
- [x] Giant Weasel
- [x] Giant Wolf Spider
- [x] Goat
- [x] Hawk
- [x] Hunter Shark
- [x] Hyena
- [x] Jackal
- [x] Killer Whale
- [x] Lion
- [x] Lizard
- [x] Mule
- [x] Octopus
- [x] Owl
- [x] Panther
- [x] Phase Spider
- [x] Plesiosaurus
- [x] Poisonous Snake
- [x] Pony
- [x] Reef Shark
- [x] Rhinoceros
- [x] Riding Horse
- [x] Saber-Toothed Tiger
- [x] Scorpion
- [x] Sea Horse
- [x] Spider
- [x] Tiger
- [x] Tyrannosaurus Rex
- [x] Vulture
- [x] Warhorse
- [x] Weasel
- [x] Wolf

### 😇 Celestiais (Celestials)
- [x] Deva
- [x] Planetar
- [x] Solar
- [x] Couatl
- [x] Pegasus

### 🗿 Constructos (Constructs)
- [x] Animated Armor
- [x] Flying Sword
- [x] Rug of Smothering
- [x] Clay Golem
- [x] Flesh Golem
- [x] Iron Golem
- [x] Stone Golem
- [x] Homunculus
- [x] Shield Guardian

### 🐉 Dragões (Dragons)
- [x] Black Dragon (Wyrmling, Young, Adult, Ancient)
- [x] Blue Dragon (Wyrmling, Young, Adult, Ancient)
- [x] Brass Dragon (Wyrmling, Young, Adult, Ancient)
- [x] Bronze Dragon (Wyrmling, Young, Adult, Ancient)
- [x] Copper Dragon (Wyrmling, Young, Adult, Ancient)
- [x] Gold Dragon (Wyrmling, Young, Adult, Ancient)
- [x] Green Dragon (Wyrmling, Young, Adult, Ancient)
- [x] Red Dragon (Wyrmling, Young, Adult, Ancient)
- [x] Silver Dragon (Wyrmling, Young, Adult, Ancient)
- [x] White Dragon (Wyrmling, Young, Adult, Ancient)
- [x] Dragon Turtle
- [x] Pseudodragon
- [x] Wyvern

### 🔥 Elementais (Elementals)
- [x] Air Elemental
- [x] Earth Elemental
- [x] Fire Elemental
- [x] Water Elemental
- [x] Azer
- [x] Djinni
- [x] Efreeti
- [x] Gargoyle
- [x] Invisible Stalker
- [x] Magmin
- [x] Mephits (Dust, Ice, Magma, Steam)
- [x] Salamander
- [x] Xorn

### 🧚 Fadas (Fey)
- [x] Dryad
- [x] Green Hag
- [x] Pixie
- [x] Satyr

### 👿 Corruptores (Fiends - Demônios, Diabos, Yugoloths)
- [x] Balor
- [x] Dretch
- [x] Glabrezu
- [x] Hezrou
- [x] Marilith
- [x] Nalfeshnee
- [x] Quasit
- [x] Shadow Demon
- [x] Vrock
- [x] Barbed Devil
- [x] Bearded Devil
- [x] Bone Devil
- [x] Chain Devil
- [x] Erinyes
- [x] Horned Devil
- [x] Ice Devil
- [x] Imp
- [x] Lemure
- [x] Pit Fiend
- [x] Hell Hound
- [x] Nightmare
- [x] Rakshasa
- [x] Succubus/Incubus

### 🏔️ Gigantes (Giants)
- [x] Cloud Giant
- [x] Fire Giant
- [x] Frost Giant
- [x] Hill Giant
- [x] Stone Giant
- [x] Storm Giant
- [x] Ettin
- [x] Ogre
- [x] Oni
- [x] Troll

### ⚔️ Humanoides & NPCs (Humanoids & NPC Statblocks)
- [x] Acolyte
- [x] Archmage
- [x] Assassin
- [x] Bandit
- [x] Bandit Captain
- [x] Berserker
- [x] Commoner
- [x] Cultist
- [x] Cult Fanatic
- [x] Druid
- [x] Gladiator
- [x] Guard
- [x] Knight
- [x] Mage
- [x] Noble
- [x] Priest
- [x] Scout
- [x] Spy
- [x] Thug
- [x] Tribal Warrior
- [x] Veteran
- [x] Bugbear
- [x] Duergar
- [x] Elf (Drow)
- [x] Gnoll
- [x] Goblin
- [x] Grimlock
- [x] Hobgoblin
- [x] Kobold
- [x] Lizardfolk
- [x] Merfolk
- [x] Orc
- [x] Sahuagin
- [x] Werebear
- [x] Wereboar
- [x] Wererat
- [x] Weretiger
- [x] Werewolf

### 🦁 Monstruosidades (Monstrosities)
- [x] Ankheg
- [x] Basilisk
- [x] Behir
- [x] Bulette
- [x] Centaur
- [x] Chimera
- [x] Cockatrice
- [x] Darkmantle
- [x] Displacer Beast
- [x] Doppelganger
- [x] Drider
- [x] Gorgon
- [x] Grick
- [x] Griffon
- [x] Harpy
- [x] Hippogriff
- [x] Hydra
- [x] Kraken
- [x] Lamia
- [x] Manticore
- [x] Medusa
- [x] Minotaur
- [x] Naga (Guardian, Spirit)
- [x] Owlbear
- [x] Purple Worm
- [x] Remorhaz
- [x] Roc
- [x] Rust Monster
- [x] Sphinx (Androsphinx, Gynosphinx)
- [x] Tarrasque
- [x] Unicorn
- [x] Yuan-ti Pureblood

### 🧪 Gosmas (Oozes)
- [x] Black Pudding
- [x] Gelatinous Cube
- [x] Gray Ooze
- [x] Ochre Jelly

### 🌿 Plantas Mágicas (Plants)
- [x] Awakened Shrub
- [x] Awakened Tree
- [x] Shambling Mound
- [x] Treant

### 💀 Mortos-Vivos (Undead)
- [x] Banshee
- [x] Ghoul
- [x] Ghast
- [x] Ghost
- [x] Lich
- [x] Minotaur Skeleton
- [x] Mummy
- [x] Mummy Lord
- [x] Ogre Zombie
- [x] Shadow
- [x] Skeleton
- [x] Specter
- [x] Vampire
- [x] Vampire Spawn
- [x] Wight
- [x] Wraith
- [x] Zombie
