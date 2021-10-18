use std::{collections::HashMap, mem::MaybeUninit, str::from_utf8};

pub struct Index<'s> {
    /// bytes that source sentences come from
    source: &'s [u8],
    // TODO: since the csv file is constant, use smallvec and experimentally
    // derive optimal value for stack storage.
    graph: HashMap<char, Vec<&'s str>>,
}

impl<'s> Index<'s> {
    /// source expected to be a csv file [id, jap, eng, _] with no headers
    /// separated by tabs.
    pub fn new(source: &'s [u8]) -> Self {
        let mut graph = HashMap::new();

        // go through the sentences and register sentences by the kanji they contain
        for sentence in read_jap(source) {
            for kanji in sentence.chars() {
                graph
                    .entry(kanji)
                    // this works because `source` outlives `graph`
                    .and_modify(|acc: &mut Vec<_>| acc.push(sentence))
                    .or_default();
            }
        }

        Index { source, graph }
    }

    // find all sentences associated with this kanji
    pub fn find(&self, kanji: &char) -> Option<&[&str]> {
        self.graph.get(kanji).map(Vec::as_ref)
    }

    // TODO: rework types for better interface
    /// find all sentences associated with each of the provided kanji
    pub fn find_all(&self, kanji: impl IntoIterator<Item = char>) -> impl Iterator<Item = &[&str]> {
        kanji
            .into_iter()
            .map(move |k| self.find(&k).unwrap_or_default())
    }
}

/// read japanese sentences only. input must be in expected format.
fn read_jap<'a>(src: &'a [u8]) -> impl Iterator<Item = &'a str> {
    const NEWLINE_DELIM: u8 = b'\n';
    const COL_DELIM: u8 = b'\t';
    src.split(|&c| c == NEWLINE_DELIM)
        .filter(|line| !line.is_empty())
        .map(|line| {
            let col = line
                // split the row into four columns
                .splitn(4, |c| *c == COL_DELIM)
                // choose the second one
                .nth(1)
                .expect("input row did not contain japanese text column");
            from_utf8(col).expect("bad utf-8")
            // TODO: actual error handling
        })
}

#[cfg(test)]
mod tests {
    mod read_jap {
        use crate::index::read_jap;

        #[test]
        fn empty() {
            let src = &[];
            let mut sent = read_jap(src);
            assert_eq!(sent.next(), None)
        }

        #[test]
        #[should_panic]
        fn malformed_row() {
            let src = "123\n456\n789\n0";
            let mut sent = read_jap(src.as_ref());
            assert_eq!(sent.next(), None)
        }

        #[test]
        #[should_panic]
        #[ignore]
        fn malformed_utf8() {
            todo!("test malformed utf-8")
        }

        #[test]
        fn test_sentences_csv() {
            let src = include_str!("../sentences.csv");
            let mut sent = read_jap(src.as_ref());

            // just test the first row for sanity
            assert_eq!(sent.next(), Some("すぐに諦めて昼寝をするかも知れない。"));
            // we can really test the content so just make sure that it reads the correct
            // number of lines.
            // add 1 because of the line we manually extracted
            assert_eq!(sent.count() + 1, src.lines().count())
        }
    }
}
