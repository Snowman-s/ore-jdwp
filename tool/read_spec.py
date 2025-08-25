from ctypes import Array
import bs4
from bs4 import BeautifulSoup, NavigableString
import re

def read_spec_file(path):
  with open(path, "r", encoding="utf-8") as f:
    return f.read()

# [
#   {set: int, cmd: int, name: str}
# ]
def extract_all_command(soup: bs4.BeautifulSoup):
  result = []
  for li in soup.find_all("li"):
    # li.contents は直下の子要素のリスト
    for child in li.contents:
      if isinstance(child, NavigableString) and "Command Set" in child:
        result.append(li)
        break

  commands = []

  for li in result:
    # コマンドセット番号を取得
    text = "".join(
      t for t in li.contents
      if isinstance(t, NavigableString) and "Command Set" in t
    )
    m = re.search(r"Command Set\s*\((\d+)\)", text)
    set_num = int(m.group(1)) if m else None
    command_set_name = li.find("a").text.strip() if li.find("a") else None

    # コマンドを抽出
    for cmd_li in li.find_all("ul"):
      for cli in cmd_li.find_all("li", recursive=False):
        cmd_name = cli.find("a").text.strip()
        m = re.search(r"\((\d+)\)", cli.text)
        cmd_num = int(m.group(1)) if m else None
        commands.append({
          "command_set_name": command_set_name,
          "set_num": set_num,
          "command_name": cmd_name,
          "command_num": cmd_num
        })

  return commands

def parse_command_table(soup, command_set_name: str, command_name: str, send_from_debugger: bool = True):  
  # まず、コマンドセットに等しい h4 を探す
  h4 = None
  for h4_tag in soup.find_all("h4"):
    a_tag = h4_tag.find("a")
    if a_tag and f"{command_set_name} Command Set" in a_tag.text:
      h4 = h4_tag
      break

  if not h4:
    raise ValueError(f"Command set not found: {command_set_name}")

  # 次の部分から、コマンド名に等しい h5 を探す
  h5 = None
  # h4の次からの範囲でh5を探す
  h4_next = h4.next_sibling
  while h4_next:
    if isinstance(h4_next, NavigableString):
      h4_next = h4_next.next_sibling
      continue
    if h4_next.name == "h5":
      a_tag = h4_next.find("a")
      if a_tag and command_name in a_tag.text:
        h5 = h4_next
        break
    if h4_next.name == "h4":
      break
    h4_next = h4_next.next_sibling

  if not h5:
    raise ValueError(f"Command not found: {command_name}")

  dl = h5.find_next_sibling("dl")

  # dl の真下にある、<dd> を探す。長さは3のはず。
  dd_tags = dl.find_all("dd", recursive=False)

  if send_from_debugger:
    if len(dd_tags) != 3:
      raise ValueError(f"Expected 3 <dd> tags, found {len(dd_tags)}")

    # それぞれの <dd> から情報を抽出
    return {
      "Send": parse_packet_table(command_set_name, dd_tags[0]),
      "Receive": parse_packet_table(command_set_name, dd_tags[1]),
      # Error だけパケットテーブルではない
      "Error": parse_error_info(dd_tags[2])
    }
  else:
    if len(dd_tags) != 1:
      raise ValueError(f"Expected 1 <dd> tags, found {len(dd_tags)}")

    # それぞれの <dd> から情報を抽出
    return {
      "Receive": parse_packet_table(command_set_name, dd_tags[0]),
    }


