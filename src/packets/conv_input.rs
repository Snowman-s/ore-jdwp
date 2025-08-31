#[derive(Debug, PartialEq, Clone)]
pub enum PrettyIOKind {
  // 文字列
  String(String),
  // 整数
  Int(i64),
  // 浮動小数点数
  Float(f64),
  // 真偽値
  Boolean(bool),

  // リスト (from_value では使用しない)
  List(Vec<Box<PrettyIOKind>>),

  // 繰り返し (from_value では使用しない)
  Repeated(Vec<Box<PrettyIOKind>>),

  // 未指定 (to_valueでは使用しない)
  Unspecified(String),
}

impl fmt::Display for PrettyIOKind {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    match self {
      PrettyIOKind::String(s) => write!(f, "s\"{}\"", s),
      PrettyIOKind::Int(i) => write!(f, "i\"{}\"", i),
      PrettyIOKind::Float(fl) => write!(f, "f\"{}\"", fl),
      PrettyIOKind::Boolean(b) => write!(f, "b\"{}\"", b),
      PrettyIOKind::List(l) => {
        let inner = l
          .iter()
          .map(|i| i.to_string())
          .collect::<Vec<_>>()
          .join(", ");
        write!(f, "[ {} ]", inner)
      }
      PrettyIOKind::Repeated(r) => {
        let inner = r
          .iter()
          .map(|i| i.to_string())
          .collect::<Vec<_>>()
          .join(", ");
        write!(f, "(* {} *)", inner)
      }
      PrettyIOKind::Unspecified(s) => write!(f, "(u\"{}\")", s),
    }
  }
}

#[derive(Debug, PartialEq, Clone)]
pub enum PrettyIOKindTypes {
  String,
  Int,
  Float,
  Boolean,
  List,
  Repeated(Vec<PrettyIOKindTypes>),
  // いろいろ長さや型が変わる
  Variable,
}

use std::fmt;

impl fmt::Display for PrettyIOKindTypes {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    match self {
      PrettyIOKindTypes::String => write!(f, "s"),
      PrettyIOKindTypes::Int => write!(f, "i"),
      PrettyIOKindTypes::Float => write!(f, "f"),
      PrettyIOKindTypes::Boolean => write!(f, "b"),
      PrettyIOKindTypes::List => write!(f, "l"),
      PrettyIOKindTypes::Repeated(inner) => {
        let inner_str = inner
          .iter()
          .map(|t| t.to_string())
          .collect::<Vec<_>>()
          .join(", ");
        write!(f, "(* {} *)", inner_str)
      }
      PrettyIOKindTypes::Variable => write!(f, "?.."),
    }
  }
}

use std::str::FromStr;

impl FromStr for PrettyIOKind {
  type Err = ();

  fn from_str(s: &str) -> Result<Self, Self::Err> {
    // 形式は '(型指定子)"(値)"' か、単に'(値)'。
    // 例えば、'i"42"' や 'f"3.14"' など。
    // 指定が無い場合、unspecified として扱う

    if let Some((type_prefix, rest)) = s.split_once('"') {
      // 最後が " で終わっていることを確認
      if !rest.ends_with('"') {
        return Err(());
      }
      let value = rest.trim_end_matches('"');
      match type_prefix {
        "i" => value.parse::<i64>().ok().map(PrettyIOKind::Int).ok_or(()),
        "f" => value.parse::<f64>().ok().map(PrettyIOKind::Float).ok_or(()),
        "b" => match value {
          "t" | "1" => Ok(PrettyIOKind::Boolean(true)),
          "f" | "0" => Ok(PrettyIOKind::Boolean(false)),
          _ => Err(()),
        },
        "s" => Ok(PrettyIOKind::String(value.to_string())),
        _ => Ok(PrettyIOKind::Unspecified(s.to_string())),
      }
    } else {
      // 型指定子がない場合は Unspecified
      Ok(PrettyIOKind::Unspecified(s.to_string()))
    }
  }
}

pub trait ConvPrettyIOValue: Sized {
  // pretty io kind を入力として受け取り、Self に変換する
  // そして、残りの配列を返す
  // この時、input は先頭から削除されていく
  fn from_value(input: &Vec<PrettyIOKind>) -> Option<(Self, Vec<PrettyIOKind>)>;

  /*
   * pretty な感じのデータにする
   */
  fn to_value(&self) -> Vec<PrettyIOKind>;

