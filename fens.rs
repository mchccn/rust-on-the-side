#![allow(dead_code, unused)]

mod fens {
    use std::fmt;

    #[derive(Debug, Clone, Copy, Eq, PartialEq)]
    pub enum PieceColor {
        Black = 0x8,
        White = 0xf,
    }

    #[derive(Debug, Clone, Copy, Eq, PartialEq)]
    pub enum PieceKind {
        Pawn   = 0x1,
        Knight = 0x2,
        Bishop = 0x3 ,
        Rook   = 0x4,
        Queen  = 0x5,
        King   = 0x6,
    }

    #[derive(Debug, Clone, Copy, Eq, PartialEq)]
    pub struct Piece {
        pub color: PieceColor,
        pub kind: PieceKind,
        pub file: u8,
        pub rank: u8,
    }

    #[derive(Debug, Clone, Copy, Eq, PartialEq)]
    pub struct Move {
        pub file: u8,
        pub rank: u8,
    }

    impl Move {
        pub fn to_algebraic_notation(&self) -> String {
            format!("{}{}", char::from_u32((self.file + b'a').into()).unwrap(), self.rank + 1)
        }
    }

    #[derive(Debug, Clone, Copy, Eq, PartialEq)]
    pub struct BoardState {
        pub pieces: [Option<Piece>; 64],
        pub next: PieceColor,
        pub castlings: [bool; 4],
        pub enpassant: Option<Move>,
        pub halfmove: u8,
        pub fullmove: u8,
    }

    #[derive(Debug, Clone)]
    pub struct FenError(String);

