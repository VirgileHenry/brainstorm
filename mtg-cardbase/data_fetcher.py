import json
import requests

TARGET_FILE = "data/cards.json"
URL = "https://api.scryfall.com/bulk-data/oracle-cards"

print(f"Fetching {URL}")
oracle_cards = requests.get(URL).json()
bulk_url = oracle_cards["download_uri"]

print(f"Fetching {bulk_url}")
cards = requests.get(bulk_url).json()

print(f"Writing to \"{TARGET_FILE}\"")
with open(TARGET_FILE, "w", encoding="utf-8") as output:
    json_str = json.dumps(cards, indent=2, ensure_ascii=False)
    output.write(json_str)
