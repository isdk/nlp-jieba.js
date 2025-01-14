use jieba_rs::Jieba;
use serde::{Deserialize, Serialize};
use std::sync::{LazyLock, Mutex};
use tsify::Tsify;
// use ts_rs::TS;
// use serde_wasm_bindgen::from_value;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsError;

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

// #[wasm_bindgen]
#[derive(Tsify, Serialize, Deserialize, Debug)]
#[tsify(into_wasm_abi, from_wasm_abi)]
// #[ts(export)]
pub struct SplitOptions {
    // #[tsify(optional)]
    mode: Option<SplitMode>,
    // #[tsify(optional)]
    hmm: Option<bool>,
}

impl Default for SplitOptions {
    fn default() -> Self {
        SplitOptions {
            mode: Some(SplitMode::Default),
            hmm: Some(false),
        }
    }
}

#[derive(Tsify, Serialize, Deserialize, Debug)]
#[tsify(into_wasm_abi, from_wasm_abi)]
// #[wasm_bindgen]
// #[derive(TS, Serialize, Deserialize, Debug)]
// #[ts(export)]
pub enum SplitMode {
    Default,
    All,
    Search,
}

#[derive(Tsify, Serialize, Deserialize)]
#[tsify(into_wasm_abi, from_wasm_abi)]
pub struct RetToken<'a> {
    /// Word of the token
    pub word: &'a str,
    /// Unicode start position of the token
    pub start: usize,
    /// Unicode end position of the token
    pub end: usize,
}

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
}

macro_rules! console_log {
    ($($t:tt)*) => (log(&format_args!($($t)*).to_string()))
}

static JIEBA: LazyLock<Mutex<Jieba>> = LazyLock::new(|| Mutex::new(Jieba::new()));

#[wasm_bindgen]
pub fn split(text: &str, options: Option<SplitOptions>) -> Result<Vec<JsValue>, JsError> {
    // console_log!("split: text: {}, options: {:?}", text, options);

    let options_obj = options.unwrap_or(SplitOptions::default());

    // console_log!("split: text: {}, options_obj: {:?}", text, options_obj);
    let words = match options_obj.mode {
        Some(SplitMode::Default) | None => JIEBA.lock().unwrap().cut(text, options_obj.hmm.unwrap_or(false)),
        Some(SplitMode::All) => JIEBA.lock().unwrap().cut_all(text),
        Some(SplitMode::Search) => JIEBA
            .lock()
            .unwrap()
            .cut_for_search(text, options_obj.hmm.unwrap_or(false)),
    };
    Ok(words.into_iter().map(JsValue::from).collect())
}

#[wasm_bindgen]
pub fn tokenize(text: &str, options: Option<SplitOptions>) -> Result<Vec<JsValue>, JsError> {
    let options_obj = options.unwrap_or(SplitOptions::default());

    let mode_enum: jieba_rs::TokenizeMode;
    match options_obj.mode {
        Some(SplitMode::Default) | None => mode_enum = jieba_rs::TokenizeMode::Default,
        Some(SplitMode::Search) => mode_enum = jieba_rs::TokenizeMode::Search,
        Some(SplitMode::All) => {
            return Err(JsError::new("Mode `all` is not valid for tokenize"));
        }
    }

    let hmm = options_obj.hmm.unwrap_or(false);

    let tokens = JIEBA
        .lock()
        .unwrap()
        .tokenize(text, mode_enum, hmm);
    let ret_tokens = tokens
        .into_iter()
        .map(|tok| {
            let t = RetToken {
                word: tok.word,
                start: tok.start,
                end: tok.end,
            };
            serde_wasm_bindgen::to_value(&t).unwrap()
        })
        .collect();
    Ok(ret_tokens)
}

#[wasm_bindgen]
#[allow(non_snake_case)]
pub fn addWord(word: &str, freq: Option<usize>, tag: Option<String>) -> usize {
    let option_str_ref = tag.as_deref();

    JIEBA.lock().unwrap().add_word(word, freq, option_str_ref)
}

#[derive(Serialize, Deserialize)]
pub struct RetTag<'a> {
    word: &'a str,
    tag: &'a str,
}

#[wasm_bindgen]
pub fn tag(sentence: &str, hmm: Option<bool>) -> Vec<JsValue> {
    let jieba = JIEBA.lock().unwrap();
    let tags = jieba.tag(sentence, hmm.unwrap_or(true));
    let ret_tags = tags
        .into_iter()
        .map(|t| {
            let r = RetTag {
                tag: t.tag,
                word: t.word,
            };
            serde_wasm_bindgen::to_value(&r).unwrap()
        })
        .collect();
    ret_tags
}
