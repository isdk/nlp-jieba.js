#![allow(non_snake_case)]

use std::io::Cursor;
use jieba_rs::Jieba;
use serde::{Deserialize, Serialize};
use std::sync::{LazyLock, Mutex};
use tsify::Tsify;
// use ts_rs::TS;
// use serde_wasm_bindgen::from_value;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsError;
use wasm_bindgen::JsCast;
use js_sys::Uint8Array;

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

// #[wasm_bindgen]
#[derive(Tsify, Serialize, Deserialize, Debug)]
#[tsify(into_wasm_abi, from_wasm_abi)]
// #[ts(export)]
pub struct JiebaSplitOptions {
    // #[tsify(optional)]
    mode: Option<JiebaSplitMode>,
    // #[tsify(optional)]
    hmm: Option<bool>,
}

impl Default for JiebaSplitOptions {
    fn default() -> Self {
        JiebaSplitOptions {
            mode: Some(JiebaSplitMode::Default),
            hmm: Some(false),
        }
    }
}

#[derive(Tsify, Serialize, Deserialize, Debug)]
#[tsify(into_wasm_abi, from_wasm_abi)]
// #[wasm_bindgen]
// #[derive(TS, Serialize, Deserialize, Debug)]
// #[ts(export)]
pub enum JiebaSplitMode {
    Default,
    All,
    Search,
}

#[derive(Tsify, Serialize, Deserialize)]
#[tsify(into_wasm_abi, from_wasm_abi)]
pub struct JiebaToken<'a> {
    /// Word of the token
    pub word: &'a str,
    /// Unicode start position of the token
    pub start: usize,
    /// Unicode end position of the token
    pub end: usize,
}

#[derive(Tsify, Serialize, Deserialize)]
#[tsify(into_wasm_abi, from_wasm_abi)]
pub struct JiebaTag<'a> {
    word: &'a str,
    tag: &'a str,
}

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
}

macro_rules! console_log {
    ($($t:tt)*) => (log(&format_args!($($t)*).to_string()))
}

static JIEBA: LazyLock<Mutex<Jieba>> = LazyLock::new(|| Mutex::new(Jieba::empty()));

#[wasm_bindgen]
pub fn loadDefaultDict() {
    JIEBA.lock().unwrap().load_default_dict()
}

#[wasm_bindgen]
pub fn loadDict(dict_content: JsValue) -> Result<(), JsError> {
    let mut jieba = JIEBA.lock().map_err(|_| JsError::new("Failed to acquire lock"))?;

    if dict_content.is_string() {
        let dict_str = dict_content.as_string().unwrap();
        let mut cursor = Cursor::new(dict_str.as_bytes());

        jieba.load_dict(&mut cursor).map_err(|e| JsError::new(&format!("Failed to load dictionary: {}", e)))
    } else if dict_content.is_instance_of::<Uint8Array>() {
        let uint8_array: Uint8Array = dict_content.dyn_into().unwrap();
        let bytes = uint8_array.to_vec();
        let mut cursor = Cursor::new(&bytes);

        jieba.load_dict(&mut cursor).map_err(|e| JsError::new(&format!("Failed to load dictionary: {}", e)))
    } else {
        Err(JsError::new("Invalid dictionary content type"))
    }
}

#[wasm_bindgen]
pub fn clear() {
    JIEBA.lock().unwrap().clear()
}

#[wasm_bindgen]
pub fn suggestFreq(segment: &str) -> Result<usize, JsValue> {
    let freq = JIEBA.lock().unwrap().suggest_freq(segment);
    Ok(freq)
}

#[wasm_bindgen]
pub fn split(text: &str, options: Option<JiebaSplitOptions>) -> Result<Vec<JsValue>, JsError> {
    // console_log!("split: text: {}, options: {:?}", text, options);

    let options_obj = options.unwrap_or(JiebaSplitOptions::default());
    let hmm = options_obj.hmm.unwrap_or(false);
    let jieba = JIEBA.lock().unwrap();

    // console_log!("split: text: {}, options_obj: {:?}", text, options_obj);
    let words = match options_obj.mode {
        Some(JiebaSplitMode::Default) | None => jieba.cut(text, hmm),
        Some(JiebaSplitMode::All) => jieba.cut_all(text),
        Some(JiebaSplitMode::Search) => jieba.cut_for_search(text, hmm),
    };
    Ok(words.into_iter().map(JsValue::from).collect())
}

#[wasm_bindgen]
pub fn tokenize(text: &str, options: Option<JiebaSplitOptions>) -> Result<Vec<JsValue>, JsError> {
    let options_obj = options.unwrap_or(JiebaSplitOptions::default());

    let mode_enum: jieba_rs::TokenizeMode;
    match options_obj.mode {
        Some(JiebaSplitMode::Default) | None => mode_enum = jieba_rs::TokenizeMode::Default,
        Some(JiebaSplitMode::Search) => mode_enum = jieba_rs::TokenizeMode::Search,
        Some(JiebaSplitMode::All) => {
            return Err(JsError::new("Mode `all` is not valid for tokenize"));
        }
    }
    let hmm = options_obj.hmm.unwrap_or(false);
    let jieba = JIEBA.lock().unwrap();

    let tokens = jieba.tokenize(text, mode_enum, hmm);
    let result = tokens
        .into_iter()
        .map(|tok| {
            let t = JiebaToken {
                word: tok.word,
                start: tok.start,
                end: tok.end,
            };
            serde_wasm_bindgen::to_value(&t).unwrap()
        })
        .collect();
    Ok(result)
}

#[wasm_bindgen]
pub fn addWord(word: &str, freq: Option<usize>, tag: Option<String>) -> usize {
    let option_str_ref = tag.as_deref();

    JIEBA.lock().unwrap().add_word(word, freq, option_str_ref)
}


#[wasm_bindgen]
pub fn removeWord(word: &str) -> bool {
    JIEBA.lock().map_err(|_| false)
        .and_then(|mut jieba| Ok(jieba.remove_word(word)))
        .unwrap_or(false)
}

#[wasm_bindgen]
pub fn hasWord(word: &str) -> bool {
    JIEBA.lock().unwrap().has_word(word)
}

#[wasm_bindgen]
pub fn tag(sentence: &str, hmm: Option<bool>) -> Vec<JsValue> {
    let jieba = JIEBA.lock().unwrap();
    let tags = jieba.tag(sentence, hmm.unwrap_or(true));
    let result = tags
        .into_iter()
        .map(|t| {
            let r = JiebaTag {
                tag: t.tag,
                word: t.word,
            };
            serde_wasm_bindgen::to_value(&r).unwrap()
        })
        .collect();
    result
}
