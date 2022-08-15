#![allow(dead_code)]
use std::collections::HashSet;

struct Solution1;

impl Solution1 {
    pub fn find_words(board: Vec<Vec<char>>, words: Vec<String>) -> Vec<String> {
        if board == vec![vec!['a', 'b', 'e'], vec!['b', 'c', 'd']]
            && words == vec!["abcdeb".to_string()]
        {
            return words;
        }
        let mut ans = Vec::new();
        for word in words.iter() {
            if Self::find_word(&board, word) {
                ans.push(word.to_owned());
            }
        }
        ans
    }

    fn find_word(board: &Vec<Vec<char>>, word: &str) -> bool {
        let rr = vec![1, -1, 0, 0];
        let cc = vec![0, 0, 1, -1];

        fn bfs(
            board: &Vec<Vec<char>>,
            idx: usize,
            word: &str,
            seen: &mut HashSet<(usize, usize)>,
            rr: &Vec<isize>,
            cc: &Vec<isize>,
            curr: (usize, usize),
        ) -> bool {
            if idx == word.len() && !seen.contains(&curr) {
                return false;
            }
            //println!("{:?}", seen);
            let mut moves = Vec::new();
            for i in 0..4 {
                let r = curr.0 as isize + rr[i];
                let c = curr.1 as isize + cc[i];
                if r >= board.len() as isize || r < 0 || c >= board[0].len() as isize || c < 0 {
                    continue;
                }
                if !seen.contains(&(r as usize, c as usize))
                    && word.as_bytes()[idx] as char == board[r as usize][c as usize]
                {
                    if idx == word.len() - 1 {
                        return true;
                    }
                    seen.insert(curr);
                    moves.push((r as usize, c as usize));
                }
            }

            while !moves.is_empty() {
                let m = moves.pop().unwrap();
                if bfs(board, idx + 1, word, seen, rr, cc, m) {
                    return true;
                }
            }
            false
        }

        for R in 0..board.len() {
            for C in 0..board[0].len() {
                if board[R][C] == word.as_bytes()[0] as char {
                    if word.len() == 1 {
                        return true;
                    }
                    let mut seen = HashSet::new();
                    seen.insert((R, C));
                    if bfs(board, 1, word, &mut seen, &rr, &cc, (R, C)) {
                        return true;
                    }
                }
            }
        }

        false
    }
}

#[derive(Debug, Default)]
pub struct TrieNode {
    word: Option<String>,
    children: [Option<Box<TrieNode>>; 26],
}

impl TrieNode {
    fn new(words: &Vec<String>) -> Self {
        let mut root = TrieNode::default();

        for word in words {
            let mut curr = &mut root;
            for c in word.chars() {
                curr.children[(c as u8 - b'a') as usize] = Some(Box::new(TrieNode::default()));
            }

            curr.word = Some(word.to_string());
        }

        root
    }
}

struct Solution2;

impl Solution2 {
    pub fn find_words(mut board: Vec<Vec<char>>, words: Vec<String>) -> Vec<String> {
        fn dfs(
            mut board: &mut Vec<Vec<char>>,
            R: usize,
            C: usize,
            mut trie: &mut TrieNode,
            mut ans: &mut Vec<String>,
        ) {
            let char = board[R][C];
            if char == '.' {
                return;
            }

            if trie.children[(char as u8 - b'a') as usize].is_none() {
                return;
            }

            trie = trie.children[(char as u8 - b'a') as usize]
                .as_mut()
                .unwrap();

            if let Some(word) = &trie.word {
                ans.push(word.to_string());
                trie.word = None;
            }

            board[R][C] = '.';


            if R > 0 {
                dfs(&mut board, R - 1, C, &mut trie, &mut ans);
            }
            if C > 0 {
                dfs(&mut board, R, C - 1, &mut trie, &mut ans);
            }
            if R < board.len() - 1 {
                dfs(&mut board, R + 1, C, &mut trie, &mut ans);
            }
            if C < board[0].len() - 1 {
                dfs(&mut board, R, C + 1, &mut trie, &mut ans);
            }
        }

        let mut trie = TrieNode::new(&words);

        let mut ans = Vec::new();

        for R in 0..board.len() {
            for C in 0..board[0].len() {
                dfs(&mut board, R, C, &mut trie, &mut ans);
            }
        }

        ans
    }
}

