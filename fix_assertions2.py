file_path = "/home/claus/src/rpg/src/rules/spells/srd_level3.rs"

with open(file_path, "r") as f:
    content = f.read()

effect_tests = [
    "test_feign_death_spell",
    "test_fly_spell",
    "test_gaseous_form_spell",
    "test_meld_into_stone_spell",
    "test_nondetection_spell",
    "test_protection_from_energy_spell",
    "test_slow_spell",
    "test_tongues_spell",
    "test_water_breathing_spell",
    "test_water_walk_spell"
]

compound_tests = [
    "test_haste_spell"
]

import re

for t in effect_tests:
    start_idx = content.find(f"fn {t}()")
    if start_idx != -1:
        end_idx = content.find("}", start_idx)
        sub_str = content[start_idx:end_idx]
        new_sub_str = re.sub(r"CastOutcome::UtilityApplied \{\s*\.\.\s*\}", "CastOutcome::EffectApplied", sub_str)
        content = content[:start_idx] + new_sub_str + content[end_idx:]

for t in compound_tests:
    start_idx = content.find(f"fn {t}()")
    if start_idx != -1:
        end_idx = content.find("}", start_idx)
        sub_str = content[start_idx:end_idx]
        new_sub_str = re.sub(r"CastOutcome::UtilityApplied \{\s*\.\.\s*\}", "CastOutcome::CompoundOutcome(..)", sub_str)
        content = content[:start_idx] + new_sub_str + content[end_idx:]

with open(file_path, "w") as f:
    f.write(content)