  /**
   * from_valueで使用する型
   */
  fn from_value_require_types() -> Vec<PrettyIOKindTypes>;
}

macro_rules! impl_conv_pretty_io_value_struct {
  ($struct_name:ident, $($field_name:ident: $field_type:ty),*,) => {
    impl ConvPrettyIOValue for $struct_name {
      fn from_value(input: &Vec<PrettyIOKind>) -> Option<(Self, Vec<PrettyIOKind>)> {
        let remaining = input;
        $(
          let ($field_name, remaining) = <$field_type>::from_value(&remaining)?;
        )*
        Some((
          Self {$($field_name),*},
          remaining
        ))
      }

      fn to_value(
        &self,
      ) -> Vec<PrettyIOKind> {
        let mut output = Vec::new();

        $(
          output.extend(self.$field_name.to_value());
        )*

        output
      }

      fn from_value_require_types() -> Vec<PrettyIOKindTypes> {
        let mut output = Vec::new();

        $(
          output.extend(<$field_type>::from_value_require_types());
        )*

        output
      }
    }
  };
}

impl ConvPrettyIOValue for String {
  fn from_value(input: &Vec<PrettyIOKind>) -> Option<(Self, Vec<PrettyIOKind>)> {
    match input.first()? {
      PrettyIOKind::String(s) | PrettyIOKind::Unspecified(s) => {
        let s = s.clone();
        let remaining = &input[1..];
        Some((s, remaining.to_vec()))
      }
      _ => None,
    }
  }

  fn to_value(&self) -> Vec<PrettyIOKind> {
    vec![PrettyIOKind::String(self.clone())]
  }

  fn from_value_require_types() -> Vec<PrettyIOKindTypes> {
    vec![PrettyIOKindTypes::String]
  }
}

impl ConvPrettyIOValue for bool {
  fn from_value(input: &Vec<PrettyIOKind>) -> Option<(Self, Vec<PrettyIOKind>)> {
    match input.first()? {
      PrettyIOKind::Boolean(b) => {
        let b = *b;
        let remaining = &input[1..];
        Some((b, remaining.to_vec()))
      }
      PrettyIOKind::Int(n) => {
        let b = *n != 0;
        let remaining = &input[1..];
        Some((b, remaining.to_vec()))
      }
      PrettyIOKind::Unspecified(s) => {
        let s = s.to_lowercase();
        let b = match s.as_str() {
          "t" | "1" => true,
          "f" | "0" => false,
          _ => return None,
        };
        let remaining = &input[1..];
        Some((b, remaining.to_vec()))
      }
      _ => None,
    }
  }

  fn to_value(&self) -> Vec<PrettyIOKind> {
    let mut output = Vec::new();
    output.push(PrettyIOKind::Boolean(*self));
    output
  }

  fn from_value_require_types() -> Vec<PrettyIOKindTypes> {
    vec![PrettyIOKindTypes::Boolean]
  }
}

macro_rules! impl_conv_pretty_io_value_primitives {
  ($($type:ty),*) => {
    $(
      impl ConvPrettyIOValue for $type {
        fn from_value(input: &Vec<PrettyIOKind>) -> Option<(Self, Vec<PrettyIOKind>)> {
          match input.first()? {
            PrettyIOKind::Int(n) => {
              let n = *n;
              let remaining = &input[1..];
              Some((n.try_into().ok()?, remaining.to_vec()))
            }
            PrettyIOKind::Unspecified(n) => {
              let remaining = &input[1..];
              let parsed = n.parse::<$type>().ok()?;
              Some((parsed, remaining.to_vec()))
            }
            _ => None,
          }
        }

        fn to_value(
          &self,
        ) -> Vec<PrettyIOKind> {
          vec![PrettyIOKind::Int(*self as i64)]
        }

        fn from_value_require_types() -> Vec<PrettyIOKindTypes> {
          vec![PrettyIOKindTypes::Int]
        }
      }
    )*
  };
}

impl_conv_pretty_io_value_primitives!(i8, i16, i32, i64, u8, u16, u32, u64);

