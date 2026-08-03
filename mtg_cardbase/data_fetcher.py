#!/usr/bin/env python3

import json
import requests

HEADERS = { 'User-Agent': 'Hexxed Dev Team' }

CARDS_TARGET_FILE = "data/cards.json"
CARDS_URL = "https://api.scryfall.com/bulk-data/oracle-cards"

print(f"Fetching {CARDS_URL}")
oracle_cards = requests.get(CARDS_URL, headers=HEADERS).json()
bulk_url = oracle_cards["download_uri"]

print(f"Fetching {bulk_url}")
cards = requests.get(bulk_url, headers=HEADERS).json()

print(f"Writing to \"{CARDS_TARGET_FILE}\"")
with open(CARDS_TARGET_FILE, "w", encoding="utf-8") as output:
    json_str = json.dumps(cards, indent=2, ensure_ascii=False)
    output.write(json_str)

TAGS_TARGET_FILE = "data/tags.json"
TAGS_URL = "https://api.scryfall.com/bulk-data/oracle-tags"

print(f"Fetching {TAGS_URL}")
oracle_tags = requests.get(TAGS_URL, headers=HEADERS).json()
bulk_url = oracle_tags["download_uri"]

print(f"Fetching {bulk_url}")
tags = requests.get(bulk_url, headers=HEADERS).json()

print(f"Writing to \"{TAGS_TARGET_FILE}\"")
with open(TAGS_TARGET_FILE, "w", encoding="utf-8") as output:
    json_str = json.dumps(tags, indent=2, ensure_ascii=False)
    output.write(json_str)
