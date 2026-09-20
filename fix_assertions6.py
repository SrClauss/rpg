import sys

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

old_str = "            CastOutcome::UtilityApplied { .. }\n        ));"
new_str_eff = "            CastOutcome::EffectApplied\n        ));"
new_str_cmp = "            CastOutcome::CompoundOutcome(..)\n        ));"

for t in effect_tests:
    start = content.find(f"fn {t}()")
    end = content.find("}", start)
    if start != -1 and end != -1:
        sub = content[start:end]
        sub = sub.replace(old_str, new_str_eff)
        content = content[:start] + sub + content[end:]

for t in compound_tests:
    start = content.find(f"fn {t}()")
    end = content.find("}", start)
    if start != -1 and end != -1:
        sub = content[start:end]
        sub = sub.replace(old_str, new_str_cmp)
        content = content[:start] + sub + content[end:]

with open(file_path, "w") as f:
    f.write(content)