impl ConvPrettyIOValue for f32 {
  fn from_value(input: &Vec<PrettyIOKind>) -> Option<(Self, Vec<PrettyIOKind>)> {
    match input.first()? {
      PrettyIOKind::Int(n) => {
        let n = *n;
        let remaining = &input[1..];
        Some((n as f32, remaining.to_vec()))
      }
      PrettyIOKind::Float(n) => {
        let n = *n;
        let remaining = &input[1..];
        Some((n as f32, remaining.to_vec()))
      }
      PrettyIOKind::Unspecified(n) => {
        let remaining = &input[1..];
        let parsed = n.parse::<f32>().ok()?;
        Some((parsed, remaining.to_vec()))
      }
      _ => None,
    }
  }

  fn to_value(&self) -> Vec<PrettyIOKind> {
    vec![PrettyIOKind::Float(*self as f64)]
  }

  fn from_value_require_types() -> Vec<PrettyIOKindTypes> {
    vec![PrettyIOKindTypes::Float]
  }
}

impl ConvPrettyIOValue for f64 {
  fn from_value(input: &Vec<PrettyIOKind>) -> Option<(Self, Vec<PrettyIOKind>)> {
    match input.first()? {
      PrettyIOKind::Int(n) => {
        let n = *n;
        let remaining = &input[1..];
        Some((n as f64, remaining.to_vec()))
      }
      PrettyIOKind::Float(n) => {
        let n = *n;
        let remaining = &input[1..];
        Some((n, remaining.to_vec()))
      }
      PrettyIOKind::Unspecified(n) => {
        let remaining = &input[1..];
        let parsed = n.parse::<f64>().ok()?;
        Some((parsed, remaining.to_vec()))
      }
      _ => None,
    }
  }

  fn to_value(&self) -> Vec<PrettyIOKind> {
    vec![PrettyIOKind::Float(*self)]
  }

  fn from_value_require_types() -> Vec<PrettyIOKindTypes> {
    vec![PrettyIOKindTypes::Float]
  }
}

pub fn input_to_pretty_io_kind(input: &str) -> Option<(String, Vec<PrettyIOKind>)> {
  // まずは空白で分割
  let mut parts = input.split_whitespace();
  let mut args = Vec::new();

  let command = parts.next()?.to_string();

  for p in parts {
    args.push(PrettyIOKind::from_str(p).ok()?);
  }

  Some((command, args))
}

mod test {
  use super::*;

  #[derive(Debug, PartialEq, Clone)]
  struct MyStruct {
    field1: String,
    field2: i64,
  }

  impl_conv_pretty_io_value_struct!(MyStruct, field1: String, field2: i64,);

  #[test]
  fn test_from_value() {
    let input = vec![PrettyIOKind::String("test".into()), PrettyIOKind::Int(42)];
    let (result, remaining) = MyStruct::from_value(&input).unwrap();
    assert_eq!(
      result,
      MyStruct {
        field1: "test".into(),
        field2: 42,
      }
    );
    assert_eq!(remaining, vec![]);
  }

  #[test]
  fn test_from_value_insufficient() {
    let input = vec![PrettyIOKind::String("test".into())];
    let result = MyStruct::from_value(&input);
    assert!(result.is_none());
  }

  #[test]
  fn test_from_value_type_mismatch() {
    let input = vec![PrettyIOKind::Int(42), PrettyIOKind::String("test".into())];
    let result = MyStruct::from_value(&input);
    assert!(result.is_none());
  }

  #[test]
  fn test_from_value_unspecified() {
    let input = vec![
      PrettyIOKind::Unspecified("test".into()),
      PrettyIOKind::Unspecified("42".into()),
    ];
    let (result, remaining) = MyStruct::from_value(&input).unwrap();
    assert_eq!(
      result,
      MyStruct {
        field1: "test".into(),
        field2: 42,
      }
    );
    assert_eq!(remaining, vec![]);
  }

  #[test]
  fn test_to_value() {
    let my_struct = MyStruct {
      field1: "test".into(),
      field2: 42,
    };
    assert_eq!(
      my_struct.to_value(),
      vec![PrettyIOKind::String("test".into()), PrettyIOKind::Int(42),]
    );
  }
}

impl ConvPrettyIOValue for () {
  fn from_value(input: &Vec<PrettyIOKind>) -> Option<(Self, Vec<PrettyIOKind>)> {
    Some(((), input.clone()))
  }

  fn to_value(&self) -> Vec<PrettyIOKind> {
    vec![]
  }

  fn from_value_require_types() -> Vec<PrettyIOKindTypes> {
    vec![]
  }
}