def parse_packet_table(cmd_set:str, soup: bs4.BeautifulSoup):
  table = soup.find("table")
  if not table:
    return None

  row_index = 0
  trs = table.find_all("tr")[1:]
  
  # 最初の tr は無視
  # 以降のtd は、 [型, 変数名, 説明]
  # ただし、[None, 型, ...] のような、内容のない td は無視する

  # ＊型がint の場合、次の行の内容が"Repeated" から始まる場合がある。
  # この場合、型を "repeated" とし、"repeat" という配列フィールドを追加する。
  # 以降の行はすべてrepeat フィールド に追加される

  # ＊型が byte の場合、次の行の内容が"Case ... is ?: " である場合がある。
  # この場合、型を "case" とし、"case" という辞書フィールドを追加する。
  # 以降の行はすべてcase フィールド に追加される
  # キーは ? の部分の値になる

  # 行から辞書を作成する処理を共通化
  def make_row(cmd_set:str, indent=0):
    nonlocal row_index, trs

    while True:
      if row_index >= len(trs):
        return None
      cells = trs[row_index].find_all("td")

      # indent >= 1 の場合、最初のセルは空白である
      # また、そのセルのcolspan は indent に等しい。
      # これにより、repeated block の判定が可能になる
      skip = 0 if indent == 0 else 1
      # colspan の値が違うなら、Noneを返す
      if indent > 0 and int(cells[0].get("colspan")) != indent:
        return None

      # セル数が足りない場合はスキップ
      if skip + 2 >= len(cells):
        row_index += 1
        continue

      type_text = cells[skip].text.strip()
      name_text = cells[skip + 1].text.strip()
      desc_text = cells[skip + 2].text.strip()

      if type_text == "int" and row_index + 1 < len(trs):
        next_tr = trs[row_index + 1]
        next_cells = next_tr.find_all("td")
        has_repeat_info = [cell.text.strip().startswith("Repeated") for cell in next_cells]
        repeat_info_index = next((i for i, v in enumerate(has_repeat_info) if v), None)
        if repeat_info_index is not None:
          row_index += 2
          child = []
          while row_index < len(trs):
            row = make_row(cmd_set,indent + 1)
            if not row:
              # この後に1足されるので、帳尻を合わせる
              row_index -= 1
              break
            child += row
            row_index += 1
          return [{
            "type": "int",
            "name": cap_first(name_text),
            "description": desc_text
          }, {
            "type": "repeated",
            "name": cap_first(name_text) + "Repeated",
            "by": name_text,
            "description": next_cells[repeat_info_index].text.strip(),
            "repeat": child
          }] 
      
      if type_text == "byte" and row_index + 1 < len(trs):
        case = {}

        first_case = True

        # Case でのループ
        while row_index < len(trs):
          next_tr = trs[row_index + 1 if first_case else row_index]
          next_cells = next_tr.find_all("td")
          # next_cells の中で、内容が case から始まるテキストを見つける
          case_str = None

          for cell in next_cells:
            if cell.text.strip().startswith("Case"):
              case_str = cell.text.strip()
              break
          if not case_str:
            break

          # ケース名を取得
          # is の後から : まで
          case_name = case_str.split("is")[1].split(":")[0].strip()
          if "." in case_name:
            case_name = case_name.split(".")[-1]
          # ケースの子のループ
          child = []
          row_index += 2 if first_case else 1
          first_case = False
          while row_index < len(trs):
            row = make_row(cmd_set, indent + 1)
            if not row:
              break
            child += row
            row_index += 1

          case |= {case_name: child}

        if case != {}:
          return [{
            "type": "case",
            "name": cap_first(name_text),
            "description": desc_text,
            "case": case
          }]

      # 行から辞書を作成
      return [{
        "type": type_text,
        "name": cap_first(name_text),
        "description": desc_text
      }]

  rows = []
  while True:
    row = make_row(cmd_set)
    if not row:
      break
    rows += row
    row_index += 1

  return rows

def parse_error_info(soup: bs4.BeautifulSoup):
  # Error もテーブルを持つ
  # ヘッダは無く、各行の構成は [名前, 説明]

  rows = []
  for tr in soup.find_all("tr"):
    cells = tr.find_all("td")
    if not cells:
      continue
    row = {
      "name": cells[0].text.strip(),
      "description": cells[1].text.strip()
    }
    rows.append(row)

  return rows