    impl fmt::Display for FenError {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            write!(f, "fen parsing error: {}", self.0)
        }
    }

    impl fmt::Display for BoardState {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            let mut board = vec![];

            for rank in 0..8 {
                let mut row = vec![];

                for file in 0..8 {
                    let piece = self.pieces[rank * 8 + file];

                    row.push(
                        if piece.is_none() {
                            ' '
                        } else {
                            let piece = piece.unwrap();

                            match (piece.color, piece.kind) {
                                (PieceColor::White, PieceKind::Pawn) => 'P',
                                (PieceColor::White, PieceKind::Knight) => 'N',
                                (PieceColor::White, PieceKind::Bishop) => 'B',
                                (PieceColor::White, PieceKind::Rook) => 'R',
                                (PieceColor::White, PieceKind::Queen) => 'Q',
                                (PieceColor::White, PieceKind::King) => 'K',
                                (PieceColor::Black, PieceKind::Pawn) => 'p',
                                (PieceColor::Black, PieceKind::Knight) => 'n',
                                (PieceColor::Black, PieceKind::Bishop) => 'b',
                                (PieceColor::Black, PieceKind::Rook) => 'r',
                                (PieceColor::Black, PieceKind::Queen) => 'q',
                                (PieceColor::Black, PieceKind::King) => 'k',
                            }
                        }
                    );
                }

                board.push(row.iter().collect::<String>());
            }

            write!(f, "{}\n{} to mv\ncst {:>4}\nen p  {:>2}\nhlmv {:>3}\nflmv {:>3}", board.join("\n"),
                if self.next == PieceColor::White { "wh" } else { "bl" },
                self.castlings.iter().enumerate().map(|(i, x)| if *x { match i {
                    0 => 'K',
                    1 => 'Q',
                    2 => 'k',
                    3 => 'q',
                    _ => panic!(),
                } } else { ' ' }).collect::<String>(),
                if self.enpassant.is_none() { "-".to_string() } else { self.enpassant.unwrap().to_algebraic_notation() },
                self.halfmove,
                self.fullmove,
            )
        }
    }

    impl BoardState {
        fn char_to_piecekind(c: char) -> Result<PieceKind, FenError> {
            match c.to_ascii_lowercase() {
                'p' => Ok(PieceKind::Pawn),
                'n' => Ok(PieceKind::Knight),
                'b' => Ok(PieceKind::Bishop),
                'r' => Ok(PieceKind::Rook),
                'q' => Ok(PieceKind::Queen),
                'k' => Ok(PieceKind::King),
                _ => Err(FenError("unknown piece type".into())),
            }
        }

        pub fn from_fen(str: &str) -> Result<BoardState, FenError> {
            let mut sections = str.split(" ");

            let mut file: u8 = 0;
            let mut rank: u8 = 8 - 1;

            let pieces = sections.next().ok_or(FenError("expected pieces representation".into()))?.chars();

            let mut board = BoardState::empty();

            for symbol in pieces {
                if symbol == '/' {
                    file = 0;
                    rank -= 1;
                } else {
                    if symbol.is_ascii_digit() {
                        file += symbol.to_digit(10).ok_or(FenError("expected number of files to skip".into()))? as u8;
                    } else {
                        let color = if symbol.is_uppercase() { PieceColor::White } else { PieceColor::Black };
                        let kind = Self::char_to_piecekind(symbol)?;
                        
                        board.pieces[(rank * 8 + file) as usize] = Some(Piece { color, kind, file, rank });
                        
                        file += 1;
                    }
                }
            }

            let tomove = sections.next().ok_or(FenError("expected color to move".into()))?;

            let mut tomove = tomove.chars();

            board.next = if tomove.next().unwrap() == 'w' { PieceColor::White } else { PieceColor::Black };

            if tomove.next().is_some() {
                return Err(FenError("expected castlings".into()));
            }

            let castlings = sections.next().ok_or(FenError("expected castlings".into()))?;

            if castlings != "-" {
                let mut iter = castlings.bytes().peekable();

                for (i, x) in b"KQkq".iter().enumerate() {
                    match iter.peek() {
                        Some(y) if x == y => {
                            board.castlings[i] = true;

                            iter.next();
                        },
                        None => break,
                        _ => (),
                    }
                }

                if iter.next().is_some() {
                    return Err(FenError("castlings are not a subsequence".into()))
                }
            }

            let enpassant = sections.next().ok_or(FenError("expected en passant target".into()))?;

            if enpassant != "-" {
                let mut enpassant = enpassant.bytes();

                let file = enpassant.next().unwrap() as i16 - b'a' as i16;

                if file < 0 || file >= 8 {
                    return Err(FenError("file out of bounds".into()));
                }

                let file = file as u8;

                let rank = enpassant.next();

                if rank.is_none() {
                    return Err(FenError("expected rank after file".into()));
                }

                let rank = rank.unwrap() as i16 - b'1' as i16;

                if rank < 0 || rank >= 8 {
                    return Err(FenError("rank out of bounds".into()));
                }

                let rank = rank as u8;

                board.enpassant = Some(Move { file, rank });
            }

            let halfmove = sections.next().ok_or(FenError("expected halfmove counter".into()))?;

            let halfmove = halfmove.parse::<u8>();

            if halfmove.is_err() {
                return Err(FenError("unable to parse halfmove into number".into()));
            }

            board.halfmove = halfmove.unwrap();

            let fullmove = sections.next().ok_or(FenError("expected fullmove counter".into()))?;

            let fullmove = fullmove.parse::<u8>();

            if fullmove.is_err() {
                return Err(FenError("unable to parse fullmove into number".into()));
            }

            board.fullmove = fullmove.unwrap();

            if sections.next().is_some() {
                return Err(FenError("unexpected trailing section".into()));
            }

            Ok(board)
        }

        pub fn empty() -> Self {
            BoardState {
                pieces: [None; 64],
                next: PieceColor::White,
                castlings: [false; 4],
                enpassant: None,
                halfmove: 0,
                fullmove: 1,
            }
        }

        pub fn default() -> Self {
            BoardState::from_fen("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1").unwrap()
        }
    }
}

fn main() {
    use fens::*;

    let board = BoardState::from_fen("rnbqkbnr/pp1ppppp/8/2p5/4P3/5N2/PPPP1PPP/RNBQKB1R b KQkq h1 1 2");

    if board.is_ok() {
        println!("{}", board.unwrap());
    } else {
        println!("{:?}", board);
    }
}
