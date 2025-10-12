import json
import requests

TARGET_FILE = "data/cards.json"
URL = "https://data.scryfall.io/oracle-cards/oracle-cards-20250914090316.json"

def to_snake_case(name):
    result = "".join([c.lower() if c.isalnum() else "_" for c in name])
    while "__" in result:
        result = result.replace("__", "_")
    return result

print(f"Fetching {URL}")
cards = requests.get(URL).json()

print(f"Writing to \"{TARGET_FILE}\"")
with open(TARGET_FILE, "w") as output:
    json.dump(cards, output, indent=2)
