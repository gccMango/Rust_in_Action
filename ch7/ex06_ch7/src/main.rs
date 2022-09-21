
use std::collections::HashMap;
use std::io;
use std::io::prelude::*;
use std::io::{BufReader, BufWriter,SeekFrom};
use std::path::Path;

/**ByteString은 별명, Vec\<u8\> String인데 byte로 해석*/
type ByteString = Vec<u8>;

/**ByteStr은 \[u8\] 이고 str이 내부적으로 u8의 배열로 이뤄진다*/
type ByteStr = [u8];

pub struct KeyValuePair {
    pub key: ByteString,
    pub value: ByteString,
}

pub struct ActionKV {
    f: File,
    pub index: HashMap< ByteString, u64>
}

#[cfg(test)]
mod tests {

}