def create_rust_structs_from_cmd_data(cmd_name: str, table_data: Array[dict]) -> str:
  if table_data == None: return ""
  ret = ""

  # まず、table_data の各フィールドから、repeated や case を探す。
  # それらは先に struct として定義する必要がある。

  for field in table_data:
    if field["type"] == "repeated":
      ret += create_rust_structs_from_cmd_data(f"{cmd_name}{cap_first(field['name'])}", field["repeat"]) + "\n\n"
    elif field["type"] == "case":
      ret += create_case_enum(cmd_name, field)
      for key_of_case, body_of_case in field["case"].items():
        ret += create_rust_structs_from_cmd_data(f"{cmd_name}{cap_first(field['name'])}{cap_first(key_of_case)}", body_of_case) + "\n\n"

  ret += "#[derive(Debug, PartialEq, Clone)]\n"
  ret += f"pub struct {snake_to_camel(cmd_name)} {{\n"
  for field in table_data:
    ret += f"  /* {field['description']} */\n"
    ret += f"  pub {camel_to_snake(field['name'])}: {type_to_rust_str(cmd_name, field)},\n"
  ret += "}\n\n"

  ret += f"impl PacketData for {snake_to_camel(cmd_name)} {{\n"
  ret += "  fn write_to<W: std::io::Write>(&self, writer: &mut W) -> Result<(), std::io::Error> {\n"
  for field in table_data:
    if field["type"] == "repeated":
      ret += f"    for item in &self.{camel_to_snake(field['name'])} {{\n"
      ret += f"      item.write_to(writer)?;\n"
      ret += f"    }}\n"
    elif field["type"] == "case":
      ret += f"    match &self.{camel_to_snake(field['name'])} {{\n"
      for key in field["case"].keys():
        ret += f"      {snake_to_camel(cmd_name)}{snake_to_camel(field['name'])}::_{snake_to_camel(key)}(inner) => inner.write_to(writer)?,\n"
      ret += f"    }}\n"
    else:
      ret += f"    self.{camel_to_snake(field['name'])}.write_{'' if 'untagged' not in field['type'] else 'untagged_'}to(writer)?;\n"
  ret += "    Ok(())\n"
  ret += "  }\n\n"
  ret += "  fn read_from<R: std::io::Read>(reader: &mut R, c: &JDWPContext) -> Result<Self, std::io::Error> {\n"
  for field in table_data:
    if field["type"] == "repeated":
      ret += f"    let mut {camel_to_snake(field['name'])} = Vec::new();\n"
      ret += f"    for _ in 0..{camel_to_snake(field['by'])} {{ \n"
      ret += f"      {camel_to_snake(field['name'])}.push({snake_to_camel(cmd_name)}{snake_to_camel(field['name'])}::read_from(reader, c)?);\n"
      ret += f"    }}\n"
    elif field["type"] == "case":
      ret += f"     let {camel_to_snake(field['name'])} = match {'u8' if field['name'] != 'EventKind' else 'JDWPEventKindConstants'}::read_from(reader, c)? {{\n"
      for key in field["case"].keys():
        ret += f"      {matching_of_case(key, field)} => {snake_to_camel(cmd_name)}{snake_to_camel(field['name'])}::_{snake_to_camel(key)}({snake_to_camel(cmd_name)}{snake_to_camel(field['name'])}{snake_to_camel(key)}::read_from(reader, c)?),\n"
      ret += "    _ => panic!(),\n"
      ret += f"    }};\n"
    else:
      # read_untagged_from は array_region でしか発生しえない。
      # untagged-value が含まれる場合に read_from を呼び出すとエラーになるが、
      # いったん妥協
      ret += f"    let {camel_to_snake(field['name'])} = {type_to_rust_str(cmd_name, field)}::read_from(reader, c)?;\n"
  ret += f"    Ok({snake_to_camel(cmd_name)} {{\n"
  for field in table_data:
    ret += f"      {camel_to_snake(field['name'])},\n"
  ret += "    })\n"
  ret += "  }\n"
  ret += "}\n"

  return ret

def create_case_enum(cmd_name: str, field: dict) -> str:
  if field["type"] != "case":
    return ""

  ret = ""
  ret += f"#[derive(Debug, PartialEq, Clone)]\n"
  ret += f"pub enum {snake_to_camel(cmd_name)}{snake_to_camel(field['name'])} {{\n"
  for key in field["case"].keys():
    ret += f"  _{snake_to_camel(key)}({cmd_name}{snake_to_camel(field['name'])}{snake_to_camel(key)}),\n"
  ret += "}\n\n"

  return ret

# field_type が case であるもの を受け取り、その中身に沿ったrust の match 式で比較対象になる式を生成する
def matching_of_case(case_key: str, case_field: dict) -> str:
  # フィールド名が EventKind の場合
  if case_field['name'] == 'EventKind':
    return f"JDWPEventKindConstants::{snake_to_camel(case_key)}"

  return snake_to_camel(case_key)

