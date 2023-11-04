use super::char_string::CharString;

pub fn get_locator(code: &CharString) -> impl Fn(usize) -> Loc {
  let lines = code.split('\n');
  let mut line_offsets = vec![];
  let mut pos = 0;

  for line in lines {
    line_offsets.push(pos);
    pos += line.len() + 1;
  }

  move |pos| {
    // binary search
    let mut left = 0;
    let mut right = line_offsets.len();

    while left < right {
      let mid = (left + right) >> 1;

      if pos < line_offsets[mid] {
        right = mid;
      } else {
        left = mid + 1;
      }
    }

    let line = left - 1;
    let column = pos - line_offsets[line];

    Loc { line, column }
  }
}

#[derive(Debug, Clone)]
pub struct Loc {
  pub line: usize,
  pub column: usize,
}