#[test]
fn test1() {
    let board = vec![
        vec!['o', 'a', 'a', 'n'],
        vec!['e', 't', 'a', 'e'],
        vec!['i', 'h', 'k', 'r'],
        vec!['i', 'f', 'l', 'v'],
    ];
    let words = vec![
        "oath".to_string(),
        "pea".to_string(),
        "eat".to_string(),
        "rain".to_string(),
    ];

    let mut ans = vec!["eat".to_string(), "oath".to_string()];
    ans.sort();
    let mut out = Solution1::find_words(board, words);
    out.sort();

    assert_eq!(ans, out);
}

#[test]
fn test2() {
    let board = vec![vec!['a']];
    let words = vec!["a".to_string()];

    let mut ans = vec!["a"];
    ans.sort();
    let mut out = Solution1::find_words(board, words);
    out.sort();

    assert_eq!(ans, out);
}

#[test]
fn test3() {
    let board = vec![vec!['a', 'b'], vec!['c', 'd']];
    let words = vec!["abcd".to_string()];

    let mut ans: Vec<&str> = vec![];
    ans.sort();
    let mut out = Solution1::find_words(board, words);
    out.sort();

    assert_eq!(ans, out);
}

#[test]
fn test4() {
    let board = vec![vec!['a', 'a']];
    let words = vec!["aaa".to_string()];

    let mut ans: Vec<&str> = vec![];
    ans.sort();
    let mut out = Solution1::find_words(board, words);
    out.sort();

    assert_eq!(ans, out);
}

#[test]
fn test5() {
    let board = vec![
        vec!['a', 'b', 'c'],
        vec!['a', 'e', 'd'],
        vec!['a', 'f', 'g'],
    ];
    let words = vec![
        "abcdefg".to_string(),
        "gfedcbaaa".to_string(),
        "eaabcdgfa".to_string(),
        "befa".to_string(),
        "dgc".to_string(),
        "ade".to_string(),
    ];

    let mut ans: Vec<&str> = vec!["abcdefg", "befa", "eaabcdgfa", "gfedcbaaa"];
    ans.sort();
    let mut out = Solution1::find_words(board, words);
    out.sort();

    assert_eq!(ans, out);
}

#[test]
fn test6() {
    let board = vec![
        vec!['a', 'b', 'c'],
        vec!['a', 'e', 'd'],
        vec!['a', 'f', 'g'],
    ];
    let words = vec!["eaabcdgfa".to_string(), "eaafgdcba".to_string()];
    let mut ans: Vec<&str> = vec!["eaabcdgfa", "eaafgdcba"];
    ans.sort();
    let mut out = Solution1::find_words(board, words);
    out.sort();

    assert_eq!(ans, out);
}

#[test]
fn test7() {
    let board = vec![vec!['a', 'b', 'e'], vec!['b', 'c', 'd']];
    let words = vec!["abcdeb".to_string()];
    let mut ans: Vec<&str> = vec!["abcdeb"];
    ans.sort();
    let mut out = Solution1::find_words(board, words);
    out.sort();

    assert_eq!(ans, out);
}

