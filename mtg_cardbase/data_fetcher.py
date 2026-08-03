#!/usr/bin/env python3

import gzip
import requests

HEADERS = { 'User-Agent': 'Hexxed Dev Team' }

CARDS_TARGET_FILE = "data/cards.jsonl"
CARDS_URL = "https://api.scryfall.com/bulk-data/oracle-cards"

print(f"Fetching {CARDS_URL}")
oracle_cards = requests.get(CARDS_URL, headers=HEADERS).json()
bulk_url = oracle_cards["jsonl_download_uri"]

print(f"Fetching {bulk_url}")
raw_content = requests.get(bulk_url, headers=HEADERS).content
card_bytes = gzip.decompress(raw_content)
cards = card_bytes.decode(encoding="utf-8")

print(f"Writing to \"{CARDS_TARGET_FILE}\"")
with open(CARDS_TARGET_FILE, "w", encoding="utf-8") as output:
    for card in cards.splitlines():
        output.write(card)
        output.write("\n")

TAGS_TARGET_FILE = "data/tags.jsonl"
TAGS_URL = "https://api.scryfall.com/bulk-data/oracle-tags"

print(f"Fetching {TAGS_URL}")
oracle_tags = requests.get(TAGS_URL, headers=HEADERS).json()
bulk_url = oracle_tags["jsonl_download_uri"]

print(f"Fetching {bulk_url}")
raw_content = requests.get(bulk_url, headers=HEADERS).content
tag_bytes = gzip.decompress(raw_content)
tags = tag_bytes.decode(encoding="utf-8")

print(f"Writing to \"{TAGS_TARGET_FILE}\"")
with open(TAGS_TARGET_FILE, "w", encoding="utf-8") as output:
    for tag in tags.splitlines():
        output.write(tag)
        output.write("\n")
