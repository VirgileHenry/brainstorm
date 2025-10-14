import json
import requests

TARGET_FILE = "data/cards.json"
URL = "https://data.scryfall.io/oracle-cards/oracle-cards-20250914090316.json"

print(f"Fetching {URL}")
cards = requests.get(URL).json()

print("Converting everything to lowercase...")
json_str = json.dumps(cards, indent=2, ensure_ascii=False)
json_str_lower = json_str.lower()

print(f"Writing to \"{TARGET_FILE}\"")
with open(TARGET_FILE, "w", encoding="utf-8") as output:
    output.write(json_str_lower)
