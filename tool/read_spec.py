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

    # コマンドを抽出
    for cmd_li in li.find_all("ul"):
      for cli in cmd_li.find_all("li", recursive=False):
        cmd_name = cli.find("a").text.strip()
        m = re.search(r"\((\d+)\)", cli.text)
        cmd_num = int(m.group(1)) if m else None
        commands.append({
          "set_num": set_num,
          "command_name": cmd_name,
          "command_num": cmd_num
        })

  return commands

def parse_command_table(soup, command_name: str, send_from_debugger: bool = True):
  h5 = None
  for h5_tag in soup.find_all("h5"):
    a_tag = h5_tag.find("a")
    if a_tag and command_name in a_tag.text:
      h5 = h5_tag
      break

  if not h5:
    return None
  
  dl = h5.find_next_sibling("dl")

  # dl の真下にある、<dd> を探す。長さは3のはず。
  dd_tags = dl.find_all("dd", recursive=False)

  if send_from_debugger:
    if len(dd_tags) != 3:
      raise ValueError(f"Expected 3 <dd> tags, found {len(dd_tags)}")

    # それぞれの <dd> から情報を抽出
    return {
      "Out": parse_packet_table(dd_tags[0]),
      "Response": parse_packet_table(dd_tags[1]),
      # Error だけパケットテーブルではない
      "Error": parse_error_info(dd_tags[2])
    }
  else:
    if len(dd_tags) != 1:
      raise ValueError(f"Expected 1 <dd> tags, found {len(dd_tags)}")

    # それぞれの <dd> から情報を抽出
    return {
      "Out": parse_packet_table(dd_tags[0]),
    }


def parse_packet_table(soup: bs4.BeautifulSoup):
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
  def make_row(indent=0):
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
            row = make_row(indent + 1)
            if not row:
              break
            child += row
            row_index += 1
          return [{
            "type": "int",
            "name": name_text,
            "description": desc_text
          }, {
            "type": "repeated",
            "name": name_text + "Repeated",
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
          # ケースの子のループ
          child = []
          row_index += 2 if first_case else 1
          first_case = False
          while row_index < len(trs):
            row = make_row(indent + 1)
            if not row:
              break
            child += row
            row_index += 1

          case |= {case_name: child}

        if case != {}:
          return [{
            "type": "byte",
            "name": name_text,
            "description": desc_text
          },{
            "type": "case",
            "name": name_text + "Cases",
            "description": "",
            "case": case
          }]

      # 行から辞書を作成
      return [{
        "type": type_text,
        "name": name_text,
        "description": desc_text
      }]

  rows = []
  while True:
    row = make_row()
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
      for key_of_case, body_of_case in field["case"]:
        ret += create_rust_structs_from_cmd_data(f"{cmd_name}{cap_first(field['name'])}{cap_first(key_of_case)}", body_of_case) + "\n\n"

  ret += "#[derive(Debug, PartialEq, Eq, Hash, Clone)]\n"
  ret += f"pub struct {cmd_name} {{\n"
  for field in table_data:
    ret += f"  /* {field['description']} */\n"
    ret += f"  pub {field['name']}: {type_to_rust_str(cmd_name, field)},\n"
  ret += "}\n\n"

  ret += f"impl PacketData for {cmd_name} {{\n"
  ret += "  fn write_to<W: std::io::Write>(&self, writer: &mut W) -> Result<(), std::io::Error> {\n"
  for field in table_data:
    if field["type"] == "repeated":
      ret += f"    for item in &self.{field['name']} {{\n"
      ret += f"      item.write_to(writer)?;\n"
      ret += f"    }}\n"
    else:
      ret += f"    self.{field['name']}.write_to(writer)?;\n"
  ret += "    Ok(())\n"
  ret += "  }\n\n"
  ret += "  fn read_from<R: std::io::Read>(reader: &mut R, c: &JDWPContext) -> Result<Self, std::io::Error> {\n"
  for field in table_data:
    if field["type"] == "repeated":
      ret += f"    let {field['name']} = (0..{field['by']}).map(|_| {cmd_name}{cap_first(field['name'])}::read_from(reader, c)).collect();\n"
    else:
      ret += f"    let {field['name']} = {type_to_rust_str(cmd_name, field)}::read_from(reader, c)?;\n"
  ret += f"    Ok({cmd_name} {{\n"
  for field in table_data:
    ret += f"      {field['name']},\n"
  ret += "    })\n"
  ret += "  }\n"
  ret += "}\n"

  return ret

def type_to_rust_str(cmd_name: str, field: dict) -> str:
  type_name = field["type"]

  type_mapping = {
    "int": "i32",
    "long": "i64",
    "byte": "u8",
    "boolean": "bool",
    "string": "JDWPString",
    "referenceTypeID": "JDWPReferenceTypeID"
  }
  if type_name in type_mapping:
    return type_mapping[type_name]
  
  if type_name == "repeated":
    return f"Vec<{cmd_name}{cap_first(field['name'])}>"
  raise ValueError(f"Unknown type: {type_name}")

def cap_first(s: str) -> str:
  return s[0].upper() + s[1:] if s else ""

if __name__ == "__main__":
  import os
  spec_path = os.path.join(os.path.dirname(__file__), "jdwp-protocol.html")
  spec_text = read_spec_file(spec_path)
  soup = bs4.BeautifulSoup(spec_text, "html.parser")

  extracted_cmd = extract_all_command(soup)
#  for cmd in extracted_cmd:
    # コマンドセット番号が64以上なら、JVM側から送信される
    # (ので、デバッガから送信されるのは 63 まで)
#    if cmd["set_num"] >= 64:
#      data = parse_command_table(soup, cmd["command_name"], False)
#      print(create_rust_structs_from_cmd_data(cmd["command_name"] + "Out", data["Out"]), end="")
#    else:
#      data = parse_command_table(soup, cmd["command_name"], True)
#      print(create_rust_structs_from_cmd_data(cmd["command_name"] + "Out", data["Out"]), end="")
#      print(create_rust_structs_from_cmd_data(cmd["command_name"] + "Response", data["Response"]), end="")

cmd = extracted_cmd[1]
data = parse_command_table(soup, cmd["command_name"], True)
print(create_rust_structs_from_cmd_data(cmd["command_name"] + "Out", data["Out"]))
print(create_rust_structs_from_cmd_data(cmd["command_name"] + "Response", data["Response"]), end="")