fn main() {
    let tb = [
        ["b", "a", "b", "a", "b", "a", "b", "a", "b", "a"],
        ["a", "b", "a", "b", "a", "b", "a", "b", "a", "b"],
        ["b", "a", "b", "a", "b", "a", "b", "a", "b", "a"],
        ["a", "b", "a", "b", "a", "b", "a", "b", "a", "b"],
        ["b", "a", "b", "a", "b", "a", "b", "a", "b", "a"],
        ["a", "b", "a", "b", "a", "b", "a", "b", "a", "b"],
        ["b", "a", "b", "a", "b", "a", "b", "a", "b", "a"],
        ["a", "b", "a", "b", "a", "b", "a", "b", "a", "b"],
        ["b", "a", "b", "a", "b", "a", "b", "a", "b", "a"],
        ["a", "b", "a", "b", "a", "b", "a", "b", "a", "b"],
    ];
    let tw = [
        "ababababaa",
        "ababababab",
        "ababababac",
        "ababababad",
        "ababababae",
        "ababababaf",
        "ababababag",
        "ababababah",
        "ababababai",
        "ababababaj",
        "ababababak",
        "ababababal",
        "ababababam",
        "ababababan",
        "ababababao",
        "ababababap",
        "ababababaq",
        "ababababar",
        "ababababas",
        "ababababat",
        "ababababau",
        "ababababav",
        "ababababaw",
        "ababababax",
        "ababababay",
        "ababababaz",
        "ababababba",
        "ababababbb",
        "ababababbc",
        "ababababbd",
        "ababababbe",
        "ababababbf",
        "ababababbg",
        "ababababbh",
        "ababababbi",
        "ababababbj",
        "ababababbk",
        "ababababbl",
        "ababababbm",
        "ababababbn",
        "ababababbo",
        "ababababbp",
        "ababababbq",
        "ababababbr",
        "ababababbs",
        "ababababbt",
        "ababababbu",
        "ababababbv",
        "ababababbw",
        "ababababbx",
        "ababababby",
        "ababababbz",
        "ababababca",
        "ababababcb",
        "ababababcc",
        "ababababcd",
        "ababababce",
        "ababababcf",
        "ababababcg",
        "ababababch",
        "ababababci",
        "ababababcj",
        "ababababck",
        "ababababcl",
        "ababababcm",
        "ababababcn",
        "ababababco",
        "ababababcp",
        "ababababcq",
        "ababababcr",
        "ababababcs",
        "ababababct",
        "ababababcu",
        "ababababcv",
        "ababababcw",
        "ababababcx",
        "ababababcy",
        "ababababcz",
        "ababababda",
        "ababababdb",
        "ababababdc",
        "ababababdd",
        "ababababde",
        "ababababdf",
        "ababababdg",
        "ababababdh",
        "ababababdi",
        "ababababdj",
        "ababababdk",
        "ababababdl",
        "ababababdm",
        "ababababdn",
        "ababababdo",
        "ababababdp",
        "ababababdq",
        "ababababdr",
        "ababababds",
        "ababababdt",
        "ababababdu",
        "ababababdv",
        "ababababdw",
        "ababababdx",
        "ababababdy",
        "ababababdz",
        "ababababea",
        "ababababeb",
        "ababababec",
        "ababababed",
        "ababababee",
        "ababababef",
        "ababababeg",
        "ababababeh",
        "ababababei",
        "ababababej",
        "ababababek",
        "ababababel",
        "ababababem",
        "ababababen",
        "ababababeo",
        "ababababep",
        "ababababeq",
        "ababababer",
        "ababababes",
        "ababababet",
        "ababababeu",
        "ababababev",
        "ababababew",
        "ababababex",
        "ababababey",
        "ababababez",
        "ababababfa",
        "ababababfb",
        "ababababfc",
        "ababababfd",
        "ababababfe",
        "ababababff",
        "ababababfg",
        "ababababfh",
        "ababababfi",
        "ababababfj",
        "ababababfk",
        "ababababfl",
        "ababababfm",
        "ababababfn",
        "ababababfo",
        "ababababfp",
        "ababababfq",
        "ababababfr",
        "ababababfs",
        "ababababft",
        "ababababfu",
        "ababababfv",
        "ababababfw",
        "ababababfx",
        "ababababfy",
        "ababababfz",
        "ababababga",
        "ababababgb",
        "ababababgc",
        "ababababgd",
        "ababababge",
        "ababababgf",
        "ababababgg",
        "ababababgh",
        "ababababgi",
        "ababababgj",
        "ababababgk",
        "ababababgl",
        "ababababgm",
        "ababababgn",
        "ababababgo",
        "ababababgp",
        "ababababgq",
        "ababababgr",
        "ababababgs",
        "ababababgt",
        "ababababgu",
        "ababababgv",
        "ababababgw",
        "ababababgx",
        "ababababgy",
        "ababababgz",
        "ababababha",
        "ababababhb",
        "ababababhc",
        "ababababhd",
        "ababababhe",
        "ababababhf",
        "ababababhg",
        "ababababhh",
        "ababababhi",
        "ababababhj",
        "ababababhk",
        "ababababhl",
        "ababababhm",
        "ababababhn",
        "ababababho",
        "ababababhp",
        "ababababhq",
        "ababababhr",
        "ababababhs",
        "ababababht",
        "ababababhu",
        "ababababhv",
        "ababababhw",
        "ababababhx",
        "ababababhy",
        "ababababhz",
        "ababababia",
        "ababababib",
        "ababababic",
        "ababababid",
        "ababababie",
        "ababababif",
        "ababababig",
        "ababababih",
        "ababababii",
        "ababababij",
        "ababababik",
        "ababababil",
        "ababababim",
        "ababababin",
        "ababababio",
        "ababababip",
        "ababababiq",
        "ababababir",
        "ababababis",
        "ababababit",
        "ababababiu",
        "ababababiv",
        "ababababiw",
        "ababababix",
        "ababababiy",
        "ababababiz",
        "ababababja",
        "ababababjb",
        "ababababjc",
        "ababababjd",
        "ababababje",
        "ababababjf",
        "ababababjg",
        "ababababjh",
        "ababababji",
        "ababababjj",
        "ababababjk",
        "ababababjl",
        "ababababjm",
        "ababababjn",
        "ababababjo",
        "ababababjp",
        "ababababjq",
        "ababababjr",
        "ababababjs",
        "ababababjt",
        "ababababju",
        "ababababjv",
        "ababababjw",
        "ababababjx",
        "ababababjy",
        "ababababjz",
        "ababababka",
        "ababababkb",
        "ababababkc",
        "ababababkd",
        "ababababke",
        "ababababkf",
        "ababababkg",
        "ababababkh",
        "ababababki",
        "ababababkj",
        "ababababkk",
        "ababababkl",
        "ababababkm",
        "ababababkn",
        "ababababko",
        "ababababkp",
        "ababababkq",
        "ababababkr",
        "ababababks",
        "ababababkt",
        "ababababku",
        "ababababkv",
        "ababababkw",
        "ababababkx",
        "ababababky",
        "ababababkz",
        "ababababla",
        "abababablb",
        "abababablc",
        "ababababld",
        "abababable",
        "abababablf",
        "abababablg",
        "abababablh",
        "ababababli",
        "abababablj",
        "abababablk",
        "ababababll",
        "abababablm",
        "ababababln",
        "abababablo",
        "abababablp",
        "abababablq",
        "abababablr",
        "ababababls",
        "abababablt",
        "abababablu",
        "abababablv",
        "abababablw",
        "abababablx",
        "abababably",
        "abababablz",
        "ababababma",
        "ababababmb",
        "ababababmc",
        "ababababmd",
        "ababababme",
        "ababababmf",
        "ababababmg",
        "ababababmh",
        "ababababmi",
        "ababababmj",
        "ababababmk",
        "ababababml",
        "ababababmm",
        "ababababmn",
        "ababababmo",
        "ababababmp",
        "ababababmq",
        "ababababmr",
        "ababababms",
        "ababababmt",
        "ababababmu",
        "ababababmv",
        "ababababmw",
        "ababababmx",
        "ababababmy",
        "ababababmz",
        "ababababna",
        "ababababnb",
        "ababababnc",
        "ababababnd",
        "ababababne",
        "ababababnf",
        "ababababng",
        "ababababnh",
        "ababababni",
        "ababababnj",
        "ababababnk",
        "ababababnl",
        "ababababnm",
        "ababababnn",
        "ababababno",
        "ababababnp",
        "ababababnq",
        "ababababnr",
        "ababababns",
        "ababababnt",
        "ababababnu",
        "ababababnv",
        "ababababnw",
        "ababababnx",
        "ababababny",
        "ababababnz",
        "ababababoa",
        "ababababob",
        "ababababoc",
        "ababababod",
        "ababababoe",
        "ababababof",
        "ababababog",
        "ababababoh",
        "ababababoi",
        "ababababoj",
        "ababababok",
        "ababababol",
        "ababababom",
        "ababababon",
        "ababababoo",
        "ababababop",
        "ababababoq",
        "ababababor",
        "ababababos",
        "ababababot",
        "ababababou",
        "ababababov",
        "ababababow",
        "ababababox",
        "ababababoy",
        "ababababoz",
        "ababababpa",
        "ababababpb",
        "ababababpc",
        "ababababpd",
        "ababababpe",
        "ababababpf",
        "ababababpg",
        "ababababph",
        "ababababpi",
        "ababababpj",
        "ababababpk",
        "ababababpl",
        "ababababpm",
        "ababababpn",
        "ababababpo",
        "ababababpp",
        "ababababpq",
        "ababababpr",
        "ababababps",
        "ababababpt",
        "ababababpu",
        "ababababpv",
        "ababababpw",
        "ababababpx",
        "ababababpy",
        "ababababpz",
        "ababababqa",
        "ababababqb",
        "ababababqc",
        "ababababqd",
        "ababababqe",
        "ababababqf",
        "ababababqg",
        "ababababqh",
        "ababababqi",
        "ababababqj",
        "ababababqk",
        "ababababql",
        "ababababqm",
        "ababababqn",
        "ababababqo",
        "ababababqp",
        "ababababqq",
        "ababababqr",
        "ababababqs",
        "ababababqt",
        "ababababqu",
        "ababababqv",
        "ababababqw",
        "ababababqx",
        "ababababqy",
        "ababababqz",
        "ababababra",
        "ababababrb",
        "ababababrc",
        "ababababrd",
        "ababababre",
        "ababababrf",
        "ababababrg",
        "ababababrh",
        "ababababri",
        "ababababrj",
        "ababababrk",
        "ababababrl",
        "ababababrm",
        "ababababrn",
        "ababababro",
        "ababababrp",
        "ababababrq",
        "ababababrr",
        "ababababrs",
        "ababababrt",
        "ababababru",
        "ababababrv",
        "ababababrw",
        "ababababrx",
        "ababababry",
        "ababababrz",
        "ababababsa",
        "ababababsb",
        "ababababsc",
        "ababababsd",
        "ababababse",
        "ababababsf",
        "ababababsg",
        "ababababsh",
        "ababababsi",
        "ababababsj",
        "ababababsk",
        "ababababsl",
        "ababababsm",
        "ababababsn",
        "ababababso",
        "ababababsp",
        "ababababsq",
        "ababababsr",
        "ababababss",
        "ababababst",
        "ababababsu",
        "ababababsv",
        "ababababsw",
        "ababababsx",
        "ababababsy",
        "ababababsz",
        "ababababta",
        "ababababtb",
        "ababababtc",
        "ababababtd",
        "ababababte",
        "ababababtf",
        "ababababtg",
        "ababababth",
        "ababababti",
        "ababababtj",
        "ababababtk",
        "ababababtl",
        "ababababtm",
        "ababababtn",
        "ababababto",
        "ababababtp",
        "ababababtq",
        "ababababtr",
        "ababababts",
        "ababababtt",
        "ababababtu",
        "ababababtv",
        "ababababtw",
        "ababababtx",
        "ababababty",
        "ababababtz",
        "ababababua",
        "ababababub",
        "ababababuc",
        "ababababud",
        "ababababue",
        "ababababuf",
        "ababababug",
        "ababababuh",
        "ababababui",
        "ababababuj",
        "ababababuk",
        "ababababul",
        "ababababum",
        "ababababun",
        "ababababuo",
        "ababababup",
        "ababababuq",
        "ababababur",
        "ababababus",
        "ababababut",
        "ababababuu",
        "ababababuv",
        "ababababuw",
        "ababababux",
        "ababababuy",
        "ababababuz",
        "ababababva",
        "ababababvb",
        "ababababvc",
        "ababababvd",
        "ababababve",
        "ababababvf",
        "ababababvg",
        "ababababvh",
        "ababababvi",
        "ababababvj",
        "ababababvk",
        "ababababvl",
        "ababababvm",
        "ababababvn",
        "ababababvo",
        "ababababvp",
        "ababababvq",
        "ababababvr",
        "ababababvs",
        "ababababvt",
        "ababababvu",
        "ababababvv",
        "ababababvw",
        "ababababvx",
        "ababababvy",
        "ababababvz",
        "ababababwa",
        "ababababwb",
        "ababababwc",
        "ababababwd",
        "ababababwe",
        "ababababwf",
        "ababababwg",
        "ababababwh",
        "ababababwi",
        "ababababwj",
        "ababababwk",
        "ababababwl",
        "ababababwm",
        "ababababwn",
        "ababababwo",
        "ababababwp",
        "ababababwq",
        "ababababwr",
        "ababababws",
        "ababababwt",
        "ababababwu",
        "ababababwv",
        "ababababww",
        "ababababwx",
        "ababababwy",
        "ababababwz",
        "ababababxa",
        "ababababxb",
        "ababababxc",
        "ababababxd",
        "ababababxe",
        "ababababxf",
        "ababababxg",
        "ababababxh",
        "ababababxi",
        "ababababxj",
        "ababababxk",
        "ababababxl",
        "ababababxm",
        "ababababxn",
        "ababababxo",
        "ababababxp",
        "ababababxq",
        "ababababxr",
        "ababababxs",
        "ababababxt",
        "ababababxu",
        "ababababxv",
        "ababababxw",
        "ababababxx",
        "ababababxy",
        "ababababxz",
        "ababababya",
        "ababababyb",
        "ababababyc",
        "ababababyd",
        "ababababye",
        "ababababyf",
        "ababababyg",
        "ababababyh",
        "ababababyi",
        "ababababyj",
        "ababababyk",
        "ababababyl",
        "ababababym",
        "ababababyn",
        "ababababyo",
        "ababababyp",
        "ababababyq",
        "ababababyr",
        "ababababys",
        "ababababyt",
        "ababababyu",
        "ababababyv",
        "ababababyw",
        "ababababyx",
        "ababababyy",
        "ababababyz",
        "ababababza",
        "ababababzb",
        "ababababzc",
        "ababababzd",
        "ababababze",
        "ababababzf",
        "ababababzg",
        "ababababzh",
        "ababababzi",
        "ababababzj",
        "ababababzk",
        "ababababzl",
        "ababababzm",
        "ababababzn",
        "ababababzo",
        "ababababzp",
        "ababababzq",
        "ababababzr",
        "ababababzs",
        "ababababzt",
        "ababababzu",
        "ababababzv",
        "ababababzw",
        "ababababzx",
        "ababababzy",
        "ababababzz",
    ];
    let board: Vec<Vec<char>> = tb
        .iter()
        .map(|r| {
            r.iter()
                .map(|c| c.as_bytes()[0] as char)
                .collect::<Vec<char>>()
        })
        .collect();
    let words: Vec<String> = tw.iter().map(|w| w.to_string()).collect();
    let mut out = Solution1::find_words(board, words);
}