#!/usr/bin/env python3
"""
Generate multilingual book entries for Faith CLI.
Uses English names as fallback for languages without data.
"""

import json
import re
from pathlib import Path

# Load existing books.rs to extract current data
books_rs_path = Path(__file__).parent.parent / "src" / "books.rs"
content = books_rs_path.read_text()

# Load multilingual names (if available)
try:
    with open(Path(__file__).parent / "book_names.json") as f:
        multilang_data = {book["canonical_id"]: book for book in json.load(f)}
except:
    multilang_data = {}

# Extract book entries using regex
pattern = r'BookEntry\s*\{\s*canonical_id:\s*"([A-Z0-9]+)",\s*helloao_id:\s*"([A-Z0-9]+)",\s*order:\s*(\d+),\s*name_en:\s*"([^"]+)",\s*name_pt:\s*"([^"]+)",\s*aliases_en:\s*&\[([^\]]*)\],\s*aliases_pt:\s*&\[([^\]]*)\],\s*chapters:\s*(\d+),\s*testament:\s*"(OT|NT)"\s*\}'

matches = re.finditer(pattern, content, re.MULTILINE | re.DOTALL)

def format_aliases(aliases_str):
    """Convert alias string to list."""
    if not aliases_str.strip():
        return []
    return [a.strip().strip('"') for a in aliases_str.split(',')]

def format_rust_aliases(aliases_list):
    """Format list as Rust array."""
    if not aliases_list:
        return "&[]"
    formatted = ', '.join(f'"{a}"' for a in aliases_list)
    return f"&[{formatted}]"

print('#[rustfmt::skip]')
print('const BOOKS: &[BookEntry] = &[')

for match in matches:
    canonical_id, helloao_id, order, name_en, name_pt, aliases_en_str, aliases_pt_str, chapters, testament = match.groups()

    aliases_en = format_aliases(aliases_en_str)
    aliases_pt = format_aliases(aliases_pt_str)

    # Get multilingual data or use fallback
    ml_data = multilang_data.get(canonical_id, {})
    name_es = ml_data.get("name_es", name_en)  # Fallback to English
    name_fr = ml_data.get("name_fr", name_en)
    name_de = ml_data.get("name_de", name_en)
    name_grc = ml_data.get("name_grc", name_en)
    name_heb = ml_data.get("name_heb", name_en)

    aliases_es = format_rust_aliases(ml_data.get("aliases_es", []))
    aliases_fr = format_rust_aliases(ml_data.get("aliases_fr", []))
    aliases_de = format_rust_aliases(ml_data.get("aliases_de", []))
    aliases_grc = format_rust_aliases(ml_data.get("aliases_grc", []))
    aliases_heb = format_rust_aliases(ml_data.get("aliases_heb", []))

    # Format the entry
    print(f'    BookEntry {{')
    print(f'        canonical_id: "{canonical_id}", helloao_id: "{helloao_id}", order: {order:>2},')
    print(f'        name_en: "{name_en}",')
    print(f'        name_pt: "{name_pt}",')
    print(f'        name_es: "{name_es}",')
    print(f'        name_fr: "{name_fr}",')
    print(f'        name_de: "{name_de}",')
    print(f'        name_grc: "{name_grc}",')
    print(f'        name_heb: "{name_heb}",')
    print(f'        aliases_en: {format_rust_aliases(aliases_en)},')
    print(f'        aliases_pt: {format_rust_aliases(aliases_pt)},')
    print(f'        aliases_es: {aliases_es},')
    print(f'        aliases_fr: {aliases_fr},')
    print(f'        aliases_de: {aliases_de},')
    print(f'        aliases_grc: {aliases_grc},')
    print(f'        aliases_heb: {aliases_heb},')
    print(f'        chapters: {chapters}, testament: "{testament}"')
    print(f'    }},')

print('];')
print()
print('// Note: Some book names use English fallback for languages without data yet.')
print('// TODO: Populate full multilingual names for all 66 books.')