def type_to_rust_str(cmd_name: str, field: dict) -> str:
  type_name = field["type"]

  type_mapping = {
    "byte": "i8",
    "boolean": "bool",
    "int": "i32",
    "long": "i64",
    "objectID": "JDWPIDLengthEqObject",
    "tagged-objectID": "JDWPTaggedObjectID",
    "threadID": "JDWPIDLengthEqObject",
    "threadGroupID": "JDWPIDLengthEqObject",
    "stringID": "JDWPIDLengthEqObject",
    "classLoaderID": "JDWPIDLengthEqObject",
    "classObjectID": "JDWPIDLengthEqObject",
    "arrayID": "JDWPIDLengthEqObject",
    "referenceTypeID": "JDWPIDLengthEqReferenceType",
    "classID": "JDWPIDLengthEqReferenceType",
    "interfaceID": "JDWPIDLengthEqReferenceType",
    "arrayTypeID": "JDWPIDLengthEqReferenceType",
    "methodID": "JDWPIDLengthEqMethod",
    "fieldID": "JDWPIDLengthEqField",
    "frameID": "JDWPIDLengthEqFrame",
    "location": "JDWPLocation",
    "string": "JDWPString",
    "value": "JDWPValue",
    "untagged-value": "JDWPValue",
    "arrayregion": "JDWPArrayRegion",
  }
  if type_name in type_mapping:
    return type_mapping[type_name]
  
  if type_name == "repeated":
    return f"Vec<{cmd_name}{snake_to_camel(field['name'])}>"
  if type_name == "case":
    return f"{cmd_name}{snake_to_camel(field['name'])}"

  raise ValueError(f"Unknown type: {type_name}")

def cap_first(s: str) -> str:
  return s[0].upper() + s[1:] if s else ""

def snake_to_camel(snake_str: str) -> str:
  components = snake_str.split("_")
  return cap_first(components[0]) + "".join(cap_first(c) for c in components[1:])

def camel_to_snake(camel_str: str) -> str:
  snake_str = ""
  # 大文字連続の場合は区切らない
  prev_lower = False
  for c in camel_str:
    if c.isupper():
      if prev_lower:
        snake_str += "_"
      snake_str += c.lower()
      prev_lower = False
    else:
      snake_str += c
      prev_lower = True
  return snake_str

if __name__ == "__main__":
  import os
  spec_path = os.path.join(os.path.dirname(__file__), "jdwp-protocol.html")
  spec_text = read_spec_file(spec_path)
  soup = bs4.BeautifulSoup(spec_text, "html.parser")

  extracted_cmd = extract_all_command(soup)
  parsed_command_table = {}
  for cmd in extracted_cmd:
    parsed_command_table[cmd["command_set_name"] + cmd["command_name"]] = parse_command_table(soup, cmd["command_set_name"], cmd["command_name"], cmd["set_num"] < 64)

  # まず、jdwp_command_map! マクロを生成する
  with open(os.path.join(os.path.dirname(__file__), "jdwp_command_map.txt"), "w", encoding="utf-8") as f:
    f.write("// Auto generated\n")
    f.write("jdwp_command_map!(\n")
    for i, cmd in enumerate(extracted_cmd):
      is_last = (i == len(extracted_cmd) - 1)
      payload = "NoData" if "Send" not in parsed_command_table[cmd["command_set_name"] + cmd["command_name"]] or \
                  parsed_command_table[cmd["command_set_name"] + cmd["command_name"]]["Send"] is None \
                else f"{cmd['command_set_name']}{snake_to_camel(cmd['command_name'])}Send"
      Receive = "NoData" if "Receive" not in parsed_command_table[cmd["command_set_name"] + cmd["command_name"]] or \
                  parsed_command_table[cmd["command_set_name"] + cmd["command_name"]]["Receive"] is None \
                else f"{cmd['command_set_name']}{snake_to_camel(cmd['command_name'])}Receive"
      f.write(f"  {cmd['command_set_name']}{snake_to_camel(cmd['command_name'])}({payload}, {Receive}) => {cmd['set_num'], cmd['command_num']}")
      if not is_last:
        f.write(",")
      f.write("\n")
    f.write(");")

  with open(os.path.join(os.path.dirname(__file__), "defs.rs"), "w", encoding="utf-8") as f:
    f.write("// Auto generated\n")
    f.write("use crate::packets::*;\nuse std::io::{Read, Write};\n\n")

    for cmd in extracted_cmd:
      # コマンドセット番号が64以上なら、JVM側から送信される
      # (ので、デバッガから送信されるのは 63 まで)
      try:
        if cmd["set_num"] >= 64:
          data = parse_command_table(soup, cmd["command_set_name"], cmd["command_name"], False)
          f.write(create_rust_structs_from_cmd_data(cmd["command_set_name"] + cmd["command_name"] + "Receive", data["Receive"]))
        else:
          data = parse_command_table(soup, cmd["command_set_name"], cmd["command_name"], True)
          f.write(create_rust_structs_from_cmd_data(cmd["command_set_name"] + cmd["command_name"] + "Send", data["Send"]))
          f.write(create_rust_structs_from_cmd_data(cmd["command_set_name"] + cmd["command_name"] + "Receive", data["Receive"]))
      except Exception as e:
        print(f"Error processing command {cmd}: {e}")
        raise e

