file_path = "/home/claus/src/rpg/src/rules/spells/srd_level3.rs"

with open(file_path, "r") as f:
    content = f.read()

part1 = content.split("    // 11. Dispel Magic")[0] + "}\n\n"

part2 = content.split("#[cfg(test)]\nmod tests {\n    use super::*;")[1]
part2 = part2.split("    #[test]\n    fn test_dispel_magic_spell()")[0]

new_content = part1 + "#[cfg(test)]\nmod tests {\n    use super::*;\n" + part2 + "}\n"

with open(file_path, "w") as f:
    f.write(new_content)
print("File restored")
