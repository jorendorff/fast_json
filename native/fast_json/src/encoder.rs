use rustler::{NifDecoder, NifEncoder, NifEnv, NifTerm, NifResult, NifError};
use rustler::atom::{NifAtom, get_atom};
use rustler::list::NifListIterator;
use rustler::map::NifMapIterator;
use json;
use json::JsonValue;

pub fn encode<'a>(env: &'a NifEnv, args: &Vec<NifTerm>) -> NifResult<NifTerm<'a>> {
    let json_val = try!(term_to_json(env, try!(args[0].decode())));
    let json_str = json::stringify(json_val);

    Ok(json_str.encode(env))
}

fn term_to_json<'a>(env: &'a NifEnv, term: NifTerm) -> NifResult<JsonValue> {
    if let Ok(string) = <&str as NifDecoder>::decode(term) {
        handle_binary(env, string)
    } else if let Ok(iter) = <NifListIterator as NifDecoder>::decode(term) {
        handle_list(env, iter)
    } else if let Ok(atom) = NifAtom::from_term(term) {
        handle_atom(env, atom)
    } else if let Ok(number) = <f64 as NifDecoder>::decode(term) {
        handle_float(env, number)
    } else if let Ok(number) = <i64 as NifDecoder>::decode(term) {
        handle_integer(env, number)
    } else if let Ok(iter) = <NifMapIterator as NifDecoder>::decode(term) {
        handle_map(env, iter)
    } else {
        panic!("fail")
    }
}

fn handle_map(env: &NifEnv, iter: NifMapIterator) -> NifResult<JsonValue> {
    use rustler::dynamic::TermType;

    let mut map = json::object::Object::new();

    for (key, value) in iter {
        let key_string = match key.get_type() {
            TermType::Atom => {
                key.atom_to_string().ok().unwrap()
            }
            TermType::Binary => {
                key.decode().ok().unwrap()
            }
            _ => return Err(NifError::BadArg)
        };
        map.insert(&key_string, term_to_json(env, value)?);
    }
    Ok(JsonValue::Object(map))
}

fn handle_list(env: &NifEnv, iter: NifListIterator) -> NifResult<JsonValue> {
    let values: NifResult<Vec<JsonValue>> = iter.map(|term| {
        term_to_json(env, term)
    }).collect();

    Ok(JsonValue::Array(try!(values)))
}

fn handle_binary(_env: &NifEnv, string: &str) -> NifResult<JsonValue> {
    Ok(JsonValue::String(string.to_string()))
}

fn handle_atom(_env: &NifEnv, atom: NifAtom) -> NifResult<JsonValue> {
    if atom == get_atom("true").unwrap() {
        Ok(JsonValue::Boolean(true))
    } else if atom == get_atom("false").unwrap() {
        Ok(JsonValue::Boolean(false))
    } else if atom == get_atom("nil").unwrap() {
        Ok(JsonValue::Null)
    } else {
        Ok(JsonValue::String("nope".to_string()))
    }
}

fn handle_float(_env: &NifEnv, num: f64) -> NifResult<JsonValue> {
    Ok(JsonValue::Number(num.into()))
}

fn handle_integer(_env: &NifEnv, num: i64) -> NifResult<JsonValue> {
    Ok(JsonValue::Number(num.into()))
}
