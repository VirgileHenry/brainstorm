import json
import requests

TARGET_DIR = "data"
URL = "https://data.scryfall.io/oracle-cards/oracle-cards-20250914090316.json"

def to_snake_case(name):
    result = "".join([c.lower() if c.isalnum() else "_" for c in name])
    while "__" in result:
        result = result.replace("__", "_")
    return result

print(f"Fetching {URL}")
cards = requests.get(URL).json()

print(f"Writing to \"{TARGET_DIR}\"")
for card in cards:
    card_name = card['name']
    filename = to_snake_case(card_name)
    with open(f"{TARGET_DIR}/{filename}.json", "w") as output:
        json.dump(card, output)
