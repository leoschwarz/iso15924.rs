#!/usr/bin/python3
#
# ISC License (ISC)
#
# Copyright (c) 2016, Austin Hellyer <hello@austinhellyer.me>
#
# Permission to use, copy, modify, and/or distribute this software for any
# purpose with or without fee is hereby granted, provided that the above
# copyright notice and this permission notice appear in all copies.
#
# THE SOFTWARE IS PROVIDED "AS IS" AND THE AUTHOR DISCLAIMS ALL WARRANTIES
# WITH REGARD TO THIS SOFTWARE INCLUDING ALL IMPLIED WARRANTIES OF
# MERCHANTABILITY AND FITNESS. IN NO EVENT SHALL THE AUTHOR BE LIABLE FOR ANY
# SPECIAL, DIRECT, INDIRECT, OR CONSEQUENTIAL DAMAGES OR ANY DAMAGES WHATSOEVER
# RESULTING FROM LOSS OF USE, DATA OR PROFITS, WHETHER IN AN ACTION OF
# CONTRACT, NEGLIGENCE OR OTHER TORTIOUS ACTION, ARISING OUT OF OR IN
# CONNECTION WITH THE USE OR PERFORMANCE OF THIS SOFTWARE.
#
# What is ISO 15924?
# | ISO 15924, Codes for the representation of names of scripts, defines two
# | sets of codes for a number of writing systems (scripts). Each script is
# | given both a four-letter code and a numeric one.[1] Script is defined as
# | "set of graphic characters used for the written form of one or more
# | languages".
# |
# | - [wikipedia](https://en.wikipedia.org/wiki/ISO_15924)
#
# Originally by zeyla on GitHub.

from bs4 import BeautifulSoup
import os
import re
import subprocess
import sys
import urllib.request

url = 'http://unicode.org/iso15924/iso15924-codes.html'
html = urllib.request.urlopen(url).read()

soup = BeautifulSoup(html, 'html.parser')

rows = soup.find_all('table')[3] \
    .find_all('tr')[1:]

text = ""

for row in rows:
    # 0: code
    # 1: num
    # 2: name
    # 3: french name
    # 4: alias [Property Value Alias]
    # 5: date
    cells = row.find_all('td')

    if len(cells) != 6:
        continue

    code = cells[0].get_text()
    num = cells[1].get_text()
    name = cells[2].get_text()
    name_french = cells[3].get_text()

    # If the alias cell contains anything more than a single space, then it has
    # a value (Rust's `Some`), otherwise it is Rust's `None`.
    if len(cells[4].get_text()) < 2:
        alias = 'None'
    else:
        alias = 'Some("{}")'.format(cells[4].get_text())

    date_data = cells[5].get_text().split("-")

    # If the date doesn't exist or incorrectly parses into other than 3 items in
    # a list, then skip this row. The date needs to parse into 3 items in the
    # list.
    if len(date_data) != 3:
        continue

    # year, month, day
    y = date_data[0]
    m = date_data[1]
    d = date_data[2]

    # Push formatted text for a `ScriptCode`.
    text += '    list.push(ScriptCode {\n'
    text += '        alias: {},\n'.format(alias)
    text += '        code: "{}",\n'.format(code)
    text += '        date: ScriptDate::new({}, {}, {}).unwrap(),\n'.format(y, m, d)
    text += '        name: "{}",\n'.format(name)
    text += '        name_french: "{}",\n'.format(name_french)
    text += '        num: "{}",\n'.format(num)
    text += '    });\n'

# Open the file with the Vec of codes.
all_path = os.path.join(os.path.dirname(__file__), '../src/codes.rs')

with open(all_path, 'r') as f:
    all_file = f.read()

# Concat in the `text` between the beginning and end of the codes file contents.
current = all_file.rsplit('// Begin.', 1)
current_e = all_file.rsplit('// End.\n', 1)

with open(all_path, 'w') as f:
    f.write(current[0] + '// Begin.\n' + text + '    // End.\n' + current_e[1])

print('Updated.')
