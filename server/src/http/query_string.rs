use std::collections::HashMap;


#[derive(Debug)]
// a=1&b=2&c&d=&e===&d=7&d=abc
pub struct QueryString<'buf> {
    data: HashMap<&'buf str, Value<'buf>>

}

#[derive(Debug)]
pub enum Value <'buf>{
    Single(&'buf str),
    Multiple(Vec<&'buf str>),
}

impl <'buf> QueryString <'buf> {
    pub fn get(&self, key: &str) -> Option<&Value> {
        self.data.get(key)
    }
}

impl<'buf> From<&'buf str> for QueryString<'buf> {
    fn from(s: &'buf str) -> Self {
        let mut data = HashMap::new();

        for sub_str in s.split('&') {
            let mut key = sub_str;
            let mut val = "";
            if let Some(i) = sub_str.find('=') {
                key = &sub_str[..i];
                val = &sub_str[i + 1..];
            }
            // 해시맵에 키가 존재하는지 체크 후, 존재하지 않으면 파라미터로 넘어온 값을 값으로 넣어줌으로써 키 삽입 처리
          //  data.entry(key).or_insert(Value::Single(val))

            // 기존 키값이 존재하는 경우
            data.entry(key).and_modify(|existing|match existing {
                Value::Single(prev_val) => {
                    *existing = Value::Multiple(vec![prev_val, val]);
                }
                Value::Multiple(vec) => vec.push(val)
            }).or_insert(Value::Single(val));
        }

        QueryString {data}
        
    }
}