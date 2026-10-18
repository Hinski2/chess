use std::{io::{BufRead, BufReader, Write}, matches, process::{Child, ChildStdin, ChildStdout, Command, Stdio}};

pub(crate) struct StockfishHandler {
    elo: Option<u32>,
    stdin: ChildStdin,
    stdout: BufReader<ChildStdout>,
    child: Child,
}

impl StockfishHandler {
    pub(crate) fn new() -> std::io::Result<Self> {
        let mut child = Self::create_stockfish_process()?;
        let stdin = child.stdin.take().unwrap();
        let stdout = BufReader::new(child.stdout.take().unwrap());

        let mut handler = Self { elo: None, stdin, stdout, child };

        handler.init_stockfish()?;
        handler.set_ready()?;

        Ok(handler)
    }

    pub(crate) fn new_game(&mut self) -> std::io::Result<()> {
        self.send("ucinewgame")?;
        self.set_ready()?;

        Ok(())
    }

    pub(crate) fn get_move(&mut self, moves: &[String]) -> std::io::Result<String> {
        self.send(&format!("position startpos moves {}", moves.join(" ")))?;
        self.send("go movetime 100")?; // in ms

        let line = self.read_until_prefix("bestmove")?;
        Ok(line.split_whitespace().nth(1).unwrap().to_string())
    }

    pub(crate) fn set_elo(&mut self, elo: u32) -> std::io::Result<()> {
        self.elo = Some(elo);

        self.send("setoption name UCI_LimitStrength value true")?;
        self.send(&format!("setoption name UCI_Elo value {}", elo))?;

        Ok(())
    }

    pub(crate) fn unset_elo(&mut self) -> std::io::Result<()> {
        if matches!(self.elo, Some(_)) {
            self.send("setoption name UCI_LimitStrength value false")?;
            self.elo = None;
        }

        Ok(())
    }

    fn set_ready(&mut self) -> std::io::Result<()> {
        self.send("isready")?;
        self.read_until_prefix("readyok")?;

        Ok(())
    }

    fn init_stockfish(&mut self) -> std::io::Result<()> {
        self.send("uci")?;
        self.read_until_prefix("uciok")?;

        Ok(())
    }

    fn send(&mut self, msg: &str) -> std::io::Result<()> {
        writeln!(self.stdin, "{msg}")?;
        self.stdin.flush()
    }

    fn read_until_prefix(&mut self, prefix: &str) -> std::io::Result<String> {
        let mut line = String::new();
        loop {
            if self.stdout.read_line(&mut line)? == 0 {
                return Err(std::io::ErrorKind::UnexpectedEof.into())
            }

            if line.starts_with(prefix) {
                return Ok(line);
            } else {
                line.clear();
            }
        }
    }

    fn create_stockfish_process()  -> std::io::Result<Child> {
        Command::new("stockfish")
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .stderr(Stdio::null())
            .spawn()
    }
}

impl Drop for StockfishHandler {
    fn drop(&mut self) {
        let _ = self.send("quit");
        let _ = self.child.wait();
    }
}